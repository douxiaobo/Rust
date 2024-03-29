use std::io::Write;
use tokio;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Baidufanyi {
    errno: i64,
    data: Vec<Datum>,
    logid: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Datum {
    k: String,
    v: String,
}

fn capitalize_first_letter(s: &str) -> String {  
    let mut chars = s.chars();  
    match chars.next() {  
        None => String::new(), // 如果字符串为空，返回空字符串  
        Some(first) => {  
            let first_upper = first.to_uppercase().to_string();  
            first_upper + &chars.collect::<String>() // 拼接大写首字母和剩余的字符串  
        }  
    }  
}  

#[tokio::main]

async fn main() -> Result<(),Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder()
        .build()?;

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/96.0.4664.45 Safari/537.36".parse()?);
    headers.insert("Content-Type", "application/x-www-form-urlencoded".parse()?);
    headers.insert("Cookie", "BAIDUID=8DC9F1CF6226539B8F14D1B7ADDBC417:FG=1; BAIDUID_BFESS=8DC9F1CF6226539B8F14D1B7ADDBC417:FG=1".parse()?);

    let mut buf = String::new();
    print!("请输入要翻译的词：");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut params = std::collections::HashMap::new();
    params.insert("kw", buf.trim());

    let request = client.request(reqwest::Method::POST, "https://fanyi.baidu.com/sug")
        .headers(headers)
        .form(&params);

    let response = request.send().await?;
    let body = response.text().await?;

    // println!("{}", body);

    let dict_obj:Baidufanyi=serde_json::from_str(&body).unwrap();

    if let Some(first_datum)=dict_obj.data.first(){
        if buf.trim()==first_datum.k {
            println!("翻译成功！\n{}",first_datum.v);
        } else if buf.to_lowercase().trim()==first_datum.k {
            println!("翻译成功！但是转换小写。\n{}",first_datum.v);
        } else if capitalize_first_letter(buf.trim())==first_datum.k{
            println!("翻译成功！但是第一个字母大写。\n{}",first_datum.v);
        } else {
            println!("翻译失败！");
        }        
    } else {
        println!("不存在此词！");
    }

    Ok(())
}