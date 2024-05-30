fn main(){
    let mut args=std::env::args(); 
    println!("{:?}",args);
    args.next(); // skip the first argument, which is the name of the program
    println!("{:?}",args);
    let mut command=String::new();
    let mut file_name=String::new();
    if let Some(arg)=args.next(){
        if arg.contains(".") {
            command="a".to_string();
            file_name=arg.to_string();
        } else if arg=="-c"  {
            command="c".to_string();
            file_name=args.next().unwrap_or_else(|| "".to_string());
        } else if arg=="-l"  {
            command="l".to_string();
            file_name=args.next().unwrap_or_else(|| "".to_string());
        } else if arg=="-w"  {
            command="w".to_string();
            file_name=args.next().unwrap_or_else(|| "".to_string());
        } else{
            println!("Invalid command");
            return;
        }
    }
    println!("{:?},{:?}",command,file_name);
}

// 有点冗长

// douxiaobo@192 Rust % ./build_my_one_wc_tool test.txt
// Args { inner: ["./build_my_one_wc_tool", "test.txt"] }
// Args { inner: ["test.txt"] }
// "a","test.txt"
// douxiaobo@192 Rust % ./build_my_one_wc_tool -c test.txt
// Args { inner: ["./build_my_one_wc_tool", "-c", "test.txt"] }
// Args { inner: ["-c", "test.txt"] }
// "c","test.txt"