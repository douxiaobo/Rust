use std::process::Command;  
use std::io::Write;
  
fn main() {  
    let child = Command::new("ls") // 替换为你想要执行的命令  
                            .arg("-l")     // 如果有任何参数，使用.arg()添加它们  
                            .output()     // 执行命令并获取输出  
                            .expect("执行命令失败");  
  
    // 输出命令的结果到标准输出  
    std::io::stdout().write_all(&child.stdout).expect("写入标准输出失败");  
}