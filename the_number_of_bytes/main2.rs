use std::fs::File;  
use std::io::{BufReader, Read};  
use std::path::Path;  
  
fn main() -> std::io::Result<()> {  

    let path = Path::new("./test.txt");  
      
    let file = File::open(path)?;  
      
    let mut reader = BufReader::new(file);  

    let mut bytes_read=0;   //统计读取的总字节数
      
    let mut buffer=[0u8;1024];
    
    while let Ok(n) = reader.read(&mut buffer) {
        bytes_read+=n;
        if n==0 { break;}
    }

    println!("The number of bytes in the file is: {}", bytes_read); 
    
    Ok(())  
}  
  
