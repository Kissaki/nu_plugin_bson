
pub struct ToBson2;

impl PluginCommand for ToBson2 {
    type Plugin = BsonPlugin;

    fn name(&self) -> &str {
        "to bson2"
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
        input: &PipelineData,
    ) -> Result<PipelineData, LabeledError> {
        match input {
            PipelineData::Empty => Ok(PipelineData::Empty),
            PipelineData::Value(value, pipeline_metadata) => {
                let encoded = encode_value(value);
                Ok(encoded)
            },
            PipelineData::ListStream(list_stream, pipeline_metadata) => {
                // Ok(list_stream.map(|value| {
                //     let doc = doc!(value);
                //     let b = bson::to_vec(doc);
                //     Value::binary(b, b.into_spanned(span))
                // }))
                todo!()
            },
            PipelineData::ByteStream(byte_stream, pipeline_metadata) => {
                // let reader = byte_stream.reader().unwrap();
                // let x = bson::Document::from_reader(reader);
                // let doc = x.unwrap();
                // bson::to_vec(x);
                todo!()
            },
        }
        // bson::Document::from_reader(&mut input.as_slice()).unwrap();
        // let writer =
        // Ok()
    }
}
