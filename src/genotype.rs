#[derive(Debug, Clone, Copy)]
pub struct Genotype {
    pub a1: char,
    pub a2: char,
    pub likelihood: f64, // negative number means empty
    pub lrt: f64,
    pub pval: f64,
}

impl Genotype {
    pub fn new_nn() -> Self {
        Genotype {
            a1: 'N',
            a2: 'N',
            likelihood: -1.0, // -1.0 means null value
            lrt: -1.0,        // -1.0 means null value
            pval: -1.0,       // -1.0 means null value
        }
    }
}

impl std::fmt::Display for Genotype {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.a1, self.a2)
    }
}
