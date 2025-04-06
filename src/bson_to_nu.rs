use std::collections::HashMap;
use bson::{Bson, Document};
use nu_protocol::{Record, Span, Value};

pub fn convert_document(doc: &Document, span: Span) -> Value {
    let mut map = HashMap::new();

    for (key, value) in doc {
        map.insert(key.clone(), convert_value(value, span));
    }

    let r = Record::from_iter(map.into_iter());
    Value::record(r, span)
}

pub fn convert_array(arr: &[Bson], span: Span) -> Value {
    Value::list(arr.iter().map(|value| convert_value(value, span)).collect(), span)
}

pub fn convert_value(val: &Bson, span: Span) -> Value {
    match val {
        Bson::Document(doc) => convert_document(doc, span),
        Bson::Array(inner_arr) => convert_array(inner_arr, span),
        Bson::String(s) => Value::string(s.clone(), span),
        Bson::Int32(i) => Value::int(*i as i64, span),
        Bson::Int64(i) => Value::int(*i, span),
        Bson::Double(d) => Value::float(*d, span),
        Bson::Boolean(b) => Value::bool(*b, span),
        Bson::Null => Value::nothing(span),
        _ => Value::nothing(span),
    }
}
