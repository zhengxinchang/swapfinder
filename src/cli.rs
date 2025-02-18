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
    #[arg(short='i', long)]
    pub bam: PathBuf,

    /// Reference file for CRAMs
    #[arg(short, long)]
    pub reference: Option<PathBuf>,

    /// Output file
    #[arg(short, long)]
    pub output: PathBuf,

    /// Barcode file
    #[arg(short = 'b', long)]
    pub barcode: PathBuf,
}

#[derive(Parser, Debug, Clone)]
pub struct CompareArgs {
    /// Input barcodes file. can be multiple
    #[arg(short = 'i', long)]
    pub barcode: Vec<PathBuf>,

    /// Input manfiest file. each line is a barcode file
    #[arg(short = 'I', long)]
    pub barcode_list: PathBuf,

    /// Name of the output signal file
    #[arg(short, long)]
    pub output: PathBuf,
}
