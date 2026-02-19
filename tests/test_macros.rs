use cdumay_base64::base64::engine::general_purpose;
use cdumay_base64::base64::Engine;
use cdumay_base64::convert_decode_result;
use cdumay_core::ErrorConverter;
use std::collections::BTreeMap;

#[test]
fn test_convert_base64decode_result_with_context() {
    let result = general_purpose::STANDARD.decode("!!! not base64 !!!");
    let mut context = BTreeMap::new();
    context.insert("test".to_string(), serde_value::Value::String("value".to_string()));

    let converted = convert_decode_result!(result, context, "Test error");
    assert!(converted.is_err());

    let err = converted.unwrap_err();
    assert!(err.message().contains("Test error"));
}

#[test]
fn test_convert_base64decode_result_without_context() {
    let result = general_purpose::STANDARD.decode("!!! not base64 !!!");
    let converted = convert_decode_result!(result);
    assert!(converted.is_err());

    let err = converted.unwrap_err();
    assert!(err.message().contains("Invalid symbol"));
}

#[test]
fn test_convert_base64decode_result_minimal() {
    let result = general_purpose::STANDARD.decode("!!! not base64 !!!");
    let converted = convert_decode_result!(result);
    assert!(converted.is_err());
}

#[test]
fn test_convert_base64decode_result_success() {
    let valid_data = general_purpose::STANDARD.encode(b"hello world");
    let result = general_purpose::STANDARD.decode(&valid_data);
    let converted = convert_decode_result!(result);
    assert!(converted.is_ok());
}

/// Macro with 2 args: (result, context) — no custom message.
#[test]
fn test_convert_base64decode_result_context_only() {
    let result = general_purpose::STANDARD.decode("!!!");
    let mut context = BTreeMap::new();
    context.insert("key".to_string(), serde_value::Value::String("v".to_string()));
    let converted = convert_decode_result!(result, context);
    assert!(converted.is_err());
    let err = converted.unwrap_err();
    assert!(!err.message().is_empty());
}
