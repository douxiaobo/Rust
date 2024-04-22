use percent_encoding::{AsciiSet, utf8_percent_encode, CONTROLS};
use minreq::get;
use std::fs::File;
use std::io::prelude::*;

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

pub fn save_to_file(text: &str,language: &str, filename:&str)->bool {
    let len=text.len();
    let text=utf8_percent_encode(text,FRAGMENT).to_string();        //sl是原文件，tl是目标语言
    if let Ok(rep)=get(format!("https://translate.google.com/translate_tts?ie=UTF-8&q={}&tl={}&total=1&idx=0&textlen={}&tl={}&client=tw-ob",text,language,len,language)).send(){
        if let Ok(mut file)=File::create(filename){
            let bytes=rep.as_bytes();
            if bytes.len()>0{
                if file.write_all(bytes).is_ok(){
                    return true;
                }
            }
        }
    }
    false
}

fn main() {
    save_to_file("Hello,World", "en", "test-en.mp3");
    save_to_file("您好！上海！", "zh-CN", "test-cn.mp3");
    println!("Hello, world!");
}
