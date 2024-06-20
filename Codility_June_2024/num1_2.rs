fn main() {
    match solution(14) {
        Ok(result) => println!("{}", result),    // 19
        Err(e) => println!("Error: {}", e),
    }
    match solution(10) {
        Ok(result) => println!("{}", result),    // 11
        Err(e) => println!("Error: {}", e),
    }
    match solution(99) {
        Ok(result) => println!("{}", result),    // 9999
        Err(e) => println!("Error: {}", e),
    }
}

fn solution(n: i32) -> Result<i32, String> {
    let digit_sum = sum_of_digits(n);
    let target_sum = digit_sum * 2;

    for i in n.. {
        if sum_of_digits(i) == target_sum {
            return Ok(i);
        }
    }

    Err("No result found".to_string())
}

fn sum_of_digits(mut num: i32) -> i32 {
    let mut sum = 0;
    while num > 0 {
        sum += num % 10;
        num /= 10;
    }
    sum
}