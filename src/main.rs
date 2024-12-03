use std::net::TcpStream;

fn main() {
    let stream = TcpStream::connect("127.0.0.1:8080");
    println!("Hello, world!");
}
