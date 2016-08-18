extern crate tinyjson;
use tinyjson::*;

const STR_OK: &'static str = r#"
          {
            "bool": true,
            "arr": [1, null, "test"],
            "nested": {
              "blah": false,
              "blahblah": 3.14
            },
            "unicode": "\u2764"
          }
        "#;

fn must_parse(s: &'static str) -> JsonValue {
    match parse_str(s) {
        Ok(j) => j,
        Err(e) => panic!("Parse failed: {:?}", e),
    }
}

#[test]
fn test_equality_property() {
    let a = must_parse(STR_OK);
    let b = must_parse(STR_OK);
    let c = must_parse(STR_OK);
    assert!(a == a);

    assert!(a == b);
    assert!(b == a);

    assert!(b == c);
    assert!(c == a);
}

#[test]
fn test_not_equal() {
    assert!(must_parse(r#"{}"#) != must_parse(r#"{"foo": 42}"#));
    assert!(must_parse(r#"{"foo": true}"#) != must_parse(r#"{"foo": 42}"#));
    assert!(must_parse(r#"{"foo": 21}"#) != must_parse(r#"{"foo": 42}"#));
    assert!(must_parse(r#"{"bar": 42}"#) != must_parse(r#"{"foo": 42}"#));
    assert!(must_parse(r#"{"arr": [1, 2, 3]}"#) != must_parse(r#"{"arr": [1, 2]}"#));
    assert!(must_parse(r#"{"foo": null}"#) != must_parse(r#"{"foo": 42}"#));
    assert!(must_parse(r#"{"foo": {"bar": 42}}"#) != must_parse(r#"{"foo": {"bar": 21}}"#));
    assert!(must_parse(r#"{"foo": [{"bar": 42}]}"#) != must_parse(r#"{"foo": [{"baz": 42}]}"#));
}

#[test]
fn test_equality_edge_cases() {
    assert!(must_parse(r#"{}"#) == must_parse(r#"{}"#));
    assert!(must_parse(r#"{"foo": []}"#) == must_parse(r#"{"foo": []}"#));
    assert!(must_parse(r#"{"a": null}"#) != must_parse(r#"{}"#));
    assert!(must_parse(r#"{"foo": [42]}"#) != must_parse(r#"{"foo": []}"#));
}

#[test]
fn test_access_with_index_operator() {
    let parsed = must_parse(STR_OK);
    assert!(parsed["bool"] == JsonValue::Boolean(true));
    assert!(parsed["nested"]["blahblah"] == JsonValue::Number(3.14));
}

#[test]
#[should_panic]
fn test_access_not_exist_value() {
    let parsed = must_parse(STR_OK);
    &parsed["unknown key"]["not exist key"];
}
