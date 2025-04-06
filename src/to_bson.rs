use crate::nu_to_bson::nu_value_to_bson;
use crate::BsonPlugin;
use bson::Bson;
use nu_plugin::{EngineInterface, EvaluatedCall, PluginCommand, SimplePluginCommand};
use nu_protocol::{Category, LabeledError, Signature, Span, Type, Value};

pub struct ToBson;

impl SimplePluginCommand for ToBson {
    type Plugin = BsonPlugin;

    fn name(&self) -> &str {
        "to bson"
    }

    fn signature(&self) -> Signature {
        Signature::build(PluginCommand::name(self))
            .category(Category::Formats)
            .input_output_type(Type::Any, Type::Binary)
            .input_output_type(Type::Bool, Type::Binary)
            .input_output_type(Type::Binary, Type::Binary)
            .input_output_type(Type::Float, Type::Binary)
            .input_output_type(Type::Int, Type::Binary)
            .input_output_type(Type::String, Type::Binary)
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
        let a: Bson = nu_value_to_bson(input);
        let b: Vec<u8> = bson::to_vec(&a.as_document().unwrap()).unwrap();
        let nu = Value::Binary {
            val: b,
            internal_span: Span::unknown(),
        };
        Ok(nu)
    }
}
