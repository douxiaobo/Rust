use std::net::TcpStream;
use std::io::{Write, Read};
use std::str;

fn main(){
    let mut stream = TcpStream::connect(("127.0.0.1", 8080)).expect("Failed to connect to server");

    println!("Hi, Here is the Client!");

    println!("Enter the message to send:");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let send_data = input.trim().to_string();
    stream.write_all(send_data.as_bytes()).expect("Failed to send data");

    let mut buffer = [0u8; 1024];  
    let bytes_read = stream.read(&mut buffer).expect("读取数据失败");  
 
    let response = str::from_utf8(&buffer[..bytes_read]).expect("接收到的数据不是有效的 UTF-8");  
    println!("Received message from server: {}", response);  

    drop(stream);
}