use std::{ env, error::Error, path::PathBuf, process };

#[derive(Debug, Default)]
struct Config {
    all: bool,
    long: bool,
    time: bool,
    directory: bool,
    size: bool,
    extension: bool,
    paths: Vec<PathBuf>, // ls can hanlde more than one dir at a time
}

impl Config {
    fn build(args: &Vec<String>) -> Result<Config, &'static str> {
        let mut config = Self::default();

        let real_args = &args[1..];
        let sub_command = &real_args[0];
        if sub_command.starts_with('-') && sub_command != "-" {
            for ch in sub_command.chars().skip(1) {
                match ch {
                    'a' => {
                        config.all = true;
                    }
                    'l' => {
                        config.long = true;
                    }
                    't' => {
                        config.time = true;
                    }
                    'd' => {
                        config.directory = true;
                    }
                    'S' => {
                        config.size = true;
                    }
                    'x' => {
                        config.extension = true;
                    }
                    _ => {
                        return Err("No such command found");
                    }
                }
            }
        } else {
            config.paths.push(PathBuf::from(sub_command));
        }

        for arg in &real_args[1..] {
            config.paths.push(PathBuf::from(arg));
        }
        if config.paths.is_empty() {
            config.paths.push(PathBuf::from("."));
        }
        Ok(config)
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
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
