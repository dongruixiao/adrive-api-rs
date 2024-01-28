use adrive_api_rs::self_hosting_app;

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:10024")
        .await
        .unwrap();
    axum::serve(listener, self_hosting_app().into_make_service())
        .await
        .unwrap();
}
