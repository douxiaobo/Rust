fn main() {
    // 1. Define the config structure.
    let config_const_values = {
        // 2. Get the path of the config file from the command line.
        let config_path = std::env::args().nth(1).unwrap();

        // 3. Load the whole file contents into a string.
        let config_text = std::fs::read_to_string(&config_path).unwrap();

        // 4. Load an unmutable config structure from the string.
        config_text.parse::<toml::Value>().unwrap()
    };

    // 5. Show the whole config structure.
    println!("Original: {:#?}", config_const_values);

    // 6. Get and show one config value.
    println!(
        "[Postgresql].Database: {}",
        config_const_values
            .get("postgresql")
            .unwrap()
            .get("database")
            .unwrap()
            .as_str()
            .unwrap()
    );
}

// douxiaobo@192 read_toml_dynamic % cargo run ./data/config.toml
//    Compiling hashbrown v0.14.5
//    Compiling equivalent v1.0.1
//    Compiling winnow v0.6.18
//    Compiling serde v1.0.208
//    Compiling indexmap v2.4.0
//    Compiling toml_datetime v0.6.8
//    Compiling serde_spanned v0.6.7
//    Compiling toml_edit v0.22.20
//    Compiling toml v0.8.19
//    Compiling read_toml_dynamic v0.1.0 (/Users/douxiaobo/Documents/Practice in Coding/Rust/read_toml_dynamic)
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.95s
//      Running `target/debug/read_toml_dynamic ./data/config.toml`
// Original: Table(
//     {
//         "input": Table(
//             {
//                 "json_file": String(
//                     "./data/sales.json",
//                 ),
//                 "xml_file": String(
//                     "./data/sales.xml",
//                 ),
//             },
//         ),
//         "postgresql": Table(
//             {
//                 "database": String(
//                     "Rust2018",
//                 ),
//                 "host": String(
//                     "localhost",
//                 ),
//                 "password": String(
//                     "post",
//                 ),
//                 "port": String(
//                     "5432",
//                 ),
//                 "username": String(
//                     "postgres",
//                 ),
//             },
//         ),
//         "redis": Table(
//             {
//                 "host": String(
//                     "localhost",
//                 ),
//             },
//         ),
//         "sqlite": Table(
//             {
//                 "db_file": String(
//                     "./data/sales.db",
//                 ),
//             },
//         ),
//     },
// )
// [Postgresql].Database: Rust2018
// douxiaobo@192 read_toml_dynamic % 