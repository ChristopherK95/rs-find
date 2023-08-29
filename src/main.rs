use std::{
    env,
    fs::{self},
    io,
    path::Path,
};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let path_param = &args[1];
    let search = &args[2];

    let path = Path::new(path_param);

    iterate_dir(path, search.to_string())?;

    Ok(())
}

fn iterate_dir(path: &Path, search: String) -> io::Result<()> {
    for entry in fs::read_dir(path)? {
        if let Ok(entry) = entry {
            let new_path = entry.path();
            match new_path.file_name() {
                Some(path_string) => {
                    if path_string.to_str().unwrap().chars().nth(0).unwrap() == '.' {
                        continue;
                    }
                    if path_string == "node_modules" {
                        continue;
                    }
                }
                None => {}
            }
            if new_path.is_dir() {
                let read_only = new_path.metadata().unwrap().permissions().readonly();
                if read_only {
                    continue;
                };
                iterate_dir(&new_path, search.to_string())?;
            } else {
                match entry.file_name().to_str() {
                    Some(e) => {
                        if e == search {
                            match new_path.to_str() {
                                Some(path) => println!("{}", path),
                                None => {}
                            };
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    Ok(())
}
