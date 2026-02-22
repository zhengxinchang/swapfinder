use std::collections::{BTreeMap, HashSet};

use crate::contamination::fingerprint::{self, BitBaseFingerPrint};


pub struct HaplotypeCmp {
    pub similarity: f32, // similarity score
    pub overlap_start_in_hap: usize,  // overlap start in haplotype
    pub overlap_start_in_fingerprint: usize,  // overlap start in fingerprint
    pub overlap_length: usize, // overlap length
    pub left_extension: usize, // length of left extension
    pub right_extension: usize, // length of right extension
}

impl HaplotypeCmp {
    pub fn new(
        similarity: f32,
        overlap_start_in_hap: usize,
        overlap_start_in_fingerprint: usize,
        overlap_length: usize,
        left_extension: usize,
        right_extension: usize,
    ) -> Self {
        HaplotypeCmp {
            similarity,
            overlap_start_in_hap,
            overlap_start_in_fingerprint,
            overlap_length,
            left_extension,
            right_extension,
        }
    }
}


pub struct Haplotype {
    pub genome_start: u32, // leftmost genome position of the haplotype, it take from the leftmost barcode in this haplotype
    pub genome_end: u32, // rightmost genome position of the haplotype, it take from the rightmost barcode in this haplotype
    pub start_idx: usize, // start index in the fingerprint bitbase, it take from the leftmost barcode in this haplotype
    pub end_idx: usize, // end index in the fingerprint bitbase, it take from the rightmost barcode in this haplotype
    pub bit_bases: Vec<u8>,  // 这里应该用dequeue更合适，要修改
    pub base_length: usize, 
    pub read_count: u32,
    pub base_position_cov: Vec<u16>, // coverage of each base position in this haplotype
    pub total_base_cov: f64, // accumulated coverage of all bases in this haplotype, this value is from all reads covering this haplotype
}

impl Haplotype {
    pub fn from_fingerprint(fingerprint: &BitBaseFingerPrint) -> Self {
        Haplotype {
            genome_start: fingerprint.get_genome_start(),
            genome_end: fingerprint.get_genome_end(),
            start_idx: fingerprint.idx,
            end_idx: fingerprint.idx + fingerprint.get_count() - 1,
            bit_bases: fingerprint.bitbase.clone(),
            base_length: fingerprint.get_count(),
            read_count: 0,
            base_position_cov: vec![1; fingerprint.get_count()],
            total_base_cov: fingerprint.get_count() as f64,
        }
    }

    pub fn get_base_raw(&self, local_idx: usize) -> u8 {
        if local_idx >= self.base_length {
            panic!("Index out of bounds in Haplotype::get_base_raw");
        }
        let byte_index = local_idx / 2;
        let is_high_nibble = local_idx % 2 == 0;
        let byte = self.bit_bases[byte_index];
        let bit = if is_high_nibble {
            (byte & 0b11110000) >> 4
        } else {
            byte & 0b00001111
        };
        bit
    }

    pub fn get_base_vec_raw(&self, start: usize, length: usize) -> Vec<u8> {
        if start + length > self.base_length || length == 0 {
            panic!("Index out of bounds in Haplotype::get_base_vec_raw");
        }
        let mut base_vec: Vec<u8> = Vec::new();
        for i in start..start + length {
            base_vec.push(self.get_base_raw(i));
        }
        base_vec
    }

    /// try to merge fingerprint into this haplotype accroding to the overlap of the barcode vector between haplotype and fingerprint
    /// threshold: minimal number of overlapping barcodes to consider merging
    /// return: similarity score between haplotype and fingerprint
    ///
    pub fn check_fingerprint_similarity(
        &mut self,
        fingerprint: &BitBaseFingerPrint,
    ) -> HaplotypeCmp {
        // check if the barcodes postions overlap with haplotype
        if fingerprint.idx > self.start_idx + self.base_length
            || fingerprint.idx + fingerprint.get_count() < self.start_idx
        {
            return HaplotypeCmp::new(0.0, 0, 0, 0, 0, 0);
        }

        let offset = fingerprint.idx as i32 - self.start_idx as i32;
        if offset >= 0 {
            // fingerprint starts after haplotype start
            // ******************** haplotype
            //       **************** fingerprint
            // or
            // ******************** haplotype
            // **************** fingerprint

            let overlap_start_in_hap = offset as usize; // will be 0 if offset == 0
            let overlap_start_in_fingerprint = 0usize;
            let overlap_length = std::cmp::min(
                self.base_length - overlap_start_in_hap,
                fingerprint.get_count(),
            );

            let hap_base_vec = self.get_base_vec_raw(overlap_start_in_hap, overlap_length);
            let fingerprint_base_vec =
                fingerprint.get_base_vec_raw(overlap_start_in_fingerprint, overlap_length);

            let fingerprint_extend_length = (fingerprint.get_count() + fingerprint.idx)
                .saturating_sub(self.start_idx + self.base_length);
            if  fingerprint_extend_length > 0 {
                return HaplotypeCmp::new(
                    Haplotype::similarity(hap_base_vec, fingerprint_base_vec),
                    overlap_start_in_hap,
                    overlap_start_in_fingerprint,
                    overlap_length,
                    0, // no left extension
                    fingerprint_extend_length,
                );
            }else {
                return HaplotypeCmp::new(
                    Haplotype::similarity(hap_base_vec, fingerprint_base_vec),
                    overlap_start_in_hap,
                    overlap_start_in_fingerprint,
                    overlap_length,
                    0, // no left extension
                    0, // no right extension
                );
            }
        } else {
            // fingerprint starts before haplotype start
            //       **************** haplotype
            // **************** fingerprint
            // or
            // ********************************** fingerprint (longer than haplotype)

            let overlap_start_in_hap = 0usize;
            let overlap_start_in_fingerprint = -offset as usize;
            let overlap_length = std::cmp::min(
                self.base_length,
                fingerprint.get_count() - overlap_start_in_fingerprint,
            );

            let hap_base_vec = self.get_base_vec_raw(overlap_start_in_hap, overlap_length);
            let fingerprint_base_vec =
                fingerprint.get_base_vec_raw(overlap_start_in_fingerprint, overlap_length);

            let fingerprint_left_extension = self.start_idx.saturating_sub(fingerprint.idx);
            let fingerprint_right_extension = (fingerprint.get_count() + fingerprint.idx)
                .saturating_sub(self.end_idx + 1);
            return HaplotypeCmp::new(
                Haplotype::similarity(hap_base_vec, fingerprint_base_vec),
                overlap_start_in_hap,
                overlap_start_in_fingerprint,
                overlap_length,
                fingerprint_left_extension,
                fingerprint_right_extension,
            );
        }
    }

    pub fn similarity(a: Vec<u8>, b: Vec<u8>) -> f32 {
        // compare each and ignore the N (0b1111)
        if a.len() != b.len() {
            panic!("Vectors must be of the same length for similarity calculation");
        }
        let mut matches = 0;
        let mut total = 0;
        for (base_a, base_b) in a.iter().zip(b.iter()) {
            total += 1;
            if base_a == base_b {
                matches += 1;
            }
        }
        if total == 0 {
            return 0.0;
        }
        if total == matches {
            return 1.0;
        } else {
            return 0.0;
        }
    }

    pub fn merge_fingerprint(&mut self, fingerprint: &BitBaseFingerPrint, cmp: &HaplotypeCmp) {
        // update genome_start and genome_end
        // update start_idx and end_idx
        // update bit_bases, if self has N in the position, replace it with fingerprint base
        // update and extend base_position_cov
        // update read_count
        // update total_base_cov
        // Note: here we assume the fingerprint is already checked for similarity and overlap
        
        let fingerprint_genome_start = fingerprint.get_genome_start();
        let fingerprint_genome_end = fingerprint.get_genome_end();
        if fingerprint_genome_start < self.genome_start {
            self.genome_start = fingerprint_genome_start;
        }
        if fingerprint_genome_end > self.genome_end {
            self.genome_end = fingerprint_genome_end;
        }

        let new_start_idx = std::cmp::min(self.start_idx, fingerprint.idx);
        let new_end_idx = std::cmp::max(
            self.start_idx + self.base_length - 1,
            fingerprint.idx + fingerprint.get_count() - 1,
        );
        self.start_idx = new_start_idx;
        self.end_idx = new_end_idx;
        let new_base_count = new_end_idx - new_start_idx + 1;
        self.base_length = new_base_count;

        for i in 0..cmp.overlap_length {
            let hap_idx = cmp.overlap_start_in_hap + i;
            let fingerprint_idx = cmp.overlap_start_in_fingerprint + i;
            let fingerprint_base = fingerprint.get_base_char(fingerprint_idx);

            // update base if finterprint base is not N
            if fingerprint_base == 0b1111 {
                continue;
            }
            let byte_index = hap_idx / 2;
            let is_high_nibble = hap_idx % 2 == 0;
            if is_high_nibble {
                self.bit_bases[byte_index] &= 0b00001111; // clear high nibble
                self.bit_bases[byte_index] |= fingerprint_base << 4; // set high nibble
            } else {
                self.bit_bases[byte_index] &= 0b11110000; // clear low nibble
                self.bit_bases[byte_index] |= fingerprint_base; // set low nibble
            }

            self.base_position_cov[hap_idx] += 1;
        }
        self.read_count += 1;
        self.total_base_cov += fingerprint.get_count() as f64;

        // extend self.bit_bases and base_position_cov if needed
        if cmp.left_extension > 0 {
            let extend_bases = fingerprint.get_base_vec_raw(0, cmp.left_extension);
            let mut new_bit_bases: Vec<u8> = Vec::new();
            let mut new_base_position_cov: Vec<u16> = Vec::new();
            for i in 0..cmp.left_extension {
                let base = extend_bases[i];
                let byte_index = i / 2;
                if i % 2 == 0 {
                    new_bit_bases.push(base << 4); // high nibble
                } else {
                    new_bit_bases[byte_index] |= base; // low nibble
                }
                new_base_position_cov.push(1);
            }
            if cmp.left_extension % 2 != 0 { // if odd, need to shift original bit_bases
                
            }


    }

    pub fn check_other_haplotype(&self, other: &Haplotype) -> bool // check if two haplotypes overlap and can be merged
    {
        todo!()
    }

    pub fn merge_other_haplotype(&mut self, other: &Haplotype) {
        todo!()
    }
}

pub struct HaplotypeTree {
    pub bin_size: usize,
    pub binned_idx_tree: BTreeMap<usize, HashSet<usize>>, // bin barcode index to haplotype index vector
    pub haplotype_list: Vec<Haplotype>,
}

impl HaplotypeTree {
    pub fn new(bin_size: usize) -> Self {
        HaplotypeTree {
            bin_size,
            binned_idx_tree: BTreeMap::new(),
            haplotype_list: Vec::new(),
        }
    }

    pub fn add_haplotype(&mut self, haplotype: Haplotype) {
        let hap_idx = self.haplotype_list.len();
        self.haplotype_list.push(haplotype);
        let start_bin = self.haplotype_list[hap_idx].start_idx / self.bin_size;
        let end_bin = self.haplotype_list[hap_idx].end_idx / self.bin_size;
        for bin in start_bin..=end_bin {
            self.binned_idx_tree
                .entry(bin)
                .or_insert(HashSet::new())
                .insert(hap_idx);
        }
    }

    pub fn add_fingerprint(&mut self, fingerprint: &BitBaseFingerPrint) {
        // only simiarity == 1.0 will be merged
        let start_bin = fingerprint.idx / self.bin_size;
        let end_bin = (fingerprint.idx + fingerprint.get_count() - 1) / self.bin_size;
        let mut unique_hap_indices: std::collections::HashSet<usize> =
            std::collections::HashSet::new();
        for bin in start_bin..=end_bin {
            if let Some(hap_indices) = self.binned_idx_tree.get(&bin) {
                for &hap_idx in hap_indices.iter() {
                    unique_hap_indices.insert(hap_idx);
                }
            }
        }

        let mut max_overlap: usize = 0;
        let mut max_hap_len: usize = 0;
        let mut best_hap_idx: Option<usize> = None;
        let mut best_hap_cmp = None;

        for &hap_idx in unique_hap_indices.iter() {
            let hapcmp: HaplotypeCmp =
                self.haplotype_list[hap_idx].check_fingerprint_similarity(fingerprint);
            if hapcmp.similarity != 1.0 {
                continue;
            }
            if hapcmp.overlap_length < 2 {
                continue;
            }
            // if similarity is 1.0, check overlap length
            if hapcmp.overlap_length > max_overlap {
                max_overlap = hapcmp.overlap_length;
                best_hap_idx = Some(hap_idx);
                // also need to update max_hap_len
                let hap_len = self.haplotype_list[hap_idx].base_length;
                max_hap_len = hap_len;
                best_hap_cmp = Some(hapcmp);
            } else if hapcmp.overlap_length == max_overlap {
                let hap_len = self.haplotype_list[hap_idx].base_length;
                if hap_len > max_hap_len {
                    max_hap_len = hap_len;
                    best_hap_idx = Some(hap_idx);
                    best_hap_cmp = Some(hapcmp);
                }

            }

        }

        if let Some(hap_idx) = best_hap_idx {
            // add fingerprint to this haplotype
            let hapcmp: HaplotypeCmp =
                best_hap_cmp.unwrap();
            self.haplotype_list[hap_idx].merge_fingerprint(fingerprint, &hapcmp);
            // update the binned_idx_tree if extension 
            if hapcmp.left_extension > 0 {
                for i in fingerprint.idx..(fingerprint.idx + hapcmp.left_extension) {
                    let bin = i / self.bin_size;
                    self.binned_idx_tree
                        .entry(bin)
                        .or_insert(HashSet::new())
                        .insert(hap_idx);
                }
            }
            if hapcmp.right_extension > 0 {
                let start = fingerprint.idx + fingerprint.get_count() - hapcmp.right_extension;
                let end = fingerprint.idx + fingerprint.get_count();
                for i in start..end {
                    let bin = i / self.bin_size;
                    self.binned_idx_tree
                        .entry(bin)
                        .or_insert(HashSet::new())
                        .insert(hap_idx);
                }
            }
        }else {
            // create a new haplotype from this fingerprint
            let new_haplotype = Haplotype::from_fingerprint(fingerprint);
            self.add_haplotype(new_haplotype);
        }

    }
}

