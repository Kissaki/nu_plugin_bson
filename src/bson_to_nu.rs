use bson::{Bson, Document};
use nu_protocol::{Record, Span, Value};
use std::collections::HashMap;
use std::io::{Cursor, Read};

// TODO: Explore RawDocument and RawDocumentBut and raw bson crate

// Possibly multiple
pub fn parse_binary(val: &Vec<u8>) -> Vec<Document> {
    let mut docs: Vec<Document> = Vec::new();

    // let raw = RawDocumentBuf::from_bytes(val).unwrap();
    // println!("{:?}", raw);
    // let doc = Document::try_from(raw).unwrap();
    // docs.push(doc);
    // docs

    let mut cursor = Cursor::new(val);

    // TODO: Should handle/forward Err
    while let Ok(doc) = Document::from_reader(cursor.by_ref()) {
        docs.push(doc);
    }

    docs
}

pub fn convert_binary(val: &Vec<u8>) -> Vec<Value> {
    let span = Span::unknown();

    let mapped: Vec<Value> = parse_binary(val)
        .iter()
        .map(|doc| convert_document(doc, span))
        .collect();

    Vec::from_iter(mapped)
}

pub fn convert_document(doc: &Document, span: Span) -> Value {
    let mut map = HashMap::new();

    for (key, value) in doc {
        map.insert(key.clone(), convert_value(value, span));
    }

    let r = Record::from_iter(map);
    Value::record(r, span)
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
        Bson::Binary(b) => {
            let bin = b.bytes.clone();
            Value::binary(bin, Span::unknown())
        }
        // _ => Value::nothing(Span::unknown()),
        _ => panic!("Unhandled BSON type {}", val),
    }
}

pub fn convert_array(arr: &[Bson], span: Span) -> Value {
    Value::list(
        arr.iter().map(|value| convert_value(value, span)).collect(),
        span,
    )
}

pub fn convert_document2(doc: &Document) -> Value {
    let mut map = HashMap::new();

    for (key, value) in doc {
        map.insert(key.clone(), convert_value(value, Span::unknown()));
    }

    let r = Record::from_iter(map);
    Value::record(r, Span::unknown())
}

pub fn convert_value2(val: &Bson) -> Value {
    match val {
        Bson::Document(doc) => convert_document(doc, Span::unknown()),
        Bson::Array(inner_arr) => convert_array(inner_arr, Span::unknown()),
        Bson::String(s) => Value::string(s.clone(), Span::unknown()),
        Bson::Int32(i) => Value::int(*i as i64, Span::unknown()),
        Bson::Int64(i) => Value::int(*i, Span::unknown()),
        Bson::Double(d) => Value::float(*d, Span::unknown()),
        Bson::Boolean(b) => Value::bool(*b, Span::unknown()),
        Bson::Null => Value::nothing(Span::unknown()),
        Bson::Binary(b) => {
            let bin = b.bytes.clone();
            Value::binary(bin, Span::unknown())
        }
        // _ => Value::nothing(Span::unknown()),
        _ => panic!("Unhandled BSON type {}", val),
    }
}
