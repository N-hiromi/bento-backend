use aws_sdk_secretsmanager::config::http::HttpResponse;
use aws_sdk_secretsmanager::config::BehaviorVersion;
use aws_sdk_secretsmanager::error::SdkError;
use aws_sdk_secretsmanager::operation::get_secret_value::GetSecretValueError;
use aws_sdk_secretsmanager::{config, Client};
use serde::de::DeserializeOwned;
use serde_json::from_str;

#[derive(Clone)]
pub struct SecretManager {
    pub(crate) client: Client,
}

impl SecretManager {
    pub async fn new() -> Self {
        // clientの作成
        let sdk_config = aws_config::load_defaults(BehaviorVersion::latest()).await;
        let config = config::Builder::from(&sdk_config).build();
        let client = Client::from_conf(config);
        Self { client }
    }

    // secretを取得する
    pub async fn get_secrets<T: DeserializeOwned>(
        &self,
        secret_name: &str,
    ) -> Result<T, SecretManagerError> {
        let secrets = self
            .client
            .get_secret_value()
            .secret_id(secret_name)
            .send()
            .await
            .map_err(|e| SecretManagerError::GetSecretError { e })?;

        // jsonに変換
        let secret: T = from_str(&secrets.secret_string.unwrap())
            .map_err(|e: serde_json::Error| SecretManagerError::JsonParseError { e })?;

        Ok(secret)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SecretManagerError {
    #[error("get secret error")]
    GetSecretError {
        e: SdkError<GetSecretValueError, HttpResponse>,
    },
    #[error("string parse json error")]
    JsonParseError { e: serde_json::Error },
}
