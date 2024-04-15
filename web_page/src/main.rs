use tokio;

use std::fs::File;
use std::io::Write;

fn input(show_input:String) -> String {
    let mut input_word = String::new();
    print!("{}",show_input);
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut input_word).expect("Failed to read line");
    input_word=input_word.trim_end().to_string();
    println!("{}",input_word);
    input_word
}

fn save_html_to_file(html_content: &str, file_path: &str) -> Result<(), std::io::Error> {
    let mut file = File::create(file_path)?;
    file.write_all(html_content.as_bytes())?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let key=input("请输入关键字：".to_string());
    let url="https://www.baidu.com/s?wd=".to_owned()+key.trim();
    let client = reqwest::Client::builder()
        .build()?;

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("User-Agent", "Mozilla/5.0 (Windows NT 6.1; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/63.0.3239.132 Safari/537.36 QIHU 360SE".parse()?);
    headers.insert("Cookie", "BAIDUID=8DC9F1CF6226539B8F14D1B7ADDBC417:FG=1; BAIDUID_BFESS=8DC9F1CF6226539B8F14D1B7ADDBC417:FG=1; BIDUPSID=8DC9F1CF6226539B8F14D1B7ADDBC417; H_PS_PSSID=40366_40379_40416_40299_40459_39662_40500_40445_60040_60026_60035_60046_40511; PSTM=1713175681".parse()?);

    let request = client.request(reqwest::Method::GET, url)
        .headers(headers);

    let response = request.send().await?;
    let body = response.text().await?;

    // println!("{}", body);

    let output_file_name=input("请输入输出文件名：".to_string());

    let output_file=output_file_name+".html";

    if let Err(e) = save_html_to_file(&body, &output_file) {
        eprintln!("Failed to save HTML file: {}", e);
    } else {
        println!("HTML content successfully saved to {}", output_file);
    }

    Ok(())
}