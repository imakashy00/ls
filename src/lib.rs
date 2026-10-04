#[cfg(unix)]
use std::os::unix::fs::MetadataExt;
use std::{ error::Error, fs::Metadata, path::PathBuf };

#[derive(Debug, PartialEq)]
pub struct DirStr {
    pub name: String,
    pub contents: Vec<PathBuf>,
}
#[derive(Debug, Default, PartialEq, Eq)]
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
        if args.len() > 1 {
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
            } else if sub_command == "-" {
                return Err("No such command found");
            } else {
                config.paths.push(PathBuf::from(sub_command));
            }

            for arg in &real_args[1..] {
                config.paths.push(PathBuf::from(arg));
            }
        }
        if config.paths.is_empty() {
            config.paths.push(PathBuf::from("."));
        }
        Ok(config)
    }
}
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut directory_contents = get_contents(&config.paths)?;
    let _ = filter_contents(&config, &mut directory_contents);
    let _ = display(&config, &directory_contents);
    Ok(())
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
pub fn filter_contents(
    config: &Config,
    dir_contents: &mut Vec<DirStr>
) -> Result<(), Box<dyn Error>> {
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
    }
    Ok(())
}

pub fn display(config: &Config, dir_contents: &[DirStr]) -> Result<(), Box<dyn Error>> {
    for dir in dir_contents {
        if !config.directory {
            println!("--{:?}--", dir.name);
        }
        for path in &dir.contents {
            let mut display_name = path
                .file_name()
                .map(|str| str.to_string_lossy().into_owned())
                .unwrap_or_else(|| dir.name.clone());

            if path.is_dir() {
                display_name.push('/');
            }
            if config.long {
                let metadata = path.metadata()?;
                let file_type_char = if path.is_dir() { 'd' } else { '-' };
                let permissions = parse_permissions(&metadata);
                let uid = if cfg!(unix) {
                    #[cfg(unix)]
                    {
                        metadata.uid()
                    }
                    #[cfg(not(unix))]
                    {
                        0
                    }
                } else {
                    0
                };
                let gid = if cfg!(unix) {
                    #[cfg(unix)]
                    {
                        metadata.gid()
                    }
                    #[cfg(not(unix))]
                    {
                        0
                    }
                } else {
                    0
                };
                let size = metadata.len();
                let modified_time = metadata
                    .modified()
                    .map(|mod_t| {
                        let datetime: chrono::DateTime<chrono::Local> = mod_t.into();
                        datetime.format("%b %d %H:%M").to_string()
                    })
                    .unwrap_or_else(|_| String::from("Unkown Time"));
                println!(
                    "{}{} 1 {:>5} {:>5} {:>8} {} {}",
                    file_type_char,
                    permissions,
                    uid,
                    gid,
                    size,
                    modified_time,
                    display_name
                );
            } else {
                print!("{}  ", display_name);
            }
            if !config.long {
                println!();
            }
        }
    }
    Ok(())
}

fn parse_permissions(metadata: &Metadata) -> String {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;

        let mode = metadata.permissions().mode();
        let mut p = String::with_capacity(9);
        // these are user permissions
        p.push(if (mode & 0o400) != 0 { 'r' } else { '-' });
        p.push(if (mode & 0o200) != 0 { 'w' } else { '-' });
        p.push(if (mode & 0o100) != 0 { 'x' } else { '-' });

        // these are group permissions
        p.push(if (mode & 0o040) != 0 { 'r' } else { '-' });
        p.push(if (mode & 0o020) != 0 { 'w' } else { '-' });
        p.push(if (mode & 0o010) != 0 { 'x' } else { '-' });

        // these are other permissions
        p.push(if (mode & 0o004) != 0 { 'r' } else { '-' });
        p.push(if (mode & 0o002) != 0 { 'w' } else { '-' });
        p.push(if (mode & 0o001) != 0 { 'x' } else { '-' });

        p
    }
    #[cfg(not(unix))] // windows OS
    {
        (if metadata.permissions().readonly() { "r--r--r--" } else { "rw-rw-rw-" }).to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_config_build() {
        let args: Vec<String> = vec![
            "target/debug/ls".into(),
            "-l".into(),
            "src".into(),
            "target".into()
        ];

        let config = Config::build(&args).unwrap();

        assert!(config.long);
        assert!(!config.all);
        assert!(!config.directory);
        assert!(!config.size);
        assert!(!config.time);

        assert_eq!(
            config.paths
                .iter()
                .map(|path| path.to_str().unwrap())
                .collect::<Vec<&str>>(),
            vec!["src", "target"]
        )
    }
    #[test]
    fn test_config_build_panic() {
        let args: Vec<String> = vec![
            "target/debug/ls".into(),
            "-".into(),
            "src".into(),
            "target".into()
        ];

        assert_eq!(Config::build(&args), Err("No such command found"))
    }
    #[test]
    fn test_get_contents() {
        let args: Vec<String> = vec!["target/debug/ls".into(), "src".into(), "target".into()];
        let config = Config::build(&args).unwrap();

        let contents = get_contents(&config.paths).unwrap();
        assert!(contents.len() > 0);
        let expected = vec![
            DirStr {
                name: "src".to_string(),
                contents: vec![PathBuf::from("src/lib.rs"), PathBuf::from("src/main.rs")],
            },
            DirStr {
                name: "target".to_string(),
                contents: vec![
                    PathBuf::from("target/.rustc_info.json"),
                    PathBuf::from("target/CACHEDIR.TAG"),
                    PathBuf::from("target/flycheck0"),
                    PathBuf::from("target/debug")
                ],
            }
        ];
        assert_eq!(contents, expected)
    }
    #[test]
    fn test_filter_contents_single_option() {
        let args: Vec<String> = vec![
            "target/debug/ls".into(),
            "-t".into(),
            "src".into(),
            "target".into()
        ];
        let config = Config::build(&args).unwrap();
        let mut contents = get_contents(&config.paths).unwrap();
        let _ = filter_contents(&config, &mut contents).unwrap();
        let expected = vec![
            DirStr {
                name: "src".to_string(),
                contents: vec![PathBuf::from("src/lib.rs"), PathBuf::from("src/main.rs")],
            },
            DirStr {
                name: "target".to_string(),
                contents: vec![
                    PathBuf::from("target/debug"),
                    PathBuf::from("target/flycheck0"),
                    PathBuf::from("target/CACHEDIR.TAG")
                ],
            }
        ];
        assert_eq!(contents, expected)
    }
    #[test]
    fn test_filter_contents_multiple_options() {
        let args: Vec<String> = vec![
            "target/debug/ls".into(),
            "-at".into(),
            "src".into(),
            "target".into()
        ];
        let config = Config::build(&args).unwrap();
        let mut contents = get_contents(&config.paths).unwrap();
        let _ = filter_contents(&config, &mut contents).unwrap();
        let expected = vec![
            DirStr {
                name: "src".to_string(),
                contents: vec![PathBuf::from("src/lib.rs"), PathBuf::from("src/main.rs")],
            },
            DirStr {
                name: "target".to_string(),
                contents: vec![
                    PathBuf::from("target/.rustc_info.json"),
                    PathBuf::from("target/debug"),
                    PathBuf::from("target/flycheck0"),
                    PathBuf::from("target/CACHEDIR.TAG")
                ],
            }
        ];
        assert_eq!(contents, expected)
    }
}
