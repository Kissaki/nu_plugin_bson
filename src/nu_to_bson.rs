use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use bson::spec::BinarySubtype::Generic;
use bson::{Binary, Bson, Document};
use nu_protocol::{Span, Value};

pub fn nu_value_to_nu_bson_binary(val: &Value) -> Value {
    let bson = nu_value_to_bson(val);
    let doc = bson.as_document().unwrap();
    document_to_nubinary(doc)
}

pub fn nu_value_to_bson(val: &Value) -> Bson {
    match val {
        Value::Bool { val, .. } => Bson::Boolean(*val),
        Value::Int { val, .. } => Bson::Int64(*val),
        Value::Float { val, .. } => Bson::Double(*val),
        Value::String { val, .. } => Bson::String(val.to_string()),
        Value::Glob { val, .. } => Bson::String(val.to_string()),
        Value::Filesize { val, .. } => Bson::Int64(i64::from(*val)),
        Value::Duration { val, .. } => Bson::Int64(*val),
        Value::Date { val, .. } => Bson::DateTime(bson::DateTime::from_chrono(*val)),
        Value::Range { val, .. } => Bson::String(val.to_string()),
        Value::Record { val, .. } => {
            let mut doc = Document::new();
            for x in val.iter() {
                let key = x.0;
                let bson_value = nu_value_to_bson(x.1);
                doc.insert(key, bson_value);
            }
            doc.into()
        }
        Value::List { vals, .. } => {
            let a = vals.iter().map(nu_value_to_bson).collect();
            Bson::Array(a)
        }
        // Closure is missing transformation mapping
        Value::Closure { .. } => Bson::Null,
        Value::Error { error, .. } => Bson::String(error.to_string()),
        Value::Binary { val, .. } => {
            let b64 = BASE64_STANDARD.encode(val);
            Binary::from_base64(b64, Generic).unwrap().into()
        }
        Value::CellPath { val, .. } => Bson::String(val.to_string()),
        // Custom is missing transformation mapping
        Value::Custom { .. } => Bson::Null,
        Value::Nothing { .. } => Bson::Null,
    }
}

pub fn bson_to_nu_binary(bson: &Bson) -> Value {
    let b: Vec<u8> = bson::to_vec(&bson.as_document().unwrap()).unwrap();
    Value::binary(b, Span::unknown())
}

pub fn document_to_nubinary(doc: &Document) -> Value {
    let b: Vec<u8> = bson::to_vec(&doc).unwrap();
    Value::binary(b, Span::unknown())
}
