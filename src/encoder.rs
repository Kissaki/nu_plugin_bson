use crate::BsonPlugin;
use bson::Bson;
use nu_plugin::{EngineInterface, EvaluatedCall, PluginCommand, SimplePluginCommand};
use nu_protocol::{
    ByteStream, ByteStreamType, Category, IntoPipelineData, LabeledError, PipelineData, Signals,
    Signature, Span, SyntaxShape, Type, Value,
};

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
            // Value::String { val, .. } => Ok(
            //     Value::int(val.len() as i64, span)
            // ),
            Value::String { val, .. } => {
                let bson_string = Bson::String(val.to_string());
                let bin = bson::to_vec(&bson_string).unwrap();
                Ok(Value::binary(bin, span))
            },
            Value::Record { val, internal_span } => {
                let bson = Bson::Document(val);
                let bin = bson::to_vec(&bson).unwrap();
                Ok(Value::binary(bin, span))
            }
            // Value::String { val, .. } => {
            //     // Ok(Value::Binary(Bson::String(val.to_string())) { val: bsonString, internal_span: input.span() } { val: bson, span: *span })
            //     // let bson: Bson = val.into_bson()?;
            //     let bson_string = Bson::String(val.to_string());
            //     Ok(Value::binary(bson_string, input.span()))
            // },
            // Value::String { val, .. } => {
            //     // Ok(Value::Binary(Bson::String(val.to_string())) { val: bsonString, internal_span: input.span() } { val: bson, span: *span })
            //     // let bson: Bson = val.into_bson()?;
            //     let bson_string = Bson::String(val.to_string());
            //     Ok(Value::Binary(bson_string, input.span()))
            // },
            _ => Err(
                LabeledError::new("Expected String input from pipeline").with_label(
                    format!("requires string input; got {}", input.get_type()),
                    call.head,
                ),
            ),
        }
        // let bson: Bson = input.into_bson()?;
    }
}
