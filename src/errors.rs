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
    /// * `context` - A map of additional metadata for debugging.
    ///
    /// # Returns
    ///
    /// Returns a structured `Error` with kind `DecodeBase64`, message, and details.
    fn convert(error: &DecodeError, text: String, context: BTreeMap<String, serde_value::Value>) -> Error {
        macro_rules! to_err {
            ($e:expr) => {
                $e.with_message(text).with_details(context).into()
            };
        }
        match error {
            DecodeError::InvalidByte(_, _) => to_err!(InvalidByteError::new()),
            DecodeError::InvalidLength(_) => to_err!(InvalidLengthError::new()),
            DecodeError::InvalidLastSymbol(_, _) => to_err!(InvalidLastSymbolError::new()),
            DecodeError::InvalidPadding => to_err!(InvalidPaddingError::new()),
        }
    }
}
