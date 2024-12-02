use crate::auth::fetch;
use crate::auth::jwk::{Jwk, Jwks};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum_extra::typed_header::TypedHeaderRejection;

#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("jwks fetch failed.")]
    FetchFailed(#[from] fetch::FetchError),
    #[error("decode header failed. {e:?}")]
    DecodeHeaderFailed { e: jsonwebtoken::errors::Error },
    #[error("decode kid header empty.")]
    DecodeKidHeaderEmpty,
    #[error("kid not found in jwks.")]
    KidNotFoundInJWKs { jwks: Jwks },
    #[error("decode public key failed. {e:?}")]
    JwkPublicKeyFailed {
        e: jsonwebtoken::errors::Error,
        jwk: Jwk,
    },
    #[error("jwk algorithm failed. {e:?}")]
    JwkAlgorithmFailed {
        e: jsonwebtoken::errors::Error,
        jwk: Jwk,
    },
    #[error("decode id_token failed. {e:?}")]
    DecodeVerifyFailed { e: jsonwebtoken::errors::Error },
    #[error("authorization header not found.")]
    AuthorizationHeaderNotFound(#[from] TypedHeaderRejection),
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        // ログを出力する
        let detail_message = format!("{:?}", self);
        println!("{:?}", detail_message);

        // ステータスコードを決定する
        let status_code = match self {
            AuthError::FetchFailed(_)
            | AuthError::DecodeHeaderFailed { .. }
            | AuthError::DecodeKidHeaderEmpty { .. }
            | AuthError::KidNotFoundInJWKs { .. }
            | AuthError::JwkPublicKeyFailed { .. }
            | AuthError::JwkAlgorithmFailed { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            AuthError::DecodeVerifyFailed { .. } | AuthError::AuthorizationHeaderNotFound(_) => {
                StatusCode::UNAUTHORIZED
            }
        };

        // 内容を生成する
        let response_body = self.to_string();

        // レスポンスを生成する
        (status_code, response_body).into_response()
    }
}
