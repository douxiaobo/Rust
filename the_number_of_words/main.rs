use std::fs::File;  
use std::io::{BufRead, BufReader};  
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
    let mut word_count = 0;  
    let mut current_word = String::new();  
  
    for line_result in reader.lines() {  
        let line = line_result.unwrap(); // 假设没有I/O错误  
        for ch in line.chars() {  
            if ch.is_whitespace() {  
                if !current_word.is_empty() {  
                    word_count += 1;  
                    current_word.clear();  
                }  
            } else {  
                current_word.push(ch);  
            }  
        }  
        // 检查行尾是否还有单词  
        if !current_word.is_empty() {  
            word_count += 1;  
            current_word.clear();  
        }  
    }  
  
    println!("单词数量: {}", word_count);  
}  
  
#[allow(dead_code)]         //#[warn(dead_code)]
fn eprintln(s: &str) {  
    eprintln!("{}", s);  
}