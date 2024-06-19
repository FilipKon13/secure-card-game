use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

use serde::de::DeserializeOwned;
use serde::Serialize;

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

    pub fn send<T: Serialize>(&mut self, message: &T) {
        let serialized = serde_json::to_string(&message).unwrap();
        let stream = self.stream.as_mut().unwrap();
        let length = serialized.len() as u32;
        let lenght_bytes = length.to_le_bytes();
        stream.write_all(&lenght_bytes).unwrap();
        stream.write_all(serialized.as_bytes()).unwrap();
    }

    fn read_u32(&mut self) -> u32 {
        let stream = self.stream.as_mut().unwrap();
        let mut buffer = [0; 4];
        stream.read_exact(&mut buffer).unwrap();
        u32::from_le_bytes(buffer)
    }

    pub fn receive<T: DeserializeOwned>(&mut self) -> T {
        let length = self.read_u32();
        let stream = self.stream.as_mut().unwrap();
        let mut buffer = vec![0 as u8; length as usize];
        stream.read_exact(&mut buffer).unwrap();
        let serialized = String::from_utf8_lossy(&buffer.as_slice()).to_string();
        let deserialized: T = serde_json::from_str(&serialized).unwrap();
        deserialized
    }
}

#[cfg(test)]
mod test {
    use crate::message::Message;
    use std::{thread, time::Duration};

    use super::*;

    #[test]
    fn test() {
        thread::spawn(move || {
            let mut server = Communication::new();
            println!("starting server");
            server.start_server();

            let msg: Message = server.receive();
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

        let msg: Message = client.receive();
        println!("client received: {} {}", msg.x, msg.s);
        assert_eq!(msg.x, 7);
    }
}
