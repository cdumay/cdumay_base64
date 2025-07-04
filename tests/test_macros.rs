use base64::Engine;
use base64::engine::general_purpose;
use cdumay_core::ErrorConverter;
use cdumay_base64::convert_decode_result;
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
