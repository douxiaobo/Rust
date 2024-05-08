use std::net::TcpStream;
use std::io::{Write, Read};
//代码不行
fn main() {
    let mut stream = TcpStream::connect(("127.0.0.1", 8080)).expect("Failed to connect to server");

    println!("Hi, Here is the Client!");

    println!("Enter the message to send:");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let send_data = input.trim().to_string();

    stream.write_all(send_data.as_bytes()).expect("Failed to send data");

    let mut recv_data = [0; 1024];
    stream.read_exact(&mut recv_data).expect("Failed to receive data");
    let recv_data_str = String::from_utf8(recv_data.to_vec()).expect("Invalid UTF-8 received");

    println!("Received message from server: {}", recv_data_str);

    stream.shutdown(std::net::Shutdown::Both).expect("Failed to shutdown the connection");
}