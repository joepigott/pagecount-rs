use clap::Parser;

mod cli;
mod count;
mod color;

fn main() {
    let cli = cli::CLI::parse();

    let mut total = 0;

    for path in cli.path {
        let pages = match count::count(&path, &cli.ignore, cli.verbose) {
            Ok(pages) => pages,
            Err(e) => {
                eprintln!("{}: {}", color::Red("Error".to_string()), e.to_string());
                return;
            }
        };
        total += pages;
        println!("{path} contains {pages} pages\n");
    }

    println!("Total pages: {total}");
}
