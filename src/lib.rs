#![cfg_attr(not(any(test, feature = "std")), no_std)]

#[cfg(feature = "js")]
use rquickjs::{class::Trace, ArrayBuffer, Class, Ctx, Exception, JsLifetime, Value};

pub mod format_mac;
mod rate;
mod view;

use format_mac::{format_mac, from_mac};
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
        let rate: Option<rate::WifiPhyRate> = match rate {
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

#[cfg(feature = "js")]
#[rquickjs::methods]
impl PeerInfo {
    #[qjs(constructor)]
    pub fn new(
        ctx: Ctx<'_>,
        id: u32,
        peer_address: ArrayBuffer<'_>,
        lmk: Option<ArrayBuffer<'_>>,
        channel: Option<u8>,
        encrypt: bool,
    ) -> rquickjs::Result<Self> {
        let lmk: Option<[u8; 16]> = match lmk {
            Some(buf) => Some(buf_to_array::<16>(&ctx, &buf, "Invalid LMK")?),
            None => None,
        };
        Ok(Self {
            id,
            peer_address: buf_to_array::<6>(&ctx, &peer_address, "Invalid peer_addr")?,
            lmk,
            channel,
            encrypt,
        })
    }
    pub fn debug(&self) -> String {
        format!("PeerInfo: {:?}", self)
    }
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

#[cfg(feature = "js")]
#[rquickjs::methods]
impl InitConfig {
    #[qjs(constructor)]
    pub fn new(
        ctx: Ctx<'_>,
        id: u32,
        api_version: u32,
        now_version: u32,
        channel: u8,
        address: ArrayBuffer<'_>,
    ) -> rquickjs::Result<Self> {
        Ok(Self {
            id,
            api_version,
            now_version,
            channel,
            address: buf_to_array::<6>(&ctx, &address, "Invalid address")?,
        })
    }
    pub fn debug(&self) -> String {
        format!("InitConfig: {:?}", self)
    }
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

#[cfg(feature = "js")]
#[rquickjs::methods]
impl RemovePeer {
    #[qjs(constructor)]
    pub fn new(ctx: Ctx<'_>, id: u32, address: ArrayBuffer<'_>) -> rquickjs::Result<Self> {
        Ok(Self {
            id,
            address: buf_to_array::<6>(&ctx, &address, "Invalid address")?,
        })
    }
    pub fn debug(&self) -> String {
        format!("RemovePeer: {:?}", self)
    }
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

#[cfg(feature = "js")]
#[rquickjs::methods]
impl Msg {
    /*
        // Single constructor with dynamic dispatch for each Msg type

        macro_rules! extract_class {
            ($ctx:expr, $obj:expr, $variant:ident, $class:ty, $expect:literal) => {
                if let Some(c) = Class::<$class>::from_object($obj) {
                    Ok(Msg::$variant(c.borrow().clone()))
                } else {
                    Err(Exception::throw_message($ctx, $expect))
                }
            };
        }

        #[qjs(constructor)]
        pub fn new(ctx: Ctx<'_>, msg_type: String, o: rquickjs::Object<'_>) -> rquickjs::Result<Self> {
            match msg_type.as_str() {
                "Init" => extract_class!(&ctx, &o, Init, InitConfig, "Expected InitConfig instance"),
                "HubConfig" => extract_class!(
                    &ctx,
                    &o,
                    HubConfig,
                    HubConfig,
                    "Expected HubConfig instance"
                ),
                "Send" => extract_class!(&ctx, &o, Send, TxData, "Expected TxData instance"),
                "Recv" => extract_class!(&ctx, &o, Recv, RxData, "Expected RxData instance"),
                "Broadcast" => extract_class!(
                    &ctx,
                    &o,
                    Broadcast,
                    BroadcastData,
                    "Expected BroadcastData instance"
                ),
                "AddPeer" => extract_class!(&ctx, &o, AddPeer, PeerInfo, "Expected PeerInfo instance"),
                "ModifyPeer" => {
                    extract_class!(&ctx, &o, ModifyPeer, PeerInfo, "Expected PeerInfo instance")
                }
                "RemovePeer" => extract_class!(
                    &ctx,
                    &o,
                    RemovePeer,
                    RemovePeer,
                    "Expected RemovePeer instance"
                ),
                "Ack" => extract_class!(&ctx, &o, Ack, Ack, "Expected Ack instance"),
                _ => Err(Exception::throw_message(&ctx, "Invalid Msg type")),
            }
        }
    */

    // We need to have constructor to register class
    #[qjs(constructor)]
    pub fn new(ctx: Ctx<'_>) -> rquickjs::Result<Self> {
        Err(Exception::throw_message(
            &ctx,
            "Use static msg_type constructors (msg.T)",
        ))
    }

    #[qjs(static, rename = "Init")]
    pub fn new_init(ic: InitConfig) -> Self {
        Msg::Init(ic)
    }

    #[qjs(static, rename = "HubConfig")]
    pub fn new_hub_config(hc: HubConfig) -> Self {
        Msg::HubConfig(hc)
    }

    #[qjs(static, rename = "Send")]
    pub fn new_send(tx: TxData) -> Self {
        Msg::Send(tx)
    }

    #[qjs(static, rename = "Recv")]
    pub fn new_recv(rx: RxData) -> Self {
        Msg::Recv(rx)
    }

    #[qjs(static, rename = "Broadcast")]
    pub fn new_broadcast(broadcast: BroadcastData) -> Self {
        Msg::Broadcast(broadcast)
    }

    #[qjs(static, rename = "AddPeer")]
    pub fn new_add_peer(peer: PeerInfo) -> Self {
        Msg::AddPeer(peer)
    }

    #[qjs(static, rename = "ModifyPeer")]
    pub fn new_modify_peer(peer: PeerInfo) -> Self {
        Msg::ModifyPeer(peer)
    }

    #[qjs(static, rename = "RemovePeer")]
    pub fn new_remove_peer(peer: RemovePeer) -> Self {
        Msg::RemovePeer(peer)
    }

    #[qjs(static, rename = "Ack")]
    pub fn new_ack(ack: Ack) -> Self {
        Msg::Ack(ack)
    }

    #[qjs(get, rename = "type")]
    pub fn get_type(&self) -> String {
        match &self {
            Msg::Init(_) => "Init",
            Msg::HubConfig(_) => "HubConfig",
            Msg::Send(_) => "Send",
            Msg::Recv(_) => "Recv",
            Msg::Broadcast(_) => "Broadcast",
            Msg::AddPeer(_) => "AddPeer",
            Msg::ModifyPeer(_) => "ModifyPeer",
            Msg::RemovePeer(_) => "RemovePeer",
            Msg::Ack(_) => "Ack",
        }
        .to_string()
    }
    #[qjs(get, rename = "message")]
    pub fn get_message<'js>(&self, ctx: Ctx<'js>) -> rquickjs::Result<Value<'js>> {
        Ok(match &self {
            Msg::Init(m) => Class::instance(ctx, m.clone())?.into_value(),
            Msg::HubConfig(m) => Class::instance(ctx, m.clone())?.into_value(),
            Msg::Send(m) => Class::instance(ctx, m.clone())?.into_value(),
            Msg::Recv(m) => Class::instance(ctx, m.clone())?.into_value(),
            Msg::Broadcast(m) => Class::instance(ctx, m.clone())?.into_value(),
            Msg::AddPeer(m) => Class::instance(ctx, m.clone())?.into_value(),
            Msg::ModifyPeer(m) => Class::instance(ctx, m.clone())?.into_value(),
            Msg::RemovePeer(m) => Class::instance(ctx, m.clone())?.into_value(),
            Msg::Ack(m) => Class::instance(ctx, m.clone())?.into_value(),
        })
    }
    pub fn debug(&self) -> String {
        format!("Msg: {:?}", self)
    }
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

fn buf_to_array<const N: usize>(
    ctx: &Ctx<'_>,
    buf: &ArrayBuffer<'_>,
    e: &str,
) -> rquickjs::Result<[u8; N]> {
    match buf.as_bytes() {
        Some(b) => b.try_into().map_err(|_| Exception::throw_message(&ctx, e)),
        None => Err(Exception::throw_message(&ctx, e)),
    }
}

#[cfg(feature = "js")]
#[rquickjs::function]
pub fn parse_mac<'js>(ctx: Ctx<'js>, mac: String) -> rquickjs::Result<ArrayBuffer<'js>> {
    let mac = from_mac(mac.as_str()).map_err(|_| Exception::throw_message(&ctx, "Invalid MAC"))?;
    Ok(ArrayBuffer::new_copy(ctx, mac)?)
}

#[cfg(feature = "js")]
#[rquickjs::function]
pub fn format_mac_js<'js>(ctx: Ctx<'js>, mac: ArrayBuffer<'js>) -> rquickjs::Result<String> {
    Ok(format_mac(&buf_to_array::<6>(&ctx, &mac, "Invalid MAC")?).to_string())
}

/// Register JS functions/classes
#[cfg(feature = "js")]
pub fn register_espnow(ctx: &Ctx<'_>) -> anyhow::Result<()> {
    // Create mqtt object
    let espnow = rquickjs::Object::new(ctx.clone())?;
    // Register classes
    rquickjs::Class::<InitConfig>::define(&espnow)?;
    rquickjs::Class::<HubConfig>::define(&espnow)?;
    rquickjs::Class::<TxData>::define(&espnow)?;
    rquickjs::Class::<RxData>::define(&espnow)?;
    rquickjs::Class::<BroadcastData>::define(&espnow)?;
    rquickjs::Class::<PeerInfo>::define(&espnow)?;
    rquickjs::Class::<RemovePeer>::define(&espnow)?;
    rquickjs::Class::<Ack>::define(&espnow)?;
    rquickjs::Class::<Msg>::define(&espnow)?;
    espnow.set("parse_mac", js_parse_mac)?;
    espnow.set("format_mac", js_format_mac_js)?;
    ctx.globals().set("espnow", espnow)?;
    // Add parse_mac / format_mac prototype methods
    ctx.eval::<(),_>(r#"
        Object.defineProperty(String.prototype, "parse_mac", { value: function () { return espnow.parse_mac(this) }});
        Object.defineProperty(ArrayBuffer.prototype, "format_mac", { value: function() { return espnow.format_mac(this) }});
    "#)?;
    Ok(())
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
