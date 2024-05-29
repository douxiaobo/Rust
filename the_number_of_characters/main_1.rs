use std::fs::File;  
use std::io::{BufRead, BufReader, Write};  
use std::path::Path;  
  
fn main() {  
    let path = Path::new("test.txt");  
    let display = path.display();  
  
    let file = match File::open(&path) {  
        Err(why) => {  
            eprintln!("无法打开文件 {}: {}", display, why);  
            return;  
        },  
        Ok(file) => file,  
    };  
  
    let reader = BufReader::new(file);  
    let mut char_count = 0;  
  
    // 逐行读取并计算字符数量  
    for result in reader.lines() {  
        let line = result.unwrap(); // 假设没有I/O错误  
        char_count += line.chars().count(); // 计算每一行的字符数量  
    }  
  
    println!("字符数量: {}", char_count);  
}  

#[allow(dead_code)]
fn eprintln(s: &str) {  
    std::io::stderr().write_all(s.as_bytes()).unwrap();  
    std::io::stderr().write_all(b"\n").unwrap();  
}