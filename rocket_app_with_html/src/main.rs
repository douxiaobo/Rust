// fn main() {
//     println!("Hello, world!");
// }



#[macro_use] extern crate rocket;

use rocket::fs::FileServer;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", FileServer::from("static"))
}


// Last login: Sat Nov 23 19:44:35 on ttys000
// douxiaobo@192 Rust % cargo new rocket_app_with_html --bin
//     Creating binary (application) `rocket_app_with_html` package
// note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
// douxiaobo@192 Rust % cd rocket_app_with_html
// douxiaobo@192 rocket_app_with_html % code .
// douxiaobo@192 rocket_app_with_html % cargo run
//    Compiling pin-project-lite v0.2.15
//    Compiling bytes v1.8.0
//    Compiling itoa v1.0.13
//    Compiling pin-utils v0.1.0
//    Compiling equivalent v1.0.1
//    Compiling hashbrown v0.15.1
//    Compiling futures-core v0.3.31
//    Compiling fnv v1.0.7
//    Compiling memchr v2.7.4
//    Compiling once_cell v1.20.2
//    Compiling libc v0.2.164
//    Compiling cfg-if v1.0.0
//    Compiling serde v1.0.215
//    Compiling futures-sink v0.3.31
//    Compiling slab v0.4.9
//    Compiling httparse v1.9.5
//    Compiling futures-task v0.3.31
//    Compiling time-core v0.1.2
//    Compiling num-conv v0.1.0
//    Compiling futures-channel v0.3.31
//    Compiling inlinable_string v0.1.15
//    Compiling tracing-core v0.1.32
//    Compiling futures-io v0.3.31
//    Compiling try-lock v0.2.5
//    Compiling powerfmt v0.2.0
//    Compiling time-macros v0.2.18
//    Compiling tower-service v0.3.3
//    Compiling want v0.3.1
//    Compiling httpdate v1.0.3
//    Compiling smallvec v1.13.2
//    Compiling deranged v0.3.11
//    Compiling futures-util v0.3.31
//    Compiling http v0.2.12
//    Compiling tracing v0.1.40
//    Compiling socket2 v0.5.7
//    Compiling mio v1.0.2
//    Compiling byteorder v1.5.0
//    Compiling percent-encoding v2.3.1
//    Compiling ref-cast v1.0.23
//    Compiling zerocopy v0.7.35
//    Compiling stable-pattern v0.1.0
//    Compiling winnow v0.6.20
//    Compiling signal-hook-registry v1.4.2
//    Compiling is-terminal v0.4.13
//    Compiling yansi v1.0.1
//    Compiling tokio v1.41.1
//    Compiling getrandom v0.2.15
//    Compiling http-body v0.4.6
//    Compiling state v0.6.0
//    Compiling either v1.13.0
//    Compiling ppv-lite86 v0.2.20
//    Compiling pear v0.2.9
//    Compiling rand_core v0.6.4
//    Compiling errno v0.3.9
//    Compiling indexmap v2.6.0
//    Compiling scopeguard v1.2.0
//    Compiling bitflags v2.6.0
//    Compiling rand_chacha v0.3.1
//    Compiling lock_api v0.4.12
//    Compiling parking_lot_core v0.9.10
//    Compiling time v0.3.36
//    Compiling futures v0.3.31
//    Compiling rustix v0.38.41
//    Compiling http v1.1.0
//    Compiling encoding_rs v0.8.35
//    Compiling spin v0.9.8
//    Compiling mime v0.3.17
//    Compiling fastrand v2.2.0
//    Compiling log v0.4.22
//    Compiling async-stream v0.3.6
//    Compiling parking_lot v0.12.3
//    Compiling rand v0.8.5
//    Compiling num_cpus v1.16.0
//    Compiling toml_datetime v0.6.8
//    Compiling serde_spanned v0.6.8
//    Compiling uncased v0.9.10
//    Compiling ubyte v0.10.4
//    Compiling atomic v0.5.3
//    Compiling binascii v0.1.4
//    Compiling tempfile v3.14.0
//    Compiling toml_edit v0.22.22
//    Compiling hyper v0.14.31
//    Compiling cookie v0.18.1
//    Compiling rocket_http v0.5.1
//    Compiling toml v0.8.19
//    Compiling tokio-util v0.7.12
//    Compiling tokio-stream v0.1.16
//    Compiling figment v0.10.19
//    Compiling h2 v0.3.26
//    Compiling multer v3.1.0
//    Compiling rocket_codegen v0.5.1
//    Compiling rocket v0.5.1
//    Compiling rocket_app_with_html v0.1.0 (/Users/douxiaobo/Documents/Practice in Coding/Rust/rocket_app_with_html)
// error[E0433]: failed to resolve: could not find `Files` in `fs`
//   --> src/main.rs:11:45
//    |
// 11 |     rocket::ignite().mount("/", rocket::fs::Files::from("static"))
//    |                                             ^^^^^ could not find `Files` in `fs`

// error[E0425]: cannot find function `ignite` in crate `rocket`
//   --> src/main.rs:11:13
//    |
// 11 |     rocket::ignite().mount("/", rocket::fs::Files::from("static"))
//    |             ^^^^^^ not found in `rocket`

// Some errors have detailed explanations: E0425, E0433.
// For more information about an error, try `rustc --explain E0425`.
// error: could not compile `rocket_app_with_html` (bin "rocket_app_with_html") due to 2 previous errors
// douxiaobo@192 rocket_app_with_html % cargo run
//    Compiling rocket_app_with_html v0.1.0 (/Users/douxiaobo/Documents/Practice in Coding/Rust/rocket_app_with_html)
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.05s
//      Running `target/debug/rocket_app_with_html`
// 🔧 Configured for debug.
//    >> address: 127.0.0.1
//    >> port: 8000
//    >> workers: 14
//    >> max blocking threads: 512
//    >> ident: Rocket
//    >> IP header: X-Real-IP
//    >> limits: bytes = 8KiB, data-form = 2MiB, file = 1MiB, form = 32KiB, json = 1MiB, msgpack = 1MiB, string = 8KiB
//    >> temp dir: /var/folders/w6/m3p5s_493_lfr5qmr1w96x7m0000gn/T/
//    >> http/2: true
//    >> keep-alive: 5s
//    >> tls: disabled
//    >> shutdown: ctrlc = true, force = true, signals = [SIGTERM], grace = 2s, mercy = 3s
//    >> log level: normal
//    >> cli colors: true
// 📬 Routes:
//    >> (FileServer: static) GET /<path..> [10]
// 📡 Fairings:
//    >> Shield (liftoff, response, singleton)
// 🛡️ Shield:
//    >> X-Content-Type-Options: nosniff
//    >> X-Frame-Options: SAMEORIGIN
//    >> Permissions-Policy: interest-cohort=()
// 🚀 Rocket has launched from http://127.0.0.1:8000
// GET / text/html:
//    >> Matched: (FileServer: static) GET /<path..> [10]
//    >> Outcome: Success(200 OK)
//    >> Response succeeded.
// GET /favicon.ico image/avif:
//    >> Matched: (FileServer: static) GET /<path..> [10]
//    >> I/O Error: Os { code: 2, kind: NotFound, message: "No such file or directory" }
//    >> Outcome: Forward(404 Not Found)
//    >> No matching routes for GET /favicon.ico image/avif.
//    >> No 404 catcher registered. Using Rocket default.
//    >> Response succeeded.
// ^CWarning: Received SIGINT. Requesting shutdown.
// Shutdown requested. Waiting for pending I/O...
// Graceful shutdown completed successfully.
