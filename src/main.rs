mod bam;
mod barcode;
mod cli;
use bam::calculate_barcode;
use clap::Parser;
fn main() {
    let cli = cli::Cli::parse();
    match cli.command {
        cli::Commands::barcode(args) => {
            calculate_barcode(args);
        }
        cli::Commands::compare(args) => {
            println!("{:?}", args);
        }
    }
}
