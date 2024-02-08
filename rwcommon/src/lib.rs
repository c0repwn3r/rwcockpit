#[cfg_attr(feature = "serde_derive", derive(serde::Serialize, serde::Deserialize))]
pub struct RwCockpitConfig {
    pub xplane: XplaneConfig
}

#[cfg_attr(feature = "serde_derive", derive(serde::Serialize, serde::Deserialize))]
pub struct XplaneConfig {
    pub subscribe: Vec<XplaneSubscription>
}

#[cfg_attr(feature = "serde_derive", derive(serde::Serialize, serde::Deserialize))]
pub struct XplaneSubscription {
    pub dataref: String,
    pub writable: bool
}

#[tarpc::service]
pub trait CockpitDaemon {
    async fn get_xplane_subscriptions() -> Vec<XplaneSubscription>;
}