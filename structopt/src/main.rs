use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt,Debug)]
#[warn(dead_code)]
struct Opt{
    #[structopt(short="v",long="verbose")]
    verbose: bool,

    #[structopt(short="r",long="result",parse(from_os_str))]
    result_file:PathBuf,

    #[structopt(name="FILE",parse(from_os_str))]
    files:Vec<PathBuf>,
}
fn main() {
    println!("{:#?}",Opt::from_args());
    
    let opt = Opt::from_args();
    if opt.verbose {
        println!("Verbose mode enabled");
    }
    for file in &opt.files {
        println!("Processing file: {:?}", file);
    }
    println!("Result will be written to: {:?}", opt.result_file);
}
