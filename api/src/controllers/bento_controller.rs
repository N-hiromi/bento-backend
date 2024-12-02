use crate::auth::claim::Claim;
use crate::controllers::controller::{to_error_response, ApiRepositoryError};
use crate::requests::bento_request::BentoRequest;
use crate::responses::bento_get_response::BentoGetResponse;
use crate::responses::bento_list_response::BentoListResponse;
use crate::services::{bento_favorite_list_service, bento_get_service, bento_list_service};
use crate::AppState;
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::routing::{delete, get};
use axum::{Json, Router};
use common::repositories::bento_repository::BentoRepository;

// routing -----------------------------------------------------
pub fn bento_router(app_state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", get(list_bentos))
        .route("/:bento_id", get(get_bento))
        .route("/favorite", get(bento_favorite_list))
        .route("/:bento_id", delete(delete_bento))
        .route("/", post(create_bento))
        .route("/:bento_id", put(update_bento))
        .with_state(app_state)
}

// list_bentos -----------------------------------------------------
async fn list_bentos(State(state): State<AppState>, claim: Claim) -> impl IntoResponse {
    match bento_list_service::execute(&state.repository, claim.get_user_id()).await {
        Ok(bentos) => Ok(Json(
            bentos
                .into_iter()
                .map(BentoListResponse::from)
                .collect::<Vec<BentoListResponse>>(),
        )),
        Err(e) => Err(to_error_response(ApiRepositoryError { e })),
    }
}

// get_bento -----------------------------------------------------
async fn get_bento(
    State(state): State<AppState>,
    claim: Claim,
    Path(bento_id): Path<&str>,
) -> impl IntoResponse {
    match bento_get_service::execute(state.repository, bento_id).await {
        Ok(bento) => Ok(Json(BentoGetResponse::from(bento))),
        Err(e) => Err(to_error_response(e)),
    }
}

// list_favorite_bentos -----------------------------------------------------
async fn bento_favorite_list(State(state): State<AppState>, claim: Claim) -> impl IntoResponse {
    match bento_favorite_list_service::execute(state.repository, &*claim.get_user_id()).await {
        Ok(bentos) => Ok(Json(
            bentos
                .into_iter()
                .map(BentoListResponse::from)
                .collect::<Vec<BentoListResponse>>(),
        )),
        Err(e) => Err(to_error_response(e)),
    }
}

// delete_bentos -----------------------------------------------------
async fn delete_bento(
    State(state): State<AppState>,
    claim: Claim,
    Path(bento_id): Path<&str>,
) -> impl IntoResponse {
    match state.repository.delete(bento_id).await {
        Ok(..) => Ok(()),
        Err(e) => Err(to_error_response(ApiRepositoryError { e })),
    }
}

// create_bentos -----------------------------------------------------
async fn create_bento(
    State(state): State<AppState>,
    claim: Claim,
    Json(req): Json<BentoRequest>,
) -> impl IntoResponse {
    match create_bento_service::execute(state.repository, claim.get_user_id(), body.into()).await {
        Ok() => Ok(()),
        Err(e) => Err(to_error_response(e)),
    }
}
