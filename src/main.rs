use axum::{
    routing::get,
    Router,
};
use axum_server::tls_rustls::RustlsConfig;
use std::{net::SocketAddr, path::PathBuf};

#[tokio::main]
async fn main() {

    let port: u16 = 3000;
    let config = RustlsConfig::from_pem_file(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("self_signed_certs")
            .join("cert.pem"),
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("self_signed_certs")
            .join("key.pem"),
    )
    .await
    .unwrap();

    let app = Router::new().route("/", get(handler));
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    println!("Server online: {addr}");
    axum_server::bind_rustls(addr, config)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[allow(dead_code)]
async fn handler() -> &'static str {
    "TLS Connection successfull!"
}
