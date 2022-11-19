use env_logger;
use std::{error::Error, io::Read, io::Write, net::TcpStream};

use protohackers as ph;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let addr = ph::bind_addr();
    ph::launch_tcp_server(addr, handle_client)?;

    Ok(())
}

fn handle_client(mut stream: &TcpStream) -> Result<(), Box<dyn Error>> {
    let mut buff = [0u8; 1024];
    loop {
        let n = stream.read(&mut buff)?;
        if n == 0 {
            break;
        }
        stream.write_all(&buff[..n])?;
    }
    Ok(())
}
