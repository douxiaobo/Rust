
use os_info::Type;
use sysinfo::{Users,System};

fn main() {
    let os =std::env::consts::OS;
    println!("操作系统：{}",os);

    let info=os_info::get();
    let os1=match info.os_type(){
        Type::Windows=>"Windows",
        Type::Macos=>"MacOS",
        Type::Linux=>"Linux",
        Type::Android=>"Android",
        Type::FreeBSD=>"FreeBSD",
        Type::OpenBSD=>"OpenBSD",
        Type::NetBSD=>"NetBSD",
        Type::DragonFly=>"Dragonfly",
        Type::Unknown=>"Other",
        _ => todo!("not support")
    };
    println!("操作系统：{}",os1);    

    let info=os_info::get();
    println!("OS information:{info}");

    let users = Users::new_with_refreshed_list();
    for user in users.list(){
        println!("user name:{}", user.name());
    }

    println!("System name:{:?}",System::host_name());   //ip地址
    println!("System name:{:?}",System::name());    //系统名称 很难理解，什么是Darwin
    println!("System name:{:?}",System::kernel_version());
    println!("System name:{}",System::kernel_version().unwrap());
    println!("System version:{:?}",System::os_version());
    println!("System version:{:?}",System::os_version().unwrap());
    println!("Long OS Version:{:?}",System::long_os_version());
    println!("Long OS Version:{:?}",System::long_os_version().unwrap_or_else(||"<unknown>".into()));
    println!("Used memory:{:.2}",System::new_all().used_memory()/ 1073741824);   //内存使用量
    println!("Available memory:{:.2} GB",System::new_all().available_memory()/ 1073741824);
    println!("Total memory:{:.2} GB",System::new_all().total_memory()/ 1073741824);             //电脑内存
    println!("CPU cores:{}",System::new_all().cpus().len());
    println!("CPU Archicture:{}",System::cpu_arch().unwrap_or_else(|| "<unknown>".into()));
    println!("CPU Architecture:{}", System::cpu_arch().unwrap_or("<unknown>".into()));
    if let Some(arch) = System::cpu_arch() {
        println!("CPU Architecture:{}", arch);
    } else {
        println!("CPU Architecture:<unknown>");
    }
}
