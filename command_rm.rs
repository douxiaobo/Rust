use std::process::Command;  
  
fn main() {  
    let mut child = Command::new("rm");  
    child.arg("hello.txt"); // 移除 -i 参数，直接删除文件  
  
    // 注意：output() 方法会捕获命令的标准输出、标准错误和退出状态。  
    // 对于 rm 命令，你可能更关心标准错误，因为它会包含任何错误信息。  
    let result = child.output().expect("执行命令失败");  
  
    // 打印标准输出（通常对于 rm 命令来说是空的）  
    println!("标准输出: {}", String::from_utf8_lossy(&result.stdout));  
  
    // 打印标准错误，这里更有可能包含有用的信息  
    println!("标准错误: {}", String::from_utf8_lossy(&result.stderr));  
  
    // 检查命令是否成功执行  
    if result.status.success() {  
        println!("文件已成功删除");  
    } else {  
        println!("删除文件时出错");  
    }  
}

// 标准输出: 
// 标准错误: 
// 文件已成功删除

// 标准输出: 
// 标准错误: rm: hello.txt: No such file or directory

// 删除文件时出错

//已经知道了不能用sudo rm来写代码。