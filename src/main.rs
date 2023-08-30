use std::{
    env,
    ffi::OsString,
    fs::{self},
    io,
    path::{Path, PathBuf},
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
            },
            Err(err) => println!("{}", err)
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
