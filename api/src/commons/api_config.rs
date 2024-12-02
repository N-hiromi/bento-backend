use common::managers::secret_manager::SecretManager;
use serde::Deserialize;

static SECRET_NAME: &str = "bento/api";

#[derive(Deserialize, Clone)]
pub struct ApiConfig {
    pub cognito_jwks_url: String,
    pub cognito_client_id: String,
}

impl ApiConfig {
    pub async fn new(env: &str) -> Result<Self, Box<dyn std::error::Error>> {
        // 取得するシークレット名を作成する
        let secret_name = format!("{env}/{SECRET_NAME}");
        // シークレットマネージャを生成する
        let secret_manager = SecretManager::new().await;
        // シークレット情報を取得する
        let secret: ApiConfig = secret_manager.get_secrets(&secret_name).await?;
        // 結果の返却
        Ok(secret)
    }
}
