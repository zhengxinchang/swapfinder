use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser, Clone, Debug)]
#[command(name = "swapfinder")]
#[command(about = "Fast identify sample swaps")]
#[command(author = "Xinchang Zheng", version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Clone, Debug)]
pub enum Commands {
    #[allow(non_camel_case_types)]
    #[command(about = "calculatge SNP barcode for a given sample")]
    barcode(BarcodeArgs),
    #[allow(non_camel_case_types)]
    #[command(about = "Compare several SNP barcode")]
    compare(CompareArgs),
}

#[derive(Parser, Debug, Clone)]
pub struct BarcodeArgs {
    /// Input file in BAM/CRAM format
    #[arg(short, long)]
    pub bam: PathBuf,

    /// Reference file for CRAMs
    #[arg(short, long)]
    pub reference: Option<PathBuf>,

    /// Output file
    #[arg(short, long)]
    pub output: PathBuf,

    /// Barcode file
    #[arg(short = 'B', long)]
    pub barcode: PathBuf,
}

#[derive(Parser, Debug, Clone)]
pub struct CompareArgs {
    /// Input barcodes file. can be multiple
    #[arg(short = 'b', long)]
    pub barcode: Vec<PathBuf>,

    /// Input manfiest file. each line is a barcode file
    #[arg(short = 'b', long)]
    pub barcode_list: PathBuf,

    /// Name of the output signal file
    #[arg(short, long)]
    pub output: PathBuf,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli() {
        let args = Cli::parse_from(&[
            "swapfinder",
            "barcode",
            "-B",
            "barcode.txt",
            "-o",
            "output.txt",
            "-b",
            "input.bam",
        ]);
        match args.command {
            Commands::barcode(args) => {
                assert_eq!(args.bam, PathBuf::from("input.bam"));
                assert_eq!(args.output, PathBuf::from("output.txt"));
                assert_eq!(args.barcode, PathBuf::from("barcode.txt"));
            }
            _ => panic!("Wrong subcommand"),
        }
    }

    #[test]
    fn test_compare() {
        let args = Cli::parse_from(&[
            "swapfinder",
            "compare",
            "-B",
            "barcode1.txt",
            "-B",
            "barcode2.txt",
            "-o",
            "output.txt",
        ]);
        match args.command {
            Commands::compare(args) => {
                assert_eq!(args.output, PathBuf::from("output.txt"));
                assert_eq!(args.barcode.len(), 2);
                assert_eq!(args.barcode[0], PathBuf::from("barcode1.txt"));
                assert_eq!(args.barcode[1], PathBuf::from("barcode2.txt"));
            }
            _ => panic!("Wrong subcommand"),
        }
    }
}
