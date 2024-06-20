use std::time::SystemTime;

fn main() {
    // let no_leap_second = [
    //     1970, 1971, 1980, 1984, 1986, 1988, 1991, 1996, 1999, 2000, 2001, 2002, 2003, 2004, 2006,
    //     2007, 2009, 2010, 2011, 2013, 2014,
    // ];
    let sys_time = SystemTime::now();
    println!("Time:{:?}", sys_time); //Time:SystemTime { tv_sec: 1718014136, tv_nsec: 591331000 }
    let duration_since_epoch = sys_time
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("Time went backwards");

    let mut total_seconds = duration_since_epoch.as_secs();

    let beijing_time = 28800;

    total_seconds += beijing_time;

    let one_year = 365 * 86400;
    let one_leap_year = 366 * 86400;

    let mut years = 1970;

    loop {
        if is_leap_year(years) {
            if total_seconds >= one_leap_year {
                total_seconds -= one_leap_year;
            } else {
                break;
            }
        } else {
            if total_seconds >= one_year {
                total_seconds -= one_year;
            } else {
                break;
            }
        }
        // if year == 1972 {
        //     total_seconds -= 2;
        // } else if no_leap_second.contains(&year) {
        // } else {
        //     total_seconds -= 1;
        // }
        years += 1;
    }

    let big_month = 31 * 86400;
    let small_month = 30 * 86400;
    let no_leap_month = 28 * 86400;
    let leap_month = 29 * 86400;
    let mut months = 1;
    loop {
        if (is_leap_year(years)) && (months == 2) {
            if total_seconds >= leap_month {
                total_seconds -= leap_month;
            } else {
                break;
            }
        } else if (!is_leap_year(years)) && (months == 2) {
            if total_seconds >= no_leap_month {
                total_seconds -= no_leap_month;
            } else {
                break;
            }
        } else if (months == 4) || (months == 6) || (months == 9) || (months == 11) {
            if total_seconds >= small_month {
                total_seconds -= small_month;
            } else {
                break;
            }
        } else {
            if total_seconds >= big_month {
                total_seconds -= big_month;
            } else {
                break;
            }
        }
        months += 1;
    }

    // let mut day = 0;
    // while total_seconds >= 86400 {
    //     total_seconds -= 86400;
    //     day += 1;
    // }
    // day += 1;

    // 计算日
    let days = total_seconds / 86400 + 1; // 直接计算日，注意加一

    total_seconds = total_seconds % 86400;

    let hours = total_seconds / 3600;

    total_seconds = total_seconds % 3600;

    let minutes = total_seconds / 60;

    let seconds = total_seconds % 60;

    println!("year:{years}");
    println!("month:{months}");
    println!("day:{days}");
    println!("hour:{hours}");
    println!("minute:{minutes}");
    println!("second:{seconds}");
    let formatted_time = format!(
        "{}-{}-{} {}:{}:{}",
        years, months, days, hours, minutes, seconds
    ); // "??" 代表未知的月份和日期
    println!("Formatted time: {}", formatted_time);
}

fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}
