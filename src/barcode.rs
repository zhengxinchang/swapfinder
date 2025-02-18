//! parse barcode file
//!
//! This module is used to parse the barcode file, which is a file that contains the SNPs

use indexmap::IndexMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone)]
pub struct Barcode {
    pub risd: String,
    pub gene: String,
    pub chrom: String,
    pub pos: u32,
}

#[derive(Debug, Clone)]
pub(crate) struct BarcodeList {
    name2barcode: IndexMap<String, Barcode>,
    pub barcode_list: Vec<Barcode>,
}

impl BarcodeList {
    pub fn new() -> BarcodeList {
        BarcodeList {
            name2barcode: IndexMap::new(),
            barcode_list: Vec::new(),
        }
    }

    pub fn load(&mut self, filename: &str) {
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);
        for line in reader.lines() {
            let line = line.unwrap();
            if line.starts_with("#") {
                continue;
            }
            let fields: Vec<&str> = line.split('\t').collect();
            let barcode = Barcode {
                risd: fields[0].to_string(),
                gene: fields[1].to_string(),
                chrom: fields[2].to_string(),
                pos: fields[3]
                    .parse()
                    .expect(&format!("Failed to parse position {}", &fields[3])),
            };

            // check if the barcode is already in the hashmap
            if self.name2barcode.contains_key(&barcode.risd) {
                panic!("Duplicate barcode, rsid: {}", barcode.risd);
            }
            self.name2barcode
                .insert(barcode.risd.clone(), barcode.clone());
            self.barcode_list.push(barcode);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_barcode() {
        let mut barcode_list = BarcodeList::new();
        barcode_list.load("barcodes/hg38.tsv");
        // assert_eq!(barcode_list.barcode_list.len(), 10);
        dbg!(barcode_list);
    }
}
