#![cfg_attr(not(test), no_std)]

mod rate;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    channel: Option<u8>,
    pmk: Option<[u8; 16]>,
    wake_window: Option<u16>,
    rate: Option<rate::WifiPhyRate>, // This is encoded as the u32 value
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RxData {
    src_addr: [u8; 6],
    dst_addr: [u8; 6],
    data: heapless::Vec<u8, 250>,
    rssi: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TxData {
    dst_addr: [u8; 6],
    data: heapless::Vec<u8, 250>,
    defer: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BroadcastData {
    data: heapless::Vec<u8, 250>,
    interval: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PeerInfo {
    peer_address: [u8; 6],
    lmk: Option<[u8; 16]>,
    channel: Option<u8>,
    encrypt: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Msg {
    HubConfig(Config),
    Send(TxData),
    Recv(TxData),
    Broadcast(BroadcastData),
    AddPeer(PeerInfo),
    ModifyPeer(PeerInfo),
    RemovePeer([u8; 6]),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_postcard() {
        let config = Config {
            channel: Some(0),
            pmk: None,
            wake_window: None,
            rate: Some(rate::WifiPhyRate::RateMcs6Lgi),
        };
        let msg = Msg::HubConfig(config);
        let data = postcard::to_vec::<Msg, 256>(&msg);
        println!("{:?}", data);
        assert!(data.is_ok());
    }
}
