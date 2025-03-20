use crate::BsonPlugin;

use nu_protocol::{CustomValue, ShellError, Span, Value, record};
use serde::{Deserialize, Serialize};
use std::any::Any;
use bson::{Bson,Document};

#[typetag::serde]
impl CustomValue for Bson {
    fn clone_value(&self, span: Span) -> Value {
        Value::custom_value(Box::new(self.clone()), span)
    }

    fn type_name(&self) -> String {
        "Bson".into()
    }

    fn to_base_value(&self, span: Span) -> Result<Value, ShellError> {
        // Construct a simple Nushell value that makes sense here.
        // It must not be a custom value.
        Ok(Value::record(record! {}, span))
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }
}
