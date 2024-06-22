pub mod con_startup;
pub mod connection;
pub mod message;

#[cfg(test)]
mod test {
    use super::{con_startup::ConStartup, message::Message};

    use std::thread;

    const ADDRESS: &str = "127.0.0.1:";
    const PORT_BASE: u32 = 6700;

    fn address() -> String {
        ADDRESS.to_string() + &PORT_BASE.to_string()
    }

    #[test]
    fn test() {
        thread::spawn(move || {
            let con_startup = ConStartup::new(2, 0);
            let mut connection = con_startup.initialize(&address());

            let msg: Message = connection.receive();
            println!("server received: {} {}", msg.x, msg.s);
            assert_eq!(msg.x, 5);

            connection.send(&Message {
                x: 7,
                s: String::from("helloo2"),
            });
        });

        let con_startup = ConStartup::new(2, 1);
        let mut connection = con_startup.initialize(&address());

        connection.send(&Message {
            x: 5,
            s: String::from("hello1"),
        });

        let msg: Message = connection.receive();
        println!("client received: {} {}", msg.x, msg.s);
        assert_eq!(msg.x, 7);
    }
}
