//! Asynchronous version of Yandex Translate client.

use crate::{AuthMethod, Error, Result, TranslateRequest, TranslateResponse, API_BASE_URL};
use reqwest::Client;

/// Asynchronous client for interacting with the Yandex Translate API.
///
/// This client uses the asynchronous `reqwest` API and is intended for use
/// in async applications such as web servers, background workers, or
/// high-concurrency services.
///
/// Authentication is handled via [`AuthMethod`], supporting both API keys
/// and IAM tokens.
pub struct YandexTranslateClient {
    /// Underlying asynchronous HTTP client
    client: Client,
    /// Authentication method used for requests
    auth: AuthMethod,
    /// Base URL of the Yandex Translate API
    base_url: String,
}

impl YandexTranslateClient {
    /// Creates a new asynchronous `YandexTranslateClient` using the given
    /// authentication method.
    ///
    /// This constructor initializes an async `reqwest` client and sets
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

    /// Creates a new asynchronous client authenticated with an API key.
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

    /// Creates a new asynchronous client authenticated with an IAM token.
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
    /// This can be useful for testing, local mocks, proxies, or alternative
    /// endpoints.
    ///
    /// # Parameters
    ///
    /// - `base_url`: Base URL to use instead of the default
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use yandex_translate::YandexTranslateClient;
    ///
    /// let client = YandexTranslateClient::with_api_key("key").unwrap()
    ///     .with_base_url("https://example.com");
    /// ```
    pub fn with_base_url(mut self, base_url: &str) -> Self {
        self.base_url = base_url.into();
        self
    }

    /// Translates text asynchronously using the Yandex Translate API.
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
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use yandex_translate::{YandexTranslateClient, Result, TranslateRequest};
    ///
    /// async fn example() -> Result<()> {
    ///     let request = TranslateRequest {
    ///         folder_id: "YOUR_FOLDER_ID",
    ///         texts: &["Hello", "World"],
    ///         target_language_code: "en",
    ///         source_language_code: None,
    ///     };
    ///
    ///     let client = YandexTranslateClient::with_api_key("my-api-key").unwrap();
    ///     let response = client.translate(&request).await.unwrap();
    ///     Ok(())
    /// }
    /// ```
    pub async fn translate<'a>(&self, request: &TranslateRequest<'a>) -> Result<TranslateResponse> {
        let url = format!("{}/translate", self.base_url);

        let mut req = self
            .client
            .post(&url)
            .header("Content-Type", "application/json");

        req = match &self.auth {
            AuthMethod::ApiKey(key) => req.header("Authorization", format!("Api-Key {}", key)),
            AuthMethod::IAMToken(token) => req.header("Authorization", format!("Bearer {}", token)),
        };

        let response = req.json(request).send().await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());

            return Err(Error::ApiError(format!(
                "API returned status {}: {}",
                status, error_text
            )));
        }

        Ok(response.json().await?)
    }
}
