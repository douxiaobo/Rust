use lazy_static::lazy_static;
use std::collections::HashMap;
lazy_static! {
    static ref DICTIONARY: HashMap<u32, &'static str> = {
        let mut m=HashMap::new();
        m.insert(0,"zero");
        m.insert(1,"one");
        m.insert(2,"two");
        m.insert(3,"three");
        println!("Initialized");
        m
    };
}
fn main() {
    println!("Started");
    println!("Dictionary: {:#?}", *DICTIONARY);
    // println!("Dictionary: {:#?}", *DICTIONARY);
    println!("Dictionary[0]:{}", DICTIONARY.get(&0).unwrap());
}
