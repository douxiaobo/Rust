use std::env;

fn main() {
    // 定义月份的英文名称数组，这里仅作为示例，实际未在当前逻辑中使用
    // let month_total = ["January", "February", ... ]; 

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("用法: {} <出生日期>", args[0]);
        return;
    }

    let birthdate = &args[1];

    // 修正了变量名的拼写错误，应为`birthdate`而非`birthddate`
    if birthdate.contains("年") {
        let parts: Vec<&str> = birthdate.split('年').collect();
        if parts.len() != 2 {
            println!("无效的日期格式，请按照 'YYYY年MM月DD日' 的格式提供出生日期。");
            return;
        }

        let (year_part, rest) = parts.split_at(1);
        let month_day_parts: Vec<&str> = rest[0].split('月').collect();
        if month_day_parts.len() != 2 {
            println!("无效的日期格式。");
            return;
        }

        let month = month_day_parts[0];
        let day_part = month_day_parts[1];

        // 安全地获取“日”之前的字符，避免发生 panic
        let day = day_part.split('日').next().unwrap_or("");

        if day.is_empty() {
            println!("缺少日期部分。");
            return;
        }

        // 尝试将年、月、日转换为数字类型，这里简单处理，实际应用中应添加更全面的错误处理
        let year: u32 = year_part.parse().unwrap_or(0);
        let month: u32 = month.parse().unwrap_or(0);
        let day: u32 = day.parse().unwrap_or(0);

        println!("年：{}", year);
        println!("月：{}", month);
        println!("日：{}", day);

        println!("生日：{}年{}月{}日", year, month, day);
    } else {
        println!("提供的出生日期中没有找到字符 '年'，这是必需的。");
        // 注意：对于月份英文名称的处理逻辑，这里没有实现，你可以根据需要添加。
    }
}