use std::net::TcpStream;
use std::io::{Write, Read};
//代码不行
fn main() {
    let mut stream = TcpStream::connect(("127.0.0.1", 8080)).expect("连接服务器失败");

    println!("你好，这里是客户端！");

    println!("请输入要发送的消息：");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("读取输入行失败");
    let send_data = input.trim().to_string();

    stream.write_all(send_data.as_bytes()).expect("发送数据失败");

    let mut recv_data = Vec::with_capacity(1024); // 初始化一个可增长的缓冲区
    let mut total_read = 0;
    loop {
        let num_bytes = stream.read(&mut recv_data[total_read..]).unwrap_or_else(|e| {
            if e.kind() == std::io::ErrorKind::UnexpectedEof {
                // 服务器优雅地关闭了连接
                break;
            } else {
                panic!("读取数据失败: {}", e);
            }
        });
        total_read += num_bytes;

        if num_bytes == 0 {
            // 已经到达流的末尾
            break;
        }
    }

    let recv_data_str = String::from_utf8(recv_data).expect("接收到的不是有效的UTF-8编码");

    println!("从服务器收到的消息: {}", recv_data_str);

    stream.shutdown(std::net::Shutdown::Both).expect("关闭连接失败");
}