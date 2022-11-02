use env_logger;
use std::{
    io::Read,
    io::Write,
    net::{Shutdown, TcpStream},
};

use protohackers as ph;

fn main() -> std::io::Result<()> {
    env_logger::init();

    let addr = ph::bind_addr();
    ph::launch_tcp_server(addr, handle_client)
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
