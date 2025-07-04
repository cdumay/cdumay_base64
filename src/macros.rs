/// Macro to convert a [`Result<T, base64::DecodeError>`] into a [`cdumay_core::Result<T>`]
#[macro_export]
macro_rules! convert_decode_result {
    ($result:expr, $context:expr, $text:expr) => {
        $result.map_err(|err| cdumay_base64::Base64DecodeErrorConverter::convert_error(&err, Some($text.to_string()), $context))
    };
    ($result:expr, $context:expr) => {
        $result.map_err(|err| cdumay_base64::Base64DecodeErrorConverter::convert_error(&err, None, $context))
    };
    ($result:expr) => {
        $result.map_err(|err| cdumay_base64::Base64DecodeErrorConverter::convert_error(&err, None, std::collections::BTreeMap::new()))
    };
}
