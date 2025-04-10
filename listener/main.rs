use std::net::{TcpListener, TcpStream};
use std::io::Read;

fn main() {
    let server : TcpListener = match TcpListener::bind("127.0.0.1:8080") {
        Ok(value) => value,
        Err(_) => panic!("Aborted due to used port")
    };

    for stream in server.incoming() {
        let mut buffer : TcpStream = stream.unwrap();

        let mut message_bytes : Vec<u8> = Vec::new();

        buffer.read_to_end(&mut message_bytes).unwrap();

        let message : String = String::from_utf8(message_bytes).unwrap();

        println!("{}", message);
    }
}
