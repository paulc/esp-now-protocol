use core::fmt::Display;
use serde::{Deserialize, Serialize};

use crate::Msg;

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Debug)]
pub enum Monitor {
    Tx(Msg),
    Rx(Msg),
    RxError,
    TxError,
}

impl Monitor {
    pub fn new_tx(msg: &Msg) -> Self {
        Self::Tx(msg.clone())
    }
    pub fn new_rx(msg: &Msg) -> Self {
        Self::Rx(msg.clone())
    }
    pub fn new_rxerror() -> Self {
        Self::RxError
    }
    pub fn new_txerror() -> Self {
        Self::TxError
    }
}

impl Display for Monitor {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Monitor::Tx(m) => write!(f, "<TX> {}", m),
            Monitor::Rx(m) => write!(f, "<RX> {}", m),
            Monitor::TxError => write!(f, "TX ERROR"),
            Monitor::RxError => write!(f, "RX ERROR"),
        }
    }
}

impl defmt::Format for Monitor {
    fn format(&self, fmt: defmt::Formatter) {
        match self {
            Monitor::Tx(m) => defmt::write!(fmt, "<TX> {}", m),
            Monitor::Rx(m) => defmt::write!(fmt, "<RX> {}", m),
            Monitor::TxError => defmt::write!(fmt, "TX ERROR"),
            Monitor::RxError => defmt::write!(fmt, "RX ERROR"),
        }
    }
}
