# swapfinder


## Project Overview

`swapfinder` is a tool for the fast identification of sample swaps. It achieves this by calculating and comparing SNP barcodes of samples.

It is inspired by [CCLHunter], which is a web-based tool for identifying cancer cell lines. `swapfinder` uses the SNP barcode from the CCLHunter project but provides a command-line tool for users to identify any sample swaps.


## Features

- Calculate SNP barcode for a given sample
- Compare SNP barcodes of multiple samples

## Installation

1. Ensure you have Rust and Cargo installed. Then, you can clone and build the project with the following commands:

```sh
git clone https://github.com/zhengxinchang/swapfinder.git
cd swapfinder
cargo build --release
```

2. Download the pre-built binaries from the [releases page](https://github.com/zhengxinchang/swapfinder/releases)


## Usage
Calculate SNP Barcode
You can calculate the SNP barcode for a given sample with the following command:

```sh
swapfinder barcode -B <barcode_file> -b <input_bam_or_cram> -o <output_file> [--reference <reference_file>]
```

Parameters:

`-B, --barcode <barcode_file>`: Barcode file

`-b, --bam <input_bam_or_cram>`: Input BAM or CRAM file, **MUST BE SORTED**

`-o, --output <output_file>`: Output file

`--reference <reference_file>`: Reference file (only for CRAM format)


## Compare SNP Barcodes
You can compare SNP barcodes of multiple samples with the following command:

```sh
swapfinder compare -b <barcode_file1> -b <barcode_file2> -o <output_file>

swapfinder compare -B <barcode_files> -o <output_file>
```

Parameters:

`-b, --barcode <barcode_file>`: Barcode file, can specify multiple

`-B, --barcode_files <barcode_files>`: File containing a list of barcode files, mutrually exclusive with `-b`

`-o, --output <output_file>`: Output file

## Examples
Calculate SNP Barcode


```sh

# barcodes.txt can be found at barcodes/ directory with different reference version.
swapfinder barcode -B barcodes.txt -b sample1.bam -o sample1_barcode.txt --reference reference.fa
```
Compare SNP Barcodes

```sh

# use -b option to specify multiple barcode files
swapfinder compare -b sample1_barcode.txt -b sample2_barcode.txt -o comparison.txt

# use -B option to specify a file containing a list of barcode files
swapfinder compare -B barcode_files.txt -o comparison.txt
```


## Contributing

Contributions are welcome! Please fork this repository and submit a pull request.

## License
This project is open-source under the MIT license. For more details, please refer to the LICENSE file.

## Citation

If you use `swapfinder` in your research, please cite the following paper:

Congfan Bu, Xinchang Zheng, Jialin Mai, Zhi Nie, Jingyao Zeng, Qiheng Qian, Tianyi Xu, Yanling Sun, Yiming Bao, Jingfa Xiao, CCLHunter: An efficient toolkit for cancer cell line authentication, Computational and Structural Biotechnology Journal, 2023, https://doi.org/10.1016/j.csbj.2023.09.040.

