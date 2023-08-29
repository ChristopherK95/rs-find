use std::{
    fs::{self},
    io,
    time::Instant,
};

fn main() -> io::Result<()> {
    let path = "/home/chrkar/";
    let search = String::from("types.ts");

    let matches = &mut Vec::new();

    let start = Instant::now();

    iterate_dir(path, &search, matches)?;
    println!("{:?}", matches.iter().map(|x| x));
    println!("{}", matches.len());
    let elapsed = start.elapsed();
    println!("{:?}", elapsed);

    Ok(())
}

fn iterate_dir(path: &str, search: &String, matches: &mut Vec<String>) -> io::Result<()> {
    println!("{path}");
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let new_path = entry.path();
        if new_path.is_dir() {
            if let Some(p) = new_path.to_str() {
                iterate_dir(p, search, matches)?;
            }
        } else {
            match entry.file_name().to_str() {
                Some(e) => {
                    if e == search {
                        let file_match = path.to_string() + "/" + search;
                        matches.push(file_match);
                    }
                }
                _ => {}
            }
        }
    }

    Ok(())
}
