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

# Calculate SNP Barcode

You can calculate the SNP barcode for a given sample with the following command:

```sh
swapfinder barcode -b <barcode_file> -i <input_bam_or_cram> -o <output_file> [-r <reference_file>]
```

Parameters:

`-b, --barcode <barcode_file>`: Barcode file

`-i, --bam <input_bam_or_cram>`: Input BAM or CRAM file, **MUST BE SORTED**

`-o, --output <output_file>`: Output file

`-r <reference_file>`: Reference file (only for CRAM format)


## Compare SNP Barcodes
You can compare SNP barcodes of multiple samples with the following command:

```sh
swapfinder compare -i <barcode_file1> -i <barcode_file2> -o <output_file>

swapfinder compare -I <barcode_files> -o <output_file>
```

Parameters:

`-i, --barcode <barcode_file>`: Barcode file, can specify multiple

`-I, --barcode_files <barcode_files>`: File containing a list of barcode files, mutrually exclusive with `-i`

`-o, --output <output_file>`: Output file

## Examples
Calculate SNP Barcode


```sh

# barcodes.txt can be found at barcodes/ directory with different reference version.
swapfinder barcode -b barcodes.txt -i sample1.bam -o sample1_barcode.txt -r reference.fa
```
Compare SNP Barcodes

```sh

# use -b option to specify multiple barcode files
swapfinder compare -i sample1_barcode.txt -i sample2_barcode.txt -o comparison.txt

# use -B option to specify a file containing a list of barcode files
swapfinder compare -I barcode_files.txt -o comparison.txt
```


## Contributing

Contributions are welcome! Please fork this repository and submit a pull request.

## License
This project is open-source under the MIT license. For more details, please refer to the LICENSE file.

## Citation

If you use `swapfinder` in your research, please cite the following paper:

Congfan Bu, Xinchang Zheng, Jialin Mai, Zhi Nie, Jingyao Zeng, Qiheng Qian, Tianyi Xu, Yanling Sun, Yiming Bao, Jingfa Xiao, CCLHunter: An efficient toolkit for cancer cell line authentication, Computational and Structural Biotechnology Journal, 2023, https://doi.org/10.1016/j.csbj.2023.09.040.

