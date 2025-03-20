use nu_plugin::{Plugin, PluginCommand};
mod decoder;
mod encoder;
use decoder::FromBson;
use encoder::ToBson;

pub struct BsonPlugin;

impl Plugin for BsonPlugin {
    fn version(&self) -> String {
        env!("CARGO_PKG_VERSION").into()
    }

    fn commands(&self) -> Vec<Box<dyn PluginCommand<Plugin = Self>>> {
        vec![Box::new(FromBson), Box::new(ToBson)]
    }
}
