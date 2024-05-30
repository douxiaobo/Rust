fn main(){
    let mut args=std::env::args();
    println!("{:?}",args);
    args.next(); // skip the program name
    println!("{:?}",args);
    let number1=args.next().unwrap().trim().parse::<i32>().unwrap();
    println!("{:?}",args);
    let operator=args.next().unwrap();  //if write trim(), it will cause error, because it will remove the whitespace.
    println!("{:?}",args);
    let number2=args.next().unwrap().trim().parse::<i32>().unwrap();
    let result=match operator.as_str(){
        "+" => number1+number2,
        "-" => number1-number2,
        "*" => number1*number2,
        "/" => number1/number2,
        _ => panic!("Invalid operator")
    };
    println!("{} {} {} = {}",number1,operator,number2,result);
}

// douxiaobo@192 Rust % rustc calculator_1.rs
// douxiaobo@192 Rust % ./calculator_1 1 + 2 
// Args { inner: ["./calculator_1", "1", "+", "2"] }
// Args { inner: ["1", "+", "2"] }
// Args { inner: ["+", "2"] }
// Args { inner: ["2"] }
// 1 + 2 = 3