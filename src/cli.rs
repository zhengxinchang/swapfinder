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
    #[arg(short = 'i', long)]
    pub bam: PathBuf,

    /// Reference file for CRAMs
    #[arg(short, long)]
    pub reference: Option<PathBuf>,

    /// Output file
    #[arg(short, long)]
    pub output: Option<PathBuf>,

    /// Sample name(optional)
    /// if not provided, the sample name will be the file name
    #[arg(short, long)]
    pub sample_name: Option<String>,

    /// Minimal read depth to call a barcode
    #[arg(short, long, default_value_t = 3)]
    pub min_depth: usize,

    /// Minimal read depth to call a barcode
    #[arg(short = 'l', long, default_value_t = 2.0)]
    pub min_likehood_ratio: f64,

    /// Barcode file
    #[arg(short = 'b', long)]
    pub barcode: PathBuf,
}

#[derive(Parser, Debug, Clone)]
pub struct CompareArgs {
    /// Input barcodes file. can be multiple
    #[arg(index = 1)]
    pub barcode: Option<Vec<PathBuf>>,

    /// Input manfiest file. each line is a barcode file
    #[arg(short = 'I', long)]
    pub barcode_list: Option<PathBuf>,

    /// Reference barcode files
    #[arg(short = 'R', long,long_help = "Reference barcode files for comparison
If reference barcode files are provided, swapfinder will operate in query mode, 
comparing each input file (specified via -I or positional arguments) individually 
against the reference files.
If no reference is provided, swapfinder will operate in compare mode, comparing 
all input files (from -I or positional arguments) against each other.")]
    pub ref_bcs: Option<PathBuf>, 

    /// Name of the output signal file
    #[arg(short, long)]
    pub output: Option<PathBuf>,
}
