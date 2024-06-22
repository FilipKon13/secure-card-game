use std::net::{TcpListener, TcpStream};

use crate::connection::Connection;

pub struct ConStartup {
    player_id: u32,
}

impl ConStartup {
    pub fn new(num_players: u32, player_id: u32) -> Self {
        assert!(player_id < num_players);
        ConStartup { player_id }
    }

    fn start_server(&self, address: &String) -> TcpStream {
        let listener = TcpListener::bind(address).unwrap();
        let mut stream_opt: Option<TcpStream> = None;
        for stream in listener.incoming() {
            if stream.is_ok() {
                stream_opt = Some(stream.unwrap());
                break;
            }
        }
        stream_opt.unwrap()
    }

    fn start_client(&self, address: &String) -> TcpStream {
        loop {
            let result = TcpStream::connect(address);
            if result.is_ok() {
                let stream = result.unwrap();
                return stream;
            }
        }
    }

    pub fn initialize(&self, address: &String) -> Connection {
        if self.player_id == 0 {
            let stream = self.start_server(address);
            Connection::new(stream)
        } else {
            let stream = self.start_client(address);
            Connection::new(stream)
        }
    }
}
