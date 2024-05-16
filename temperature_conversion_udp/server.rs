use std::net::UdpSocket;

fn main(){
    let socket=UdpSocket::bind("127.0.0.1:8888").expect("bind failed");
    loop{
        let mut buf=[0u8;1024];
        let (amt,src)=socket.recv_from(&mut buf).expect("recvfrom failed");
        let buf=&mut buf[..amt];
        // buf.reverse();
        let message=String::from_utf8_lossy(&buf[..amt]);
        println!("Received from {}: {}",src.ip(),message);
        let mut message=message.parse::<f64>().unwrap();
        message=(message*1.8+32.0).round();
        socket.send_to(format!("{:.2}°F",message).as_bytes(),src).expect("sendto failed");
    }
}