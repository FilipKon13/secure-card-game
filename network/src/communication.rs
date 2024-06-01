use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

use crate::message::Message;

const ADDRESS: &str = "127.0.0.1:6789";

pub struct Communication {
    is_server: bool,
    stream: Option<TcpStream>,
}

impl Communication {
    pub fn new() -> Self {
        Communication {
            is_server: false,
            stream: None,
        }
    }

    pub fn start_client(&mut self) {
        self.is_server = false;
        let stream = TcpStream::connect(ADDRESS).unwrap();
        self.stream = Some(stream);
    }

    pub fn start_server(&mut self) {
        self.is_server = true;
        let listener = TcpListener::bind(ADDRESS).unwrap();
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            self.stream = Some(stream);
            break;
        }
    }

    pub fn send(&mut self, message: &Message) {
        let serialized = serde_json::to_string(&message).unwrap();
        let stream = self.stream.as_mut().unwrap();
        stream.write(serialized.as_bytes()).unwrap();
    }

    pub fn receive(&mut self) -> Message {
        let stream = self.stream.as_mut().unwrap();
        let mut buffer = [0; 512];
        let size = stream.read(&mut buffer).unwrap();
        let serialized = String::from_utf8_lossy(&buffer[0..size]).to_string();
        let deserialized: Message = serde_json::from_str(&serialized).unwrap();
        deserialized
    }
}

#[cfg(test)]
mod test {
    use std::{thread, time::Duration};

    use super::*;

    #[test]
    fn test() {
        thread::spawn(move || {
            let mut server = Communication::new();
            println!("starting server");
            server.start_server();

            let msg = server.receive();
            println!("server received: {} {}", msg.x, msg.s);
            assert_eq!(msg.x, 5);

            server.send(&Message {
                x: 7,
                s: String::from("hello2"),
            });
        });

        thread::sleep(Duration::from_millis(100));

        let mut client = Communication::new();
        client.start_client();

        client.send(&Message {
            x: 5,
            s: String::from("hello1"),
        });

        let msg = client.receive();
        println!("client received: {} {}", msg.x, msg.s);
        assert_eq!(msg.x, 7);
    }
}
