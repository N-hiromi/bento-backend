use reqwest::Client;
use serde::de::DeserializeOwned;

pub async fn fetch<S: DeserializeOwned>(url: &String) -> Result<S, FetchError> {
    // jwksを取得する
    let response = Client::new()
        .get(url)
        .send()
        .await
        .map_err(|e| FetchError::JWKsUrlFailed {
            e,
            url: url.clone(),
        })?;

    // 取得したレスポンスをjwksに変換する
    let response_body = format!("{:?}", response);
    response
        .json::<S>()
        .await
        .map_err(|e| FetchError::JWKsResponseFailed { e, response_body })
}

#[derive(Debug, thiserror::Error)]
pub enum FetchError {
    #[error("jwks public key failed.")]
    JWKsUrlFailed { e: reqwest::Error, url: String },
    #[error("jwks public key failed. {e:?}")]
    JWKsResponseFailed {
        e: reqwest::Error,
        response_body: String,
    },
}
