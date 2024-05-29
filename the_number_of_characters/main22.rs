use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn main() -> Result<(), Error> {
    let path = "test.txt"; // 文件路径
    let input = File::open(path)?; // 打开文件
    let buffered = BufReader::new(input); // 创建缓冲读取器

    let mut char_count = 0; // 字符数计数器

    for line in buffered.lines() {
        if let Ok(line) = line {
            char_count += line.chars().count(); // 计算每行字符数并累加
        }
    }

    println!("文件中的字符数：{}", char_count);

    Ok(())
}

文件中的字符数：325002
