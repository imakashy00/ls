use std::{ env, process };

use ls::{ Config, run };

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("{:?}", err);
        process::exit(1)
    });
    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
