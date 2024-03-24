use std::io::Write;
fn check_len(pwd:String)->bool{
    if pwd.len()>=8{
        return true;
    } else {
        return false;
    }
}
fn check(pwd:String)->bool{
    let mut check:[i32;4]=[0,0,0,0];
    for ch in pwd.chars(){
        if ch.is_lowercase(){
            check[0]=1;
        }
        if ch.is_uppercase(){
            check[1]=1;
        }
        if ch.is_numeric(){
            check[2]=1;
        }
        if !(ch.is_alphabetic()|ch.is_numeric()|ch.is_whitespace()){
            check[3]=1;
        }
    }
    let mut sum:i32=0;
    let mut i:usize=0;
    while i<4{
        sum+=check[i];
        i+=1;
    }
    if sum<4{
        return false;
    } else {
        return true;
    }
}
fn check_rep(pwd:String)->bool{
    for i in 0..(pwd.len()-4){
        let str1=&pwd[i..i+4];
        let str2=&pwd[i+4..];
        if str2.contains(str1){
            return false;
        }
    }
    return true;
}
fn main(){
    let msg="请设置密码，密码要求符合以下条件：
    1、密码长度不小于8位
    2、密码必须由字母大、小写、数字、其它符号组成
    3、密码中不能重复包含长度超4的子串".to_string();
    println!("{}",msg);
    
    loop{
        let mut pwd=String::new();
        print!("请录入密码：");
        std::io::stdout().flush().unwrap();        
        std::io::stdin().read_line(&mut pwd).expect("Failed to read line.");
        //println!("{}",pwd);
        if pwd.trim()=="q"{
            println!("退出程序。。。");
            break;
        }
        if !check_len(pwd.clone()){
            println!("密码长度不够8位！请重新录入。");
            continue;
        }
        if !check(pwd.clone()){
            println!("密码必须由字母大、小写、数字、其它符号组成！请重新录入。");
            continue;
        }
        if !check_rep(pwd){
            println!("密码包含两个以上重复子串（4位以上的子串）！请查看并重新录入重新录入。");
            continue;
        }
        println!("密码正确。");
        break;
    }
}