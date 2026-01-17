use core::fmt::Write;

pub fn format_mac(mac: &[u8; 6]) -> heapless::String<17> {
    let mut s = heapless::String::<17>::new(); // "xx:xx:xx:xx:xx:xx"
    if *mac == [0xff, 0xff, 0xff, 0xff, 0xff, 0xff] {
        write!(&mut s, "<BROADCAST>").unwrap();
    } else {
        for (i, &byte) in mac.iter().enumerate() {
            if i > 0 {
                write!(&mut s, ":{:02x}", byte).unwrap();
            } else {
                write!(&mut s, "{:02x}", byte).unwrap();
            }
        }
    }
    s
}

pub fn from_mac(s: &str) -> Result<[u8; 6], ()> {
    if s == "<BROADCAST>" {
        return Ok([0xff, 0xff, 0xff, 0xff, 0xff, 0xff]);
    }

    // Count colons to verify format
    let mut colons = 0;
    for c in s.bytes() {
        if c == b':' {
            colons += 1;
        }
    }
    if colons != 5 {
        return Err(());
    }

    let mut mac = [0u8; 6];
    let mut part_start = 0;
    let mut part_index = 0;

    for (i, b) in s.bytes().enumerate() {
        if b == b':' {
            if i - part_start != 2 {
                return Err(());
            }
            mac[part_index] = parse_hex_byte(&s[part_start..i]).map_err(|_| ())?;
            part_start = i + 1;
            part_index += 1;
        }
    }

    // Handle last part
    if s.len() - part_start != 2 {
        return Err(());
    }
    mac[part_index] = parse_hex_byte(&s[part_start..]).map_err(|_| ())?;

    // Ensure we parsed exactly 6 parts
    if part_index != 5 {
        return Err(());
    }

    Ok(mac)
}

fn parse_hex_byte(s: &str) -> Result<u8, ()> {
    if s.len() != 2 {
        return Err(());
    }
    let bytes = s.as_bytes();
    let high = hex_digit_to_u8(bytes[0])?;
    let low = hex_digit_to_u8(bytes[1])?;
    Ok((high << 4) | low)
}

fn hex_digit_to_u8(b: u8) -> Result<u8, ()> {
    match b {
        b'0'..=b'9' => Ok(b - b'0'),
        b'a'..=b'f' => Ok(b - b'a' + 10),
        b'A'..=b'F' => Ok(b - b'A' + 10),
        _ => Err(()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_mac_broadcast() {
        let mac = [0xff, 0xff, 0xff, 0xff, 0xff, 0xff];
        assert_eq!(format_mac(&mac), "<BROADCAST>");
    }

    #[test]
    fn test_format_mac_normal() {
        let mac = [0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc];
        assert_eq!(format_mac(&mac), "12:34:56:78:9a:bc");
    }

    #[test]
    fn test_format_mac_zeros() {
        let mac = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        assert_eq!(format_mac(&mac), "00:00:00:00:00:00");
    }

    #[test]
    fn test_from_mac_valid() {
        let cases = &[
            ("<BROADCAST>", [0xff; 6]),
            ("12:34:56:78:9a:bc", [0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc]),
            ("12:34:56:78:9A:BC", [0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc]),
            ("Ab:CD:ef:01:23:45", [0xab, 0xcd, 0xef, 0x01, 0x23, 0x45]),
            ("00:00:00:00:00:00", [0x00; 6]),
            ("ff:ff:ff:ff:ff:ff", [0xff; 6]),
        ];

        for &(input, expected) in cases {
            assert_eq!(from_mac(input), Ok(expected), "failed to parse: {}", input);
        }
    }

    #[test]
    fn test_from_mac_invalid() {
        let cases = &[
            "broadcast",
            "<broadcast>",
            "BROADCAST",
            "12:34:56:78:9a",
            "12:34:56:78:9a:bc:de",
            "123:45:67:89:ab:cd",
            "1:34:56:78:9a:bc",
            "12:34:56:78:9g:bc",
            "12:34:5z:78:9a:bc",
            "123456789abc",
            "1234:5678:9abc",
            "",
        ];

        for &input in cases {
            assert!(from_mac(input).is_err(), "unexpectedly accepted: {}", input);
        }
    }
}
