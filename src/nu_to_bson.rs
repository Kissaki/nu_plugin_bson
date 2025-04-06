use bson::{Bson, Document};
use nu_protocol::{Span, Value};

pub fn nu_value_to_nu_bson_binary(val: &Value) -> Value {
    let bson = nu_value_to_bson(val);
    let doc = bson.as_document().unwrap();
    let nu_binary = document_to_nubinary(doc);
    nu_binary
}

pub fn nu_value_to_bson(val: &Value) -> Bson {
    match val {
        Value::Record { val, .. } => {
            let mut doc = Document::new();
            for x in val.iter() {
                let key = x.0;
                let bson_value = nu_value_to_bson(x.1);
                doc.insert(key, bson_value);
            };
            doc.into()
        },
        // Value::List { vals, internal_span } => {
        //     let a = vals.iter().map(convert_v);
        //     Bson::Array(a)
        // },
        // Value::Binary { val, internal_span } => {
        //     Binary::from(val);
        // },
        Value::Bool { val, .. } => Bson::Boolean(*val),
        Value::Int { val, .. } => Bson::Int64(*val),
        Value::Float { val, .. } => Bson::Double(*val),
        Value::String { val, .. } => Bson::String(val.to_string()),
        //Value::Duration { val, internal_span } => Bson::from(val),
        //Value::Date { val, internal_span } => Bson::from(val.to_string()),
        //Value::Date { val, internal_span } => Bson::DateTime(val),
        Value::Nothing { .. } => Bson::Null,
        _ => Bson::Null,
    }
}

pub fn bson_to_nu_binary(bson: &Bson) -> Value {
    let b: Vec<u8> = bson::to_vec(&bson.as_document().unwrap()).unwrap();
    Value::Binary {
        val: b,
        internal_span: Span::unknown(),
    }
}

pub fn document_to_nubinary(doc: &Document) -> Value {
    let b: Vec<u8> = bson::to_vec(&doc).unwrap();
    Value::Binary {
        val: b,
        internal_span: Span::unknown(),
    }
}
