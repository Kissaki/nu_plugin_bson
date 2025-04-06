use bson::Document;
use crate::BsonPlugin;
use nu_plugin::{EngineInterface, EvaluatedCall, PluginCommand, SimplePluginCommand};
use nu_protocol::{
    Category, LabeledError,
    Signature, Value,
};

pub struct FromBson;
//pub struct FromBson2;

impl SimplePluginCommand for FromBson {
    type Plugin = BsonPlugin;

    fn name(&self) -> &str {
        "from bson"
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
        input: &Value,
    ) -> Result<Value, LabeledError> {
        let span = input.span();

        match input {
            Value::Binary { val, internal_span } => {
                let doc = Document::from_reader(val.as_slice()).unwrap();
                //let doc = Document::from_reader(&mut Cursor::new(&val[..])).unwrap();
                let r = crate::bson_to_nu::convert_document(&doc, *internal_span);
                Ok(r)
                // Value::record(doc, span)
                // Ok(doc.to)
                //
                // let bin_slice = from_slice(val).unwrap();
                // let a: Document = bin_slice;
                // //let bson: Document = Document::from_reader(&mut val.as_slice()).unwrap();
                // //let r = Value::Record { val: a, internal_span: span, };
                //
                // let mut sections = Record::new();
                //
                // //let m: IndexMap = a;
                // //let r = Value::record(m.into_iter().collect(), span);
                // //Ok(r)
                // Ok(Value::record(sections, span))
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

// impl PluginCommand for FromBson2 {
//     type Plugin = BsonPlugin;
//
//     fn name(&self) -> &str {
//         "from bson2"
//     }
//
//     fn signature(&self) -> Signature {
//         Signature::build(PluginCommand::name(self)).category(Category::Formats)
//     }
//
//     fn description(&self) -> &str {
//         "Convert BSON to table in a stream"
//     }
//
//     fn run(
//         &self,
//         _plugin: &BsonPlugin,
//         _engine: &EngineInterface,
//         call: &EvaluatedCall,
//         input: &PipelineData,
//     ) -> Result<Value, LabeledError> {
//         match input {
//             PipelineData::Empty => Ok(Value::nothing(input.span().unwrap())),
//             PipelineData::ByteStream(byte_stream, pipeline_metadata) => {
//                 let doc = Document::from_reader(byte_stream.into_bytes().unwrap().as_slice()).unwrap();
//
//             },
//             _ => Err(
//                 LabeledError::new("Can only parse binary data as BSON").with_label(
//                     format!("requires binary input; got {}", input.get_type()),
//                     call.head,
//                 ),
//             ),
//         }
//     }
// }
