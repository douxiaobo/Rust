use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    // 打开文件
    let file = File::open("test.txt")?;
    let reader = BufReader::new(file);

    // 用于存储字符总数的变量
    let mut char_count = 0;

    // 逐行读取文件，正确处理UTF-8字符
    for line_result in reader.lines() {
        let line = line_result?;
        // 计算这一行的字符数
        char_count += line.chars().count();
    }

    // 输出字符总数
    println!("The number of characters: {}", char_count);

    Ok(())
}

The number of characters: 325002