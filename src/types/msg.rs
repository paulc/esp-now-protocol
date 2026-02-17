#[cfg(feature = "js")]
use rquickjs::{class::Trace, Class, Ctx, Exception, JsLifetime, Value};

use crate::types::{
    Ack, BroadcastData, HubConfig, InitConfig, PeerAddress, PeerInfo, RxData, TxData,
};

use core::fmt::Display;
use serde::{Deserialize, Serialize};

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
    RemovePeer(PeerAddress),
    Ack(Ack),
}

/// Cast object as Msg class
#[cfg(feature = "js")]
macro_rules! extract_class {
    ($ctx:expr, $obj:expr, $variant:ident, $class:tt) => {
        if let Some(c) = Class::<$class>::from_object($obj) {
            Ok(Msg::$variant(c.borrow().clone()))
        } else {
            Err(Exception::throw_message(
                $ctx,
                format!("Expected {} instance", stringify!(ty)).as_str(),
            ))
        }
    };
}

#[cfg(feature = "js")]
#[rquickjs::methods]
impl Msg {
    // Single constructor with dynamic dispatch for each Msg type
    // >>> new Msg("Init",InitConfig) etc.
    #[qjs(constructor)]
    pub fn new(ctx: Ctx<'_>, msg_type: String, o: rquickjs::Object<'_>) -> rquickjs::Result<Self> {
        match msg_type.as_str() {
            "Init" => extract_class!(&ctx, &o, Init, InitConfig),
            "HubConfig" => extract_class!(&ctx, &o, HubConfig, HubConfig),
            "Send" => extract_class!(&ctx, &o, Send, TxData),
            "Recv" => extract_class!(&ctx, &o, Recv, RxData),
            "Broadcast" => extract_class!(&ctx, &o, Broadcast, BroadcastData),
            "AddPeer" => extract_class!(&ctx, &o, AddPeer, PeerInfo),
            "ModifyPeer" => extract_class!(&ctx, &o, ModifyPeer, PeerInfo),
            "RemovePeer" => extract_class!(&ctx, &o, RemovePeer, PeerAddress),
            "Ack" => extract_class!(&ctx, &o, Ack, Ack),
            _ => Err(Exception::throw_message(&ctx, "Invalid Msg type")),
        }
    }

    // Type safe constructors
    // >>> new Msg.Init(InitConfig) etc.
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
    pub fn new_remove_peer(peer: PeerAddress) -> Self {
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

    #[qjs(get, rename = "id")]
    pub fn get_id_js(&self) -> u32 {
        self.get_id()
    }

    #[qjs(get, rename = "msg")]
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

#[derive(Debug)]
pub enum MsgError {
    PostcardError,
    CapacityError,
}

/// Postcard serialisation helpers for Msg
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
