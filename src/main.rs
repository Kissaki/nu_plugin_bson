use nu_plugin::{MsgPackSerializer, serve_plugin};
use nu_plugin_bson::BsonPlugin;

fn main() {
    serve_plugin(&BsonPlugin, MsgPackSerializer {})
}
