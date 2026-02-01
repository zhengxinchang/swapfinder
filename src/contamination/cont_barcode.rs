use log::{error, info};
use std::{
    collections::{HashMap, btree_map::BTreeMap},
    io::BufRead, path::{Path, PathBuf},
};

use crate::utils::trim_chr_prefix;
pub struct BarcodeItem {
    pub pos: u32,
    pub idx: usize,
    pub ref_base: char,
    pub alt_base: char,
    pub metadata: String,
}

pub struct ChrIntervalTree {
    pub chrom: String,
    pub genome_pos_tree: BTreeMap<u32, usize>,
    pub item_list: Vec<BarcodeItem>,
    pub item_count: usize,

}


impl ChrIntervalTree {
    pub fn new(chrom: &str) -> Self {
        ChrIntervalTree {
            chrom: chrom.to_owned(),
            genome_pos_tree: BTreeMap::new(),
            item_list: Vec::new(),
            item_count: 0,
        }
    }

    pub fn insert(&mut self, barcode_item: BarcodeItem) {

        self.item_count += 1;
        self.genome_pos_tree.insert(barcode_item.pos, self.item_count - 1);
        self.item_list.push(barcode_item);

        
    }

    pub fn find_range(&self, start: u32, end: u32) -> Vec<&BarcodeItem> {
        let mut result = Vec::new();
        for (_pos, item_index) in self.genome_pos_tree.range(start..=end) {
            result.push(&self.item_list[*item_index]);
        }
        result
    }

    /// Get the genome position by index
    /// input: index: usize ->  the index of the record in the barcode file in the chromosome
    /// output: Option<u32>
    pub fn get_genome_pos_by_idx(&self, index: usize) -> Option<u32> {
        let barcode = self.item_list.get(index);
        if let Some(barcode) = barcode {
            Some(barcode.pos)
        } else {
            None    
        }
    }
}

pub struct BarcodeManager {
    pub chr_tree_map: HashMap<String, ChrIntervalTree>,
}

impl BarcodeManager {
    pub fn insert(&mut self, chrom: String, barcode_item: BarcodeItem) {
        let chrom_trimmed = trim_chr_prefix(&chrom).to_string();
        if !self.chr_tree_map.contains_key(&chrom_trimmed) {
            self.chr_tree_map
                .insert(chrom_trimmed.clone(), ChrIntervalTree::new(&chrom_trimmed));
        }
        if let Some(chr_tree) = self.chr_tree_map.get_mut(&chrom_trimmed) {
            chr_tree.insert(barcode_item);
        }
    }

    pub fn from_disk<P:AsRef<Path>>(path:P) -> Self {
        let mut manager = BarcodeManager {
            chr_tree_map: HashMap::new(),
        };
        let file = std::fs::File::open(&path).expect("Failed to open barcode file.");
        let mut reader = std::io::BufReader::new(file);
        let mut line_n = 0;
        let mut chr_n = ("".to_string(), 0);
        let mut buf = String::new();

        while reader
            .read_line(&mut buf)
            .expect("Failed to read line from barcode file.")
            > 0
        {
            line_n += 1;
            
                let line = buf.clone();

                if line.starts_with('#') || line.is_empty() {
                    continue;
                }

                let fields: Vec<String> = line.split('\t').map(|f|{f.to_string()}).collect();

                if fields.len() != 5 {
                    error!("Can not parse line, fields must equal to 5 {}", buf);
                    panic!();
                }

                let chrom = trim_chr_prefix(&fields[0]);
                let chrom = chrom.to_string();
                if chrom != chr_n.0 {
                    chr_n.0 = chrom.clone();
                    chr_n.1 += 1;
                }
                

                let pos: u32 = fields[1]
                    .parse()
                    .expect(&format!("Failed to parse position at line {}", line_n));
                let ref_base = fields[2]
                    .chars()
                    .next()
                    .expect(&format!("Failed to parse ref base at line {}", line_n));
                let alt_base = fields[3]
                    .chars()
                    .next()
                    .expect(&format!("Failed to parse alt base at line {}", line_n));
                let metadata = fields[4].to_string();

                let barcode_item = BarcodeItem {
                    pos,
                    idx: line_n - 1,
                    ref_base,
                    alt_base,
                    metadata,
                };
                manager.insert(chrom, barcode_item);
            

            buf.clear();
        }
        info!(
            "Loaded {} barcodes from {} chromosomes from barcode file {}",
            line_n, chr_n.1, path.as_ref().display()
        );

        manager
    }

    pub fn find_range(
        &self,
        chrom: &str,
        start: u32,
        end: u32,
    ) -> Option<Vec<&BarcodeItem>> {
        let chrom_trimmed = trim_chr_prefix(chrom);
        if let Some(chr_tree) = self.chr_tree_map.get(chrom_trimmed) {
            let items = chr_tree.find_range(start, end);
            if items.is_empty() {
                None
            } else {
                Some(items)
            }
        } else {
            None
        }
    }

    pub fn has_chrom(&self, chrom: &str) -> bool {
        self.chr_tree_map.contains_key(chrom)
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_barcode_manager() {
        let barcode_manager = BarcodeManager::from_disk("test/contam_barcode.chr22.tsv");
        assert_eq!(barcode_manager.chr_tree_map.len(), 1);

        let items = barcode_manager
            .find_range("chr22", 10514994, 10547326)
            .unwrap();
        assert_eq!(items.len(), 3);
        for item in items {
            println!(
                "pos:{} ref:{} alt:{} metadata:{}",
                item.pos, item.ref_base, item.alt_base, item.metadata
            );
        }
    }
}