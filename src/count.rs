use std::path::Path;

pub fn count(path: &String, ignore: &Vec<String>, verbose: bool) -> usize {
    let path = Path::new(&path);
    println!("Recursing through {}...", path.display());

    return count_r(path, ignore, verbose, 0);
}

fn count_r(path: &Path, ignore: &Vec<String>, verbose: bool, level: u8) -> usize {
    if verbose {
        print_lines(level);

        if level == 0 {}
        println!("{}", path.display());
    }

    let mut total = 0;

    for item in std::fs::read_dir(path).unwrap() {
        let item = item.unwrap();
        let item_path = item.path();

        if item_path.is_dir() && !ignore.contains(&item_path.file_name().unwrap().to_str().unwrap().to_string()) {
            total += count_r(item_path.as_path(), ignore, verbose, level + 1);
        }

        match item_path.extension() {
            Some(extension) => {
                if extension == "pdf" {
                    total += count_pages(item_path.as_path(), verbose, level);
                    if verbose {
                        print_lines(level);
                        let name = item_path.file_name().unwrap().to_str().unwrap();
                        println!("{}", crate::color::Blue(format!("{name}")));
                    }
                } else {
                    if verbose {
                        print_lines(level);
                        let name = item_path.file_name().unwrap().to_str().unwrap();
                        println!("{}", crate::color::Gray(format!("{name}")));
                    }
                }
            }
            None => continue
        }
    }

    return total;
}

fn count_pages(path: &Path, verbose: bool, level: u8) -> usize {
    let pdf = match pdf::file::FileOptions::cached().open(path) {
        Ok(pdf) => pdf,
        Err(_) => {
            if verbose {
                print_lines(level);
                println!("{}", crate::color::Red(format!("Error opening {}", path.display())));
            }
            return 0;
        }
    };

    return pdf.num_pages() as usize;
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
