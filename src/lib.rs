use log::{error, info};
use std::error::Error;
use std::io;
use std::net::{Shutdown, TcpListener, TcpStream};
use std::thread;
use std::{env, net::SocketAddr};

pub mod budget_chat;
pub mod means_to_an_end;
pub mod prime_time;

pub fn launch_tcp_server<F>(addr: SocketAddr, handle_client: F) -> io::Result<()>
where
    F: FnMut(&TcpStream) -> Result<(), Box<dyn Error>> + Clone + Send + 'static,
{
    info!("listening on {}", addr);
    let listener = TcpListener::bind(addr)?;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                info!("received connection from {}", stream.peer_addr()?);
                let mut handle_client = handle_client.clone();
                thread::spawn(move || {
                    if let Err(e) = handle_client(&stream) {
                        error!("{}", e);
                    }
                    _ = stream.shutdown(Shutdown::Both);
                });
            }
            Err(e) => {
                error!("{}", e);
            }
        };
    }
    Ok(())
}

pub fn bind_addr() -> SocketAddr {
    let host = env::var("HOST").unwrap_or("127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or("5000".to_string());
    SocketAddr::new(host.parse().unwrap(), port.parse().unwrap())
}
