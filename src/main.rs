//use std::env;
//use std::process;

//use minigrep::Config;

fn main() {
    println!("Rust book chapter 12 minigrep");

    let args: Vec<String> = std::env::args().collect();
    //println!("{:?}", args);
    let config = minigrep::Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });

    //println!("Searching for {}", config.query);
    //println!("In file {}", config.filename);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }
}
