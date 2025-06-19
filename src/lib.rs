//! [![License: BSD-3-Clause](https://img.shields.io/badge/license-BSD--3--Clause-blue)](./LICENSE)
//! [![cdumay_base64 on crates.io](https://img.shields.io/crates/v/cdumay_base64)](https://crates.io/crates/cdumay_base64)
//! [![cdumay_base64 on docs.rs](https://docs.rs/cdumay_base64/badge.svg)](https://docs.rs/cdumay_base64)
//! [![Source Code Repository](https://img.shields.io/badge/Code-On%20GitHub-blue?logo=GitHub)](https://github.com/cdumay/cdumay_base64)
//!
//! A small crate to manipulate base64 data.
//!
//! ## Features
//!
//! - Maps all variants of `base64::DecodeError` into structured `cdumay_core::Error` types.
//! - Provides unique error codes, HTTP status codes, and human-readable messages.
//! - Easily attach contextual metadata for better debugging.
//! - Simple integration into any Rust project using `base64` and `cdumay_core`.
//! - Provides convenient macro for error conversion
//!
//! ## Usage
//!
//! Using the `Base64DecodeErrorConverter` directly:
//! ```rust
//! use base64::{engine::general_purpose, Engine as _};
//! use std::collections::BTreeMap;
//! use cdumay_core::{ErrorConverter, Error};
//! use cdumay_base64::Base64DecodeErrorConverter;
//!
//! fn decode_base64(input: &str) -> cdumay_core::Result<Vec<u8>> {
//!     general_purpose::STANDARD.decode(input).map_err(|e| {
//!         let mut context = BTreeMap::new();
//!         context.insert("input".to_string(), serde_value::Value::String(input.to_string()));
//!         Base64DecodeErrorConverter::convert(&e, "Failed to decode base64".to_string(), context)
//!     })
//! }
//! ```
//! Using the `convert_decode_result!` macro:
//! ```rust
//! use base64::{engine::general_purpose, Engine as _};
//! use cdumay_core::{ErrorConverter, Error};
//! use std::collections::BTreeMap;
//! use cdumay_base64::convert_decode_result;
//!
//! fn decode_base64(input: &str) -> cdumay_core::Result<Vec<u8>> {
//!     let mut context = BTreeMap::new();
//!     context.insert("input".to_string(), serde_value::Value::String(input.to_string()));
//!     convert_decode_result!(general_purpose::STANDARD.decode(input), context, "Failed to decode base64")
//! }
//! ```
//!
#[macro_use]
mod macros;

mod errors;
pub use errors::*;
