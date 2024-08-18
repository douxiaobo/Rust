use dioxus::prelude::*;
fn main() {
    launch(app);
}

fn app()->Element {
    rsx!{
        h1 {"Hello, Dioxus 0.5!"}
    }
}










// 这个错误信息表明你的 Dioxus 项目在启动时没有启用任何平台特性。Dioxus 需要你明确指定一个或多个平台特性（如 liveview, desktop, mobile, web, tui, fullstack）才能正常运行。

// 要解决这个问题，你需要在你的 Cargo.toml 文件中启用至少一个平台特性。例如，如果你想在桌面上运行你的 Dioxus 应用，你需要在 Cargo.toml 中添加 desktop 特性：

// [dependencies]
// dioxus = { version = "0.5.6", features = ["desktop"] }
// CopyInsert
// 以下是一些常见的特性及其用途：

// desktop: 用于桌面应用
// mobile: 用于移动应用
// web: 用于 Web 应用
// tui: 用于终端用户界面
// fullstack: 用于全栈应用
// 根据你的需求选择合适的特性，并将其添加到 Cargo.toml 文件中。然后重新运行 cargo run 命令。

// 例如，如果你希望创建一个桌面应用，你的 Cargo.toml 文件应该包含以下内容：

// [package]
// name = "my-dioxus-app"
// version = "0.1.0"
// edition = "2018"

// [dependencies]
// dioxus = { version = "0.5.6", features = ["desktop"] }
// CopyInsert
// 这样配置后，再次运行 cargo run 应该可以正常启动你的 Dioxus 应用。




// douxiaobo@192 Rust % cargo new my-dioxus-app
//     Creating binary (application) `my-dioxus-app` package
// note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
// douxiaobo@192 Rust % cd my-dioxus-app
// douxiaobo@192 my-dioxus-app % code .
// douxiaobo@192 my-dioxus-app % cargo run
//     Blocking waiting for file lock on package cache
//     Blocking waiting for file lock on package cache
//    Compiling proc-macro2 v1.0.86
//    Compiling unicode-ident v1.0.12
//    Compiling autocfg v1.3.0
//    Compiling serde v1.0.207
//    Compiling cfg-if v1.0.0
//    Compiling once_cell v1.19.0
//    Compiling version_check v0.9.5
//    Compiling smallvec v1.13.2
//    Compiling pin-project-lite v0.2.14
//    Compiling scopeguard v1.2.0
//    Compiling parking_lot_core v0.9.10
//    Compiling libc v0.2.155
//    Compiling semver v1.0.23
//    Compiling futures-core v0.3.30
//    Compiling zerocopy v0.7.35
//    Compiling log v0.4.22
//    Compiling pin-utils v0.1.0
//    Compiling futures-task v0.3.30
//    Compiling overload v0.1.1
//    Compiling lazy_static v1.5.0
//    Compiling tracing-core v0.1.32
//    Compiling sharded-slab v0.1.7
//    Compiling nu-ansi-term v0.46.0
//    Compiling thread_local v1.1.8
//    Compiling thiserror v1.0.63
//    Compiling allocator-api2 v0.2.18
//    Compiling ahash v0.8.11
//    Compiling lock_api v0.4.12
//    Compiling slab v0.4.9
//    Compiling slotmap v1.0.7
//    Compiling tracing-log v0.2.0
//    Compiling longest-increasing-subsequence v0.1.0
//    Compiling crossbeam-utils v0.8.20
//    Compiling tracing-subscriber v0.3.18
//    Compiling serde_json v1.0.125
//    Compiling rustc-hash v1.1.0
//    Compiling quote v1.0.36
//    Compiling syn v2.0.74
//    Compiling itoa v1.0.11
//    Compiling ryu v1.0.18
//    Compiling camino v1.1.7
//    Compiling memchr v2.7.4
//    Compiling num-traits v0.2.19
//    Compiling fnv v1.0.7
//    Compiling ident_case v1.0.1
//    Compiling equivalent v1.0.1
//    Compiling futures-channel v0.3.30
//    Compiling fixedbitset v0.4.2
//    Compiling unicode-segmentation v1.11.0
//    Compiling parking v2.2.0
//    Compiling cfg-expr v0.15.8
//    Compiling convert_case v0.6.0
//    Compiling futures-io v0.3.30
//    Compiling atomic-waker v1.1.2
//    Compiling fastrand v2.1.0
//    Compiling piper v0.2.4
//    Compiling futures-lite v2.3.0
//    Compiling async-task v4.7.1
//    Compiling prettyplease v0.2.20
//    Compiling hashbrown v0.14.5
//    Compiling intmap v0.7.1
//    Compiling darling_core v0.20.10
//    Compiling indexmap v2.4.0
//    Compiling to_method v1.1.0
//    Compiling dioxus-debug-cell v0.1.1
//    Compiling dioxus v0.5.6
//    Compiling constcat v0.3.1
//    Compiling petgraph v0.6.5
//    Compiling parking_lot v0.12.3
//    Compiling generational-box v0.5.6
//    Compiling futures-util v0.3.30
//    Compiling serde_derive v1.0.207
//    Compiling tracing-attributes v0.1.27
//    Compiling thiserror-impl v1.0.63
//    Compiling darling_macro v0.20.10
//    Compiling darling v0.20.10
//    Compiling concurrent-queue v2.5.0
//    Compiling event-listener v5.3.1
//    Compiling event-listener-strategy v0.5.2
//    Compiling tracing v0.1.40
//    Compiling async-trait v0.1.81
//    Compiling async-channel v2.3.1
//    Compiling serde_repr v0.1.19
//    Compiling blocking v1.6.1
//    Compiling dioxus-html-internal-macro v0.5.6
//    Compiling internment v0.7.5
//    Compiling spinning v0.1.0
//    Compiling dioxus-config-macro v0.5.6
//    Compiling ordered-float v2.10.1
//    Compiling enumset_derive v0.10.0
//    Compiling enumset v1.1.5
//    Compiling cargo-platform v0.1.8
//    Compiling bitflags v2.6.0
//    Compiling serde-value v0.7.0
//    Compiling euclid v0.22.10
//    Compiling keyboard-types v0.7.0
//    Compiling rustc_version v0.4.0
//    Compiling interprocess-docfix v1.2.2
//    Compiling dioxus-core v0.5.6
//    Compiling cargo_metadata v0.18.1
//    Compiling dioxus-html v0.5.6
//    Compiling dioxus-signals v0.5.7
//    Compiling krates v0.16.10
//    Compiling dioxus-hooks v0.5.6
//    Compiling dioxus-rsx v0.5.6
//    Compiling dioxus-core-macro v0.5.6
//    Compiling dioxus-hot-reload v0.5.6
//    Compiling my-dioxus-app v0.1.0 (/Users/douxiaobo/Documents/Practice in Coding/Rust/my-dioxus-app)
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 1m 32s
//      Running `target/debug/my-dioxus-app`
// thread 'main' panicked at /Users/douxiaobo/.cargo/registry/src/index.crates.io-6f17d22bba15001f/dioxus-0.5.6/src/launch.rs:274:9:
// No platform feature enabled. Please enable one of the following features: liveview, desktop, mobile, web, tui, fullstack to use the launch API.
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
// douxiaobo@192 my-dioxus-app % cargo run
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.06s
//      Running `target/debug/my-dioxus-app`
// thread 'main' panicked at /Users/douxiaobo/.cargo/registry/src/index.crates.io-6f17d22bba15001f/dioxus-0.5.6/src/launch.rs:274:9:
// No platform feature enabled. Please enable one of the following features: liveview, desktop, mobile, web, tui, fullstack to use the launch API.
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
// douxiaobo@192 my-dioxus-app % cargo run
//     Blocking waiting for file lock on package cache
//    Compiling scopeguard v1.2.0
//    Compiling crossbeam-utils v0.8.20
//    Compiling cc v1.0.101
//    Compiling fnv v1.0.7
//    Compiling malloc_buf v0.0.6
//    Compiling core-foundation-sys v0.8.6
//    Compiling bitflags v1.3.2
//    Compiling block v0.1.6
//    Compiling foreign-types-macros v0.2.3
//    Compiling foreign-types-shared v0.3.1
//    Compiling bytes v1.6.0
//    Compiling tinyvec_macros v0.1.1
//    Compiling lock_api v0.4.12
//    Compiling crc32fast v1.4.0
//    Compiling darling_core v0.20.10
//    Compiling tinyvec v1.6.0
//    Compiling simd-adler32 v0.3.7
//    Compiling tokio-macros v2.3.0
//    Compiling num_cpus v1.16.0
//    Compiling percent-encoding v2.3.1
//    Compiling core-foundation v0.9.4
//    Compiling parking_lot v0.12.3
//    Compiling unicode-bidi v0.3.15
//    Compiling adler v1.0.2
//    Compiling miniz_oxide v0.7.4
//    Compiling form_urlencoded v1.2.1
//    Compiling core-graphics-types v0.1.3
//    Compiling generational-box v0.5.6
//    Compiling foreign-types v0.5.0
//    Compiling cfg_aliases v0.1.1
//    Compiling raw-window-handle v0.6.2
//    Compiling md5 v0.7.0
//    Compiling wry v0.37.0
//    Compiling dioxus-core v0.5.6
//    Compiling tokio v1.38.0
//    Compiling dioxus-interpreter-js v0.5.6
//    Compiling internment v0.7.5
//    Compiling unicode-normalization v0.1.23
//    Compiling spinning v0.1.0
//    Compiling fdeflate v0.3.4
//    Compiling lru v0.12.4
//    Compiling sledgehammer_bindgen_macro v0.5.1
//    Compiling flate2 v1.0.28
//    Compiling signal-hook v0.3.17
//    Compiling dispatch v0.2.0
//    Compiling byteorder v1.5.0
//    Compiling uuid v1.8.0
//    Compiling objc_exception v0.1.2
//    Compiling idna v0.5.0
//    Compiling dioxus-cli-config v0.5.6
//    Compiling rfd v0.14.1
//    Compiling png v0.17.13
//    Compiling cfb v0.7.3
//    Compiling dioxus-rsx v0.5.6
//    Compiling concurrent-queue v2.5.0
//    Compiling core-graphics v0.23.2
//    Compiling crossbeam-channel v0.5.13
//    Compiling event-listener v5.3.1
//    Compiling darling_macro v0.20.10
//    Compiling event-listener-strategy v0.5.2
//    Compiling url v2.5.0
//    Compiling sledgehammer_bindgen v0.5.0
//    Compiling async-channel v2.3.1
//    Compiling dioxus-signals v0.5.7
//    Compiling blocking v1.6.1
//    Compiling sledgehammer_utils v0.2.1
//    Compiling darling v0.20.10
//    Compiling interprocess-docfix v1.2.2
//    Compiling http v0.2.12
//    Compiling signal-hook-registry v1.4.2
//    Compiling instant v0.1.12
//    Compiling raw-window-handle v0.5.2
//    Compiling dioxus-desktop v0.5.6
//    Compiling dioxus-hooks v0.5.6
//    Compiling objc v0.2.7
//    Compiling webbrowser v0.8.15
//    Compiling cocoa-foundation v0.1.2
//    Compiling objc_id v0.1.1
//    Compiling objc-foundation v0.1.1
//    Compiling infer v0.11.0
//    Compiling dioxus v0.5.6
//    Compiling urlencoding v2.1.3
//    Compiling cocoa v0.25.0
//    Compiling dunce v1.0.5
//    Compiling dioxus-config-macro v0.5.6
//    Compiling dioxus-core-macro v0.5.6
//    Compiling enumset_derive v0.10.0
//    Compiling global-hotkey v0.5.5
//    Compiling tao v0.26.2
//    Compiling muda v0.11.5
//    Compiling enumset v1.1.5
//    Compiling dioxus-html v0.5.6
//    Compiling dioxus-hot-reload v0.5.6
//    Compiling my-dioxus-app v0.1.0 (/Users/douxiaobo/Documents/Practice in Coding/Rust/my-dioxus-app)
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 4m 01s
//      Running `target/debug/my-dioxus-app`
// douxiaobo@192 my-dioxus-app % 