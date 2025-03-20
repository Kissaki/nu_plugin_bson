use crate::BsonPlugin;
use bson::*;
use indexmap::IndexMap;
use nu_plugin::*;
use nu_plugin::{EngineInterface, EvaluatedCall, PluginCommand, SimplePluginCommand};
use nu_protocol::*;
use nu_protocol::{
    ByteStream, ByteStreamType, Category, IntoPipelineData, LabeledError, PipelineData, Signals,
    Signature, Span, SyntaxShape, Type, Value,
};

pub struct FromBson;

impl SimplePluginCommand for FromBson {
    type Plugin = BsonPlugin;

    fn name(&self) -> &str {
        "from bson"
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
        input: &Value,
    ) -> Result<Value, LabeledError> {
        let span = input.span();

        match input {
            // Value::Binary { val, internal_span } => {
            //     let bin_slice = from_slice(val).unwrap();
            //     let a: Document = bin_slice;
            //     //let bson: Document = Document::from_reader(&mut val.as_slice()).unwrap();
            //     //let r = Value::Record { val: a, internal_span: span, };
            //
            //     let m: IndexMap = a;
            //     let r = Value::record(m.into_iter().collect(), span);
            //     Ok(r)
            // }
            _ => Err(
                LabeledError::new("Can only parse binary data as BSON").with_label(
                    format!("requires binary input; got {}", input.get_type()),
                    call.head,
                ),
            ),
        }
    }
}
