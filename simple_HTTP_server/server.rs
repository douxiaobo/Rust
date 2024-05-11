use std::io::{Read, Write};  
use std::net::TcpListener;  
use std::net::TcpStream;  
use std::str;  
  
fn main() {  
    // 创建一个TCP监听器，监听本地8080端口  
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();  
  
    // 无限循环，等待客户端连接  
    for stream in listener.incoming() {  
        // 匹配连接结果，如果成功则处理连接  
        match stream {  
            Ok(stream) => handle_connection(stream),  
            Err(_e) => { /* 连接错误处理，这里我们简单地忽略它 */ },  
        }  
    }  
}  
  
fn handle_connection(mut stream: TcpStream) {  
    // 创建一个缓冲区来存储接收到的数据  
    let mut buffer = [0u8; 1024];  
  
    // 读取客户端发送的数据  
    stream.read(&mut buffer).unwrap();  
  
    // 将缓冲区中的数据转换为字符串（注意：这可能不是有效的UTF-8）  
    let _request_str = str::from_utf8(&buffer).unwrap_or_else(|_| "<invalid UTF-8>");  
  
    // 响应客户端（这里我们简单地发送一个固定的响应）  
    let response = "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\nHello, World!";  
    stream.write_all(response.as_bytes()).unwrap();  
  
    // 关闭连接  
    stream.flush().unwrap();  
    stream.shutdown(std::net::Shutdown::Both).unwrap();  
}  

// curl http://127.0.0.1:8080
// Hello, World!%                     