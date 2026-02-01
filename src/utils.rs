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

pub fn is_url(s: &str) -> bool {
    s.starts_with("http://")
        || s.starts_with("https://")
        || s.starts_with("s3://")
        || s.starts_with("ftp://")
        || s.starts_with("sftp://")
}

pub fn phredu8_to_prob(phredu8: u8) -> f64 {
    10.0_f64.powf(-1.0 * phredu8 as f64 / 10.0)
}

pub fn trim_chr_prefix(s: &str) -> &str {
    // consider the case senstive
    let s2 = s.to_uppercase();
    if s2.starts_with("CHR") {
        &s[3..]
    } else {
        s
    }
}
