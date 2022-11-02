use log::{error, info};
use std::io;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::{env, net::SocketAddr};

pub fn launch_tcp_server(
    addr: SocketAddr,
    handle_client: fn(TcpStream) -> io::Result<()>,
) -> io::Result<()> {
    info!("listening on {}", addr);
    let listener = TcpListener::bind(addr)?;

    for stream in listener.incoming() {
        thread::spawn(move || {
            if let Err(e) = stream
                .and_then(|stream| {
                    info!("received connection from {}", stream.peer_addr()?);
                    Ok(stream)
                })
                .and_then(handle_client)
            {
                error!("{}", e);
            }
        });
    }
    Ok(())
}

pub fn bind_addr() -> SocketAddr {
    let host = env::var("HOST").unwrap_or("127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or("5000".to_string());
    SocketAddr::new(host.parse().unwrap(), port.parse().unwrap())
}

/// # Examples
///
/// ```
/// assert_eq!(protohackers::is_prime(0), false);
/// assert_eq!(protohackers::is_prime(1), false);
/// assert_eq!(protohackers::is_prime(2), true);
/// assert_eq!(protohackers::is_prime(3), true);
/// assert_eq!(protohackers::is_prime(4), false);
/// assert_eq!(protohackers::is_prime(5), true);
/// assert_eq!(protohackers::is_prime(6), false);
/// assert_eq!(protohackers::is_prime(7), true);
/// ```
pub fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    let cap = (n as f64).sqrt() as u64 + 1;
    !(2..cap).any(|i| n % i == 0)
}
