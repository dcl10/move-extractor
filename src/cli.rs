use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, help = "The number of threads to use")]
    pub n_threads: usize,
    #[arg(
        short,
        long,
        default_value_t = 100,
        help = "The number of records to parse per thread"
    )]
    pub batch_size: usize,
}
