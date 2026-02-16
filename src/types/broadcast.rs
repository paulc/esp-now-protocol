#[cfg(feature = "js")]
use rquickjs::{class::Trace, ArrayBuffer, Ctx, Exception, JsLifetime};

use crate::util::display_vec;
use crate::MAX_DATA_LEN;

use core::fmt::Display;
use serde::{Deserialize, Serialize};

// Server -> Hub :: Broadcast esp-now msg

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "js", derive(Trace, JsLifetime), rquickjs::class())]
pub struct BroadcastData {
    pub id: u32,
    #[cfg_attr(feature = "js", qjs(skip_trace))]
    pub data: heapless::Vec<u8, MAX_DATA_LEN>,
    pub interval: Option<u32>,
}

#[cfg(feature = "js")]
#[rquickjs::methods]
impl BroadcastData {
    #[qjs(constructor)]
    pub fn new(
        ctx: Ctx<'_>,
        id: u32,
        data: ArrayBuffer<'_>,
        interval: Option<u32>,
    ) -> rquickjs::Result<Self> {
        Ok(Self {
            id,
            data: heapless::Vec::from_slice(data.as_bytes().unwrap_or(&[]))
                .map_err(|_| Exception::throw_message(&ctx, "data invalid"))?,
            interval,
        })
    }
    pub fn debug(&self) -> String {
        format!("BroadcastData: {:?}", self)
    }
}

impl Display for BroadcastData {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "[{}] BroadcastData: interval={:?} data=\"{}\"",
            self.id,
            self.interval,
            display_vec::<64, MAX_DATA_LEN>(&self.data)
        )
    }
}

impl defmt::Format for BroadcastData {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(
            fmt,
            "[{}] BroadcastData: interval={:?} data=\"{}\"",
            self.id,
            self.interval,
            display_vec::<64, MAX_DATA_LEN>(&self.data)
        )
    }
}
