use std::fs::File;  
use std::io::{ BufReader, Read, Write};  
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
    let mut contents = Vec::new();  
  
    // 读取文件内容到Vec<u8>  
    match reader.read_to_end(&mut contents) {  
        Err(why) => {  
            eprintln!("读取文件时出错: {}", why);  
        },  
        Ok(_) => {  
            // 计算字符数量（注意这里将每个字节视为一个字符，对于UTF-8编码文本通常是正确的）  
            let char_count = contents.len() as u64;  
            println!("字符数量: {}", char_count);  
        },  
    }  
}  
#[allow(dead_code)]  
fn eprintln(s: &str) {  
    std::io::stderr().write_all(s.as_bytes()).unwrap();  
    std::io::stderr().write_all(b"\n").unwrap();  
}

字符数量: 342190
不对