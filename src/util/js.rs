#[cfg(feature = "js")]
use rquickjs::{ArrayBuffer, Ctx, Exception};

#[cfg(feature = "js")]
pub fn buf_to_array<const N: usize>(
    ctx: &Ctx<'_>,
    buf: &ArrayBuffer<'_>,
    e: &str,
) -> rquickjs::Result<[u8; N]> {
    match buf.as_bytes() {
        Some(b) => b.try_into().map_err(|_| Exception::throw_message(&ctx, e)),
        None => Err(Exception::throw_message(&ctx, e)),
    }
}

#[cfg(feature = "js")]
#[rquickjs::function]
pub fn parse_mac<'js>(ctx: Ctx<'js>, mac: String) -> rquickjs::Result<ArrayBuffer<'js>> {
    let mac = crate::util::parse_mac(mac.as_str())
        .map_err(|_| Exception::throw_message(&ctx, "Invalid MAC"))?;
    Ok(ArrayBuffer::new_copy(ctx, mac)?)
}

#[cfg(feature = "js")]
#[rquickjs::function]
pub fn format_mac<'js>(ctx: Ctx<'js>, mac: ArrayBuffer<'js>) -> rquickjs::Result<String> {
    Ok(crate::util::format_mac(&buf_to_array::<6>(&ctx, &mac, "Invalid MAC")?).to_string())
}
