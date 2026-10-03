use std::{ error::Error, path::PathBuf };

#[derive(Debug)]
pub struct DirStr {
    pub name: String,
    pub contents: Vec<PathBuf>,
}
#[derive(Debug, Default)]
pub struct Config {
    pub all: bool,
    pub long: bool,
    pub time: bool,
    pub directory: bool,
    pub size: bool,
    pub paths: Vec<PathBuf>, // ls can hanlde more than one dir at a time
}
impl Config {
    pub fn build(args: &Vec<String>) -> Result<Config, &'static str> {
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

pub fn get_contents(config_path: &Vec<PathBuf>) -> Result<Vec<DirStr>, Box<dyn Error>> {
    // loop over all the all the items in config and chain them all
    let mut contents = Vec::new();
    let paths = config_path;
    for path in paths {
        if path.is_dir() {
            let mut dir_contents: Vec<PathBuf> = Vec::new();
            let things = path.read_dir()?;
            for entry in things {
                let entry = entry?; // Unwrap the individual DirEntry
                dir_contents.push(entry.path());
            }
            contents.push(DirStr {
                name: path.to_string_lossy().into_owned(),
                contents: dir_contents,
            });
        } else {
            return Err("No such dir ".into());
        }
    }

    Ok(contents)
}
pub fn filter_contents(config: Config, dir_contents: &mut Vec<DirStr>) {
    // println!("{:?}", contents);
    for dir in dir_contents {
        // -a filter
        if !config.all {
            dir.contents.retain(|path| {
                if let Some(os_path) = path.file_name() {
                    let name_str = os_path.to_string_lossy();
                    !name_str.starts_with('.')
                } else {
                    true
                }
            });
        }
        // -t filter
        if config.time {
            dir.contents.sort_by(|a, b| {
                let time_a = a
                    .metadata()
                    .and_then(|m| m.modified())
                    .unwrap_or(std::time::SystemTime::UNIX_EPOCH);

                let time_b = b
                    .metadata()
                    .and_then(|m| m.modified())
                    .unwrap_or(std::time::SystemTime::UNIX_EPOCH);
                time_b.cmp(&time_a)
            });
        } else if config.size {
            // -s filter
            dir.contents.sort_by(|a, b| {
                let size_a = a
                    .metadata()
                    .map(|m| m.len())
                    .unwrap_or(0);

                let size_b = b
                    .metadata()
                    .map(|m| m.len())
                    .unwrap_or(0);
                size_b.cmp(&size_a)
            });
        } else {
            // alphabetically by filename
            dir.contents.sort_by(|a, b| {
                let name_a = a.file_name().unwrap_or_default();
                let name_b = b.file_name().unwrap_or_default();
                name_a.cmp(name_b)
            });
        }
        // -l filter
        // TODO
        // -d filter
        // TODO
    }
}
