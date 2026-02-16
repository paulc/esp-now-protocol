#![cfg_attr(not(any(test, feature = "std")), no_std)]

pub mod monitor;
pub mod types;
pub mod util;

pub use types::*;
pub use util::{format_mac, parse_mac};

pub const VERSION: u32 = 0;
pub const MAX_DATA_LEN: usize = 250;

/// Register JS functions/classes
#[cfg(feature = "js")]
use rquickjs::Ctx;

#[cfg(feature = "js")]
pub fn register_espnow(ctx: &Ctx<'_>) -> anyhow::Result<()> {
    // Create mqtt object
    let espnow = rquickjs::Object::new(ctx.clone())?;
    // Register classes
    rquickjs::Class::<InitConfig>::define(&espnow)?;
    rquickjs::Class::<HubConfig>::define(&espnow)?;
    rquickjs::Class::<TxData>::define(&espnow)?;
    rquickjs::Class::<RxData>::define(&espnow)?;
    rquickjs::Class::<BroadcastData>::define(&espnow)?;
    rquickjs::Class::<PeerInfo>::define(&espnow)?;
    rquickjs::Class::<PeerAddress>::define(&espnow)?;
    rquickjs::Class::<Ack>::define(&espnow)?;
    rquickjs::Class::<Msg>::define(&espnow)?;
    espnow.set("parse_mac", crate::util::js_parse_mac)?;
    espnow.set("format_mac", crate::util::js_format_mac)?;
    ctx.globals().set("espnow", espnow)?;
    // Add parse_mac / format_mac prototype methods
    ctx.eval::<(),_>(r#"
        Object.defineProperty(String.prototype, "parse_mac", { value: function () { return espnow.parse_mac(this) }});
        Object.defineProperty(ArrayBuffer.prototype, "format_mac", { value: function() { return espnow.format_mac(this) }});
    "#)?;
    Ok(())
}
