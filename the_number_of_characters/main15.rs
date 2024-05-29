use std::fs::File;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    // 打开文件
    let mut file = File::open("test.txt")?;

    // 读取整个文件到一个字节向量中
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    // 字节向量的长度即为字符数（每个字节计为一个字符）
    let char_count = buffer.len();

    // 输出字符总数
    println!("The number of characters: {}", char_count);

    Ok(())
}
The number of characters: 342190