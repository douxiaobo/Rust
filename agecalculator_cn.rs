use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} <birthdate>", args[0]);
        return;
    }

    let birthdate = &args[1];

    let parts: Vec<&str> = birthdate.split('年').collect();
    if parts.len() != 2 {
        println!("Invalid date format. Please provide a birthdate like 'YYYY年MM月DD日'.");
        return;
    }

    let (year_part, rest) = parts.split_at(1);
    let month_day_parts: Vec<&str> = rest[0].split('月').collect();
    if month_day_parts.len() != 2 {
        println!("Invalid date format.");
        return;
    }

    let month = month_day_parts[0];
    let day_part = month_day_parts[1];

    let day = day_part.split('日').next().unwrap(); // Get the day before the '日'

    let year = year_part[0];
    let month = month.to_string();
    let day = day.to_string();

    println!("年：{}", year);
    println!("月：{}", month);
    println!("日：{}", day);

    println!("生日：{}年{}月{}日", year, month, day);
}