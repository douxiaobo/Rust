use std::process::Command;  
  
fn main() {  
    let mut cmd = Command::new("osascript");  
    cmd.arg("-e").arg("tell application \"Terminal\" to do script \"echo Hello, World!\"");  
    let output = cmd.output().expect("执行osascript失败");  
    println!("{}", String::from_utf8_lossy(&output.stdout));  
}