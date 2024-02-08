use xplm::debug;
use xplm::plugin::{Plugin, PluginInfo};

struct RwPlugin;

impl Plugin for RwPlugin {
    type Error = std::convert::Infallible;

    fn start() -> Result<Self, Self::Error> {
        debug("attempting to connect to cockpitd");
        Ok(RwPlugin)
    }

    fn info(&self) -> PluginInfo {
        PluginInfo {
            name: "RWCockpit Integration Plugin".into(),
            signature: "dev.coredoes.rwcockpit.xplane".into(),
            description: "Plugin to allow RWCockpit to properly integrate with X-Plane".into()
        }
    }
}