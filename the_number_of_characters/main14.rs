use std::fs::File;
use std::io::{self, Read, BufReader};

fn main() -> io::Result<()> {
    // 打开文件
    let file = File::open("test.txt")?;
    let reader = BufReader::new(file);

    // 用于存储字符总数的变量
    let mut char_count = 0;

    // 逐字节读取文件
    for byte_result in reader.bytes() {
        let byte = byte_result?;
        // 每个字节都是一个字符，直接增加计数
        char_count += 1;
    }

    // 输出字符总数
    println!("The number of characters: {}", char_count);

    Ok(())
}
The number of characters: 342190
也不对。