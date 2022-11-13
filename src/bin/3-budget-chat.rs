use env_logger;
use std::{
    error::Error,
    io::{self, BufRead, BufReader, LineWriter, Write},
    mem::drop,
    net::TcpStream,
    thread,
};

use protohackers as ph;
use protohackers::budget_chat::{is_legal_name, Message, Room};

fn main() -> std::io::Result<()> {
    env_logger::init();

    let addr = ph::bind_addr();
    ph::launch_tcp_server(addr, make_handle_client())
}

fn make_handle_client(
) -> impl FnMut(&TcpStream) -> Result<(), Box<dyn Error>> + Clone + Send + 'static {
    let mut room = Room::new();

    move |stream: &TcpStream| {
        let mut rstream = BufReader::new(stream.try_clone()?);
        let mut wstream = LineWriter::new(stream.try_clone()?);

        let mut name = String::new();
        wstream.write_all(b"Welcome to budgetchat! What shall I call you?\n")?;
        if rstream.read_line(&mut name)? == 0 {
            return Ok(());
        };
        let name = name.trim_end();
        if !is_legal_name(name) {
            return Ok(());
        }

        wstream
            .write_all(format!("* The room contains: {}\n", room.users().join(", ")).as_bytes())?;
        let (sender, receiver) = room.join(&name);
        sender.send(Message::Join {
            user: name.to_string(),
        })?;

        let th = thread::spawn(move || -> io::Result<()> {
            for msg in receiver {
                let msg = match msg {
                    Message::Join { user } => format!("* {} has entered the room\n", user),
                    Message::Leave { user } => format!("* {} has left the room\n", user),
                    Message::Chat { user, content } => format!("[{}] {}\n", user, content),
                };
                wstream.write_all(msg.as_bytes())?;
            }
            Ok(())
        });

        for line in rstream.lines() {
            let line = line?;
            let line = line.trim_end();
            sender.send(Message::Chat {
                user: name.to_string(),
                content: line.to_string(),
            })?;
        }
        sender.send(Message::Leave {
            user: name.to_string(),
        })?;
        drop(sender);

        th.join().unwrap()?;

        Ok(())
    }
}
