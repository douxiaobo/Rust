use std::net::UdpSocket;
use std::io::{self,Write};

fn main() {
    let socket = UdpSocket::bind("127.0.0.1:0").unwrap();

    print!("Enter temperature in Celsius: ");
    std::io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    input=input.trim().to_string();

    socket.send_to(input.as_bytes(), "127.0.0.1:8888").expect("send_to failed");

    let mut buf = [0u8; 1024];
    let amt=socket.recv(&mut buf).expect("recv failed");
    let buf=&mut buf[..amt];
    println!("Received: {}", std::str::from_utf8(buf).expect("Invalid UTF-8"));
    
}