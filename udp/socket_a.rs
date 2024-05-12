use std::net::UdpSocket;
use std::sync::mpsc;

mod get_string;

// 写代码不完整。

fn main() {
    let (sender,_receiver)=mpsc::channel();
    let socket = UdpSocket::bind("127.0.0.1:50000").unwrap();
    socket.send_to(b"hello, Here is A.", "127.0.0.1:50001").unwrap();
    let message=get_string::get_string(socket);
    println!("来自 B 的消息：{}",message);
    sender.send(message).unwrap();
}