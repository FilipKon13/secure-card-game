use std::io::prelude::*;
use std::net::TcpStream;

use serde::de::DeserializeOwned;
use serde::Serialize;

pub trait Connection {
    fn send<T: Serialize>(&mut self, message: &T);
    fn receive<T: DeserializeOwned>(&mut self) -> T;
}

pub struct TcpConnection {
    stream: TcpStream,
}

impl TcpConnection {
    pub fn new(stream: TcpStream) -> Self {
        TcpConnection { stream }
    }
    fn read_u32(&mut self) -> u32 {
        let mut buffer = [0; 4];
        self.stream.read_exact(&mut buffer).unwrap();
        u32::from_le_bytes(buffer)
    }
}

impl Connection for TcpConnection {
    fn send<T: Serialize>(&mut self, message: &T) {
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

    fn receive<T: DeserializeOwned>(&mut self) -> T {
        let length = self.read_u32();
        let mut buffer = vec![0_u8; length as usize];
        self.stream.read_exact(&mut buffer).unwrap();
        let serialized = String::from_utf8_lossy(buffer.as_slice()).to_string();
        let deserialized: T = serde_json::from_str(&serialized).unwrap();
        deserialized
    }
}
