use std::env;
use std::time::SystemTime;
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

    let own_year:i32 =  year_part[0].parse().expect("Year part is not a valid number");
    let own_month:i32 = month_day_parts[0].parse().expect("Month part is not a valid number");
    let day_part = month_day_parts[1];
    let day = day_part.split('日').next().unwrap(); // Get the day before the '日'
    let own_day:i32 = day.parse().expect("Day part is not a valid number");

    println!("生日：{}年{}月{}日", own_year, own_month, own_day);

    let sys_time = SystemTime::now();

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

    let days = (total_seconds / 86400 + 1) as i32; // 直接计算日，注意加一

    println!("今天是{}年{}月{}日",years, months, days);

    let diff_years = if(own_month>months) || (own_month==months && own_day>days) {
        years - own_year - 1
    } else {
        years - own_year
    };

    println!("今天与您的生日还有{}年",diff_years);

    let diff_months = if (own_month>months && own_day>days) || (own_month>months && own_day>days) {
        (months + 12 - own_month) - 1
    } else if own_month>months && own_day<days {
        months+12-own_month
    } else if  own_month==months && own_day<days {
        months - own_month - 1
    } else {
        months-own_month
    };

    println!("今天与您的生日还有{}个月",diff_months);

    let total_month = diff_years * 12 + diff_months;

    println!("您的生日已经过去了{}个月",total_month);

    let diff_days= if(own_month>months) || (own_month==months && own_day>days) {
        (days + (months + 12 - own_month) * 30 + (years - own_year - 1) * 365) - 1
    } else if own_month<months {
        (days + (months - own_month) * 30 + (years - own_year) * 365) - 1
    } else {
        days - own_day - 1
    };

    println!("您的生日还有{}天",diff_days);
}

fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}
