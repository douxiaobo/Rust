use std::io::{Read, Write};  
use std::net::TcpStream;  
  
fn main() {  
    // 连接到服务器  
    let mut stream = TcpStream::connect("127.0.0.1:8080").unwrap();  
  
    // 构建HTTP GET请求  
    let request = "GET / HTTP/1.1\r\nHost: 127.0.0.1:8080\r\n\r\n";  
  
    // 发送请求到服务器  
    stream.write_all(request.as_bytes()).unwrap();  
  
    // 读取服务器的响应  
    let mut buffer = [0u8; 1024];  
    stream.read(&mut buffer).unwrap();  
  
    // 打印服务器的响应  
    let response = std::str::from_utf8(&buffer).unwrap_or_else(|_| "<invalid UTF-8>");  
    println!("{}", response);  
  
    // 关闭连接  
    stream.shutdown(std::net::Shutdown::Both).unwrap();  
}

// ./client
// HTTP/1.1 200 OK
// Content-Type: text/plain

// Hello, World!
// thread 'main' panicked at client.rs:23:47:
// called `Result::unwrap()` on an `Err` value: Os { code: 57, kind: NotConnected, message: "Socket is not connected" }
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace