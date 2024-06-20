// fn passwd_pre(pwd: &str) -> String {
//     let mut vret = Vec::new();
//     for char in pwd.chars() {
//         match char {
//             'a' | 'b' | 'c' => vret.push('!'),
//             'd' | 'e' | 'f' => vret.push('@'),
//             'g' | 'h' | 'i' => vret.push('#'),
//             'j' | 'k' | 'l' => vret.push('%'),
//             'm' | 'n' | 'o' => vret.push('^'),
//             'p' | 'q' | 'r'  => vret.push('&'),
//             's' | 't' | 'u'  => vret.push('*'),
//             'v' | 'w' | 'x'  => vret.push('>'),
//             'y' | 'z'   => vret.push('?'),
//             'Z'  => vret.push('a'),
//             // _ if char.is_uppercase() => vret.push((char.to_lowercase() as u8 - b'A' + 1) as char),
//             _ if char.is_uppercase() => {
//                 let lower_char = char.to_lowercase().next().unwrap(); // Convert to lowercase
//                 let idx = lower_char as u8 - b'a' + 1; // Safe subtraction because we know it's a lowercase letter
//                 vret.push(std::char::from_u32(idx as u32).unwrap()); // Convert back to char
//             },
//             _ => vret.push(char),
//         }
//     }
//     vret.into_iter().collect()
// }

fn passwd_pre(pwd: &str) -> String {
    let mut vret = Vec::new();
    for char in pwd.chars() {
        match char.to_ascii_lowercase() {
            'a'..='c' => vret.push('!'),
            'd'..='f' => vret.push('@'),
            'g'..='i' => vret.push('#'),
            'j'..='l' => vret.push('%'),
            'm'..='o' => vret.push('^'),
            'p'..='r' => vret.push('&'),
            's'..='t' => vret.push('*'),
            'u'..='x' => vret.push('>'),
            'y' | 'z' => vret.push('?'),
            _ if char.is_uppercase() => {
                let lower = char.to_ascii_lowercase();
                vret.push(('a' as u8 + lower as u8 - ('a' as u8)) as char);
            },
            _ => vret.push(char),
        }
    }
    vret.into_iter().collect()
}

fn change_txt(pwd: &str, str1: &str, str2: &str) -> String {
    let mut vret = String::new();
    for char in pwd.chars() {
        if let Some(j) = str1.find(char) {
            vret.push(str2.chars().nth(j).unwrap_or(char));
        } else {
            vret.push(char);
        }
    }
    vret
}

fn change_password(pwd: &str) -> String {
    if pwd.is_empty() {
        return "-1".to_string();
    }
    let vpre = passwd_pre(pwd);
    let vlen = pwd.len();
    let mut vret = String::new();
    vret.push_str(&vpre);

    // 使用闭包和迭代器来实现转换逻辑
    let converters = [
        ("1234567890abcdefghijklmnopqrstuvwxyz", "abcdefghijklmnopqrstuvwxyz1234567890"),
        ("1234567890abcdefghijklmnopqrstuvwxyz", "qwertyuiopasdfghjklmnbvcxz0987654321"),
        ("1234567890abcdefghijklmnopqrstuvwxyz", "1qaz2wsx3edc4rfv5tgb6yhn7ujm8ik9ol0p"),
        ("1234567890abcdefghijklmnopqrstuvwxyz", "pl0okm9ijn8uhb7ygv6tfc5rdx4esz3wa2q1"),
    ];

    for (str1, str2) in &converters {
        let vstr = change_txt(pwd, *str1, *str2);
        vret.push_str(&vstr[..vlen.min(4)]);
    }

    vret
}


fn main() {
    loop {
        let mut pwd = String::new();
        println!("请录入密码：");
        std::io::stdin().read_line(&mut pwd).expect("Failed to read line");
        pwd = pwd.trim().to_string();
        if pwd == "q" {
            println!("退出程序...");
            break;
        } else {
            let pwdnew = change_password(&pwd);
            println!("您录入的密码是: {}, 该密码加密后为：{}", pwd, pwdnew);
        }
    }
}


// douxiaobo@192 Rust % rustc password_change.rs
// douxiaobo@192 Rust % ./password_change
// 请录入密码：
// testT123@/
// 您录入的密码是: testT123@/, 该密码加密后为：*@**123@/4o347g87if8iz7sz
// 请录入密码：
// ttT22#liK
// 您录入的密码是: ttT22#liK, 该密码加密后为：**22#%#
//                                                   44Tb77TwiiTqzzTl
// 请录入密码：
// q
// 退出程序...
// douxiaobo@192 Rust % rustc password_change.rs
// douxiaobo@192 Rust % ./password_change       
// 请录入密码：
// testT123@/
// 您录入的密码是: testT123@/, 该密码加密后为：*@***123@/4o347g87if8iz7sz
// 请录入密码：
// ttT22#liK
// 您录入的密码是: ttT22#liK, 该密码加密后为：***22#%#%44Tb77TwiiTqzzTl
// 请录入密码：
// q
// 退出程序...
// douxiaobo@192 Rust % 


// 错误