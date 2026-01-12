#![cfg_attr(not(any(test, feature = "std")), no_std)]

mod format_mac;
mod rate;
mod view;

use format_mac::format_mac;
use view::display_vec;

use core::fmt::Display;
use serde::{Deserialize, Serialize};

pub const VERSION: u32 = 0;
pub const MAX_DATA_LEN: usize = 250;

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, defmt::Format)]
pub struct HubConfig {
    pub id: u32,
    pub channel: Option<u8>,
    pub pmk: Option<[u8; 16]>,
    pub wake_window: Option<u16>,
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

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, defmt::Format)]
pub struct RxData {
    pub id: u32,
    pub src_addr: [u8; 6],
    pub dst_addr: [u8; 6],
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

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, defmt::Format)]
pub struct TxData {
    pub id: u32,
    pub dst_addr: [u8; 6],
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

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, defmt::Format)]
pub struct BroadcastData {
    pub id: u32,
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

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, defmt::Format)]
pub struct PeerInfo {
    pub id: u32,
    pub peer_address: [u8; 6],
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

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, defmt::Format)]
pub struct Status {
    pub id: u32,
    pub status: bool,
}

impl Display for Status {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "[{}] Status: id={} status={}",
            self.id, self.id, self.status,
        )
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, defmt::Format)]
pub struct InitConfig {
    pub id: u32,
    pub api_version: u32,
    pub now_version: u32,
    pub channel: u8,
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

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, defmt::Format)]
pub struct PeerAddress([u8; 6]);

impl Display for PeerAddress {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", format_mac(&self.0),)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, defmt::Format)]
pub enum Msg {
    Init(InitConfig),
    HubConfig(HubConfig),
    Send(TxData),
    Recv(RxData),
    Broadcast(BroadcastData),
    AddPeer(PeerInfo),
    ModifyPeer(PeerInfo),
    RemovePeer(PeerAddress),
    Response(Status),
}

impl Display for Msg {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Msg::Init(m) => write!(f, "Msg -> {}", m),
            Msg::HubConfig(m) => write!(f, "Msg -> {}", m),
            Msg::Send(m) => write!(f, "Msg -> {}", m),
            Msg::Recv(m) => write!(f, "Msg -> {}", m),
            Msg::Broadcast(m) => write!(f, "Msg -> {}", m),
            Msg::AddPeer(m) => write!(f, "Msg -> {}", m),
            Msg::ModifyPeer(m) => write!(f, "Msg -> {}", m),
            Msg::RemovePeer(m) => write!(f, "Msg -> {}", m),
            Msg::Response(m) => write!(f, "Msg -> {}", m),
        }
    }
}

#[derive(Debug)]
pub enum MsgError {
    PostcardError,
    CapacityError,
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
            Msg::RemovePeer(PeerAddress([17_u8; 6])),
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
