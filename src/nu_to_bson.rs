use bson::{Bson, Document};
use nu_protocol::Value;

pub fn convert_value(val: &Value) -> Bson {
    match val {
        Value::Record { val, .. } => {
            let mut doc = Document::new();
            for x in val.iter() {
                let key = x.0;
                let bson_value = convert_value(x.1);
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
