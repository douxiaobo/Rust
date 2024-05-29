use std::fs::File;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    // 打开文件
    let mut file = File::open("test.txt")?;

    // 创建一个Vec<u8>来存储文件内容
    let mut contents = Vec::new();
    
    // 读取整个文件到contents
    file.read_to_end(&mut contents)?;

    // 计算字符总数
    let char_count = contents.len();

    // 输出字符总数
    println!("The number of characters: {}", char_count);

    Ok(())
}

The number of characters: 342190
也不对。