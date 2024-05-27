use std::fs::File;  
use std::io::{BufReader, Read};  
use std::path::Path;  
  
fn main() -> std::io::Result<()> {  

    let path = Path::new("./test.txt");  
      
    let file = File::open(path)?;  
      
    let mut reader = BufReader::new(file);  
      
    let mut buffer=Vec::new();

    let _ =reader.read_to_end(&mut buffer)?;

    println!("The number of bytes in the file is: {}", buffer.len()); 
    
    Ok(())  
}  
  
