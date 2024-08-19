use std::{
    env,
    ffi::OsString,
    fs::{self},
    io,
    path::{Path, PathBuf},
};

struct Options {
    case: bool,
    help: bool,
}

impl Options {
    fn new(args: &Vec<String>) -> Self {
        let mut opt = Options::default();

        for arg in args {
            match arg.as_ref() {
                "--case" => opt.case = true,
                "--help" => opt.help = true,
                _ => {}
            }
        }

        return opt;
    }
}

impl Default for Options {
    fn default() -> Self {
        Options {
            case: false,
            help: false,
        }
    }
}

fn main() -> io::Result<()> {
    let mut args: Vec<String> = env::args().collect();

    let mut path_param: String = Default::default();
    let mut search: String = Default::default();
    let options: Options = Options::new(&args);

    if options.help {
        println!("Help");

        return Ok(());
    }

    clear_option_args(&mut args);

    validate_arguments(args, &mut path_param, &mut search);

    if path_param == "" || search == "" {
        return Ok(());
    };

    let path = Path::new(&path_param);

    let mut matches: Vec<String> = Vec::new();

    iterate_dir(path, search.to_string(), &mut matches, options.case)?;

    if matches.len() == 0 {
        println!("No matches found");
    } else {
        for file_match in matches {
            println!("{}", file_match);
        }
    }

    Ok(())
}

fn iterate_dir(
    path: &Path,
    search: String,
    matches: &mut Vec<String>,
    case: bool,
) -> io::Result<()> {
    for entry in fs::read_dir(path)? {
        match entry {
            Ok(entry) => {
                let new_path = entry.path();
                match new_path.file_name() {
                    Some(path_string) => {
                        if path.starts_with(".") {
                            continue;
                        }
                        if path_string == "node_modules" {
                            continue;
                        }
                    }
                    None => continue,
                }
                if new_path.is_dir() && !is_read_only(&new_path) {
                    iterate_dir(&new_path, search.to_string(), matches, case)?;
                } else if entry.file_name() == OsString::from(&search) && case {
                    match entry.file_name().into_string() {
                        Ok(p) => matches.push(p),
                        Err(_) => {}
                    }
                } else {
                    let file_name = entry.file_name();
                    let lower_case = entry.file_name().to_ascii_lowercase();
                    match (
                        case,
                        file_name == OsString::from(&search),
                        lower_case == OsString::from(&search).to_ascii_lowercase(),
                    ) {
                        (true, true, false) => match file_name.into_string() {
                            Ok(s) => matches.push(s),
                            Err(_) => {}
                        },
                        (false, true, true) | (false, false, true) => {
                            match file_name.into_string() {
                                Ok(s) => matches.push(s),
                                Err(_) => {}
                            }
                        }
                        _ => {}
                    }
                }
            }
            Err(err) => println!("BREEEH {}", err),
        }
    }

    Ok(())
}

fn is_read_only(path_buf: &PathBuf) -> bool {
    match path_buf.metadata() {
        Ok(metadata) => metadata.permissions().readonly(),
        Err(..) => return true,
    }
}

fn validate_arguments(args: Vec<String>, path_param: &mut String, search: &mut String) {
    match args.len() {
        1 => println!("No arguments given... whelp"),
        2 => match env::current_dir() {
            Ok(current_dir) => match current_dir.to_str() {
                Some(path) => {
                    *path_param = path.to_string();
                    *search = String::from(&args[1]);
                }
                _ => {}
            },
            Err(err) => println!("{}", err),
        },
        3 => {
            *path_param = String::from(&args[1]);
            *search = String::from(&args[2]);
        }
        _ => println!("Too many arguments!"),
    }
}

fn clear_option_args(args: &mut Vec<String>) {
    args.retain(|s| !s.starts_with("--"))
}
