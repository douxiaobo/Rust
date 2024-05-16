use std::net::UdpSocket;
use std::io::{self, Write};

fn main() {
    // 绑定到8888端口，与Python代码一致
    let socket = UdpSocket::bind("127.0.0.1:0").expect("bind failed");

    print!("Enter temperature in Celsius: ");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // 去除输入字符串的换行符
    let input = input.trim();

    // 发送数据到8888端口，与Python代码一致
    socket.send_to(input.as_bytes(), "127.0.0.1:8888").expect("send_to failed");

    // 创建一个缓冲区来接收数据
    let mut buf = [0u8; 1024];

    // 使用recv_from而不是recv，因为recv_from可以获取发送方的地址和端口信息
    let (amt, _) = socket.recv_from(&mut buf).expect("recv_from failed");

    // 截取接收到的数据部分
    let response = &buf[..amt];

    // 打印接收到的响应
    println!("Received: {}", String::from_utf8_lossy(response));
}