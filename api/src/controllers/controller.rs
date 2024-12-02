use axum::http::StatusCode;
use axum::response::IntoResponse;
use common::repositories::repository::RepositoryError;
use std::fmt;
use std::fmt::{Debug, Display};

#[derive(Debug)]
pub struct ApiRepositoryError {
    pub e: RepositoryError,
}
impl From<ApiRepositoryError> for StatusCode {
    fn from(_: ApiRepositoryError) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}
impl Display for ApiRepositoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ApiRepositoryError: {:?}", self.e)
    }
}

pub fn to_error_response<T: Into<StatusCode> + Debug + Display>(error: T) -> impl IntoResponse {
    // ログを出力する
    let detail_message = format!("{:?}", error);
    println!("{:?}", detail_message);

    // 内容を生成する
    let response_body = error.to_string();

    // status
    let status_code: StatusCode = error.into();

    // レスポンスを生成する
    (status_code, response_body)
}
