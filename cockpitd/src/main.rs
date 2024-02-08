use std::fs;
use tarpc::context::Context;
use rwcommon::{CockpitDaemon, RwCockpitConfig, XplaneSubscription};

#[derive(Clone)]
struct CockpitDaemonServer;

impl CockpitDaemon for CockpitDaemonServer {
    async fn get_xplane_subscriptions(self, _: Context) -> Vec<XplaneSubscription> {
        todo!()
    }
}

fn main() {
    let file = std::env::args().nth(1).unwrap();

    println!("cockpitd - loading config from {file}");

    let f_str = fs::read_to_string(file).unwrap();
    let config: RwCockpitConfig = toml::from_str(&f_str).unwrap();
}
