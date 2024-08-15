use log::{error, warn, info, debug};
extern crate env_logger;
fn main() {
    env_logger::init();
    error!("Error message");
    warn!("Warning message");
    info!("Info message");
    debug!("Debug message");
}

// douxiaobo@192 log_lib % cargo run
//    Compiling log_lib v0.1.0 (/Users/douxiaobo/Documents/Practice in Coding/Rust/log_lib)
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
//      Running `target/debug/log_lib`
// [2024-07-20T12:11:14Z ERROR log_lib] Error message
// douxiaobo@192 log_lib % cargo build
//    Compiling log_lib v0.1.0 (/Users/douxiaobo/Documents/Practice in Coding/Rust/log_lib)
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
// douxiaobo@192 log_lib % ./target/debug/log_lib
// [2024-07-20T12:15:11Z ERROR log_lib] Error message
// douxiaobo@192 log_lib % rust_log=debug ./target/debug/log_lib
// [2024-07-20T12:16:42Z ERROR log_lib] Error message
// douxiaobo@192 log_lib % RUST_LOG=debug ./target/debug/log_lib
// [2024-07-20T12:17:06Z ERROR log_lib] Error message
// [2024-07-20T12:17:06Z WARN  log_lib] Warning message
// [2024-07-20T12:17:06Z INFO  log_lib] Info message
// [2024-07-20T12:17:06Z DEBUG log_lib] Debug message
// douxiaobo@192 log_lib % RUST_LOG=info ./target/debug/log_lib
// [2024-07-20T12:17:31Z ERROR log_lib] Error message
// [2024-07-20T12:17:31Z WARN  log_lib] Warning message
// [2024-07-20T12:17:31Z INFO  log_lib] Info message
// douxiaobo@192 log_lib % RUST_LOG=warn ./target/debug/log_lib
// [2024-07-20T12:18:40Z ERROR log_lib] Error message
// [2024-07-20T12:18:40Z WARN  log_lib] Warning message
// douxiaobo@192 log_lib % RUST_LOG=error ./target/debug/log_lib
// [2024-07-20T12:19:00Z ERROR log_lib] Error message
// douxiaobo@192 log_lib % ./target/debug/log_lib
// [2024-07-20T12:19:08Z ERROR log_lib] Error message
// douxiaobo@192 log_lib % 
