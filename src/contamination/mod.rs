pub mod cont_barcode;
pub mod fingerprint;
pub mod haplotype;
use cont_barcode::*;
use fingerprint::*;
use haplotype::*;

use std::{env, io::Write};

use bio_types::sequence::SequenceRead;
use log::warn;
use rust_htslib::bam::{self, ext::BamRecordExtensions, Read};
use url::Url;

use crate::{
    cli::EstConstArgs,
    profile::BarcodeList,
    read_base::{ReadBase, SNPPileup},
    utils::{check_file_format, is_url, trim_chr_prefix},
};

pub fn estimation(args: EstConstArgs) {
    if env::var("CURL_CA_BUNDLE").is_err() {
        env::set_var("CURL_CA_BUNDLE", "/etc/ssl/certs/ca-certificates.crt");
    }

    let bam = if is_url(&args.bam.to_str().unwrap()) {
        let url = Url::parse(&args.bam.to_str().unwrap()).unwrap();
        let b = bam::IndexedReader::from_url(&url);
        b
    } else {
        bam::IndexedReader::from_path(&args.bam)
    };

    // read bam and print error if failed
    let mut bam = match bam {
        Ok(bam) => bam,
        Err(e) => {
            eprintln!("Error reading bam file: {}\n", e);
            std::process::exit(1);
        }
    };

    let header_view = bam.header().to_owned();

    // check if tname is start with chr
    let tname = header_view.tid2name(0);
    let has_chr = tname.starts_with(b"chr");

    if has_chr {
        eprint!("alignment file has chr prefix in chromsome names\n");
    } else {
        eprint!("alignment file does not have chr prefix in chromsome names\n");
    }

    if check_file_format(&args.bam.to_str().unwrap()).unwrap() == "CRAM" {
        bam.set_reference(args.reference.unwrap().to_str().unwrap())
            .unwrap();
    }

    let mut skipped_rec_n = 0;
    let mut processed = 0;

    let barcode_trees = BarcodeManager::from_disk(&args.barcode);

    // read each read from bam file

    bam.records().for_each(|r| {
        match r {
            Ok(record) => {
                processed += 1;
                if processed % 100000 == 0 {
                    eprint!("Processed {} reads\n", processed);
                }

                // find the related barcode based on reference start and end
                let ref_name =
                    String::from_utf8(header_view.tid2name(record.tid() as u32).to_vec())
                        .expect("Can not parse chromosome name from bam record");
                let ref_name = trim_chr_prefix(&ref_name);

                if barcode_trees.has_chrom(ref_name) {
                    if let Some(hits_barcodes) = barcode_trees.find_range(
                        ref_name,
                        record.reference_start() as u32,
                        record.reference_end() as u32,
                    ) {
                        // if hits_barcodes is larger than args.min_barcode_hits, process
                        if hits_barcodes.len() < args.min_barcode_hits {
                            return; // exit the closure for this record
                        }

                        let cigarview = record.cigar();

                        let mut fingerprint = BitBaseFingerPrint::new(&hits_barcodes);

                        for barcode in hits_barcodes {
                            if let Ok(Some(read_pos)) =
                                cigarview.read_pos(barcode.pos - 1, false, false)
                            {
                                let base = record.base(read_pos as usize);
                                fingerprint.add_base(base);
                            } else {
                                warn!("can not process read due to failure on read base");
                                return;
                            }
                        }

                        // add figerprint to haplotype tree 
                    }
                }
            }
            Err(e) => {
                warn!("Error reading bam record: {}\n", e);
                skipped_rec_n += 1;
            }
        }
    });

    eprint!(
        "Skipped {} records due to the out-of-bound issue",
        skipped_rec_n
    );

    let new_path = args.output.with_file_name(format!(
        "{}.freq.txt",
        args.output
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("output")
    ));

    let mut output_composition =
        std::fs::File::create(new_path).expect("Can not create output file");

    output_composition
        .write_all(SNPPileup::get_tbl_header().as_bytes())
        .unwrap();
}
