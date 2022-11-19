use env_logger;
use log::{debug, info};
use std::{collections::HashMap, error::Error, net::UdpSocket, str};

use protohackers as ph;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let addr = ph::bind_addr();
    info!("listening on {}", addr);
    let socket = UdpSocket::bind(addr)?;
    handle(socket)?;

    Ok(())
}

fn handle(socket: UdpSocket) -> Result<(), Box<dyn Error>> {
    let mut db: HashMap<Vec<u8>, Vec<u8>> = HashMap::new();
    db.insert(b"version".to_vec(), b"Ken's Key-Value Store 1.0".to_vec());

    let mut buf = vec![0u8; 1000];
    loop {
        let (mut n, addr) = socket.recv_from(&mut buf)?;
        debug!("received message {}", str::from_utf8(&buf[..n])?);

        match buf[..n].iter().position(|c| *c == b'=') {
            Some(idx) => {
                let (k, v) = buf[..n].split_at(idx);
                let v = &v[1..];
                if k != b"version" {
                    db.insert(k.to_vec(), v.to_vec());
                }
            }
            None => {
                buf[n] = b'=';
                if let Some(v) = db.get(&buf[..n]) {
                    buf[n + 1..n + 1 + v.len()].copy_from_slice(v);
                    n += v.len();
                }
                n += 1;

                debug!("sending message {}", str::from_utf8(&buf[..n])?);
                socket.send_to(&buf[..n], &addr)?;
            }
        }
    }
}
