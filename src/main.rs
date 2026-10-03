use std::{ env, error::Error, process };

use ls::{ Config, display, filter_contents, get_contents };

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut directory_contents = get_contents(&config.paths)?;
    let _ = filter_contents(&config, &mut directory_contents);
    let _ = display(&config, &directory_contents);
    Ok(())
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("{:?}", err);
        process::exit(1)
    });
    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
