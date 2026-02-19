use cdumay_core::ErrorConverter;
use cdumay_base64::base64::{DecodeError, Engine as _, engine::general_purpose};
use cdumay_base64::Base64DecodeErrorConverter;
use std::collections::BTreeMap;

/// Helper to decode and convert (for coverage of convert path).
fn decode_and_convert(input: &str) -> cdumay_core::Error {
    let err = general_purpose::STANDARD.decode(input).unwrap_err();
    Base64DecodeErrorConverter::convert_error(&err, Some("decode failed".into()), BTreeMap::new())
}

#[test]
fn test_invalid_byte_error() {
    let e = decode_and_convert("!!!");
    assert!(e.message().contains("decode failed"));
}

#[test]
fn test_invalid_length_error() {
    let e = decode_and_convert("A");
    assert!(e.message().contains("decode failed"));
}

#[test]
fn test_invalid_last_symbol_error() {
    let e = decode_and_convert("ABB=");
    assert!(e.message().contains("decode failed"));
}

#[test]
fn test_invalid_padding_error() {
    let e = decode_and_convert("A===");
    assert!(e.message().contains("decode failed"));
}

/// Cover all four DecodeError branches by calling convert with each variant.
#[test]
fn test_convert_all_decode_error_variants() {
    let ctx = BTreeMap::new();
    let msg = "test".to_string();

    let _ = Base64DecodeErrorConverter::convert_error(
        &DecodeError::InvalidByte(0, 0),
        Some(msg.clone()),
        ctx.clone(),
    );
    let _ = Base64DecodeErrorConverter::convert_error(
        &DecodeError::InvalidLength(1),
        Some(msg.clone()),
        ctx.clone(),
    );
    let _ = Base64DecodeErrorConverter::convert_error(
        &DecodeError::InvalidLastSymbol(0, 0),
        Some(msg.clone()),
        ctx.clone(),
    );
    let _ = Base64DecodeErrorConverter::convert_error(
        &DecodeError::InvalidPadding,
        Some(msg),
        ctx,
    );
}

#[test]
fn test_invalid_base64_decode_error() {
    // Attempt to decode an invalid Base64 string
    let invalid_data = "!!! not base64 !!!";
    let result = general_purpose::STANDARD.decode(invalid_data);

    // Ensure that decoding fails
    assert!(result.is_err());

    let err = result.unwrap_err();
    let ctx = BTreeMap::new();

    // Convert the base64 error into a structured application error
    let custom = Base64DecodeErrorConverter::convert_error(&err, Some("Failed to decode base64".to_string()), ctx);

    // Validate the custom error message
    assert_eq!(custom.message(), "Failed to decode base64");
}

#[test]
fn test_valid_base64_decoding() {
    // Encode "hello world" into base64
    let valid_data = general_purpose::STANDARD.encode(b"hello world");

    // Attempt to decode it back
    let decoded = general_purpose::STANDARD.decode(&valid_data);

    // Ensure decoding succeeds and the content is correct
    assert!(decoded.is_ok());
    assert_eq!(decoded.unwrap(), b"hello world");
}
