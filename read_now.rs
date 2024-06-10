use std::time::SystemTime;
fn main() {
    let sys_time = SystemTime::now();
    println!("Time:{:?}", sys_time); //Time:SystemTime { tv_sec: 1718014136, tv_nsec: 591331000 }
    let duration_since_epoch = sys_time
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("Time went backwards");

    let total_seconds = duration_since_epoch.as_secs();

    // 这里只是示例，并没有考虑闰年、闰秒和时区变化
    // 实际上，你需要一个日历库或算法来计算精确的日期
    // 假设一年有365天，一天有86400秒（忽略闰秒和闰年）

    let years = total_seconds / (365 * 86400) + 1970;
    let remaining_seconds = total_seconds % (365 * 86400);

    let months = remaining_seconds / (29 * 86400);
    let remaining_seconds = remaining_seconds % (29 * 86400);

    let days = remaining_seconds / 86400;
    let remaining_seconds = remaining_seconds % 86400;

    let hours = remaining_seconds / 3600;
    let remaining_seconds = remaining_seconds % 3600;

    let minutes = remaining_seconds / 60;
    let seconds = remaining_seconds % 60;

    // 这里只是简单地打印结果，并没有考虑月份和日期的具体计算
    // 你需要一个日历算法来计算准确的月份和日期
    println!("Year: {}", years); // 这只是一个粗略的估计
    println!("Month:{}", months);
    println!("Day: {}", days); // 同样，这也是一个粗略的估计
    println!("Hour: {}", hours);
    println!("Minute: {}", minutes);
    println!("Second: {}", seconds);

    // 如果你想将这些信息组合成一个字符串，可以这样做：
    let formatted_time = format!(
        "{}-{}-{} {}:{}:{}",
        years, months, days, hours, minutes, seconds
    ); // "??" 代表未知的月份和日期
    println!("Formatted time: {}", formatted_time);
}
