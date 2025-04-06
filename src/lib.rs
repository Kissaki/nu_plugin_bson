use nu_plugin::{Plugin, PluginCommand};
mod from_bson;
mod to_bson;
mod nu_to_bson;
mod bson_to_nu;

use from_bson::FromBson;
use to_bson::ToBson;

pub struct BsonPlugin;

impl Plugin for BsonPlugin {
    fn version(&self) -> String {
        env!("CARGO_PKG_VERSION").into()
    }

    fn commands(&self) -> Vec<Box<dyn PluginCommand<Plugin = Self>>> {
        vec![
            Box::new(FromBson),
            Box::new(ToBson),
        ]
    }
}
