fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() != 4 {
        println!("Usage: {} <operation> <number1> <number2>", args[0]);
        return;
    }
    let operation = args[1].as_str(); // 借用字符串
    let number1: i32 = args[2].parse().unwrap();
    let number2: i32 = args[3].parse().unwrap();

    // let number1_str = args[2].clone();  
    // let number2_str = args[3].clone();  

    // let number1_str = &args[2];  
    // let number2_str = &args[3]; 

    // let number1 = match number1_str.parse::<i32>() {  
    //     Ok(n) => n,  
    //     Err(e) => {  
    //         println!("Error parsing number1: {}", e);  
    //         return;  
    //     }  
    // };  
  
    // let number2 = match number2_str.parse::<i32>() {  
    //     Ok(n) => n,  
    //     Err(e) => {  
    //         println!("Error parsing number2: {}", e);  
    //         return;  
    //     }  
    // };  

    println!("Operation: {}, Numbers: {}, {}", operation, number1, number2); 

    match calculator(operation, number1, number2) {
        Ok(result) => println!("{} {} {} = {}", args[2], args[1], args[3], result),
        Err(e) => println!("Error: {}", e),
    }
}

fn calculator(operation: &str, number1: i32, number2: i32) -> Result<i32, &'static str> {
    match operation {
        "add" => Ok(number1 + number2),
        "sub" => Ok(number1 - number2),
        "mul" => Ok(number1 * number2),
        "div" => if number2 != 0 {
            Ok(number1 / number2)
        } else {
            Err("Cannot divide by zero")
        },
        _ => Err("Invalid operation"),
    }
}