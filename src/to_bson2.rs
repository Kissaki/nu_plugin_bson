use crate::nu_to_bson::nu_value_to_nu_bson_binary;
use crate::BsonPlugin;
use nu_plugin::{EngineInterface, EvaluatedCall, PluginCommand};
use nu_protocol::{Category, LabeledError, PipelineData, Signature, Type};

pub struct ToBson2;

impl PluginCommand for ToBson2 {
    type Plugin = BsonPlugin;

    fn name(&self) -> &str {
        "to bson2"
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
        "to bson2"
    }

    fn run(
        &self,
        _plugin: &BsonPlugin,
        _engine: &EngineInterface,
        call: &EvaluatedCall,
        input: PipelineData,
    ) -> Result<PipelineData, LabeledError> {
        match input {
            PipelineData::Empty => Ok(PipelineData::Empty),
            PipelineData::Value(value, _pipeline_metadata) => {
                let encoded = nu_value_to_nu_bson_binary(&value);
                Ok(PipelineData::Value(encoded, None))
            },
            PipelineData::ListStream(list_stream, _pipeline_metadata) => {
                let values = list_stream.map(|x| nu_value_to_nu_bson_binary(&x));
                Ok(PipelineData::ListStream(values, None))
            },
            _ => Err(
                LabeledError::new("Can only parse byte stream as BSON").with_label(
                    format!("requires binary input; got {}", input.get_type()),
                    call.head,
                ),
            ),
        }
    }
}
