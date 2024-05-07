fn main(){
    let mut args=std::env::args();
    args.next(); // skip the program name
    let number1=args.next().unwrap().trim().parse::<i32>().unwrap();
    let operator=args.next().unwrap();  //if write trim(), it will cause error, because it will remove the whitespace.
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