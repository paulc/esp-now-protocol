#[cfg(feature = "js")]
use rquickjs::{class::Trace, ArrayBuffer, Ctx, JsLifetime};

#[cfg(feature = "js")]
use crate::util::buf_to_array;

use crate::util::format_mac;

use core::fmt::Display;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "js", derive(Trace, JsLifetime), rquickjs::class())]
pub struct PeerInfo {
    pub id: u32,
    #[cfg_attr(feature = "js", qjs(skip_trace))]
    pub peer_address: [u8; 6],
    #[cfg_attr(feature = "js", qjs(skip_trace))]
    pub lmk: Option<[u8; 16]>,
    pub channel: Option<u8>,
    pub encrypt: bool,
}

#[cfg(feature = "js")]
#[rquickjs::methods]
impl PeerInfo {
    #[qjs(constructor)]
    pub fn new(
        ctx: Ctx<'_>,
        id: u32,
        peer_address: ArrayBuffer<'_>,
        lmk: Option<ArrayBuffer<'_>>,
        channel: Option<u8>,
        encrypt: bool,
    ) -> rquickjs::Result<Self> {
        let lmk: Option<[u8; 16]> = match lmk {
            Some(buf) => Some(buf_to_array::<16>(&ctx, &buf, "Invalid LMK")?),
            None => None,
        };
        Ok(Self {
            id,
            peer_address: buf_to_array::<6>(&ctx, &peer_address, "Invalid peer_addr")?,
            lmk,
            channel,
            encrypt,
        })
    }
    pub fn debug(&self) -> String {
        format!("PeerInfo: {:?}", self)
    }
}

impl Display for PeerInfo {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "[{}] PeerInfo: address={} lmk={:?} channel={:?} encrypt={}",
            self.id,
            format_mac(&self.peer_address),
            self.lmk,
            self.channel,
            self.encrypt,
        )
    }
}

impl defmt::Format for PeerInfo {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(
            fmt,
            "[{}] PeerInfo: address={} lmk={:?} channel={:?} encrypt={}",
            self.id,
            format_mac(&self.peer_address),
            self.lmk,
            self.channel,
            self.encrypt,
        )
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "js", derive(Trace, JsLifetime), rquickjs::class())]
pub struct PeerAddress {
    pub id: u32,
    #[cfg_attr(feature = "js", qjs(skip_trace))]
    pub address: [u8; 6],
}

#[cfg(feature = "js")]
#[rquickjs::methods]
impl PeerAddress {
    #[qjs(constructor)]
    pub fn new(ctx: Ctx<'_>, id: u32, address: ArrayBuffer<'_>) -> rquickjs::Result<Self> {
        Ok(Self {
            id,
            address: buf_to_array::<6>(&ctx, &address, "Invalid address")?,
        })
    }
    pub fn debug(&self) -> String {
        format!("PeerAddress: {:?}", self)
    }
}

impl Display for PeerAddress {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "[{}] PEER: address={}",
            self.id,
            format_mac(&self.address)
        )
    }
}

impl defmt::Format for PeerAddress {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(
            fmt,
            "[{}] PEER: address={}",
            self.id,
            format_mac(&self.address)
        )
    }
}
