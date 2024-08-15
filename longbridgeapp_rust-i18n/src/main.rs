use rust_i18n::t;
rust_i18n::i18n!("locales");
fn main() {
    println!("{}",t!("hello"));
    println!("{}",t!("hello",locale="zh-CN"));
    println!("{}",t!("messages.hello",name="Microsoft"));
    println!("{}",t!("messages.hello",name="晚安"));
    println!("{}",t!("messages.hello",name="晚安",locale="zh-CN"));
    println!("{:?}", rust_i18n::available_locales!());    
}

// https://github.com/longbridgeapp/rust-i18n



// Last login: Thu Aug 15 13:08:39 on console
// douxiaobo@192 Rust % code .
// douxiaobo@192 Rust % cargo new longbridgeapp_rust-i18n
//     Creating binary (application) `longbridgeapp_rust-i18n` package
// note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
// douxiaobo@192 Rust % cd long*
// douxiaobo@192 longbridgeapp_rust-i18n % cargo run
//     Updating crates.io index
//   Downloaded siphasher v1.0.1
//   Downloaded base62 v2.0.2
//   Downloaded serde_spanned v0.6.7
//   Downloaded stable_deref_trait v1.2.0
//   Downloaded rust-i18n-macro v3.1.1
//   Downloaded rust-i18n-support v3.1.1
//   Downloaded lazy_static v1.5.0
//   Downloaded globwalk v0.8.1
//   Downloaded toml_datetime v0.6.8
//   Downloaded rust-i18n v3.1.1
//   Downloaded globset v0.4.14
//   Downloaded either v1.13.0
//   Downloaded normpath v1.3.0
//   Downloaded arc-swap v1.7.1
//   Downloaded triomphe v0.1.13
//   Downloaded tempfile v3.12.0
//   Downloaded bitflags v2.6.0
//   Downloaded toml v0.7.8
//   Downloaded ignore v0.4.22
//   Downloaded serde_derive v1.0.207
//   Downloaded serde v1.0.207
//   Downloaded indexmap v2.4.0
//   Downloaded libyml v0.0.4
//   Downloaded toml_edit v0.19.15
//   Downloaded serde_yml v0.0.11
//   Downloaded itertools v0.11.0
//   Downloaded serde_json v1.0.125
//   Downloaded syn v2.0.74
//   Downloaded regex v1.10.6
//   Downloaded bstr v1.10.0
//   Downloaded 30 crates (2.2 MB) in 1m 16s
//     Blocking waiting for file lock on package cache
//    Compiling proc-macro2 v1.0.86
//    Compiling unicode-ident v1.0.12
//    Compiling memchr v2.7.4
//    Compiling serde v1.0.207
//    Compiling crossbeam-utils v0.8.20
//    Compiling regex-syntax v0.8.4
//    Compiling log v0.4.22
//    Compiling libc v0.2.155
//    Compiling same-file v1.0.6
//    Compiling equivalent v1.0.1
//    Compiling hashbrown v0.14.5
//    Compiling rustix v0.38.34
//    Compiling bitflags v2.6.0
//    Compiling bitflags v1.3.2
//    Compiling serde_json v1.0.125
//    Compiling cfg-if v1.0.0
//    Compiling walkdir v2.5.0
//    Compiling ryu v1.0.18
//    Compiling once_cell v1.19.0
//    Compiling winnow v0.5.40
//    Compiling fastrand v2.1.0
//    Compiling itoa v1.0.11
//    Compiling aho-corasick v1.1.3
//    Compiling bstr v1.10.0
//    Compiling indexmap v2.4.0
//    Compiling serde_yml v0.0.11
//    Compiling arc-swap v1.7.1
//    Compiling either v1.13.0
//    Compiling stable_deref_trait v1.2.0
//    Compiling libyml v0.0.4
//    Compiling itertools v0.11.0
//    Compiling siphasher v1.0.1
//    Compiling lazy_static v1.5.0
//    Compiling base62 v2.0.2
//    Compiling normpath v1.3.0
//    Compiling glob v0.3.1
//    Compiling regex-automata v0.4.7
//    Compiling smallvec v1.13.2
//    Compiling globset v0.4.14
//    Compiling regex v1.10.6
//    Compiling errno v0.3.9
//    Compiling crossbeam-epoch v0.9.18
//    Compiling crossbeam-deque v0.8.5
//    Compiling ignore v0.4.22
//    Compiling globwalk v0.8.1
//    Compiling tempfile v3.12.0
//    Compiling rust-i18n v3.1.1
//    Compiling quote v1.0.36
//    Compiling syn v2.0.74
//    Compiling serde_derive v1.0.207
//    Compiling toml_datetime v0.6.8
//    Compiling serde_spanned v0.6.7
//    Compiling triomphe v0.1.13
//    Compiling toml_edit v0.19.15
//    Compiling toml v0.7.8
//    Compiling rust-i18n-support v3.1.1
//    Compiling rust-i18n-macro v3.1.1
//    Compiling longbridgeapp_rust-i18n v0.1.0 (/Users/douxiaobo/Documents/Practice in Coding/Rust/longbridgeapp_rust-i18n)
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 1m 28s
//      Running `target/debug/longbridgeapp_rust-i18n`
// Hello world
// ["en", "zh-CN"]
// douxiaobo@192 longbridgeapp_rust-i18n % 
