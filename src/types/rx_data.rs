#[cfg(feature = "js")]
use rquickjs::{class::Trace, ArrayBuffer, Ctx, Exception, JsLifetime};

#[cfg(feature = "js")]
use crate::util::buf_to_array;

use crate::util::{display_vec, format_mac};
use crate::MAX_DATA_LEN;

use core::fmt::Display;
use serde::{Deserialize, Serialize};

// Hub -> Server :: RX esp-now msg

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "js", derive(Trace, JsLifetime), rquickjs::class())]
pub struct RxData {
    pub id: u32,
    #[cfg_attr(feature = "js", qjs(skip_trace))]
    pub src_addr: [u8; 6],
    #[cfg_attr(feature = "js", qjs(skip_trace))]
    pub dst_addr: [u8; 6],
    #[cfg_attr(feature = "js", qjs(skip_trace))]
    pub data: heapless::Vec<u8, MAX_DATA_LEN>,
    pub rssi: i32,
}

#[cfg(feature = "js")]
#[rquickjs::methods]
impl RxData {
    #[qjs(constructor)]
    pub fn new(
        ctx: Ctx<'_>,
        id: u32,
        src_addr: ArrayBuffer<'_>,
        dst_addr: ArrayBuffer<'_>,
        data: ArrayBuffer<'_>,
        rssi: i32,
    ) -> rquickjs::Result<Self> {
        Ok(Self {
            id,
            src_addr: buf_to_array::<6>(&ctx, &src_addr, "Invalid src_addr")?,
            dst_addr: buf_to_array::<6>(&ctx, &dst_addr, "Invalid dst_addr")?,
            data: heapless::Vec::from_slice(data.as_bytes().unwrap_or(&[]))
                .map_err(|_| Exception::throw_message(&ctx, "data invalid"))?,
            rssi,
        })
    }
    #[qjs(get, rename = "id")]
    pub fn get_id(&self) -> u32 {
        self.id
    }
    #[qjs(get, rename = "src_addr")]
    pub fn get_src_addr<'js>(&self, ctx: Ctx<'js>) -> rquickjs::Result<ArrayBuffer<'js>> {
        ArrayBuffer::new_copy(ctx, self.src_addr.as_slice())
    }
    #[qjs(get, rename = "dst_addr")]
    pub fn get_dst_addr<'js>(&self, ctx: Ctx<'js>) -> rquickjs::Result<ArrayBuffer<'js>> {
        ArrayBuffer::new_copy(ctx, self.dst_addr.as_slice())
    }
    #[qjs(get, rename = "data")]
    pub fn get_data<'js>(&self, ctx: Ctx<'js>) -> rquickjs::Result<ArrayBuffer<'js>> {
        ArrayBuffer::new_copy(ctx, self.data.as_slice())
    }
    #[qjs(get, rename = "rssi")]
    pub fn get_rssi(&self) -> i32 {
        self.rssi
    }
    pub fn debug(&self) -> String {
        format!("RxData: {:?}", self)
    }
}

impl Display for RxData {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "[{}] RxData: src={} dst={} rssi={} data=\"{}\"",
            self.id,
            format_mac(&self.src_addr),
            format_mac(&self.dst_addr),
            self.rssi,
            display_vec::<64, MAX_DATA_LEN>(&self.data)
        )
    }
}

impl defmt::Format for RxData {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(
            fmt,
            "[{}] RxData: src={} dst={} rssi={} data=\"{}\"",
            self.id,
            format_mac(&self.src_addr),
            format_mac(&self.dst_addr),
            self.rssi,
            display_vec::<64, MAX_DATA_LEN>(&self.data)
        )
    }
}
