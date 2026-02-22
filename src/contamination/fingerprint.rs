use crate::contamination::cont_barcode::BarcodeItem;

/// A fragment of read with bitbase representation
/// four bit for each base:
/// 0000
/// 0001: C
/// 0010: G
/// 0011: T
#[derive(Debug, Clone)]
pub struct BitBaseFingerPrint {
    pub idx: usize,
    pub count: usize,
    pub genome_start: u32,
    pub genome_end: u32,
    pub bitbase: Vec<u8>,
}

impl BitBaseFingerPrint {
    pub fn new(barcodes: &Vec<&BarcodeItem>) -> Self {
        let first = &barcodes[0];
        let last = &barcodes[barcodes.len() - 1];

        BitBaseFingerPrint {
            idx: first.idx,
            count: 0,
            genome_start: first.pos,
            genome_end: last.pos,
            bitbase: Vec::new(),
        }
    }

    pub fn add_base(&mut self, base: u8) {
        let bit: u8 = match base {
            b'A' | b'a' => 0b0000,
            b'C' | b'c' => 0b0001,
            b'G' | b'g' => 0b0010,
            b'T' | b't' => 0b0011,
            _ => 0b1111, // default to A for unknown base
        };
        // pack four bases into one byte
        if self.count % 2 == 0 {
            self.bitbase.push(bit << 4);
        } else {
            let last_index = self.bitbase.len() - 1;
            self.bitbase[last_index] |= bit;
        }
        self.count += 1;
    }

    pub fn get_base_char(&self, local_idx: usize) -> u8 {
        if local_idx >= self.count {
            panic!("Index out of bounds in BitBaseFingerPrint::get_base");
        }
        let byte_index = local_idx / 2;
        let is_high_nibble = local_idx % 2 == 0;
        let byte = self.bitbase[byte_index];
        let bit = if is_high_nibble {
            (byte & 0b11110000) >> 4
        } else {
            byte & 0b00001111
        };
        match bit {
            0b0000 => b'A',
            0b0001 => b'C',
            0b0010 => b'G',
            0b0011 => b'T',
            _ => b'N',
        }
    }

    pub fn get_base_raw(&self, local_idx: usize) -> u8 {
        if local_idx >= self.count {
            panic!("Index out of bounds in BitBaseFingerPrint::get_base");
        }
        let byte_index = local_idx / 2;
        let is_high_nibble = local_idx % 2 == 0;
        let byte = self.bitbase[byte_index];
        if is_high_nibble {
            (byte & 0b11110000) >> 4
        } else {
            byte & 0b00001111
        }
    }

    pub fn get_base_vec_raw(&self,start: usize, length:usize) -> Vec<u8> {
        if start + length > self.count || length == 0 {
            panic!("Index out of bounds in BitBaseFingerPrint::get_base_vec");
        }
        let mut base_vec: Vec<u8> = Vec::new();
        for i in start..start+length {
            base_vec.push(self.get_base_raw(i));
        }
        base_vec
    }

    pub fn get_idx(&self) -> usize {
        self.idx
    }

    pub fn get_count(&self) -> usize {
        self.count
    }

    pub fn get_genome_start(&self) -> u32 {
        self.genome_start
    }

    pub fn get_genome_end(&self) -> u32 {
        self.genome_end
    }

    pub fn get_string(&self) -> String {
        let mut seq = String::new();
        for i in 0..self.count {
            let base = self.get_base_char(i);
            seq.push(base as char);
        }
        seq
    }
}
