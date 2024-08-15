use rand::prelude::*;

fn main() {
    let mut rng = thread_rng();
    println!("{}",rng.gen_range(0..20));
    println!("{}",rng.gen::<f64>());
    println!("{}",if rng.gen::<bool>() {"Heads"} else {"Tails"});
    println!("Hello, world!");
}

// Last login: Tue Jul 16 17:59:48 on ttys000
// douxiaobo@192 Rust % code .
// douxiaobo@192 Rust % cargo new rand_lib
//     Creating binary (application) `rand_lib` package
// note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
// douxiaobo@192 Rust % cd rand_lib
// douxiaobo@192 rand_lib % cargo run    
//    Compiling rand_lib v0.1.0 (/Users/douxiaobo/Documents/Practice in Coding/Rust/rand_lib)
// error[E0433]: failed to resolve: use of undeclared crate or module `rand`
//  --> src/main.rs:1:5
//   |
// 1 | use rand::prelude::*;
//   |     ^^^^ use of undeclared crate or module `rand`

// error[E0425]: cannot find value `rgn` in this scope
//  --> src/main.rs:7:22
//   |
// 7 |     println!("{}",if rgn.gen() {"Heads"} else {"Tails"});
//   |                      ^^^ help: a local variable with a similar name exists: `rng`

// error[E0425]: cannot find function `thread_rng` in this scope
//  --> src/main.rs:4:19
//   |
// 4 |     let mut rng = thread_rng();
//   |                   ^^^^^^^^^^ not found in this scope

// Some errors have detailed explanations: E0425, E0433.
// For more information about an error, try `rustc --explain E0425`.
// error: could not compile `rand_lib` (bin "rand_lib") due to 3 previous errors
// douxiaobo@192 rand_lib % cargo run
//     Updating crates.io index
//   Downloaded getrandom v0.2.15
//   Downloaded 1 crate (37.2 KB) in 0.54s
//    Compiling libc v0.2.155
//    Compiling cfg-if v1.0.0
//    Compiling ppv-lite86 v0.2.17
//    Compiling getrandom v0.2.15
//    Compiling rand_core v0.6.4
//    Compiling rand_chacha v0.3.1
//    Compiling rand v0.8.5
//    Compiling rand_lib v0.1.0 (/Users/douxiaobo/Documents/Practice in Coding/Rust/rand_lib)
// error[E0425]: cannot find value `rgn` in this scope
//  --> src/main.rs:7:22
//   |
// 7 |     println!("{}",if rgn.gen() {"Heads"} else {"Tails"});
//   |                      ^^^ help: a local variable with a similar name exists: `rng`

// error[E0277]: the trait bound `{integer}: SampleRange<_>` is not satisfied
//    --> src/main.rs:5:33
//     |
// 5   |     println!("{}",rng.gen_range(0,20));
//     |                       --------- ^ the trait `SampleRange<_>` is not implemented for `{integer}`
//     |                       |
//     |                       required by a bound introduced by this call
//     |
//     = help: the following other types implement trait `SampleRange<T>`:
//               std::ops::Range<T>
//               RangeInclusive<T>
// note: required by a bound in `gen_range`
//    --> /Users/douxiaobo/.cargo/registry/src/index.crates.io-6f17d22bba15001f/rand-0.8.5/src/rng.rs:132:12
//     |
// 129 |     fn gen_range<T, R>(&mut self, range: R) -> T
//     |        --------- required by a bound in this associated function
// ...
// 132 |         R: SampleRange<T>
//     |            ^^^^^^^^^^^^^^ required by this bound in `Rng::gen_range`

// error[E0061]: this method takes 1 argument but 2 arguments were supplied
//    --> src/main.rs:5:23
//     |
// 5   |     println!("{}",rng.gen_range(0,20));
//     |                       ^^^^^^^^^  ---
//     |                                  ||
//     |                                  |unexpected argument of type `{integer}`
//     |                                  help: remove the extra argument
//     |
// note: method defined here
//    --> /Users/douxiaobo/.cargo/registry/src/index.crates.io-6f17d22bba15001f/rand-0.8.5/src/rng.rs:129:8
//     |
// 129 |     fn gen_range<T, R>(&mut self, range: R) -> T
//     |        ^^^^^^^^^

// Some errors have detailed explanations: E0061, E0277, E0425.
// For more information about an error, try `rustc --explain E0061`.
// error: could not compile `rand_lib` (bin "rand_lib") due to 3 previous errors
// douxiaobo@192 rand_lib % cargo run
//    Compiling rand_lib v0.1.0 (/Users/douxiaobo/Documents/Practice in Coding/Rust/rand_lib)
// error[E0425]: cannot find value `rgn` in this scope
//  --> src/main.rs:7:22
//   |
// 7 |     println!("{}",if rgn.gen::<bool>() {"Heads"} else {"Tails"});
//   |                      ^^^ help: a local variable with a similar name exists: `rng`

// error[E0277]: the trait bound `{integer}: SampleRange<_>` is not satisfied
//    --> src/main.rs:5:33
//     |
// 5   |     println!("{}",rng.gen_range(0,20));
//     |                       --------- ^ the trait `SampleRange<_>` is not implemented for `{integer}`
//     |                       |
//     |                       required by a bound introduced by this call
//     |
//     = help: the following other types implement trait `SampleRange<T>`:
//               std::ops::Range<T>
//               RangeInclusive<T>
// note: required by a bound in `gen_range`
//    --> /Users/douxiaobo/.cargo/registry/src/index.crates.io-6f17d22bba15001f/rand-0.8.5/src/rng.rs:132:12
//     |
// 129 |     fn gen_range<T, R>(&mut self, range: R) -> T
//     |        --------- required by a bound in this associated function
// ...
// 132 |         R: SampleRange<T>
//     |            ^^^^^^^^^^^^^^ required by this bound in `Rng::gen_range`

// error[E0061]: this method takes 1 argument but 2 arguments were supplied
//    --> src/main.rs:5:23
//     |
// 5   |     println!("{}",rng.gen_range(0,20));
//     |                       ^^^^^^^^^  ---
//     |                                  ||
//     |                                  |unexpected argument of type `{integer}`
//     |                                  help: remove the extra argument
//     |
// note: method defined here
//    --> /Users/douxiaobo/.cargo/registry/src/index.crates.io-6f17d22bba15001f/rand-0.8.5/src/rng.rs:129:8
//     |
// 129 |     fn gen_range<T, R>(&mut self, range: R) -> T
//     |        ^^^^^^^^^

// Some errors have detailed explanations: E0061, E0277, E0425.
// For more information about an error, try `rustc --explain E0061`.
// error: could not compile `rand_lib` (bin "rand_lib") due to 3 previous errors
// douxiaobo@192 rand_lib % cargo run
//    Compiling rand_lib v0.1.0 (/Users/douxiaobo/Documents/Practice in Coding/Rust/rand_lib)
// error[E0425]: cannot find value `rgn` in this scope
//  --> src/main.rs:7:22
//   |
// 7 |     println!("{}",if rgn.gen::<bool>() {"Heads"} else {"Tails"});
//   |                      ^^^ help: a local variable with a similar name exists: `rng`

// For more information about this error, try `rustc --explain E0425`.
// error: could not compile `rand_lib` (bin "rand_lib") due to 1 previous error
// douxiaobo@192 rand_lib % cargo run
//    Compiling rand_lib v0.1.0 (/Users/douxiaobo/Documents/Practice in Coding/Rust/rand_lib)
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
//      Running `target/debug/rand_lib`
// 10
// 0.3551569771524272
// Tails
// Hello, world!
// douxiaobo@192 rand_lib % 
