
fn main() {
    println!("{}", solution(14));    // 19
    println!("{}", solution(10));    // 11
    println!("{}", solution(99));    // 9999
}

fn solution(n: i32) -> i32 {
    let digit_sum = sum_of_digits(n);
    let target_sum = digit_sum * 2;

    for i in n.. {
        if sum_of_digits(i) == target_sum {
            return i;
        }
    }

    -1
}

fn sum_of_digits(mut num: i32) -> i32 {
    let mut sum = 0;
    while num > 0 {
        sum += num % 10;
        num /= 10;
    }
    sum
}