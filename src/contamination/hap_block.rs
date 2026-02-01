use crate::contamination::fingerprint::BitBaseFingerPrint;

pub struct Haplotype {
    pub genome_start: u32, // leftmost genome position of the haplotype, it take from the leftmost barcode in this haplotype
    pub genome_end: u32, // rightmost genome position of the haplotype, it take from the rightmost barcode in this haplotype
    pub start_idx: usize, // start index in the fingerprint bitbase, it take from the leftmost barcode in this haplotype
    pub end_idx: usize, // end index in the fingerprint bitbase, it take from the rightmost barcode in this haplotype
    pub bit_bases: Vec<u8>,
    pub base_count: usize,
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
            base_count: fingerprint.get_count(),
            read_count: 0,
            base_position_cov: vec![1; fingerprint.get_count()],
            total_base_cov: fingerprint.get_count() as f64,
        }
    }

    pub fn get_base_at(&self, local_idx: usize) -> u8 {
        if local_idx >= self.base_count {
            panic!("Index out of bounds in Haplotype::get_base_at");
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

    pub fn get_base_vec(&self, start: usize, length: usize) -> Vec<u8> {
        if start + length > self.base_count || length == 0 {
            panic!("Index out of bounds in Haplotype::get_base_vec");
        }
        let mut base_vec: Vec<u8> = Vec::new();
        for i in start..start + length {
            base_vec.push(self.get_base_at(i));
        }
        base_vec
    }

    /// try to merge fingerprint into this haplotype accroding to the overlap of the barcode vector between haplotype and fingerprint
    /// threshold: minimal number of overlapping barcodes to consider merging
    /// return: similarity score between haplotype and fingerprint
    ///
    pub fn check_fingerprint_similarity(&mut self, fingerprint: &BitBaseFingerPrint) -> (f32,usize,usize,usize) {
        // check if the barcodes postions overlap with haplotype
        if fingerprint.idx > self.start_idx + self.base_count
            || fingerprint.idx + fingerprint.get_count() < self.start_idx
        {
            return (0.0,0,0,0);
        }

        let offset = fingerprint.idx as i32 - self.start_idx as i32;
        if offset >= 0 {
            // fingerprint starts after haplotype start
            // ******************** haplotype
            //       **************** fingerprint
            // or
            // ******************** haplotype
            // **************** fingerprint

            let overlap_start_in_hap = offset as usize;  // will be 0 if offset == 0
            let overlap_start_in_fingerprint = 0usize;
            let overlap_length = std::cmp::min(
                self.base_count - overlap_start_in_hap,
                fingerprint.get_count(),
            );

            let hap_base_vec = self.get_base_vec(overlap_start_in_hap, overlap_length);
            let fingerprint_base_vec =
                fingerprint.get_base_vec(overlap_start_in_fingerprint, overlap_length);

            return (Haplotype::similarity(hap_base_vec, fingerprint_base_vec), overlap_start_in_hap, overlap_start_in_fingerprint, overlap_length);
        } else {

            // fingerprint starts before haplotype start
            //       **************** haplotype
            // **************** fingerprint

            let overlap_start_in_hap = 0usize;
            let overlap_start_in_fingerprint = -offset as usize;
            let overlap_length = std::cmp::min(
                self.base_count,
                fingerprint.get_count() - overlap_start_in_fingerprint,
            );

            let hap_base_vec = self.get_base_vec(overlap_start_in_hap, overlap_length);
            let fingerprint_base_vec =
                fingerprint.get_base_vec(overlap_start_in_fingerprint, overlap_length);

            return (Haplotype::similarity(hap_base_vec, fingerprint_base_vec), overlap_start_in_hap, overlap_start_in_fingerprint, overlap_length);
        } 
    }

    pub fn similarity(a:Vec<u8>, b:Vec<u8>) -> f32 {
        // compare each and ignore the N (0b1111)
        if a.len() != b.len() {
            panic!("Vectors must be of the same length for similarity calculation");
        }
        let mut matches = 0;
        let mut total = 0;
        for (base_a, base_b) in a.iter().zip(b.iter()) {
            if *base_a == 0b1111 || *base_b == 0b1111 {
                continue;
            }
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
        }else {
            return 0.0;
        }
    }

    pub fn merge_fingerprint(&mut self, fingerprint: &BitBaseFingerPrint,sim:(f32,usize,usize,usize)) {
       // update genome_start and genome_end
       // update start_idx and end_idx 
       // update bit_bases, if self has N in the position, replace it with fingerprint base
       // update and extend base_position_cov
       // update read_count
       // update total_base_cov 
       // Note: here we assume the fingerprint is already checked for similarity and overlap
        let (_, overlap_start_in_hap, overlap_start_in_fingerprint, overlap_length) = sim;
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
            self.start_idx + self.base_count - 1,
            fingerprint.idx + fingerprint.get_count() - 1,
        );
        self.start_idx = new_start_idx;
        self.end_idx = new_end_idx;
        let new_base_count = new_end_idx - new_start_idx + 1;
        self.base_count = new_base_count;


        for i in 0..overlap_length {
            let hap_idx = overlap_start_in_hap + i;
            let fingerprint_idx = overlap_start_in_fingerprint + i;
            let fingerprint_base = fingerprint.get_base(fingerprint_idx);


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
    }   

    pub fn check_other_haplotype(&self,other: &Haplotype)-> bool // check if two haplotypes overlap and can be merged
     {
        todo!()

    }
}

pub struct HaplotypeBlock {
    pub idx: usize,
    pub count: usize,
    pub genome_start: u32,
    pub genome_end: u32,
    pub vec: Vec<Haplotype>,
}

impl HaplotypeBlock {
    pub fn new(idx: usize) -> Self {
        HaplotypeBlock {
            idx,
            count: 0,
            genome_start: 0,
            genome_end: 0,
            vec: Vec::new(),
        }
    }

    pub fn add_fingerprint(&mut self, fingerprint: &BitBaseFingerPrint) {
        // chekc if the fpingerprint can be merged into existing haplotypes
        // if yes, merge it
        // if no, create a new haplotype

        for hap in self.vec.iter_mut() {
            let sim = hap.check_fingerprint_similarity(fingerprint);
            if sim.0 > 0.8 {
                hap.merge_fingerprint(fingerprint,sim);
                return;
            }
        }
        let new_haplotype = Haplotype::from_fingerprint(fingerprint);
        self.vec.push(new_haplotype);
        self.count += 1;
    }

    /// purge haplotypes with read_count < min_read_count
    pub fn purge_haplotypes(&mut self, min_read_count: u32) {
        self.vec.retain(|hap| hap.read_count >= min_read_count);
        self.count = self.vec.len();
    }

    /// merge haplotypes that are similar enough
    pub fn merge_haplotypes(&mut self, similarity_threshold: f32) {
        todo!()
    }

    /// estimate contamination level based on haplotype distribution
    /// find the largest two haplotypes and calculate the contamination level??
    pub fn estimate_contamination(&self) -> f32 {
        todo!()
    }
}


pub struct HaplotypeBlockTree {
    pub hap_block_map: std::collections::btree_map::BTreeMap<usize, usize>, // barcode idx of teh haplotype block -> index in the hap_block_vec
    pub hap_block_vec: Vec<HaplotypeBlock>,
}

impl HaplotypeBlockTree {
    pub fn new() -> Self {
        HaplotypeBlockTree {
            hap_block_map: std::collections::btree_map::BTreeMap ::new(),
            hap_block_vec: Vec::new(),
        }
    }

    /// add fingerprint into the corresponding haplotype block
    /// if a finterprint has multiple overlapping haplotype blocks, it will be added to all of them,
    /// this is ok beacuse only overlapped barcodes will be added into the haplotype
    /// at end of the day, only barcodes that meet the min-reads threshold will be used 
    /// to estimate contamination
    /// *****************|**************** hpblocks
    ///              *********** fingerprint <- will be splited and added to two hapblocks
    pub fn add_fingerprint(&mut self, fingerprint: &BitBaseFingerPrint) {


        let query_idx_start = fingerprint.idx;
        let query_idx_end = fingerprint.idx + fingerprint.get_count() - 1;
        // find overlapping haplotype blocks
        let block_index = self.hap_block_map.range(query_idx_start..=query_idx_end);
    
        for (_block_idx, hap_block_vec_idx) in block_index {
            let hap_block = &mut self.hap_block_vec[*hap_block_vec_idx];
            hap_block.add_fingerprint(fingerprint);
        }
    }
}