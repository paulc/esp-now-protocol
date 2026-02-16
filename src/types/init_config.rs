#[cfg(feature = "js")]
use rquickjs::{class::Trace, ArrayBuffer, Ctx, JsLifetime};

#[cfg(feature = "js")]
use crate::util::buf_to_array;

use crate::util::format_mac;

use core::fmt::Display;
use serde::{Deserialize, Serialize};

// Hub -> Server :: Init

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "js", derive(Trace, JsLifetime), rquickjs::class())]
pub struct InitConfig {
    pub id: u32,
    pub api_version: u32,
    pub now_version: u32,
    pub channel: u8,
    #[cfg_attr(feature = "js", qjs(skip_trace))]
    pub address: [u8; 6],
}

#[cfg(feature = "js")]
#[rquickjs::methods]
impl InitConfig {
    #[qjs(constructor)]
    pub fn new(
        ctx: Ctx<'_>,
        id: u32,
        api_version: u32,
        now_version: u32,
        channel: u8,
        address: ArrayBuffer<'_>,
    ) -> rquickjs::Result<Self> {
        Ok(Self {
            id,
            api_version,
            now_version,
            channel,
            address: buf_to_array::<6>(&ctx, &address, "Invalid address")?,
        })
    }
    pub fn debug(&self) -> String {
        format!("InitConfig: {:?}", self)
    }
}

impl Display for InitConfig {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "[{}] INIT: address={} channel={} api_version={} now_version={}",
            self.id,
            format_mac(&self.address),
            self.channel,
            self.api_version,
            self.now_version
        )
    }
}

impl defmt::Format for InitConfig {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(
            fmt,
            "[{}] INIT: address={} channel={} api_version={} now_version={}",
            self.id,
            format_mac(&self.address),
            self.channel,
            self.api_version,
            self.now_version
        )
    }
}
