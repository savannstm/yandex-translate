//! Blocking version of Yandex Translate client.

use crate::{AuthMethod, Error, Result, TranslateRequest, TranslateResponse, API_BASE_URL};
use reqwest::blocking::Client;

/// Blocking client for interacting with the Yandex Translate API.
///
/// This client uses `reqwest::blocking` internally and is intended for
/// synchronous / non-async use cases such as CLI tools or simple applications.
///
/// Authentication is handled via [`AuthMethod`], supporting both API keys
/// and IAM tokens.
pub struct YandexTranslateClient {
    /// Underlying HTTP client
    client: Client,
    /// Authentication method used for requests
    auth: AuthMethod,
    /// Base URL of the Yandex Translate API
    base_url: String,
}

impl YandexTranslateClient {
    /// Creates a new `YandexTranslateClient` using the given authentication method.
    ///
    /// This constructor initializes a blocking `reqwest` client and sets
    /// the base URL to [`API_BASE_URL`].
    ///
    /// # Parameters
    ///
    /// - `auth`: Authentication method (API key or IAM token)
    ///
    /// # Errors
    ///
    /// Returns an error if the underlying HTTP client cannot be created.
    pub fn new(auth: AuthMethod) -> Result<Self> {
        let client = Client::builder().build()?;

        Ok(Self {
            client,
            auth,
            base_url: API_BASE_URL.to_string(),
        })
    }

    /// Creates a new client authenticated with an API key.
    ///
    /// This is a convenience constructor equivalent to calling [`Self::new`]
    /// with [`AuthMethod::ApiKey`].
    ///
    /// # Parameters
    ///
    /// - `api_key`: Yandex Cloud API key
    ///
    /// # Errors
    ///
    /// Returns an error if the HTTP client cannot be initialized.
    ///
    pub fn with_api_key(api_key: &str) -> Result<Self> {
        Self::new(AuthMethod::ApiKey(api_key.into()))
    }

    /// Creates a new client authenticated with an IAM token.
    ///
    /// This is a convenience constructor equivalent to calling [`Self::new`]
    /// with [`AuthMethod::IAMToken`].
    ///
    /// # Parameters
    ///
    /// - `iam_token`: IAM bearer token
    ///
    /// # Errors
    ///
    /// Returns an error if the HTTP client cannot be initialized.
    pub fn with_iam_token(iam_token: &str) -> Result<Self> {
        Self::new(AuthMethod::IAMToken(iam_token.into()))
    }

    /// Sets a custom base URL for the API.
    ///
    /// This can be useful for testing, proxies, or alternative endpoints.
    ///
    /// # Parameters
    ///
    /// - `base_url`: Base URL to use instead of the default
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use yandex_translate_v2::YandexTranslateClient;
    ///
    /// let client = YandexTranslateClient::with_api_key("key").unwrap()
    ///     .with_base_url("https://example.com");
    /// ```
    pub fn with_base_url(mut self, base_url: &str) -> Self {
        self.base_url = base_url.into();
        self
    }

    /// Translates text using the Yandex Translate API.
    ///
    /// Sends a POST request to the `/translate` endpoint with the provided
    /// [`TranslateRequest`] and returns a [`TranslateResponse`] on success.
    ///
    /// Authentication headers are added automatically based on the configured
    /// [`AuthMethod`].
    ///
    /// # Parameters
    ///
    /// - `request`: Translation request payload
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The HTTP request fails
    /// - The API responds with a non-success status code
    /// - The response body cannot be parsed
    ///
    /// In case of an API error, the response body is included in the error
    /// message for easier debugging.
    pub fn translate(&self, request: &TranslateRequest) -> Result<TranslateResponse> {
        let url = format!("{}/translate", self.base_url);

        let mut req = self
            .client
            .post(&url)
            .header("Content-Type", "application/json");

        req = match &self.auth {
            AuthMethod::ApiKey(key) => req.header("Authorization", format!("Api-Key {}", key)),
            AuthMethod::IAMToken(token) => req.header("Authorization", format!("Bearer {}", token)),
        };

        let response = req.json(request).send()?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response
                .text()
                .unwrap_or_else(|_| "Unknown error".to_string());

            return Err(Error::ApiError(format!(
                "API returned status {}: {}",
                status, error_text
            )));
        }

        Ok(response.json()?)
    }
}
