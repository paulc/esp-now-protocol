#[cfg(all(test, feature = "js"))]
mod js_test {

    use crate::*;

    use std::sync::atomic::{AtomicU32, Ordering};

    use rquickjs::{async_with, AsyncContext, AsyncRuntime, Class};
    use rquickjs_utils::run::run_script;
    use rquickjs_utils::utils::register_fns;

    static MSG_ID: AtomicU32 = AtomicU32::new(0);

    #[rquickjs::function]
    pub fn next_id() -> u32 {
        MSG_ID.fetch_add(1, Ordering::Relaxed)
    }

    /// Check output from JS matches class instance
    async fn check_class<T>(script: String, expected: T) -> anyhow::Result<bool>
    where
        T: for<'js> rquickjs::class::JsClass<'js>
            + for<'js> rquickjs::IntoJs<'js>
            + Clone
            + Send
            + PartialEq
            + Eq
            + 'static,
    {
        let rt = AsyncRuntime::new()?;
        let ctx = AsyncContext::full(&rt).await?;

        async_with!(ctx => |ctx| {
            register_fns(&ctx)?;
            register_espnow(&ctx)?;
            ctx.globals().set("next_id", js_next_id)?;
            // Make sure we can set Rust struct as JS obj
            ctx.globals().set("expected", expected.clone())?;
            let v = run_script(ctx.clone(), script).await?;
            // Get JS Object
            let o = v.as_object().ok_or_else(|| anyhow::anyhow!("<as_object>"))?;
            // Convert into Class
            let c = Class::<T>::from_object(o).ok_or_else(|| anyhow::anyhow!("<from_object>"))?;
            // Check match
            let eq = *c.borrow() == expected;
            Ok::<_,anyhow::Error>(eq)
        })
        .await
    }

    #[tokio::test]
    async fn test_recv() -> anyhow::Result<()> {
        assert_eq!(
            check_class(
                r#"
                    const m = espnow.Msg.Recv(
                        new espnow.RxData(
                            1234,
                            "01:02:03:04:05:06".parse_mac(),
                            "f1:f2:f3:f4:f5:f6".parse_mac(),
                            "HELLO".to_buffer(),
                            -5
                    ));
                    m
                "#
                .into(),
                Msg::Recv(RxData {
                    id: 1234,
                    src_addr: [0x01, 0x02, 0x03, 0x04, 0x05, 0x06],
                    dst_addr: [0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6],
                    data: heapless::Vec::from_slice("HELLO".as_bytes())?,
                    rssi: -5,
                })
            )
            .await?,
            true
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_send() -> anyhow::Result<()> {
        assert_eq!(
            check_class(
                r#"
                    const m = espnow.Msg.Send(
                        new espnow.TxData(
                            1234,
                            "f1:f2:f3:f4:f5:f6".parse_mac(),
                            "HELLO".to_buffer(),
                            false
                    ));
                    m
                "#
                .into(),
                Msg::Send(TxData {
                    id: 1234,
                    dst_addr: [0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6],
                    data: heapless::Vec::from_slice("HELLO".as_bytes())?,
                    defer: false
                })
            )
            .await?,
            true
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_broadcast() -> anyhow::Result<()> {
        assert_eq!(
            check_class(
                r#"
                    const m = espnow.Msg.Broadcast(
                        new espnow.BroadcastData(
                            1234,
                            "HELLO".to_buffer(),
                            10
                    ));
                    m
                "#
                .into(),
                Msg::Broadcast(BroadcastData {
                    id: 1234,
                    data: heapless::Vec::from_slice("HELLO".as_bytes())?,
                    interval: Some(10)
                })
            )
            .await?,
            true
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_ack() -> anyhow::Result<()> {
        assert_eq!(
            check_class(
                r#"
                    const m = espnow.Msg.Ack(
                        new espnow.Ack(
                            1234,
                            9876,
                            true
                    ));
                    m
                "#
                .into(),
                Msg::Ack(Ack {
                    id: 1234,
                    rx_id: 9876,
                    status: true
                })
            )
            .await?,
            true
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_add_peer() -> anyhow::Result<()> {
        assert_eq!(
            check_class(
                r#"
                    const m = espnow.Msg.AddPeer(
                        new espnow.PeerInfo(
                            1234,
                            "f1:f2:f3:f4:f5:f6".parse_mac(),
                            new ArrayBuffer(16),
                            5,
                            true
                    ));
                    m
                "#
                .into(),
                Msg::AddPeer(PeerInfo {
                    id: 1234,
                    peer_address: [0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6],
                    lmk: Some([0; 16]),
                    channel: Some(5),
                    encrypt: true
                })
            )
            .await?,
            true
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_modify_peer() -> anyhow::Result<()> {
        assert_eq!(
            check_class(
                r#"
                    const m = espnow.Msg.ModifyPeer(
                        new espnow.PeerInfo(
                            1234,
                            "f1:f2:f3:f4:f5:f6".parse_mac(),
                            new ArrayBuffer(16),
                            5,
                            true
                    ));
                    m
                "#
                .into(),
                Msg::ModifyPeer(PeerInfo {
                    id: 1234,
                    peer_address: [0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6],
                    lmk: Some([0; 16]),
                    channel: Some(5),
                    encrypt: true
                })
            )
            .await?,
            true
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_remove_peer() -> anyhow::Result<()> {
        assert_eq!(
            check_class(
                r#"
                    const m = espnow.Msg.RemovePeer(
                        new espnow.PeerAddress(
                            1234,
                            "f1:f2:f3:f4:f5:f6".parse_mac(),

                    ));
                    m
                "#
                .into(),
                Msg::RemovePeer(PeerAddress {
                    id: 1234,
                    address: [0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6],
                })
            )
            .await?,
            true
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_init_config() -> anyhow::Result<()> {
        assert_eq!(
            check_class(
                r#"
                    const m = espnow.Msg.Init(
                        new espnow.InitConfig(
                            1234,
                            0,
                            1,
                            2,
                            "f1:f2:f3:f4:f5:f6".parse_mac(),
                    ));
                    m
                "#
                .into(),
                Msg::Init(InitConfig {
                    id: 1234,
                    api_version: 0,
                    now_version: 1,
                    channel: 2,
                    address: [0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6],
                })
            )
            .await?,
            true
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_hub_config() -> anyhow::Result<()> {
        assert_eq!(
            check_class(
                r#"
                    const m = espnow.Msg.HubConfig(
                        new espnow.HubConfig(
                            1234,
                            1,
                            new ArrayBuffer(16),
                            2,
                            "Mcs0Sgi"
                    ));
                    m
                "#
                .into(),
                Msg::HubConfig(HubConfig {
                    id: 1234,
                    channel: Some(1),
                    pmk: Some([0; 16]),
                    wake_window: Some(2),
                    rate: Some(rate::WifiPhyRate::RateMcs0Sgi)
                })
            )
            .await?,
            true
        );
        Ok(())
    }
}
