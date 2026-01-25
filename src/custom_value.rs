use crate::serde::{Deserialize, Serialize};
use bson::{Bson, Document};
use nu_protocol::{CustomValue, ShellError, Span, Value, record};
use std::any::Any;

#[derive(Debug, Clone, Serialize, Deserialize)]
enum Wrap {
    Bson(Bson),
    Document(Document),
}

#[typetag::serde]
impl CustomValue for Wrap {
    fn clone_value(&self, span: Span) -> Value {
        
        Value::custom_value(Box::new(self.clone()), span)
    }

    fn type_name(&self) -> String {
        "Wrap".into()
    }

    fn to_base_value(&self, span: Span) -> Result<Value, ShellError> {
        match self {
            Bson => Ok(bson::from_bson(span)),
        }
        let b = from_bson(span).unwrap();
        // Construct a simple Nushell value that makes sense here.
        // It must not be a custom value.
        Ok(Value::string(self.to_string(), span))
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }
}
