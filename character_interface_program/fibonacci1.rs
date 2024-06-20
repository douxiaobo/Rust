fn main() {
    println!("{:?}", fibonacci(11));
}

fn fibonacci(n: u32) -> Result<u32, &'static str> {
    if n == 0 {
        Err("传递的参数必须是大于0的正整数！")
    } else if n == 1 {
        Ok(0)
    } else {
        Ok(fibonacci_recursive(n))
    }
}

fn fibonacci_recursive(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2),
    }
}
