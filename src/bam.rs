//! This file is used to parse the BAM file, fetch reads that located in a given region

use std::{env, io::Write};

use rust_htslib::bam::{self, Read as BAMRead};

use crate::{barcode::BarcodeList, cli::BarcodeArgs};

use crate::genotype::{self};
use crate::likelihood::ReadBase;
use crate::likelihood::{self, calculate_likelihoods_all_genotype};
use statrs::distribution::{ChiSquared, ContinuousCDF};
use url::Url;

pub fn check_file_format(path: &str) -> Result<String, std::io::Error> {
    // check the suffix of the file
    let suffix = path.split('.').last().unwrap();
    let suffix = suffix.to_uppercase();

    eprint!("processing file format: {}\n", suffix);

    // check if the file is a bam file or cram file
    if suffix == "BAM" {
        Ok("BAM".to_string())
    } else if suffix == "CRAM" {
        Ok("CRAM".to_string())
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Unknown file format",
        ))
    }
}

fn is_url(s: &str) -> bool {
    s.starts_with("http://")
        || s.starts_with("https://")
        || s.starts_with("s3://")
        || s.starts_with("ftp://")
}

pub fn calculate_barcode(args: BarcodeArgs) {
    let mut barcode_list = BarcodeList::new();
    barcode_list.load(args.barcode.to_str().unwrap());

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

    let mut base_vec = Vec::new();
    for (idx, barcode) in barcode_list.barcode_list.clone().iter().enumerate() {
        eprint!(
            "fetching barcode(#{}): {}:{}\n",
            idx + 1,
            barcode.chrom,
            barcode.pos
        );

        let query_pos = if has_chr {
            format!("chr{}:{}-{}", barcode.chrom, barcode.pos - 1, barcode.pos)
        } else {
            format!("{}:{}-{}", barcode.chrom, barcode.pos - 1, barcode.pos)
        };

        bam.fetch(&query_pos).unwrap();
        let mut gt = genotype::Genotype::new_nn();
        // dbg!("----------");
        for plup in bam.pileup() {
            let plup = plup.unwrap_or_else(|err| panic!("Can not read pipeup,due to {}", err));

            let tname = match has_chr {
                true => String::from_utf8_lossy(
                    header_view
                        .tid2name(plup.tid())
                        .strip_prefix(b"chr")
                        .unwrap(),
                ),
                false => String::from_utf8_lossy(header_view.tid2name(plup.tid())),
            };

            if tname == barcode.chrom && plup.pos() == barcode.pos - 1 {
                // dbg!(&plup);
                let bases: Vec<ReadBase> = plup
                    .alignments()
                    .filter_map(|alignment| {
                        // ignore the secondary and supplementary alignments incase the seq() is emtpy and the slice will panic
                        if alignment.record().is_secondary()
                            || alignment.record().is_supplementary()
                        {
                            return None;
                        }

                        alignment.qpos().map(|qpos| {
                            // dbg!(&qpos);
                            ReadBase {
                                base: alignment.record().seq()[qpos] as char,
                                phred: alignment.record().qual()[qpos] as f64,
                            }
                        })
                    })
                    .collect();

                if bases.len() >= args.min_depth {
                    // println!("{:?}",&bases);
                    let mut gt_likelihoods = calculate_likelihoods_all_genotype(bases);
                    gt_likelihoods.sort_by(|a, b| {
                        let cmp = b.likelihood.partial_cmp(&a.likelihood).unwrap();
                        cmp
                    });

                    let mut best_gt = gt_likelihoods[0].clone();
                    let second_gt = gt_likelihoods[1].clone();

                    // here we use the second_gt as the null hypotheisis to make the LRT statistic positive.
                    let log_likelihood = (best_gt.likelihood / second_gt.likelihood).ln();

                    let test_statistic = 2.0 * log_likelihood;

                    // degree of freedom is 1 for the LRT test
                    let chi_squared = ChiSquared::new(1.0).unwrap();

                    let p_value = 1.0 - chi_squared.cdf(test_statistic);
                    // dbg!(&p_value);
                    best_gt.pval = p_value;
                    best_gt.lrt = log_likelihood;

                    gt = best_gt;
                }

                break;
            }
        }
        base_vec.push(gt);
    }

    let mut output: Box<dyn std::io::Write> = match args.output {
        Some(output_file) => Box::new(std::io::BufWriter::new(
            std::fs::File::create(output_file).unwrap(),
        )),
        None => Box::new(std::io::BufWriter::new(std::io::stdout())),
    };

    let filename = args.bam.file_name().unwrap().to_str().unwrap();

    if let Some(sample) = &args.sample_name {
        output
            .write_all(format!("#{}\n", sample).as_bytes())
            .unwrap();
    } else {
        output
            .write_all(format!("#{}\n", filename).as_bytes())
            .unwrap();
    }

    output
        .write_all(b"#chrom\tpos\tgenotype\tlog_likelihood_ratio\tpval\tmeta\n")
        .unwrap();

    for (barcode, genotype) in barcode_list.barcode_list.iter().zip(base_vec.iter()) {
        output
            .write_all(
                format!(
                    "{}\t{}\t{}\t{}\t{}\t{}\n",
                    barcode.chrom,
                    barcode.pos,
                    genotype,
                    likelihood::normalization(genotype.lrt),
                    genotype.pval,
                    barcode.meta
                )
                .as_bytes(),
            )
            .unwrap();
    }
}
