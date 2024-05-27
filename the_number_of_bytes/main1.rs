use std::env;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

fn main() -> std::io::Result<()> {
    // 获取命令行参数
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Missing filename"));
    }

    let path = Path::new(&args[1]);

    // 打开文件
    let file = File::open(path)?;
    
    let mut reader = BufReader::new(file);
    
    let mut buffer = Vec::new();

    let _ = reader.read_to_end(&mut buffer)?;

    println!("The number of bytes in the file is: {}", buffer.len());

    Ok(())
}