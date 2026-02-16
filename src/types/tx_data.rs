#[cfg(feature = "js")]
use rquickjs::{class::Trace, ArrayBuffer, Ctx, Exception, JsLifetime};

#[cfg(feature = "js")]
use crate::util::buf_to_array;

use crate::util::{display_vec, format_mac};
use crate::MAX_DATA_LEN;

use core::fmt::Display;
use serde::{Deserialize, Serialize};

// Server -> Hub :: TX esp-now msg

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "js", derive(Trace, JsLifetime), rquickjs::class())]
pub struct TxData {
    pub id: u32,
    #[cfg_attr(feature = "js", qjs(skip_trace))]
    pub dst_addr: [u8; 6],
    #[cfg_attr(feature = "js", qjs(skip_trace))]
    pub data: heapless::Vec<u8, MAX_DATA_LEN>,
    pub defer: bool,
}

#[cfg(feature = "js")]
#[rquickjs::methods]
impl TxData {
    #[qjs(constructor)]
    pub fn new(
        ctx: Ctx<'_>,
        id: u32,
        dst_addr: ArrayBuffer<'_>,
        data: ArrayBuffer<'_>,
        defer: bool,
    ) -> rquickjs::Result<Self> {
        Ok(Self {
            id,
            dst_addr: buf_to_array::<6>(&ctx, &dst_addr, "Invalid dst_addr")?,
            data: heapless::Vec::from_slice(data.as_bytes().unwrap_or(&[]))
                .map_err(|_| Exception::throw_message(&ctx, "data invalid"))?,
            defer,
        })
    }
    pub fn debug(&self) -> String {
        format!("TxData: {:?}", self)
    }
}

impl Display for TxData {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "[{}] TxData: dst={} defer={} data=\"{}\"",
            self.id,
            format_mac(&self.dst_addr),
            self.defer,
            display_vec::<64, MAX_DATA_LEN>(&self.data)
        )
    }
}

impl defmt::Format for TxData {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(
            fmt,
            "[{}] TxData: dst={} defer={} data=\"{}\"",
            self.id,
            format_mac(&self.dst_addr),
            self.defer,
            display_vec::<64, MAX_DATA_LEN>(&self.data)
        )
    }
}
