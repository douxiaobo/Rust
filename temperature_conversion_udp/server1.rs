use std::net::UdpSocket;
use std::str;

fn main() {
    let socket = UdpSocket::bind("127.0.0.1:8888").expect("bind failed");
    loop {
        let mut buf = [0u8; 1024];
        let (amt, src) = socket.recv_from(&mut buf).expect("recvfrom failed");
        let data = &buf[..amt];

        // 尝试将接收到的数据转换为字符串
        let message = str::from_utf8(data).expect("Invalid UTF-8 sequence");

        println!("Received from {}: {}", src.ip(), message);

        // 尝试将字符串转换为浮点数
        let mut temp: f64 = message.parse().expect("Failed to parse temperature");

        // 转换温度并发送
        temp = (temp * 1.8 + 32.0).round();
        let send_data = format!("转换后的温度（单位：华氏温度）：{}", temp);
        socket.send_to(send_data.as_bytes(), src).expect("sendto failed");
    }
}