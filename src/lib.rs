#![cfg_attr(not(test), no_std)]

mod rate;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct Config {
    id: u32,
    channel: Option<u8>,
    pmk: Option<[u8; 16]>,
    wake_window: Option<u16>,
    rate: Option<rate::WifiPhyRate>, // This is encoded as the u32 value
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct RxData {
    id: u32,
    src_addr: [u8; 6],
    dst_addr: [u8; 6],
    data: heapless::Vec<u8, 250>,
    rssi: u32,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct TxData {
    id: u32,
    dst_addr: [u8; 6],
    data: heapless::Vec<u8, 250>,
    defer: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct BroadcastData {
    id: u32,
    data: heapless::Vec<u8, 250>,
    interval: Option<u32>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct PeerInfo {
    id: u32,
    peer_address: [u8; 6],
    lmk: Option<[u8; 16]>,
    channel: Option<u8>,
    encrypt: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct Ack {
    id: u32,
    status: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct PeerAddress([u8; 6]);

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub enum Msg {
    HubConfig(Config),
    Send(TxData),
    Recv(TxData),
    Broadcast(BroadcastData),
    AddPeer(PeerInfo),
    ModifyPeer(PeerInfo),
    RemovePeer(PeerAddress),
    Ack(Ack),
}

pub const MAX_MSG_LENGTH: usize = 256;

#[derive(Debug)]
pub enum MsgError {
    PostcardError,
    CapacityError,
}

impl Msg {
    fn from_slice(buf: &[u8]) -> Result<Self, MsgError> {
        postcard::from_bytes::<Self>(buf).map_err(|_| MsgError::PostcardError)
    }
    fn to_slice<'a>(&self, buf: &'a mut [u8]) -> Result<&'a mut [u8], MsgError> {
        postcard::to_slice(&self, buf).map_err(|_| MsgError::PostcardError)
    }
    // Workaround to get heapless::Vec as postcard imports heapless 0.7.17 [vs 0.9.2]
    fn to_vec(&self) -> Result<heapless::Vec<u8, MAX_MSG_LENGTH>, MsgError> {
        let mut buf = [0_u8; MAX_MSG_LENGTH];
        let s = postcard::to_slice(&self, &mut buf).map_err(|_| MsgError::PostcardError)?;
        Ok(heapless::Vec::<u8, 256>::from_slice(&s).map_err(|_| MsgError::CapacityError)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_postcard() {
        let config = Config {
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
                data: heapless::Vec::<u8, 250>::from_slice(b"123456789").unwrap(),
                interval: Some(30),
            }),
        ] {
            let mut buf: [u8; 256] = [0; 256];
            let slice = msg.to_slice(&mut buf).unwrap();
            let vec = msg.to_vec().unwrap();
            println!("{:?}", slice);
            assert_eq!(&slice, &vec);
            assert_eq!(
                Msg::from_slice(slice).unwrap(),
                Msg::from_slice(&vec).unwrap()
            );
        }
    }
}
