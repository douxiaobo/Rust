use std::fs;

fn main() {
    let contents = fs::read_to_string("test.txt")
        .expect("无法读取文件 test.txt");

    let chars = contents.chars().count();   //统计文件中的字符数。
    // contents.chars() 方法将 contents 字符串转换为字符迭代器，count() 方法计算迭代器的长度。

    println!("文件 test.txt 中的字符数为：{}", chars);
}

//OK

// 文件 test.txt 中的字符数为：339292