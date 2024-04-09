fn main(){
    // for c in "中国人".chars() {
    //     println!("{}", c);
    // }

    // for b in "中国人".bytes() {
    //     println!("{}", b);
    // }

    for c in "中国人".chars() {
        println!("{}", c.escape_unicode());
    }

    let unicode_str="\u{4e2d}\u{56fd}\u{4eba}";
    println!("{}", unicode_to_chinese(unicode_str));

    println!("中国人");
}


fn unicode_to_chinese(unicode: &str) -> String {
    unicode.chars()
        .map(|c| if c == '\\' && unicode.len() > 6 {
            let code_point_str = &unicode[2..6];
            code_point_str.parse::<u32>().ok()
                .and_then(std::char::from_u32)
        } else {
            Some(c)
        })
        .flatten()
        .collect()
}