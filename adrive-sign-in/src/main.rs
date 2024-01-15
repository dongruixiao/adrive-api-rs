use adrive_api::data_structures::auth::*;
use adrive_api::data_structures::Request;
use axum::routing::{get, post};
use axum::Json;
use axum::Router;
use serde::Deserialize;
use std::env;

async fn sid() -> String {
    let client_id = env::var("ADRIVE_CLIENT_ID").unwrap();
    let client_secret = env::var("ADRICE_CLIENT_SECRET").unwrap();
    let resp = GetQRCodeRequest::new(&client_id, &client_secret)
        .dispatch(None, None)
        .await
        .unwrap();
    resp.sid
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

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/sid", get(sid))
        .route("/token", post(token));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:10024")
        .await
        .unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
