//! This file is used to parse the BAM file, fetch reads that located in a given region
//!

use std::{
    fs::File,
    io::{Read, Write},
};

use rust_htslib::bam::{self, Read as BAMRead};

use crate::{barcode::BarcodeList, cli::BarcodeArgs};

pub fn check_file_format(path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(path)?;
    let mut buffer = [0u8; 4];

    file.read_exact(&mut buffer)?;

    match &buffer {
        b"BAM\x01" => Ok("BAM".to_string()),
        b"CRAM" => Ok("CRAM".to_string()),
        _ => Ok("Unknown".to_string()),
    }
}

// convert base pair vector to genotype string
pub fn vec2genotype(bases: &[char]) -> String {
    let mut base_vec = [0; 5]; // A, C, G, T, N

    for &base in bases {
        match base {
            'A' => base_vec[0] += 1,
            'C' => base_vec[1] += 1,
            'G' => base_vec[2] += 1,
            'T' => base_vec[3] += 1,
            _ => base_vec[4] += 1,
        }
    }

    // find the most frequent base and second most frequent base
    let (mut max_base_c, mut max_base_index) = (0, 0);
    let (mut second_base_c, mut second_base_index) = (0, 0);

    for (i, &count) in base_vec.iter().enumerate() {
        if count > max_base_c {
            second_base_c = max_base_c;
            second_base_index = max_base_index;
            max_base_c = count;
            max_base_index = i;
        } else if count > second_base_c {
            second_base_c = count;
            second_base_index = i;
        }
    }

    // convert the index to base
    let max_base_char = match max_base_index {
        0 => 'A',
        1 => 'C',
        2 => 'G',
        3 => 'T',
        _ => 'N',
    };

    let second_base_char = match second_base_index {
        0 => 'A',
        1 => 'C',
        2 => 'G',
        3 => 'T',
        _ => 'N',
    };

    // parse the max and second max index and generate the genotype
    let total = bases.len() as f64;
    let max_pct = max_base_c as f64 / total;
    let second_pct = second_base_c as f64 / total;

    if max_pct > 0.8 {
        format!("{}{}", max_base_char, max_base_char)
    } else if second_pct > 0.2 {
        let mut genotype_vec = vec![max_base_char, second_base_char];
        genotype_vec.sort();
        genotype_vec.into_iter().collect()
    } else {
        "NN".to_string()
    }
}

pub fn calculate_barcode(args: BarcodeArgs) {
    let mut barcode_list = BarcodeList::new();
    barcode_list.load(args.barcode.to_str().unwrap());

    let mut bam = bam::IndexedReader::from_path(&args.bam).unwrap();
    let header_view = bam.header().to_owned();

    // check if tname is start with chr
    let tname = header_view.tid2name(0);
    let has_chr = tname.starts_with(b"chr");

    if check_file_format(&args.bam.to_str().unwrap()).unwrap() == "CRAM" {
        bam.set_reference(args.reference.unwrap().to_str().unwrap())
            .unwrap();
    }

    let mut base_vec = Vec::new();
    for barcode in &barcode_list.barcode_list {
        let query_pos = if has_chr {
            format!("chr{}:{}-{}", barcode.chrom, barcode.pos, barcode.pos + 1)
        } else {
            format!("{}:{}-{}", barcode.chrom, barcode.pos, barcode.pos + 1)
        };

        bam.fetch(&query_pos).unwrap();

        for plup in bam.pileup() {
            let plup = plup.unwrap();

            if String::from_utf8_lossy(header_view.tid2name(plup.tid())) == barcode.chrom
                && plup.pos() == barcode.pos - 1
            {
                let bases: Vec<char> = plup
                    .alignments()
                    .filter_map(|alignment| {
                        alignment
                            .qpos()
                            .map(|qpos| alignment.record().seq()[qpos] as char)
                    })
                    .collect();
                let gt = vec2genotype(&bases);
                base_vec.push(gt);
                break;
            }
        }
    }

    let mut output = File::create(args.output).unwrap();
    let filename = args.bam.file_name().unwrap().to_str().unwrap();
    output
        .write_all(format!("#{}\n", filename).as_bytes())
        .unwrap();
    output
        .write_all(b"#rsid\tgene\tchrom\tpos\tgenotype\n")
        .unwrap();

    for (barcode, genotype) in barcode_list.barcode_list.iter().zip(base_vec.iter()) {
        output
            .write_all(
                format!(
                    "{}\t{}\t{}\t{}\t{}\n",
                    barcode.risd, barcode.gene, barcode.chrom, barcode.pos, genotype
                )
                .as_bytes(),
            )
            .unwrap();
    }
}
