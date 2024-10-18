mod cli;

use clap::Parser;
use cli::Args;

fn main() {
    let args: Args = Args::parse();
    let n_threads = args.n_threads;
    let batch_size = args.batch_size;
    println!("n threads: {}; batch size: {}", n_threads, batch_size)
}
