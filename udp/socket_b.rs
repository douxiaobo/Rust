use std::net::UdpSocket;
use std::sync::mpsc;

mod get_string;

// 写代码不完整。

fn main() {
    let (_sender,receiver)=mpsc::channel::<String>();
    let socket = UdpSocket::bind("127.0.0.1:50001").unwrap();
    socket.send_to(b"Hello, world! Here is B.", "127.0.0.1:50000").unwrap();
    let message=get_string::get_string(socket);
    println!("来自 A的消息：{}",message);
    receiver.recv().unwrap();
}