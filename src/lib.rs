#![cfg_attr(not(any(test, feature = "std")), no_std)]

#[cfg(feature = "js")]
use rquickjs::{class::Trace, ArrayBuffer, Ctx, Exception, JsLifetime, Value};

pub mod format_mac;
mod rate;
mod view;

use format_mac::format_mac;
use view::display_vec;

use core::fmt::Display;
use serde::{Deserialize, Serialize};

pub const VERSION: u32 = 0;
pub const MAX_DATA_LEN: usize = 250;

// Server -> Hub :: Config

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "js", derive(Trace, JsLifetime), rquickjs::class())]
pub struct HubConfig {
    pub id: u32,
    pub channel: Option<u8>,
    #[cfg_attr(feature = "js", qjs(skip_trace))]
    pub pmk: Option<[u8; 16]>,
    pub wake_window: Option<u16>,
    #[cfg_attr(feature = "js", qjs(skip_trace))]
    pub rate: Option<rate::WifiPhyRate>, // This is encoded as the u32 value
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

// Server -> Hub :: Broadcast esp-now msg

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "js", derive(Trace, JsLifetime), rquickjs::class())]
pub struct BroadcastData {
    pub id: u32,
    #[cfg_attr(feature = "js", qjs(skip_trace))]
    pub data: heapless::Vec<u8, MAX_DATA_LEN>,
    pub interval: Option<u32>,
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

// Bidirectional :: Msg respoonse

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "js", derive(Trace, JsLifetime), rquickjs::class())]
pub struct Ack {
    pub id: u32,
    pub rx_id: u32,
    pub status: bool,
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

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "js", derive(Trace, JsLifetime), rquickjs::class())]
pub struct RemovePeer {
    pub id: u32,
    #[cfg_attr(feature = "js", qjs(skip_trace))]
    pub address: [u8; 6],
}

impl Display for RemovePeer {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "[{}] REMOVE_PEER: address={}",
            self.id,
            format_mac(&self.address)
        )
    }
}

impl defmt::Format for RemovePeer {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(
            fmt,
            "[{}] REMOVE_PEER: address={}",
            self.id,
            format_mac(&self.address)
        )
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "js", derive(Trace, JsLifetime), rquickjs::class())]
pub enum Msg {
    Init(InitConfig),
    HubConfig(HubConfig),
    Send(TxData),
    Recv(RxData),
    Broadcast(BroadcastData),
    AddPeer(PeerInfo),
    ModifyPeer(PeerInfo),
    RemovePeer(RemovePeer),
    Ack(Ack),
}

impl Msg {
    pub fn get_id(&self) -> u32 {
        match self {
            Msg::Init(m) => m.id,
            Msg::HubConfig(m) => m.id,
            Msg::Send(m) => m.id,
            Msg::Recv(m) => m.id,
            Msg::Broadcast(m) => m.id,
            Msg::AddPeer(m) => m.id,
            Msg::ModifyPeer(m) => m.id,
            Msg::RemovePeer(m) => m.id,
            Msg::Ack(m) => m.id,
        }
    }
}

impl Display for Msg {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Msg::Init(m) => write!(f, "{}", m),
            Msg::HubConfig(m) => write!(f, "{}", m),
            Msg::Send(m) => write!(f, "{}", m),
            Msg::Recv(m) => write!(f, "{}", m),
            Msg::Broadcast(m) => write!(f, "{}", m),
            Msg::AddPeer(m) => write!(f, "{}", m),
            Msg::ModifyPeer(m) => write!(f, "{}", m),
            Msg::RemovePeer(m) => write!(f, "{}", m),
            Msg::Ack(m) => write!(f, "{}", m),
        }
    }
}

impl defmt::Format for Msg {
    fn format(&self, fmt: defmt::Formatter) {
        match self {
            Msg::Init(m) => defmt::write!(fmt, "{}", m),
            Msg::HubConfig(m) => defmt::write!(fmt, "{}", m),
            Msg::Send(m) => defmt::write!(fmt, "{}", m),
            Msg::Recv(m) => defmt::write!(fmt, "{}", m),
            Msg::Broadcast(m) => defmt::write!(fmt, "{}", m),
            Msg::AddPeer(m) => defmt::write!(fmt, "{}", m),
            Msg::ModifyPeer(m) => defmt::write!(fmt, "{}", m),
            Msg::RemovePeer(m) => defmt::write!(fmt, "{}", m),
            Msg::Ack(m) => defmt::write!(fmt, "{}", m),
        }
    }
}

#[derive(Debug)]
pub enum MsgError {
    PostcardError,
    CapacityError,
}

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

impl Msg {
    pub fn from_slice(buf: &[u8]) -> Result<Self, MsgError> {
        postcard::from_bytes::<Self>(buf).map_err(|_| MsgError::PostcardError)
    }
    pub fn to_slice<'a>(&self, buf: &'a mut [u8]) -> Result<&'a mut [u8], MsgError> {
        postcard::to_slice(&self, buf).map_err(|_| MsgError::PostcardError)
    }
    // Workaround to get heapless::Vec as postcard imports heapless 0.7.17 [vs 0.9.2]
    pub fn to_heapless<const N: usize>(&self) -> Result<heapless::Vec<u8, N>, MsgError> {
        let mut buf = [0_u8; N];
        let s = postcard::to_slice(&self, &mut buf).map_err(|_| MsgError::PostcardError)?;
        Ok(heapless::Vec::<u8, N>::from_slice(&s).map_err(|_| MsgError::CapacityError)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_postcard() {
        let config = HubConfig {
            id: 1234,
            channel: Some(0),
            pmk: None,
            wake_window: None,
            rate: Some(rate::WifiPhyRate::RateMcs6Lgi),
        };
        for msg in &[
            Msg::HubConfig(config),
            Msg::RemovePeer(RemovePeer {
                id: 9999,
                address: [17_u8; 6],
            }),
            Msg::Broadcast(BroadcastData {
                id: 9876,
                data: heapless::Vec::<u8, MAX_DATA_LEN>::from_slice(b"123456789").unwrap(),
                interval: Some(30),
            }),
        ] {
            let mut buf: [u8; 256] = [0; 256];
            let slice = msg.to_slice(&mut buf).unwrap();
            let vec: heapless::Vec<u8, 256> = msg.to_heapless().unwrap();
            println!("{:?}", slice);
            assert_eq!(&slice, &vec);
            assert_eq!(
                Msg::from_slice(slice).unwrap(),
                Msg::from_slice(&vec).unwrap()
            );
        }
    }
}
