use std::io::{ Read, Write};  
use std::net::TcpStream;  
use std::str;  
  
fn main() {  
    let host = "127.0.0.1";  
    let port = 8080;  
  
    // 连接到服务器  
    let addr = format!("{}:{}", host, port);  
    let mut stream = TcpStream::connect(addr).expect("无法连接到服务器");  
  
    // 发送数据  
    let message = "Hi, Here is the Client!\n";  
    stream.write_all(message.as_bytes()).expect("发送数据失败");  
  
    // 读取服务器的响应  
    let mut buffer = [0u8; 1024];  
    let bytes_read = stream.read(&mut buffer).expect("读取数据失败");  
  
    // 打印接收到的消息  
    let response = str::from_utf8(&buffer[..bytes_read]).expect("接收到的数据不是有效的 UTF-8");  
    println!("Received message from server: {}", response);  
  
    // 关闭套接字  
    drop(stream); // Rust 的 RAII (Resource Acquisition Is Initialization) 机制确保在变量离开作用域时关闭连接  
}