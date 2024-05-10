use std::io::{self, Read, Write};
use std::net::TcpStream;

fn main() -> io::Result<()> {
    let host = match std::env::var("HOST") {
        Ok(val) => val,
        Err(_) => "localhost".to_string(), // 默认为localhost，或者您可以通过环境变量HOST指定
    };
    let port: u16 = 12345; // 您可以通过环境变量PORT指定端口

    let mut stream = TcpStream::connect(format!("{}:{}", host, port))?;
    println!("Connected to server");

    loop {
        let mut input = String::new();
        print!("Enter message to send to server (type 'byebye' to exit): ");
        io::stdout().flush()?; // 确保提示信息被输出
        io::stdin().read_line(&mut input)?;
        let input = input.trim_end_matches('\n');

        if input == "byebye" {
            println!("Client said goodbye.");
            break;
        }

        stream.write_all(input.as_bytes())?;
        println!("Message sent to server: {}", input);

        let mut buffer = [0; 1024];
        let nbytes = stream.read(&mut buffer)?;
        if nbytes == 0 {
            println!("Server closed the connection.");
            break;
        }
        let response = String::from_utf8_lossy(&buffer[..nbytes]);
        println!("Message received from server: {}", response);
    }

    Ok(())
}