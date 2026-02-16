#[cfg(feature = "js")]
use rquickjs::{class::Trace, ArrayBuffer, Ctx, Exception, JsLifetime};

#[cfg(feature = "js")]
use crate::util::buf_to_array;

use crate::types::rate::WifiPhyRate;

use core::fmt::Display;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "js", derive(Trace, JsLifetime), rquickjs::class())]
pub struct HubConfig {
    pub id: u32,
    pub channel: Option<u8>,
    #[cfg_attr(feature = "js", qjs(skip_trace))]
    pub pmk: Option<[u8; 16]>,
    pub wake_window: Option<u16>,
    #[cfg_attr(feature = "js", qjs(skip_trace))]
    pub rate: Option<WifiPhyRate>, // This is encoded as the u32 value
}

#[cfg(feature = "js")]
#[rquickjs::methods]
impl HubConfig {
    #[qjs(constructor)]
    pub fn new(
        ctx: Ctx<'_>,
        id: u32,
        channel: Option<u8>,
        pmk: Option<ArrayBuffer<'_>>,
        wake_window: Option<u16>,
        rate: Option<String>,
    ) -> rquickjs::Result<Self> {
        let pmk: Option<[u8; 16]> = match pmk {
            Some(buf) => Some(buf_to_array::<16>(&ctx, &buf, "Invalid PMK")?),
            None => None,
        };
        let rate: Option<WifiPhyRate> = match rate {
            Some(s) => Some(
                (s.as_str())
                    .try_into()
                    .map_err(|_| Exception::throw_message(&ctx, "Invalid WifiPhyRate"))?,
            ),
            None => None,
        };
        Ok(Self {
            id,
            channel,
            pmk,
            wake_window,
            rate,
        })
    }
    pub fn debug(&self) -> String {
        format!("HubConfig: {:?}", self)
    }
}

impl Display for HubConfig {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "[{}] HubConfig: channel={:?} pmk={:?} wake_window={:?} rate={:?}",
            self.id, self.channel, self.pmk, self.wake_window, self.rate
        )
    }
}

impl defmt::Format for HubConfig {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(
            fmt,
            "[{}] HubConfig: channel={:?} pmk={:?} wake_window={:?} rate={:?}",
            self.id,
            self.channel,
            self.pmk,
            self.wake_window,
            self.rate
        )
    }
}
