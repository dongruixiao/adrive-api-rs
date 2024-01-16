use adrive_api::data_structures::auth::*;
use adrive_api::data_structures::Request;
use axum::routing::{get, post};
use axum::Json;
use axum::Router;
use serde::Deserialize;
use std::env;

async fn sid() -> Json<GetQRCodeResponse> {
    let client_id = env::var("ADRIVE_CLIENT_ID").unwrap();
    let client_secret = env::var("ADRIVE_CLIENT_SECRET").unwrap();
    let resp = GetQRCodeRequest::new(&client_id, &client_secret)
        .dispatch(None, None)
        .await
        .unwrap();
    Json(resp)
}

#[derive(Deserialize)]
struct AuthCodePayload {
    auth_code: String,
}

async fn token(Json(payload): Json<AuthCodePayload>) -> Json<GetAccessTokenResponse> {
    let client_id = env::var("ADRIVE_CLIENT_ID").unwrap();
    let client_secret = env::var("ADRIVE_CLIENT_SECRET").unwrap();
    let resp =
        GetAccessTokenRequest::new(&client_id, &client_secret, Some(&payload.auth_code), None)
            .dispatch(None, None)
            .await
            .unwrap();
    Json(resp)
}

#[derive(Deserialize)]
struct RefreshTokenPayload {
    refresh_token: String,
}

async fn refresh_token(payload: Json<RefreshTokenPayload>) -> Json<GetAccessTokenResponse> {
    let client_id = env::var("ADRIVE_CLIENT_ID").unwrap();
    let client_secret = env::var("ADRIVE_CLIENT_SECRET").unwrap();
    let resp = GetAccessTokenRequest::new(
        &client_id,
        &client_secret,
        None,
        Some(&payload.refresh_token),
    )
    .dispatch(None, None)
    .await
    .unwrap();
    Json(resp)
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/sid", get(sid))
        .route("/token", post(token))
        .route("/refresh_token", post(refresh_token));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:10024")
        .await
        .unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
