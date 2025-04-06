use nu_plugin::{EngineInterface, EvaluatedCall, PluginCommand, SimplePluginCommand};
use nu_protocol::{
    Category, LabeledError,
    Signature, Value,
};

use crate::BsonPlugin;

pub struct FromBson;

impl SimplePluginCommand for FromBson {
    type Plugin = BsonPlugin;

    fn name(&self) -> &str {
        "from bson"
    }

    fn signature(&self) -> Signature {
        Signature::build(PluginCommand::name(self)).category(Category::Formats)
    }

    fn description(&self) -> &str {
        "Convert from BSON"
    }

    fn run(
        &self,
        _plugin: &BsonPlugin,
        _engine: &EngineInterface,
        call: &EvaluatedCall,
        input: &Value,
    ) -> Result<Value, LabeledError> {
        let span = input.span();

        match input {
            Value::Binary { val, .. } => {
                let a = crate::bson_to_nu::convert_binary(val);
                let b = Value::list(a, span);
                Ok(b)
            }
            _ => Err(
                LabeledError::new("Can only parse binary data as BSON").with_label(
                    format!("requires binary input; got {}", input.get_type()),
                    call.head,
                ),
            ),
        }
    }
}
