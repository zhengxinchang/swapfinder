//! compare all combinations of sample barcodes.
//!

use std::{io::BufRead, path::PathBuf};

use crate::cli::CompareArgs;

/// parse a barcode file and return (sample_name,Vec<barcode>)
/// the first line of the file is the sample name
/// the rest of the lines without # are barcodes
pub fn parse_barcode_file(file: &str) -> (String, Vec<String>) {
    let mut barcodes = Vec::new();
    let file = std::fs::File::open(file).expect("file not found");
    let reader = std::io::BufReader::new(file);
    let mut sample = String::new();
    for (idx, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        if idx == 0 {
            sample = line.trim_end().to_string().strip_prefix('#').unwrap().to_string();
        } else {
            if !line.starts_with("#") {
                let fields: Vec<&str> = line.trim_end().split('\t').collect();
                barcodes.push(fields[4].to_owned());
            }
        }
    }
    (sample, barcodes)
}

/// implement the Jaccard similarity between two barcodes
/// Jaccard similarity = |A and B| / |A or B|
pub fn calc_similarity(barcode1: Vec<String>, barcode2: Vec<String>) -> f64 {
    let mut intersection = 0;
    for i in 0..barcode1.len() {
        if barcode2.contains(&barcode1[i]) {
            intersection += 1;
        }
    }
    let  union = barcode1.len() + barcode2.len() - intersection;
    intersection as f64 / union as f64
}

pub fn compare_main(barcode_file_list: Vec<PathBuf>, args: CompareArgs) {


    let mut barcode_vec = Vec::new();

    for barcode_file in barcode_file_list {
        let (sample, barcodes) = parse_barcode_file(barcode_file.to_str().unwrap());
        barcode_vec.push((sample, barcodes));
    }

    // make output file handler, if not provided use stdout
    let mut output: Box<dyn std::io::Write> = match args.output {
        Some(output_file) => Box::new(std::io::BufWriter::new(std::fs::File::create(output_file).unwrap())),
        None => Box::new(std::io::BufWriter::new(std::io::stdout())),
    };
    // let mut output = std::io::BufWriter::new(std::io::stdout());
    output.write_all(b"Sample1\tSample2\tSimilarity\n").unwrap();
    for i in 0..barcode_vec.len() {
        for j in i+1..barcode_vec.len() {
            let similarity = calc_similarity(barcode_vec[i].1.clone(), barcode_vec[j].1.clone());
            // print the similarity as float 
            output.write_all(format!("{}\t{}\t{:.2}", barcode_vec[i].0, barcode_vec[j].0, similarity*100.0).as_bytes()).unwrap();
        }
    }

}
