fn main() {
    match fibonacci_sequence(11) {
        Ok(sequence) => println!("{:?}", sequence),
        Err(e) => println!("错误: {}", e),
    }
}

fn fibonacci_sequence(n: usize) -> Result<Vec<u32>, &'static str> {
    if n == 0 {
        return Err("传递的参数必须是大于0的正整数！");
    }

    let mut fib_list = vec![0, 1];
    for i in 2..n {
        let next = fib_list[i - 1] + fib_list[i - 2];
        fib_list.push(next);
    }

    Ok(fib_list.into_iter().take(n).collect())
}
