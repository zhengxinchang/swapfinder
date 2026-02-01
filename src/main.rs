mod profile;
mod barcode_calc;
mod cli;
mod sawp;
mod contamination;
mod genotype;
mod likelihood;
mod read_base;
mod utils;
use std::{
    io::{BufRead, Read},
    path::{self, Path, PathBuf},
};

use barcode_calc::calculate_barcode;
use clap::Parser;
use sawp::compare_main;

fn main() {
    let cli = cli::Cli::parse();
    match cli.command {
        cli::Commands::profile(args) => {
            calculate_barcode(args);
        }
        cli::Commands::compare(args) => {
            // construct the barcode file list

            let mut barcode_file_list: Vec<PathBuf> = Vec::new();
            if let Some(barcode_files) = args.barcode.clone() {
                barcode_file_list.extend(barcode_files.clone().into_iter());
            }

            let mut ref_barcode_file_list: Vec<PathBuf> = Vec::new();

            if let Some(ref_barcode_files) = args.ref_bcs.clone() {
                let mut f = std::fs::File::open(ref_barcode_files)
                    .expect("Failed to open reference barcode file.");

                let mut ref_bc_files = String::new();
                f.read_to_string(&mut ref_bc_files)
                    .expect("Failed to read reference barcode file.");

                ref_bc_files.split('\n').for_each(|f| {
                    let path = Path::new(f.trim());
                    if path.exists() {
                        ref_barcode_file_list.push(path.to_path_buf());
                    } else {
                        eprintln!(
                            "reference barcode file:{} not exists in the manifest file",
                            path.to_str().unwrap()
                        );
                        std::process::exit(1);
                    }
                });
            }

            if let Some(barcode_manifest_path) =
                args.barcode_list.as_ref().map(|p| p.to_str().unwrap())
            {
                let manifest_file = std::fs::File::open(barcode_manifest_path).unwrap();
                let reader = std::io::BufReader::new(manifest_file);
                for line in reader.lines() {
                    let line = line.unwrap();
                    let f = PathBuf::from(line.trim_end());
                    if f.exists() {
                        barcode_file_list.push(f);
                    } else {
                        eprintln!(
                            "barcode file:{} not exists in the manifest file",
                            f.to_str().unwrap()
                        );
                        std::process::exit(1);
                    }
                }
            }

            compare_main(barcode_file_list, ref_barcode_file_list, args);
        }
        cli::Commands::estcon(cont_est_args) => {
            contamination::estimation(cont_est_args);
        }
    }
}
