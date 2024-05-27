use core::num;

const HELP:&str="Usage: simplemath <operation> <number1> <number2>
Available commands:
    - add
    - subtract
    - multiply
    - divide
    - help
";

enum Number{
    Int(i32),
    Float(f64),
}


pub struct Num{
    num1:Number,
    num2:Number,
}



trait Addable<T> {
    fn add(self, other: T) -> T;
}

impl Addable<f64> for f64 {
    fn add(self, other: f64) -> f64 {
        self + other
    }
}

impl Addable<i32> for i32 {
    fn add(self, other: i32) -> i32 {
        self + other
    }
}

impl Addable<u32> for u32 {
    fn add(self, other: u32) -> u32 {
        self + other
    }
}

impl Addable<i64> for i64 {
    fn add(self, other: i64) -> i64 {
        self + other
    }
}


pub fn add<T: Addable<T>>(num1: T, num2: T) -> T {
    num1.add(num2)
}

pub fn help(){
    println!("{}", HELP);
}