use env_logger;
use log::debug;
use std::{
    error::Error,
    io::{self, BufRead, Write},
    net::TcpStream,
};

use protohackers as ph;
use protohackers::prime_time::{is_prime, Request, Response};

fn main() -> std::io::Result<()> {
    env_logger::init();

    let addr = ph::bind_addr();
    ph::launch_tcp_server(addr, handle_client)
}

fn handle_client(mut stream: &TcpStream) -> Result<(), Box<dyn Error>> {
    let rstream = io::BufReader::new(stream.try_clone()?);
    for line in rstream.lines() {
        let line = line?;
        debug!("got line {}", line);
        match serde_json::from_str::<Request>(&line) {
            Ok(req) if req.method == "isPrime" => {
                let resp = Response {
                    method: req.method,
                    prime: req.number.as_u64().map(is_prime).unwrap_or(false),
                };
                serde_json::to_writer(stream, &resp)?;
                stream.write_all("\n".as_bytes())?;
            }
            Ok(req) => {
                debug!("unknown method {}", req.method);
                stream.write_all("{}\n".as_bytes())?;
                break;
            }
            Err(e) => {
                debug!("{}", e);
                stream.write_all("{}\n".as_bytes())?;
                break;
            }
        }
    }
    Ok(())
}
