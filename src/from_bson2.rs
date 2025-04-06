use crate::bson_to_nu::convert_document2;
use crate::BsonPlugin;
use bson::Document;
use nu_plugin::{EngineInterface, EvaluatedCall, PluginCommand};
use nu_protocol::{Category, LabeledError, ListStream, PipelineData, Reader, Signals, Signature, Span, Type, Value};
use std::io::Read;

pub struct FromBson2;

impl PluginCommand for FromBson2 {
    type Plugin = BsonPlugin;

    fn name(&self) -> &str {
        "from bson2"
    }

    fn signature(&self) -> Signature {
        Signature::build(PluginCommand::name(self))
            .category(Category::Formats)
            .input_output_type(Type::Binary, Type::Any)
    }

    fn description(&self) -> &str {
        "from bson2"
    }

    fn run(
        &self,
        _plugin: &Self::Plugin,
        _engine: &EngineInterface,
        call: &EvaluatedCall,
        input: PipelineData,
    ) -> Result<PipelineData, LabeledError> {
        match input {
            PipelineData::Empty => Ok(input),
            PipelineData::ByteStream(byte_stream, _pipeline_metadata) => {
                // let mut cursor = Cursor::new(byte_stream);

                let mut reader: Reader = byte_stream.reader().unwrap();

                let mut docs: Vec<Document> = Vec::new();
                while let Ok(doc) = Document::from_reader(reader.by_ref()) {
                    docs.push(doc);
                }

                let values: Vec<Value> = docs.iter().map(convert_document2).collect();

                let out_stream = ListStream::new(values.into_iter(), Span::unknown(), Signals::empty());
                Ok(PipelineData::ListStream(out_stream, None))
            },
            // PipelineData::Value(value, _pipeline_metadata) => {
            //     match value {
            //         Value::Binary { val, .. } => {
            //             let a = crate::bson_to_nu::convert_binary(val);
            //             let b = Value::list(a, Span::unknown());
            //             Ok(b)
            //         }
            //     }
            // }
            _ => Err(
                LabeledError::new("Can only parse byte stream as BSON").with_label(
                    format!("requires binary input; got {}", input.get_type()),
                    call.head,
                ),
            ),
        }
    }
}
