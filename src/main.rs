use std::{
    env,
    ffi::OsString,
    fs::{self},
    io,
    path::{Path, PathBuf},
};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let mut path_param: String = Default::default();
    let mut search: String = Default::default();

    validate_arguments(args, &mut path_param, &mut search);

    if path_param == "" || search == "" {
        return Ok(());
    };

    let path = Path::new(&path_param);

    iterate_dir(path, search.to_string())?;

    Ok(())
}

fn iterate_dir(path: &Path, search: String) -> io::Result<()> {
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
                    iterate_dir(&new_path, search.to_string())?;
                } else if entry.file_name() == OsString::from(&search) {
                    println!("{:?}", new_path);
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
