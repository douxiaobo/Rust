use std::io::{self, Read, Write};
use std::net::{TcpListener};

这代码不完美，以后要花些时间修改代码。

fn main() -> io::Result<()> {
    // 直接使用 "localhost" 或 "0.0.0.0" 作为主机名
    let host = "localhost";
    let port = 12345;

    let listener = TcpListener::bind(format!("{}:{}", host, port))?;

    println!("Waiting for connections on port {}", port);

    let mut byebye=false;

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let addr = stream.peer_addr()?;
                println!("Connected by {}", addr);

                loop {
                    let mut buffer = [0; 1024];
                    let nbytes = stream.read(&mut buffer)?;
                    if nbytes == 0 {
                        break;
                    }
                    let msg = String::from_utf8_lossy(&buffer[..nbytes]);
                    println!("Received: {}", msg);

                    if msg.trim() == "byebye" {
                        println!("Client said goodbye.");
                        byebye=true;

                        break;
                    }

                    print!("Enter message to send to client (type 'byebye' to exit): ");
                    io::stdout().flush()?; // 确保提示信息被输出
                    let mut input = String::new();
                    io::stdin().read_line(&mut input)?;
                    let input = input.trim_end_matches('\n');

                    if input == "byebye" {
                        println!("Server said goodbye.");
                        byebye=true;
                        break;
                    }

                    stream.write_all(input.as_bytes())?;
                    println!("Sent: {}", input);
                }
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
        if byebye {
            break;
        }
    }

    Ok(())
}
