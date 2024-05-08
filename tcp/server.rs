use std::net::{TcpListener};
use std::io::{Read, Write};
use std::net::Shutdown;

fn main() {
    let host = "127.0.0.1"; // 主机名IP
    let port = 8080; // 端口号

    let listener = TcpListener::bind((host, port)).unwrap(); // 绑定IP和端口
    println!("Hi, Here is the server");
    println!("Server is listening on port {}", port);

    // 开启死循环
    loop {
        match listener.accept() {
            Ok((mut conn, addr)) => {
                println!("Connection from {}", addr);
                let mut buffer = [0; 1024]; // 创建接收数据的缓冲区
                conn.read(&mut buffer).unwrap(); // 获取客户端发送的数据
                let data = String::from_utf8_lossy(&buffer);
                println!("Received data: {}", data.trim());
                println!("{}", data); // 打印接收到的数据

                let response = b"\nHello, client!\n";
                conn.write_all(response).unwrap();

                let http_response = b"HTTP/1.1 200 OK\r\n\r\nHello, World!";
                conn.write_all(http_response).unwrap();
                // conn.shutdown(std::net::Shutdown::Both).unwrap(); // 关闭连接
                conn.shutdown(Shutdown::Write).unwrap(); // 关闭连接
            },
            Err(e) => println!("Accept error: {}", e),
        }
    }
}