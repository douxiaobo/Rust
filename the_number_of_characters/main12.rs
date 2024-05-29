use std::fs::File;
use std::io::{self, BufReader,Read};

fn main() -> io::Result<()> {
    // 打开文件
    let file = File::open("test.txt")?;
    let reader = BufReader::new(file);

    // 用于存储字符总数的变量
    let mut char_count = 0;

    // 逐字节读取文件
    for byte in reader.bytes() {
        let byte = byte?;
        // 将字节转换为字符，并增加字符计数
        char_count += std::str::from_utf8(&[byte]).ok().map_or(0, |_| 1);
    }

    // 输出字符总数
    println!("The number of characters: {}", char_count);

    Ok(())
}

The number of characters: 337634