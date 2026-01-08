# yandex-translate

Simple Yandex Translate API client vibe-coded by Claude.

## Installation

```bash
cargo add yandex-translate # Blocking
cargo add yandex-translate --no-default-features -F "async" # Asynchronous
```

## Example

### Blocking

```rust no_run
use yandex_translate::{
    AuthMethod,
    TranslateRequest,
    YandexTranslateClient,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YandexTranslateClient::with_api_key("YOUR_API_KEY")?;

    let texts = ["Hello world", "How are you?"];

    let request = TranslateRequest {
        folder_id: "YOUR_FOLDER_ID",
        texts: &texts,
        target_language_code: "ru",
        source_language_code: Some("en"),
    };

    let response = client.translate(&request)?;

    for translation in response.translations {
        println!("Translated: {}", translation.text);

        if let Some(lang) = translation.detected_language_code {
            println!("Detected language: {}", lang);
        }
    }

    Ok(())
}
```

### Asynchronous

```rust no_run
use yandex_translate::{
    TranslateRequest,
    YandexTranslateClient,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YandexTranslateClient::with_iam_token("YOUR_IAM_TOKEN")?;

    let texts = ["Good morning", "Nice to meet you"];

    let request = TranslateRequest {
        folder_id: "YOUR_FOLDER_ID",
        texts: &texts,
        target_language_code: "de",
        source_language_code: None, // let API detect language
    };

    let response = client.translate(&request).await?;

    for translation in response.translations {
        println!("Translated: {}", translation.text);
    }

    Ok(())
}
```

## Features

- `blocking` - enabled by default. Uses blocking/synchronous requests.
- `async` - Uses asynchronous requests. Mutually exclusive with `blocking`.

## License

Project is licensed under WTFPL.
