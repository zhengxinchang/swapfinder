use crate::genotype::Genotype;
#[derive(Debug, Clone, Copy)]
pub struct ReadBase {
    pub base: char,
    pub phred: f64,
}

impl ReadBase {
    fn phrad_to_prob(&self) -> f64 {
        10.0_f64.powf(-self.phred / 10.0)
    }

    // if the ReadBase is same as expected base
    fn expected_to_prob(&self, expected: &char) -> f64 {
        let prob = self.phrad_to_prob();
        if self.base == *expected {
            return 1.0 - prob;
        } else {
            return prob / 3.0;
        }
    }
}


// A G 
// A A A A A G G G G G 
fn genotype_likelihood(genotype: (char, char), reads: &Vec<ReadBase>) -> f64 {
    let (a1, a2) = genotype;
    let likelihood = reads.iter().fold(1.0, |acc, read| {
        let p1 = read.expected_to_prob(&a1);
        let p2 = read.expected_to_prob(&a2);
        acc * (p1 + p2) / 2.0
    });

    likelihood
}

pub fn calculate_likelihoods_all_genotype(read_bases: Vec<ReadBase>) -> Vec<Genotype> {
    let nucleotides: Vec<char> = vec!['A', 'C', 'G', 'T'];
    let mut genotype_combinations: Vec<(char, char)> = Vec::new();

    for (i, base1) in nucleotides.iter().enumerate() {
        for base2 in &nucleotides[i..] {
            genotype_combinations.push((*base1, *base2));
        }
    }

    let likelihoods = genotype_combinations
        .into_iter()
        .map(|genotype| {
            let likelihood = genotype_likelihood(genotype, &read_bases);
            Genotype {
                a1: genotype.0,
                a2: genotype.1,
                likelihood,
                lrt: -1.0,
                pval: -1.0,
            }
        })
        .collect();

    likelihoods
}

/// normalize the log likelihood
pub fn normalization(v: f64) -> f64 {
    if v < 0.0 {
        return -1.0;
    }
    v / (v + 1.0)
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_cal_genotype_lh() {
        let b1 = vec![
            ReadBase {
                base: 'A',
                phred: 10.0,
            },
            ReadBase {
                base: 'A',
                phred: 20.0,
            },
            ReadBase {
                base: 'A',
                phred: 30.0,
            },
            ReadBase {
                base: 'A',
                phred: 30.0,
            },
            ReadBase {
                base: 'G',
                phred: 30.0,
            },
            ReadBase {
                base: 'G',
                phred: 20.0,
            },
            ReadBase {
                base: 'C',
                phred: 10.0,
            },
        ];

        let mut lhs = calculate_likelihoods_all_genotype(b1);

        lhs.sort_by(|a, b| {
            let cmp = b.likelihood.partial_cmp(&a.likelihood).unwrap();
            cmp
        });
        dbg!(&lhs);
    }
}
