use std::env;

fn main() {
    let month_total = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} <birthdate>", args[0]);
        return;
    }

    let birthdate = &args[1];

    let mut month = String::new();
    for month_name in month_total.iter() {
        if birthdate.contains(month_name) {
            month = month_name.to_string();
            break;
        }
    }

    if !month.is_empty() {
        println!("生日的月份是：{}", month);
        / 假设月份后面紧跟着的是日和年，中间以逗号分隔
    let month_len = month.len();
    let remaining = &birthdate[month_len..]; // 从月份之后的字符串开始处理

    // 查找逗号位置来分割日和年
    if let Some(comma_pos) = remaining.find(',') {
        let day_part = &remaining[..comma_pos].trim(); // 日
        let year_part = &remaining[comma_pos + 1..].trim(); // 年

        // 确认日和年都是数字
        if day_part.chars().all(char::is_numeric) && year_part.chars().all(char::is_numeric) {
            let day: u32 = day_part.parse().expect("Day should be numeric");
            let year: u32 = year_part.parse().expect("Year should be numeric");

            println!("生日的日份是：{}", day);
            println!("生日的年份是：{}", year);
        } else {
            println!("日期格式不正确，日或年份非数字。");
        }
    } else {
        println!("未找到正确的逗号分隔符来区分日和年。");
    }
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
    } else {
        println!("提供的出生日期格式不被识别。");
    }
}