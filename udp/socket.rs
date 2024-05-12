use std::net::UdpSocket;
use std::sync::mpsc;
use std::thread;

mod get_string;

fn main() {
    let (sender,receiver)=mpsc::channel();
    let socket_a = UdpSocket::bind("127.0.0.1:50000").unwrap();
    thread::spawn(move || {
        socket_a.send_to(b"hello, Here is A.", "127.0.0.1:50001").unwrap();
        let message=get_string::get_string(socket_a);
        println!("来自 B 的消息：{}",message);
        sender.send(message).unwrap();
    });

    let socket_b = UdpSocket::bind("127.0.0.1:50001").unwrap();
    socket_b.send_to(b"Hello, world! Here is B.", "127.0.0.1:50000").unwrap();
    let message=get_string::get_string(socket_b);
    println!("来自 A的消息：{}",message);
    receiver.recv().unwrap();
}