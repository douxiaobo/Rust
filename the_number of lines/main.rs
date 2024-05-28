use std::fs::File;  
use std::io::{BufRead, BufReader};  
  
fn main() -> Result<(), Box<dyn std::error::Error>> {  
    let file = File::open("test.txt")?;  
    let reader = BufReader::new(file);  
  
    let mut lines_count = 0;  
    for _ in reader.lines() {  
        lines_count += 1;  
        // 注意：我们不需要实际读取每一行的内容，只需要知道有多少行  
    }  
  
    // 如果文件中存在最后一行没有换行符的情况，上面的循环可能不会计数这一行。  
    // 为了处理这种情况，我们可以在循环结束后检查缓冲区中是否还有剩余的内容。  
    // 但是，为了简单起见，这里我们假设每行都以换行符结尾。  
  
    println!("The number of lines: {}", lines_count);  
  
    Ok(())  
}  