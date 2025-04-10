use std::net::TcpStream;
use std::io::Write;
use std::env::args;

fn main() {
    let mut tx : TcpStream = match TcpStream::connect("127.0.0.1:8080") {
        Ok(value) => value,
        Err(_) => panic!("Could not connect to the provided adress")
    };

    let args : Vec<String> = args().collect::<Vec<String>>();

    tx.write_all(args[1].as_bytes()).unwrap();

}
