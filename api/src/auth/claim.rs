use std::str::FromStr;

use axum::extract::{FromRef, FromRequestParts};
use axum::http::request::Parts;
use axum::{async_trait, RequestPartsExt};
use axum_extra::headers::authorization::Bearer;
use axum_extra::headers::Authorization;
use axum_extra::TypedHeader;
use chrono::{serde::ts_seconds, DateTime, Utc};
use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use serde::Deserialize;

use crate::auth::errors::AuthError;
use crate::auth::errors::AuthError::JwkAlgorithmFailed;
use crate::auth::fetch::fetch;
use crate::auth::jwk::{Jwk, Jwks};
use crate::{ApiConfig, AppState};

#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone)]
pub struct Claim {
    #[serde(with = "ts_seconds")]
    exp: DateTime<Utc>,
    #[serde(with = "ts_seconds")]
    iat: DateTime<Utc>,
    aud: String,
    iss: String,
    sub: String,
    #[serde(with = "ts_seconds")]
    auth_time: DateTime<Utc>,
}

impl Claim {
    pub fn get_user_id(&self) -> String {
        self.sub.clone()
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for Claim
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        // get token from header
        let bearer: TypedHeader<Authorization<Bearer>> = parts
            .extract()
            .await
            .map_err(|e| AuthError::AuthorizationHeaderNotFound(e))?;

        // env設定値を取得する
        let env_config = AppState::from_ref(state).api_config;

        // kidを取得する
        let header =
            decode_header(bearer.token()).map_err(|e| AuthError::DecodeHeaderFailed { e })?;
        let kid = header.kid.ok_or(AuthError::DecodeKidHeaderEmpty)?;

        // jwksを取得する
        let jwks: Jwks = fetch(&env_config.cognito_jwks_url).await?;

        // kidからjwkを特定する
        let jwk = jwks
            .find(kid.as_str())
            .ok_or(AuthError::KidNotFoundInJWKs { jwks })?;

        // tokenをデコードする
        decode_token(&env_config, jwk, bearer.token())
    }
}

fn decode_token(env_config: &ApiConfig, jwk: Jwk, token: &str) -> Result<Claim, AuthError> {
    let decoding_key = DecodingKey::from_rsa_components(&jwk.n, &jwk.e).map_err(|e| {
        AuthError::JwkPublicKeyFailed {
            e,
            jwk: jwk.clone(),
        }
    })?;
    let algorithm = Algorithm::from_str(&jwk.alg).map_err(|e| JwkAlgorithmFailed {
        e,
        jwk: jwk.clone(),
    })?;
    let mut validation = Validation::new(algorithm);
    validation.set_audience(&[env_config.cognito_client_id.as_str()]);

    let claim = decode::<Claim>(token, &decoding_key, &validation)
        .map(|token_data| token_data.claims)
        .map_err(|e| AuthError::DecodeVerifyFailed { e });
    claim
}
