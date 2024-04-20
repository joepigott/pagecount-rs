use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct CLI {
    /// Paths to directories to search
    #[arg(short, long, num_args = 1..)]
    pub path: Vec<String>,
    
    /// Paths to ignore during recursion
    #[arg(short, long, num_args = 1..)]
    pub ignore: Vec<String>,

    /// Print additional debug information
    #[arg(short, long, global = true)]
    pub verbose: bool
}
