use crate::BsonPlugin;
use bson::Bson;
use nu_plugin::{EngineInterface, EvaluatedCall, PluginCommand, SimplePluginCommand};
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
        //input: PipelineData,
        input: &Value,
        //) -> Result<PipelineData, LabeledError> {
    ) -> Result<Value, LabeledError> {
        let span = input.span();
        match input {
            Value::Binary { val, internal_span } => Ok(Value::string("aha!", span)),
            _ => Err(LabeledError::new("oh no!").with_label(
                format!("requires binary input; got {}", input.get_type()),
                call.head,
            )),
        }
        // let bson: Bson = input.into_bson()?;
    }
}
