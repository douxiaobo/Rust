use std::fs::File;
use std::io::{self, BufRead};


fn main() -> io::Result<()> {
    // 打开文件
    let file = File::open("test.txt")?;
    
    // 创建一个BufReader来读取文件
    let reader = io::BufReader::new(file);
    
    // 用于存储字符总数的变量
    let mut char_count = 0;
    
    // 逐行读取文件
    for line in reader.lines() {
        // 将行转换为Result<String>，如果成功则处理
        let line = line?;
        // 增加字符数，包括换行符
        char_count += line.chars().count();
    }
    
    // 输出字符总数
    println!("The number of characters: {}", char_count);
    
    Ok(())
}

// The number of characters: 325002
// 不对