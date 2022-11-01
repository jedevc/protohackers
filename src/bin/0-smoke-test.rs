use env_logger;
use log::{error, info};
use std::{
    io::Read,
    io::Write,
    net::{Shutdown, TcpListener, TcpStream},
};

use protohackers as ph;

fn main() -> std::io::Result<()> {
    env_logger::init();

    let addr = ph::bind_addr();
    info!("listening on {}", addr);
    let listener = TcpListener::bind(addr)?;

    for stream in listener.incoming() {
        if let Err(e) = stream
            .and_then(|stream| {
                info!("received connection from {}", stream.peer_addr()?);
                Ok(stream)
            })
            .and_then(handle_client)
        {
            error!("{}", e)
        }
    }
    Ok(())
}

fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    let mut buff = [0u8; 1024];
    loop {
        let n = stream.read(&mut buff)?;
        if n == 0 {
            break;
        }
        stream.write(&buff[..n])?;
    }
    stream.shutdown(Shutdown::Both)?;
    Ok(())
}
