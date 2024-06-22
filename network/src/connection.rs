use std::io::prelude::*;
use std::net::TcpStream;

use serde::de::DeserializeOwned;
use serde::Serialize;

pub struct Connection {
    stream: TcpStream,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Self {
        Connection { stream }
    }

    pub fn send<T: Serialize>(&mut self, message: &T) {
        let serialized = serde_json::to_string(&message).unwrap();
        let length = serialized.len() as u32;
        let lenght_bytes = length.to_le_bytes();
        let mut buffer = vec![0; 4 + length as usize];
        lenght_bytes
            .chain(serialized.as_bytes())
            .read_exact(&mut buffer)
            .unwrap();
        self.stream.write_all(&buffer).unwrap();
    }

    fn read_u32(&mut self) -> u32 {
        let mut buffer = [0; 4];
        self.stream.read_exact(&mut buffer).unwrap();
        u32::from_le_bytes(buffer)
    }

    pub fn receive<T: DeserializeOwned>(&mut self) -> T {
        let length = self.read_u32();
        let mut buffer = vec![0_u8; length as usize];
        self.stream.read_exact(&mut buffer).unwrap();
        let serialized = String::from_utf8_lossy(buffer.as_slice()).to_string();
        let deserialized: T = serde_json::from_str(&serialized).unwrap();
        deserialized
    }
}
