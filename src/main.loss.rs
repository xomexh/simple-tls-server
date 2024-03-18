use rustls::{Certificate, ServerConfig};
use std::fs;
use std::sync::Arc;
use rustls::server::NoClientAuth;

fn main() {
    // Load the server certificate
    let cert_file = "path/to/server.crt";
    let cert_chain = {
        let cert_bytes = fs::read(cert_file).expect("Failed to read certificate file");
        vec![Certificate(cert_bytes)]
    };

    // Load the private key
    let key_file = "path/to/server.key";
    let key_bytes = fs::read(key_file).expect("Failed to read private key file");

    // Build TLS configuration
    let mut config = ServerConfig::new(Arc::new(rustls::NoClientAuth::new()));
    config
        .set_single_cert(cert_chain, key_bytes)
        .expect("Failed to set certificate and private key");

    // Configure TLS 1.2
    config.versions.clear();
    config.versions.push(rustls::ProtocolVersion::TLSv1_2);

    // Create a new Arc wrapped ServerConfig
    let server_config = Arc::new(config);

    // Now you can use `server_config` to configure your TLS server
}
