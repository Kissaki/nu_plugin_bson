use nu_plugin::{Plugin, PluginCommand};
mod from_bson;
mod from_bson2;
mod to_bson;
mod to_bson2;
mod nu_to_bson;
mod bson_to_nu;

use from_bson::FromBson;
use to_bson::ToBson;
use crate::from_bson2::FromBson2;
use crate::to_bson2::ToBson2;

pub struct BsonPlugin;

impl Plugin for BsonPlugin {
    fn version(&self) -> String {
        env!("CARGO_PKG_VERSION").into()
    }

    fn commands(&self) -> Vec<Box<dyn PluginCommand<Plugin = Self>>> {
        vec![
            Box::new(FromBson),
            Box::new(FromBson2),
            Box::new(ToBson),
            Box::new(ToBson2),
        ]
    }
}
