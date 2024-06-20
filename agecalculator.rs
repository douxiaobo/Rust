use std::env;

fn main() {
    let month_total=["January","Febrary","March","April","May","June","July","August","September","October","November","December"];
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} <birthdate>", args[0]);
        return;
    }

    let birthdate = &args[1];

    if birthdate.contains(month_total) {
        let month=things.new();
        if birthday.contains("January"){
            month="January";
        } else if birthdate.contains("February"){
            month="February";
        } else if birthdate.contains("March"){
            month="March";
        } else if birthdate.contains("April"){
            month="April";onth="April";
        } else if birthdate.contains("May"){
            month="May";
        } else if birthdate.contains("June"){
            month="June";
        } else if birthdate.contains("July"){
            month="July";
        } else if birthdate.contains("August"){
            month="August";
        } else if birthdate.contains("September"){
            month="September";
        } else if birthdate.contains("October"){
            month="October";
        } else if birthdate.contains("November"){
            month="November";
        } else if birthdate.contains("December"){
            month="December";
        }
        println!("生日的月份是：{}", month);
        let parts:Vec<&str> = birthdate.split(',').collect();
        let year = parts[1];
        println!("生日的年份是：{}", year);
        let mut day = String::new();
        for ch in parts[0].chars() {
            if ch.is_digit(10) {
                day+ = ch.to_string();
            }
        }
        println!("生日的日份是：{}", day);
    } else if birthdate.contains("年") {
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

    
}