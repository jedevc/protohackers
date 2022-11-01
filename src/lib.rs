use std::{env, net::SocketAddr};

pub fn bind_addr() -> SocketAddr {
    let host = env::var("HOST").unwrap_or("127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or("5000".to_string());
    SocketAddr::new(host.parse().unwrap(), port.parse().unwrap())
}
