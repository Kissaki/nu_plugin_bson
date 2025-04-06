use crate::nu_to_bson::convert_value;
use crate::BsonPlugin;
use bson::Bson;
use nu_plugin::{EngineInterface, EvaluatedCall, PluginCommand, SimplePluginCommand};
use nu_protocol::{Category, LabeledError, Signature, Span, Value};

pub struct ToBson;

impl SimplePluginCommand for ToBson {
    type Plugin = BsonPlugin;

    fn name(&self) -> &str {
        "to bson"
    }

    fn signature(&self) -> Signature {
        Signature::build(PluginCommand::name(self)).category(Category::Formats)
    }

    fn description(&self) -> &str {
        "Convert to BSON (binary data)"
    }

    fn run(
        &self,
        _plugin: &BsonPlugin,
        _engine: &EngineInterface,
        _call: &EvaluatedCall,
        input: &Value,
    ) -> Result<Value, LabeledError> {
        let span_unknown = Span::unknown();

        let a: Bson = convert_value(input);
        let b: Vec<u8> = bson::to_vec(&a.as_document().unwrap()).unwrap();
        let nu = Value::Binary {
            val: b,
            internal_span: span_unknown,
        };
        Ok(nu)
    }
}
