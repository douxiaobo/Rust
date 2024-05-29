use std::fs::File;  
use std::io::{ BufReader, Read,Write};  
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
  
    let mut reader = BufReader::new(file);  
    let mut contents = String::new();  
  
    // 读取文件内容到String  
    match reader.read_to_string(&mut contents) {  
        Err(why) => {  
            eprintln!("读取文件时出错: {}", why);  
            return;  
        },  
        Ok(_) => {  
            // 计算字符数量（这里计算的是Unicode字符）  
            let char_count = contents.chars().count();  
            println!("字符数量: {}", char_count);  
        },  
    }  
}  

#[allow(dead_code)]
fn eprintln(s: &str) {  
    std::io::stderr().write_all(s.as_bytes()).unwrap();  
    std::io::stderr().write_all(b"\n").unwrap();  
}

// 字符数量: 339292
// OK
// 正确的字符数量，而不是字节数量。