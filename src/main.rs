mod bam;
mod barcode;
mod cli;
mod compare;
mod genotype;
mod likelihood;
use std::{io::BufRead, path::PathBuf};

use bam::calculate_barcode;
use clap::Parser;
use compare::compare_main;

fn main() {
    let cli = cli::Cli::parse();
    match cli.command {
        cli::Commands::barcode(args) => {
            calculate_barcode(args);
        }
        cli::Commands::compare(args) => {
            // construct the barcode file list

            let mut barcode_file_list: Vec<PathBuf> = Vec::new();
            if let Some(barcode_files) = args.barcode.clone() {
                barcode_file_list.extend(barcode_files.clone().into_iter());
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
                    }
                }
            }

            compare_main(barcode_file_list, args);
        }
    }
}
