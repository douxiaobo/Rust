use std::net::UdpSocket;

pub fn get_string(socket: UdpSocket)->String{
    let mut buffer = [0u8; 1024];
    let (size, _)= socket.recv_from(&mut buffer).unwrap();
    let binary=Vec::from(&buffer[0..size]);
    String::from_utf8(binary).unwrap()
}