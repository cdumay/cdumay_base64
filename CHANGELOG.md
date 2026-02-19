# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.2]

### Added

- Re-export of the `base64` crate as `cdumay_base64::base64`, so dependent crates can use base64 without adding it as a direct dependency.

### Changed

- Doctests and README examples now use the re-export (`cdumay_base64::base64`) instead of a direct `base64` import.
- README: document the re-export in Features, align structure, and add a License section.

### Testing

- Extended test suite to cover all four `DecodeError` variants (InvalidByte, InvalidLength, InvalidLastSymbol, InvalidPadding).
- Added test for `convert_decode_result!` with two arguments (result, context only).
- Integration tests now use `cdumay_base64::base64` for engine and types.
- Code coverage raised to 100% for library code.
