use std::result::Result;  
  
fn main() {  
    let args = std::env::args().collect::<Vec<String>>();  
    if args.len() != 2 {  
        println!("Usage: {} <string>", args[0]);  
        return;  
    }  
  
    let str = args[1].clone();  
  
    let mut operation_before = false;  
    let mut number1 = 0; // 初始化 number1  
    let mut number2 = 0; // 初始化 number2  
    let mut operation = '+'; // 假设一个默认的操作符  
  
    for c in str.chars() {  
        if c.is_ascii_digit() {  
            if operation_before {  
                number2 = number2 * 10 + c.to_digit(10).unwrap() as i32;  
            } else {  
                number1 = number1 * 10 + c.to_digit(10).unwrap() as i32;  
            }  
        } else if c == '+' || c == '-' || c == '*' || c == '/' { // 只处理这些操作符  
            operation_before = true;  
            operation = c;  
        } else {  
            println!("Invalid character: {}", c);  
            return;  
        }  
    }  
  
    // 处理Result类型  
    match calculate(operation, number1, number2) {  
        Ok(result) => println!("{} {} {} = {}", number1, operation, number2, result),  
        Err(err) => println!("Error: {}", err),  
    }  
}  
  
fn calculate(operation: char, number1: i32, number2: i32) -> Result<i32, &'static str> {  
    match operation {  
        '+' => Ok(number1 + number2),  
        '-' => Ok(number1 - number2),  
        '*' => Ok(number1 * number2),  
        '/' => if number2 != 0 {  
            Ok(number1 / number2)  
        } else {  
            Err("Cannot divide by zero")  
        },  
        _ => Err("Invalid operation"),  
    }  
}