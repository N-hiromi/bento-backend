extern crate core;

use crate::commons::api_config::ApiConfig;
use crate::controllers::index_controller::{index, index_path};
use axum::routing::get;
use axum::Router;
use common::repositories::repository::Repository;
use std::env;

mod auth;
mod commons;
mod controllers;
mod requests;
mod responses;
mod services;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 環境名を取得する
    let env = env::var("ENV")?;
    // Stateを作成する
    let app_state = AppState {
        api_config: ApiConfig::new(&env).await?,
        repository: Repository::new(&env).await,
    };
    // Routerを作成する
    let router = create_router(app_state);
    // 起動設定を行う
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    // サーバを起動する
    axum::serve(listener, router).await?;
    Ok(())
}

fn create_router(app_state: AppState) -> Router<()> {
    Router::new()
        .route(index_path(), get(index))
        .with_state(app_state)
}

#[derive(Clone)]
pub struct AppState {
    pub api_config: ApiConfig,
    pub repository: Repository,
}
