use aws_sdk_dynamodb::config::BehaviorVersion;
use aws_sdk_dynamodb::error::SdkError;
use aws_sdk_dynamodb::operation::delete_item::DeleteItemError;
use aws_sdk_dynamodb::operation::get_item::GetItemError;
use aws_sdk_dynamodb::operation::put_item::PutItemError;
use aws_sdk_dynamodb::operation::query::QueryError;
use aws_sdk_dynamodb::operation::scan::ScanError;
use aws_sdk_dynamodb::operation::update_item::UpdateItemError;
use aws_sdk_dynamodb::Client;

#[derive(Clone)]
pub struct Repository {
    pub(crate) env: String,
    pub(crate) client: Client,
}

impl Repository {
    pub async fn new(env: &str) -> Self {
        // DB接続を生成する
        let sdk_config = aws_config::load_defaults(BehaviorVersion::latest()).await;
        let config = aws_sdk_dynamodb::config::Builder::from(&sdk_config).build();
        let client = Client::from_conf(config);
        Self {
            env: env.to_string(),
            client,
        }
    }
    pub fn env_table_name(&self, table_name: &str) -> String {
        let env = &self.env;
        format!("{env}_{table_name}")
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RepositoryError {
    #[error("database error")]
    ScanError(#[from] SdkError<ScanError>),
    #[error("database error.")]
    QueryError(#[from] SdkError<QueryError>),
    #[error("database error.")]
    GetItemError(#[from] SdkError<GetItemError>),
    #[error("database error.")]
    DeleteItemError(#[from] SdkError<DeleteItemError>),
    #[error("database error.")]
    PutItemError(#[from] SdkError<PutItemError>),
    #[error("database error.")]
    UpdateItemError(#[from] SdkError<UpdateItemError>),
    #[error("mapping failed.")]
    MappingError(#[from] serde_dynamo::Error),
    #[error("this id not found")]
    NotFound,
}
