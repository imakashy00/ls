use std::{ env, error::Error, process };

use ls::{ Config, filter_contents, get_contents };

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Get all the contents of the dir
    let mut directory_contents = get_contents(&config.paths)?;
    // println!("{:?}", get_contents(config));
    // send these contents for filteration
    filter_contents(config, &mut directory_contents);
    Ok(())
}
fn main() {
    // let args = Config::default();
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("No such command:{:?}", err);
        process::exit(1)
    });
    println!("args: {:?}", config);

    println!("Hello, world!:{:?},", &args[1..]);
    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
