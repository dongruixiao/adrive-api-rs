use crate::data::{
    GetAccessTokenRequest, GetAccessTokenRequest2, GetAccessTokenRequest3, GetAccessTokenResponse,
    GetQRCodeRequest, GetQRCodeResponse, Request,
};
use axum::routing::{get, post};
use axum::Json;
use axum::Router;
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

async fn token(Json(payload): Json<GetAccessTokenRequest2>) -> Json<GetAccessTokenResponse> {
    let client_id = env::var("ADRIVE_CLIENT_ID").unwrap();
    let client_secret = env::var("ADRIVE_CLIENT_SECRET").unwrap();
    let resp =
        GetAccessTokenRequest::new(&client_id, &client_secret, Some(&payload.auth_code), None)
            .dispatch(None, None)
            .await
            .unwrap();
    Json(resp)
}

async fn refresh_token(payload: Json<GetAccessTokenRequest3>) -> Json<GetAccessTokenResponse> {
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

pub fn app() -> Router {
    Router::new()
        .route("/sid", get(sid))
        .route("/token", post(token))
        .route("/refresh_token", post(refresh_token))
}
