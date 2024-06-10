use std::time::SystemTime;

fn main() {
    let no_leap_second = [
        1970, 1971, 1980, 1984, 1986, 1988, 1991, 1996, 1999, 2000, 2001, 2002, 2003, 2004, 2006,
        2007, 2009, 2010, 2011, 2013, 2014,
    ];
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

    let mut year = 1970;

    loop {
        if is_leap_year(year) {
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
        if year == 1972 {
            total_seconds -= 2;
        } else if no_leap_second.contains(&year) {
        } else {
            total_seconds -= 1;
        }
        year += 1;
    }

    let big_month = 31 * 86400;
    let small_month = 30 * 86400;
    let no_leap_month = 28 * 86400;
    let leap_month = 29 * 86400;
    let mut month = 1;
    loop {
        if (is_leap_year(year)) && (month == 2) {
            if total_seconds >= leap_month {
                total_seconds -= leap_month;
            } else {
                break;
            }
        } else if (!is_leap_year(year)) && (month == 2) {
            if total_seconds >= no_leap_month {
                total_seconds -= no_leap_month;
            } else {
                break;
            }
        } else if (month == 4) || (month == 6) || (month == 9) || (month == 11) {
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
        month += 1;
    }

    // let mut day = 0;
    // while total_seconds >= 86400 {
    //     total_seconds -= 86400;
    //     day += 1;
    // }
    // day += 1;

    // 计算日
    let day = total_seconds / 86400 + 1; // 直接计算日，注意加一

    println!("year:{year}");
    println!("month:{month}");
    println!("day:{day}");
}

fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}
