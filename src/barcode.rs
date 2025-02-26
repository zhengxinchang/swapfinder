//! parse barcode file
//!
//! This module is used to parse the barcode file, which is a file that contains the SNPs

use core::panic;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone)]
pub struct Barcode {
    pub chrom: String,
    pub pos: u32,
    pub meta: String,
}

#[derive(Debug, Clone)]
pub(crate) struct BarcodeList {
    // name2barcode: IndexMap<String, Barcode>,
    pub barcode_list: Vec<Barcode>,
}

impl BarcodeList {
    pub fn new() -> BarcodeList {
        BarcodeList {
            // name2barcode: IndexMap::new(),
            barcode_list: Vec::new(),
        }
    }

    pub fn load(&mut self, filename: &str) {
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);
        let mut dedup = HashSet::new();
        for line in reader.lines() {
            let line = line.unwrap();
            if line.starts_with("#") {
                continue;
            }
            let fields: Vec<&str> = line.split('\t').collect();
            let barcode = Barcode {
                chrom: fields[0].to_string(),
                pos: fields[1]
                    .parse()
                    .expect(&format!("Failed to parse position {}", &fields[3])),
                meta: fields.clone().split_off(2).join("\t"),
            };

            // check if the barcode is already in the hashmap
            let uniqkey = format!("{}:{}", barcode.chrom, barcode.pos);
            if dedup.contains(&uniqkey) {
                panic!("Duplicated barcode: {}", uniqkey);
            } else {
                dedup.insert(uniqkey);
            }
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
