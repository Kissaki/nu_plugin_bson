
pub struct FromBson2;

impl PluginCommand for FromBson2 {
    type Plugin = BsonPlugin;

    fn name(&self) -> &str {
        "from bson2"
    }

    fn signature(&self) -> Signature {
        Signature::build(PluginCommand::name(self)).category(Category::Formats)
    }

    fn description(&self) -> &str {
        "Convert BSON to table in a stream"
    }

    fn run(
        &self,
        _plugin: &BsonPlugin,
        _engine: &EngineInterface,
        call: &EvaluatedCall,
        input: &PipelineData,
    ) -> Result<Value, LabeledError> {
        match input {
            PipelineData::Empty => Ok(Value::nothing(input.span().unwrap())),
            PipelineData::ByteStream(byte_stream, pipeline_metadata) => {
                let doc = Document::from_reader(byte_stream.into_bytes().unwrap().as_slice()).unwrap();

            },
            _ => Err(
                LabeledError::new("Can only parse binary data as BSON").with_label(
                    format!("requires binary input; got {}", input.get_type()),
                    call.head,
                ),
            ),
        }
    }
}
