#[cfg(feature = "js")]
use rquickjs::{class::Trace, JsLifetime};

use core::fmt::Display;
use serde::{Deserialize, Serialize};

// Bidirectional :: Msg respoonse

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "js", derive(Trace, JsLifetime), rquickjs::class())]
pub struct Ack {
    pub id: u32,
    pub rx_id: u32,
    pub status: bool,
}

#[cfg(feature = "js")]
#[rquickjs::methods]
impl Ack {
    #[qjs(constructor)]
    pub fn new(id: u32, rx_id: u32, status: bool) -> rquickjs::Result<Self> {
        Ok(Self { id, rx_id, status })
    }
    pub fn debug(&self) -> String {
        format!("Ack: {:?}", self)
    }
}

impl Display for Ack {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "[{}] Ack: rx_id={} status={}",
            self.id, self.rx_id, self.status,
        )
    }
}

impl defmt::Format for Ack {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(
            fmt,
            "[{}] Ack: rx_id={} status={}",
            self.id,
            self.rx_id,
            self.status,
        )
    }
}
