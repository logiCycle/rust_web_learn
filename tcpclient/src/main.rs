use std::{
    io::{Read, Write},
    net::TcpStream,
};
fn main() {
    let mut stream = TcpStream::connect("159.75.96.101:7879").unwrap();
    stream.write("hello".as_bytes()).unwrap();

    let mut buffer = [0; 5];
    stream.read(&mut buffer).unwrap();

    println!(
        "Received from Server:{:?}",
        String::from_utf8_lossy(&buffer)
    );
}
