//! compare all combinations of sample barcodes.

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
            sample = line
                .trim_end()
                .to_string()
                .strip_prefix('#')
                .unwrap()
                .to_string();
        } else {
            if !line.starts_with("#") {
                let fields: Vec<&str> = line.trim_end().split('\t').collect();
                barcodes.push(fields[2].to_owned());
            }
        }
    }
    (sample, barcodes)
}

pub fn calc_similarity(barcode1: Vec<String>, barcode2: Vec<String>) -> (f64, u32) {
    if barcode1.len() != barcode2.len() {
        panic!("Must compare the same number of barcodes");
    }

    // find the NN position in the barcode1 and barcode2

    let barcode1_n_pos: Vec<i32> = barcode1
        .iter()
        .map(|barcode| if barcode == "NN" { 1 } else { 0 })
        .collect::<Vec<i32>>();

    let barcode2_n_pos: Vec<i32> = barcode2
        .iter()
        .map(|barcode| if barcode == "NN" { 1 } else { 0 })
        .collect();

    // merge the two barcode_n_pos and deduplicate

    let barcode_need_remove: Vec<i32> = barcode1_n_pos
        .iter()
        .zip(barcode2_n_pos.iter())
        .map(|(a, b)| if (a + b) > 0 { 1 } else { 0 })
        .collect();

    // remove the NN position from barcode1 and barcode2
    let barcode1: Vec<String> = barcode1
        .iter()
        .zip(barcode_need_remove.iter())
        .filter(|(_, &b)| b == 0)
        .map(|(a, _)| a.to_owned())
        .collect();
    let barcode2: Vec<String> = barcode2
        .iter()
        .zip(barcode_need_remove.iter())
        .filter(|(_, &b)| b == 0)
        .map(|(a, _)| a.to_owned())
        .collect();

    let intersection: usize = barcode1
        .iter()
        .zip(barcode2.iter())
        .filter(|&(a, b)| a == b)
        .count();

    let union: usize = barcode1.len();

    (intersection as f64 / union as f64, barcode1.len() as u32)
}

pub fn compare_main(
    barcode_file_list: Vec<PathBuf>,
    ref_barcode_list: Vec<PathBuf>,
    args: CompareArgs,
) {
    let mut barcode_vec = Vec::new();

    for barcode_file in barcode_file_list {
        let (sample, barcodes) = parse_barcode_file(barcode_file.to_str().unwrap());
        barcode_vec.push((sample, barcodes));
    }

    // make output file handler, if not provided use stdout
    let mut output: Box<dyn std::io::Write> = match args.output {
        Some(output_file) => Box::new(std::io::BufWriter::new(
            std::fs::File::create(output_file).unwrap(),
        )),
        None => Box::new(std::io::BufWriter::new(std::io::stdout())),
    };
    // let mut output = std::io::BufWriter::new(std::io::stdout());

    if ref_barcode_list.len() == 0 {
        output.write_all(b"#Full-comparison mode\n").unwrap();

        output
            .write_all(b"Sample1\tSample2\tSimilarity_score\tNum_barcode\n")
            .unwrap();

        for i in 0..barcode_vec.len() {
            for j in i + 1..barcode_vec.len() {
                let (similarity, nbc) =
                    calc_similarity(barcode_vec[i].1.clone(), barcode_vec[j].1.clone());
                // print the similarity as float
                output
                    .write_all(
                        format!(
                            "{}\t{}\t{:.2}\t{}\n",
                            barcode_vec[i].0,
                            barcode_vec[j].0,
                            similarity * 100.0,
                            nbc
                        )
                        .as_bytes(),
                    )
                    .unwrap();
            }
        }
    } else {
        output.write_all(b"#Reference-comparison mode\n").unwrap();

        output
            .write_all(b"Sample1\tSample2\tSimilarity_score\tNum_barcode\n")
            .unwrap();

        let mut ref_barcode_vec = Vec::new();

        for ref_barcode_file in ref_barcode_list {
            let (sample, barcodes) = parse_barcode_file(ref_barcode_file.to_str().unwrap());
            ref_barcode_vec.push((sample, barcodes));
        }

        for i in 0..barcode_vec.len() {
            for j in 0..ref_barcode_vec.len() {
                let (similarity, nbc) =
                    calc_similarity(barcode_vec[i].1.clone(), ref_barcode_vec[j].1.clone());
                // print the similarity as float
                output
                    .write_all(
                        format!(
                            "{}\t{}\t{:.2}\t{}\n",
                            barcode_vec[i].0,
                            ref_barcode_vec[j].0,
                            similarity * 100.0,
                            nbc
                        )
                        .as_bytes(),
                    )
                    .unwrap();
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_calc_similarity() {}
}
