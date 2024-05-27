use simplemath::help;

fn main() {
    let num=Num::new().expect("Invalid input");
    let args:Vec<String> = std::env::args().collect();
    if args.len()>1 {
        let command=&args[1];
        match &command[..]{
            "add"|"Add"|"sum"|"Sum"|"+"=>num.add(&args[2..]),
            "subtract"|"Subtract"|"difference"|"Difference"|"-" =>num.subtract(&args[2..]),
            "multiply"|"Multiply"|"product"|"Product"|"*" =>num.multiply(&args[2..]),
            "divide"|"Divide"|"quotient"|"Quotient"|"/" =>num.divide(&args[2..]),
            "help"|"--help"|"-h"|"HELP"|"帮助"|"--HELP"|"-H"|_ =>help(),
        }
    }
}
