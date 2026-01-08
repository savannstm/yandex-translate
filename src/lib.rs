#![warn(clippy::all, clippy::pedantic)]
#![doc = include_str!("../README.md")]

#[cfg(all(feature = "blocking", feature = "async"))]
compile_error!("features `blocking` and `async` are mutually exclusive");

use serde::{Deserialize, Serialize};
use thiserror::Error;

const API_BASE_URL: &str = "https://translate.api.cloud.yandex.net/translate/v2";

#[derive(Debug, Error)]
pub enum Error {
    #[error("HTTP request failed: {0}")]
    RequestFailed(#[from] reqwest::Error),

    #[error("JSON serialization/deserialization failed: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("API error: {0}")]
    ApiError(String),

    #[error("Invalid configuration: {0}")]
    ConfigError(String),
}

pub type Result<T> = std::result::Result<T, Error>;

/// Authentication method for the Yandex Translate API.
///
/// This enum defines how requests to the API are authenticated.
/// Exactly one authentication method must be provided when creating
/// a translate client.
#[derive(Debug, Clone)]
pub enum AuthMethod {
    /// Authenticate using a Yandex Cloud API key.
    ///
    /// The API key is sent in the `Authorization` header using the
    /// `Api-Key` scheme.
    ApiKey(String),

    /// Authenticate using an IAM bearer token.
    ///
    /// The token is sent in the `Authorization` header using the
    /// `Bearer` scheme.
    IAMToken(String),
}

/// Request body for a translation operation.
///
/// This structure is serialized to JSON and sent to the `/translate`
/// endpoint of the Yandex Translate API.
///
/// The lifetime parameter ensures that the request can borrow data
/// without requiring additional allocations.
#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TranslateRequest<'a> {
    /// Identifier of the Yandex Cloud folder.
    ///
    /// This specifies the cloud folder where the translation request
    /// is billed and authorized.
    pub folder_id: &'a str,

    /// Texts to be translated.
    ///
    /// Each element in the slice is translated independently, and the
    /// results are returned in the same order.
    pub texts: &'a [&'a str],

    /// Target language code.
    ///
    /// Uses ISO 639-1 language codes (for example, `"en"`, `"ru"`, `"de"`).
    pub target_language_code: &'a str,

    /// Optional source language code.
    ///
    /// If not provided, the API will attempt to automatically detect
    /// the source language for each input text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_language_code: Option<&'a str>,
}

/// Individual translation result.
///
/// Each input text produces one `Translation` entry in the response.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Translation {
    /// Translated text.
    pub text: String,

    /// Detected source language code.
    ///
    /// This field is present only when the source language was not
    /// explicitly specified in the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detected_language_code: Option<String>,
}

/// Response returned by the Yandex Translate API.
///
/// The response contains one [`Translation`] for each input text,
/// in the same order as they were provided in the request.
#[derive(Debug, Clone, Deserialize)]
pub struct TranslateResponse {
    /// List of translation results.
    pub translations: Vec<Translation>,
}

#[cfg(feature = "blocking")]
pub mod blocking;
#[cfg(feature = "blocking")]
pub use blocking::YandexTranslateClient;

#[cfg(feature = "async")]
pub mod asynchronous;
#[cfg(feature = "async")]
pub use asynchronous::YandexTranslateClient;
