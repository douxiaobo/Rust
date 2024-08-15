use std::collections::HashMap;

fn color_set(str: &str, sel_color: &str) -> String {
    let color_dic: HashMap<&str, &str> = HashMap::from([
        ("RED", "\x1b[31m"),    // 红色
        ("GREEN", "\x1b[32m"),  // 绿色
        ("YELLOW", "\x1b[33m"), // 黄色
        ("BLUE", "\x1b[34m"),   // 蓝色
        ("FUCHSIA", "\x1b[35m"),// 紫红色
        ("CYAN", "\x1b[36m"),   // 青蓝色
        ("WHITE", "\x1b[37m"),   // 白色
        ("NORMAL", "\x1b[0m"),  // 终端默认颜色
    ]);

    let sel_color_upper = sel_color.to_uppercase();
    match color_dic.get(&sel_color_upper.as_str()) {
        Some(color_code) => format!("{}{}{}", color_code, str, color_dic["NORMAL"]),
        None => {
            println!("没有找到对应颜色，采用终端默认颜色...");
            format!("{}{}{}", color_dic["NORMAL"], str, color_dic["NORMAL"])
        }
    }
}

fn main() {
    println!("{}", color_set("这一句话是红色", "RED"));
    println!("{}", color_set("这一句话是绿色", "green"));
    println!("{}", color_set("这一句话是黄色", "yellow"));
    println!("{}", color_set("这一句话是蓝色", "blue"));
    println!("{}", color_set("这一句话是紫红色", "fuchsia"));
    println!("{}", color_set("这一句话是颜色未设置", "test"));
}