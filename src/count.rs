use std::path::Path;

pub fn count(path: &String, ignore: &Vec<String>, verbose: bool) -> Result<usize, String> {
    let path = Path::new(&path);
    println!("Recursing through {}...", path.display());

    return count_r(path, ignore, verbose, 0);
}

fn count_r(path: &Path, ignore: &Vec<String>, verbose: bool, level: u8) -> Result<usize, String> {
    if verbose {
        print_lines(level);

        println!("{}", path.display());
    }

    let mut total = 0;

    let directory = std::fs::read_dir(path).map_err(|e| e.to_string())?;

    for item in directory {
        let item = item.unwrap();
        let item_path = item.path();

        if item_path.is_dir() && !ignore.contains(&item_path.file_name().unwrap().to_str().unwrap().to_string()) {
            total += count_r(item_path.as_path(), ignore, verbose, level + 1)?;
        }

        match item_path.extension() {
            Some(extension) => {
                if extension == "pdf" {
                    total += count_pages(item_path.as_path(), verbose, level);
                } else {
                    if verbose {
                        print_lines(level + 1);
                        let name = item_path.file_name().unwrap().to_str().unwrap();
                        println!("{}", crate::color::Gray(format!("{name}")));
                    }
                }
            }
            None => continue
        }
    }

    return Ok(total);
}

fn count_pages(path: &Path, verbose: bool, level: u8) -> usize {
    let pdf = match pdf::file::FileOptions::cached().open(path) {
        Ok(pdf) => {
            pdf
        },
        Err(e) => {
            if verbose {
                print_lines(level + 1);
                let name = path.file_name().unwrap().to_str().unwrap();
                println!("{}", crate::color::Red(format!("{name} - {}", e)));
            }
            return 0;
        }
    };

    let pages = pdf.num_pages();

    if verbose {
        print_lines(level + 1);
        let name = path.file_name().unwrap().to_str().unwrap();
        println!("{}", crate::color::Blue(format!("{name} - {pages} pages")));
    }

    return pages as usize;
}

fn print_lines(level: u8) {
    if level != 0 {
        print!("|   ");
        for i in 4..(level * 4) {
            if (i) % 4 == 0 {
                print!("|");
            } else {
                print!(" ");
            }
        }
    }
}
