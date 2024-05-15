use std::io::{Read, Write};  
use std::net::{TcpListener, TcpStream};  
use std::thread;  
  
const DATA: &str = "{\"testweb\": \"hello world！\"}";  
  
fn handle_client(mut stream: TcpStream) {  
    let mut buffer = [0u8; 1024];  
    // 读取请求（但在这个例子中我们不会真正处理它）  
    stream.read(&mut buffer).unwrap();  
  
    // 发送响应  
    let response = format!("HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{}", DATA);  
    stream.write_all(response.as_bytes()).unwrap();  
    stream.flush().unwrap();  
}  
  
fn main() {  
    let listener = TcpListener::bind("localhost:8080").unwrap();  
  
    println!("Starting server, listen at: localhost:8080");  
  
    for stream in listener.incoming() {  
        match stream {  
            Err(e) => { eprintln!("Error accepting: {}", e); },  
            Ok(stream) => {  
                // 为每个连接创建一个新的线程  
                thread::spawn(move || {  
                    handle_client(stream);  
                });  
            }  
        }  
    }  
}