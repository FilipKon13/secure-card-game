use std::net::{TcpListener, TcpStream};

use crate::connection::Connection;

pub struct ConStartup {
    num_players: u32,
    player_id: u32,
}

const ADDRESS: &str = "127.0.0.1:";
const PORT_BASE: u32 = 6700;

impl ConStartup {
    pub fn new(num_players: u32, player_id: u32) -> Self {
        assert!(player_id < num_players);
        ConStartup {
            num_players,
            player_id,
        }
    }

    fn start_server(&self) -> TcpStream {
        let address = String::from(ADDRESS) + &PORT_BASE.to_string();
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

    fn start_client(&self) -> TcpStream {
        loop {
            let address = String::from(ADDRESS) + &PORT_BASE.to_string();
            let result = TcpStream::connect(address);
            if result.is_ok() {
                let stream = result.unwrap();
                return stream;
            }
        }
    }

    pub fn initialize(&self) -> Connection {
        if self.player_id == 0 {
            let stream = self.start_server();
            Connection::new(stream)
        } else {
            let stream = self.start_client();
            Connection::new(stream)
        }
    }
}
