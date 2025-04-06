use bson::{Bson, Document, RawDocument, RawDocumentBuf};
use nu_protocol::{Record, Span, Value};
use std::collections::HashMap;
use std::io::{Cursor, Read};

// Possibly multiple
pub fn parse_binary(val: &Vec<u8>) -> Vec<Document> {
    let mut docs: Vec<Document> = Vec::new();

    // TODO: Should handle/forward Err
    // let raw = RawDocumentBuf::from_bytes(val).unwrap();
    // println!("{:?}", raw);
    // let doc = Document::try_from(raw).unwrap();
    // docs.push(doc);
    // docs
    
    let mut cursor = Cursor::new(val);
    
    while let Ok(doc) = Document::from_reader(cursor.by_ref()) {
        docs.push(doc);
    }

    docs
}

pub fn convert_binary(val: &Vec<u8>) -> Vec<Value> {
    let span = Span::new(0, 0);

    let mapped: Vec<Value> = parse_binary(val).iter().map(|doc| convert_document(&doc, span)).collect();

    Vec::from_iter(mapped)
}

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
