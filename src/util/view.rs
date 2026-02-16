use core::fmt::Write;

pub fn display_vec<const N: usize, const M: usize>(
    data: &heapless::Vec<u8, M>,
) -> heapless::String<N> {
    let mut output = heapless::String::<N>::new();

    // First, try full UTF-8 decode
    if let Ok(s) = core::str::from_utf8(data) {
        if s.len() <= N {
            output.push_str(s).unwrap_or(()); // SAFE
            return output;
        } else {
            // Truncate at char boundary
            let max_len = N.saturating_sub(3);
            let mut end = 0;
            for (i, _) in s.char_indices() {
                if i > max_len {
                    break;
                }
                end = i;
            }
            if end > 0 {
                output.push_str(&s[..end]).unwrap_or(()); // SAFE
                output.push_str("...").unwrap_or(()); // SAFE
                return output;
            }
            // else: can't fit even one char → fall through to hex
        }
    }

    // Hexdump fallback (need at least 3 bytes)
    if N >= 3 {
        let hex_len = (N - 3) / 3; // each byte needs 2 chars + 1 space + save 3 bytes for ...
        let show_bytes = core::cmp::min(data.len(), hex_len);
        let will_truncate = data.len() > hex_len;

        for &b in &data[..show_bytes] {
            write!(output, "{:02x} ", b).unwrap_or(());
        }

        // Remove trailing space if any
        if !output.is_empty() {
            output.pop();
        }

        // Append ellipsis if needed and there's room
        if will_truncate && output.len() != 0 && output.len() + 3 <= N {
            output.push_str("...").unwrap_or(());
        }
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA_LEN: usize = 256;

    #[test]
    fn test_valid_utf8_fits() {
        let mut data = heapless::Vec::<u8, DATA_LEN>::new();
        data.extend_from_slice(b"hello").unwrap();
        let s: heapless::String<16> = display_vec(&data);
        assert_eq!(s, "hello");
    }

    #[test]
    fn test_valid_utf8_too_long() {
        let mut data = heapless::Vec::<u8, DATA_LEN>::new();
        data.extend_from_slice(b"hello world, this is a long string")
            .unwrap();
        let s: heapless::String<20> = display_vec(&data);
        // Should truncate to 17 bytes + "..."
        let expected_prefix = &b"hello world, this"[..]; // 17 bytes
        let expected_str = core::str::from_utf8(expected_prefix).unwrap();
        assert_eq!(s.as_str(), format!("{}...", expected_str));
    }

    #[test]
    fn test_invalid_utf8_short() {
        let mut data = heapless::Vec::<u8, DATA_LEN>::new();
        data.extend_from_slice(&[0xFF, 0xFE, 0xFD]).unwrap(); // invalid UTF-8
        let s: heapless::String<16> = display_vec(&data);
        assert_eq!(s, "ff fe fd");
    }

    #[test]
    fn test_invalid_utf8_long_hex_truncated() {
        let mut data = heapless::Vec::<u8, DATA_LEN>::new();
        // Fill with 100 invalid bytes
        for _ in 0..100 {
            data.push(0xFF).unwrap();
        }
        // N=12 → max hex bytes = 12 / 3 = 4 → "ff ff ff ff" (11 chars)
        let s: heapless::String<16> = display_vec(&data);
        assert_eq!(s, "ff ff ff ff...");
    }

    #[test]
    fn test_invalid_utf8_no_room_for_ellipsis() {
        let mut data = heapless::Vec::<u8, DATA_LEN>::new();
        data.extend_from_slice(&[0xFF, 0xFE, 0xFD, 0xFC]).unwrap();
        // N=8 → max hex bytes = 8/3 = 2 → "ff fe" (5 chars), room for "..."? 5+3=8 → yes
        let s: heapless::String<8> = display_vec(&data);
        assert_eq!(s, "ff...");
    }

    #[test]
    fn test_invalid_utf8_tiny_buffer() {
        let mut data = heapless::Vec::<u8, DATA_LEN>::new();
        data.extend_from_slice(&[0xFF, 0xFE]).unwrap();
        // N=2 → not enough for even one byte hex + space; max_bytes = 0
        let s: heapless::String<2> = display_vec(&data);
        assert_eq!(s, "");
    }

    #[test]
    fn test_valid_utf8_exactly_full() {
        let mut data = heapless::Vec::<u8, DATA_LEN>::new();
        data.extend_from_slice(b"1234567890").unwrap(); // 10 bytes
        let s: heapless::String<10> = display_vec(&data);
        assert_eq!(s, "1234567890");
    }

    #[test]
    fn test_valid_utf8_one_over_limit() {
        let mut data = heapless::Vec::<u8, DATA_LEN>::new();
        data.extend_from_slice(b"12345678901").unwrap(); // 11 bytes
        let s: heapless::String<10> = display_vec(&data);
        // Truncate to 7 bytes + "..." = 10
        assert_eq!(s, "1234567...");
    }

    #[test]
    fn test_multibyte_utf8() {
        let mut data = heapless::Vec::<u8, DATA_LEN>::new();
        data.extend_from_slice("café".as_bytes()).unwrap(); // 5 bytes (é = 2 bytes)
        let s: heapless::String<10> = display_vec(&data);
        assert_eq!(s, "café");
    }

    #[test]
    fn test_multibyte_utf8_truncate() {
        let mut data = heapless::Vec::<u8, DATA_LEN>::new();
        data.extend_from_slice("éééééééé".as_bytes()).unwrap(); // 16 bytes (é = 2 bytes)
        let s: heapless::String<12> = display_vec(&data);
        assert_eq!(s, "éééé...");
    }
}
