use percent_encoding::{AsciiSet, utf8_percent_encode, CONTROLS};
use std::io::prelude::*;
use serde_json::Value;
use my_library::Engine;

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

pub fn save_to_file(text: &str, language: &str, filename: &str) -> bool {
    let len = text.len();
    let encoded_text = utf8_percent_encode(text, FRAGMENT).to_string();
    let url = format!("https://translate.google.com/translate_tts?ie=UTF-8&q={}&tl={}&total=1&idx=0&textlen={}&tl={}&client=tw-ob", encoded_text,language,len,language);

    if let Ok(response) = reqwest::blocking::get(url) {
        if response.status().is_success() {
            let json: Value = response.json().unwrap();
            let mut data = json["data"]
                .as_array()
                .unwrap()[0]
                .as_object()
                .unwrap()
                ["audio"]
                .as_str()
                .unwrap();

            // Remove the "data:" prefix from the Base64-encoded data
            data = &data[5..];

            let decoded_data = Engine::decode(data).unwrap();
            if let Ok(mut file) = std::fs::File::create(filename) {
                if file.write_all(&decoded_data).is_ok() {
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
