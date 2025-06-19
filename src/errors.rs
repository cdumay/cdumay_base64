use base64::DecodeError;
use cdumay_core::{define_errors, define_kinds, Error, ErrorConverter};
use std::collections::BTreeMap;

define_kinds! {
    Base64Decode = (400, "Base64 decode error"),
}

define_errors! {
    InvalidByteError = Base64Decode,
    InvalidLengthError = Base64Decode,
    InvalidLastSymbolError = Base64Decode,
    InvalidPaddingError = Base64Decode,
}

/// A helper structure that converts `base64::DecodeError` into a structured `Error`.
pub struct Base64DecodeErrorConverter;

impl ErrorConverter for Base64DecodeErrorConverter {
    type Error = DecodeError;
    /// Converts a `base64::DecodeError` into a custom structured `Error`.
    ///
    /// # Arguments
    ///
    /// * `err` - The error returned by the `base64` crate.
    /// * `text` - A human-readable error message.
    /// * `context` - A mutable reference to a `Context` containing additional metadata.
    ///
    /// # Returns
    ///
    /// Returns a structured `Error` with kind `DecodeBase64`, message, and details.
    fn convert(error: &DecodeError, text: String, context: BTreeMap<String, serde_value::Value>) -> Error {
        match error {
            DecodeError::InvalidByte(_, _) => InvalidByteError::new().with_message(text).with_details(context).into(),
            DecodeError::InvalidLength(_) => InvalidLengthError::new().with_message(text).with_details(context).into(),
            DecodeError::InvalidLastSymbol(_, _) => InvalidLastSymbolError::new().with_message(text).with_details(context).into(),
            DecodeError::InvalidPadding => InvalidPaddingError::new().with_message(text).with_details(context).into(),
        }
    }
}
