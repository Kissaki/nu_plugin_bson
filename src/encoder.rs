use crate::BsonPlugin;
use bson::*;
use nu_plugin::{EngineInterface, EvaluatedCall, PluginCommand, SimplePluginCommand};
use nu_protocol::{Category, LabeledError, Signature, Value};

pub struct ToBson;

impl SimplePluginCommand for ToBson {
    type Plugin = BsonPlugin;

    fn name(&self) -> &str {
        "to bson"
    }

    fn description(&self) -> &str {
        "Convert BSON to table in a stream"
    }

    fn signature(&self) -> Signature {
        Signature::build(PluginCommand::name(self)).category(Category::Formats)
    }

    fn run(
        &self,
        _plugin: &BsonPlugin,
        _engine: &EngineInterface,
        call: &EvaluatedCall,
        //input: PipelineData,
        input: &Value,
        //) -> Result<PipelineData, LabeledError> {
    ) -> Result<Value, LabeledError> {
        let span = input.span();

        match input {
            Value::Bool { val, .. } => {
                let b: Bson = bson!(val);
                let x = Value::Bool {
                    val: b.as_bool().unwrap(),
                    internal_span: span,
                };
                Ok(x)
            }
            // Value::Record {
            //     val, ..
            // } => {
            //     let d = doc!(val);
            //     let x = Value
            //     Ok(x)
            // },
            _ => {
                let doc: Document = to_document(&input).unwrap();
                let bson: Bson = to_bson(&input).unwrap();
                let bin: Vec<u8> = bson::to_vec(&doc).unwrap();
                let binary: Value = Value::binary(bin, span);
                Ok(binary)
            }
        }
    }
}
