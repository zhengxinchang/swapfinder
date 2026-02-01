use std::fmt::Formatter;

use bio_types::strand::ReqStrand;

use crate::utils::phredu8_to_prob;
#[derive(Debug, Clone, Copy)]
pub struct ReadBase {
    pub base: char,
    pub phred: f64,
}

impl ReadBase {
    pub fn phred_to_prob(&self) -> f64 {
        10.0_f64.powf(-self.phred / 10.0)
    }

    // if the ReadBase is same as expected base
    pub fn expected_to_prob(&self, expected: &char) -> f64 {
        let prob = self.phred_to_prob();
        if self.base == *expected {
            return 1.0 - prob;
        } else {
            return prob / 3.0;
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Nucleotide {
    A,
    T,
    C,
    G,
}

#[derive(Debug, Clone)]
pub struct BaseGroup {
    pub nuc: Nucleotide,
    pub qual_vec: Vec<f64>,
    pub count: u32,
    pub strand_vec: Vec<ReqStrand>,
}

// rust_htslib::bam::record::Record::Strand

impl BaseGroup {
    // pub fn is_empty(&self) -> bool {
    //     self.count == 0
    // }

    pub fn new(nuc: Nucleotide) -> Self {
        BaseGroup {
            nuc: nuc,
            qual_vec: Vec::new(),
            count: 0,
            strand_vec: Vec::new(),
        }
    }

    pub fn add(&mut self, qual: f64, strand: ReqStrand) {
        self.qual_vec.push(qual);
        self.strand_vec.push(strand);
        self.count += 1;
    }
}

#[derive(Debug, Clone)]
pub struct SNPPileup {
    pub chrom: String,
    pub pos: u32,
    pub base_a: BaseGroup,
    pub base_t: BaseGroup,
    pub base_c: BaseGroup,
    pub base_g: BaseGroup,
    pub depth: u32,
}

impl SNPPileup {
    pub fn new(chrom: &str, pos: u32) -> Self {
        SNPPileup {
            chrom: chrom.to_owned(),
            pos: pos,
            base_a: BaseGroup::new(Nucleotide::A),
            base_t: BaseGroup::new(Nucleotide::T),
            base_c: BaseGroup::new(Nucleotide::C),
            base_g: BaseGroup::new(Nucleotide::G),
            depth: 0,
        }
    }

    pub fn add(&mut self, base: char, qual: u8, strand: ReqStrand) {
        self.depth += 1;
        match base {
            'A' | 'a' => {
                self.base_a.add(phredu8_to_prob(qual), strand);
            }
            'T' | 't' => {
                self.base_t.add(phredu8_to_prob(qual), strand);
            }
            'C' | 'c' => {
                self.base_c.add(phredu8_to_prob(qual), strand);
            }

            'G' | 'g' => {
                self.base_g.add(phredu8_to_prob(qual), strand);
            }
            _ => {
                eprint!("ignore invalid base {}", base);
            }
        }
    }
    pub fn get_tbl_header() -> String {
        String::from("#chrom\tpos\tA\tC\tG\tT\tTotal\n")
    }
}

impl std::fmt::Display for SNPPileup {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\n",
            self.chrom,
            self.pos,
            self.base_a.count,
            self.base_c.count,
            self.base_g.count,
            self.base_t.count,
            self.depth
        )
    }
}
