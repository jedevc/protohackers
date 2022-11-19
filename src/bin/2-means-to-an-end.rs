use env_logger;
use std::{
    error::Error,
    io::{ErrorKind, Read, Write},
    net::TcpStream,
};

use protohackers as ph;
use protohackers::means_to_an_end::{Request, Response, DB};

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let addr = ph::bind_addr();
    ph::launch_tcp_server(addr, handle_client)?;

    Ok(())
}

fn handle_client(mut stream: &TcpStream) -> Result<(), Box<dyn Error>> {
    let mut db = DB::new();
    loop {
        let mut buf = [0u8; 9];
        if let Err(e) = stream.read_exact(&mut buf) {
            if e.kind() == ErrorKind::Interrupted {
                break;
            }
            return Err(e.into());
        }

        let req = Request::parse(&buf).ok_or("failed to parse request")?;
        match req {
            Request::Insert { timestamp, price } => {
                db.insert(timestamp, price);
            }
            Request::Query { low, high } => {
                let resp = Response {
                    result: db.query(low, high),
                };
                stream.write_all(&resp.output())?;
            }
        };
    }

    Ok(())
}
