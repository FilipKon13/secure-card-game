fn main() {}

#[cfg(test)]
mod test {
    use network::{con_startup::ConStartup, message::Message};

    use std::thread;

    #[test]
    fn test() {
        thread::spawn(move || {
            let con_startup = ConStartup::new(2, 0);
            let mut connection = con_startup.initialize();

            let msg: Message = connection.receive();
            println!("server received: {} {}", msg.x, msg.s);
            assert_eq!(msg.x, 5);

            connection.send(&Message {
                x: 7,
                s: String::from("helloo2"),
            });
        });

        let con_startup = ConStartup::new(2, 1);
        let mut connection = con_startup.initialize();

        connection.send(&Message {
            x: 5,
            s: String::from("hello1"),
        });

        let msg: Message = connection.receive();
        println!("client received: {} {}", msg.x, msg.s);
        assert_eq!(msg.x, 7);
    }
}
