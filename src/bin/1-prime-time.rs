use env_logger;
use log::debug;
use serde::{Deserialize, Serialize};
use std::{
    io::{self, BufRead, Write},
    net::{Shutdown, TcpStream},
};

use protohackers as ph;

#[derive(Serialize, Deserialize)]
struct Request {
    method: String,
    number: serde_json::Number,
}

#[derive(Serialize, Deserialize)]
struct Response {
    method: String,
    prime: bool,
}

fn main() -> std::io::Result<()> {
    env_logger::init();

    let addr = ph::bind_addr();
    ph::launch_tcp_server(addr, handle_client)
}

fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    let rstream = io::BufReader::new(stream.try_clone()?);
    for line in rstream.lines() {
        let line = line?;
        debug!("got line {}", line);
        match serde_json::from_str::<Request>(&line) {
            Ok(req) if req.method != "isPrime" => {
                debug!("unknown method {}", req.method);
                stream.write("{}".as_bytes())?;
            }
            Ok(req) => {
                let resp = Response {
                    method: req.method,
                    prime: req.number.as_u64().map(ph::is_prime).unwrap_or(false),
                };
                serde_json::to_writer(&stream, &resp)?;
            }
            Err(e) => {
                debug!("{}", e);
                stream.write("{}".as_bytes())?;
            }
        }
        stream.write("\n".as_bytes())?;
    }
    stream.shutdown(Shutdown::Both)?;
    Ok(())
}
