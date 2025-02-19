# swapfinder


<div style="display: flex; align-items: center;">
    <img src="/img/logo.png" alt="logo" style="width: 150px; margin-right: 20px;">
    <div>
        Sample swaps are an unavoidable issue in sequencing. There are already some tools available for identification, such as <a href="https://pmc.ncbi.nlm.nih.gov/articles/PMC5499645/#ref-list1">NGScheckmate</a>, <a href="https://pubmed.ncbi.nlm.nih.gov/32728101/">Crosscheck</a>, and <a href="https://pubmed.ncbi.nlm.nih.gov/23559639/">idcheck</a>. However, there is still room for improvement in terms of speed, accuracy, and ease of use.
        <br><br>
        <code>swapfinder</code> is a tool for the fast identification of sample swaps. It achieves this by calculating and comparing SNP barcodes of samples.
        <br><br>
        It is inspired by <a href="https://doi.org/10.1016/j.csbj.2023.09.040">CCLHunter</a>, which is a web-based tool for identifying cancer cell lines. <code>swapfinder</code> uses the SNP barcode from the CCLHunter project but provides a command-line tool for users to identify any sample swaps.
    </div>
    </div>
</div>

<!-- Sample swaps are an unavoidable issue in sequencing. There are already some tools available for identification, such as [NGScheckmate](https://pmc.ncbi.nlm.nih.gov/articles/PMC5499645/#ref-list1), [Crosscheck](https://pubmed.ncbi.nlm.nih.gov/32728101/), and [idcheck](https://pubmed.ncbi.nlm.nih.gov/23559639/). However, there is still a room for improvement in terms of speed, accuracy, and ease of use.

`swapfinder` is a tool for the fast identification of sample swaps. It achieves this by calculating and comparing SNP barcodes of samples.

It is inspired by [CCLHunter](https://doi.org/10.1016/j.csbj.2023.09.040), which is a web-based tool for identifying cancer cell lines. `swapfinder` uses the SNP barcode from the CCLHunter project but provides a command-line tool for users to identify any sample swaps. -->


## Features


1. Precisely distinguish samples using hundreds of highly heterogeneous SNP sites.
2. Quickly detect sample swaps by comparing SNP barcodes.
3. Standalone program with no dependencies or installation required.
4. Supports BAM and CRAM formats.
5. Supports multiple distance calculation methods.
6. Supports reading BAM files from URLs.
7. Supports custom SNP barcode lists.

## Installation

1. Ensure you have Rust and Cargo installed. Then, you can clone and build the project with the following commands:

```sh
git clone https://github.com/zhengxinchang/swapfinder.git
cd swapfinder
cargo build --release
```

2. Download the pre-built binaries from the [releases page](https://github.com/zhengxinchang/swapfinder/releases)


## Usage

### Calculate SNP Barcode

You can calculate the SNP barcode for a given sample with the following command:

```sh
swapfinder barcode -b <barcode_file> -i <input_bam_or_cram> -o <output_file> [-r <reference_file>]
```

Parameters:

`-b, --barcode <barcode_file>`: Barcode file

`-i, --bam <input_bam_or_cram>`: Input BAM or CRAM file, **MUST BE SORTED**

`-o, --output <output_file>`: Output file

`-r <reference_file>`: Reference file (only for CRAM format)


### Compare SNP Barcodes
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

# use -i option to specify multiple barcode files
swapfinder compare -i sample1_barcode.txt -i sample2_barcode.txt -o comparison.txt

# use -I option to specify a file containing a list of barcode files
swapfinder compare -I barcode_files.txt -o comparison.txt
```



## SNP Barcode selection criteria

The screening criteria revolve around the ability to be inherited as stably as possible, and the accuracy problems caused by sequence complexity are minimized. 436 SNP sites and corresponding genes were filtered for building the SNP barcode.
1) Each allele should be located within the CDS regions of recognized coding genes.
2) Each allele should be recognized as a biallelic variant, meaning that only two variants (including its reference) could appear in this location, as determined by the dbSNP ALFA project's statistics (build id 20201027095038), and its variation type should be a transversion.
3) The allele frequency of each locus in dbSNP should fall within the range of 0.4–0.6.
4) The allele frequency of each locus in our curated CCLs should also fall within the range of 0.4–0.6.
5) Each locus should not be within the tandem repeat regions identified by the tandem repeat finder with default parameters.
6) Each SNP should not be located within the linkage disequilibrium regions.
7) Only the one farthest from the CDS boundary will be retained if two or more alleles are located on the same coding gene.


## Contributing

Contributions are welcome! Please fork this repository and submit a pull request.

## License
This project is open-source under the MIT license. For more details, please refer to the LICENSE file.

## Citation

If you use `swapfinder` in your research, please cite the following paper:

Congfan Bu, Xinchang Zheng, Jialin Mai, Zhi Nie, Jingyao Zeng, Qiheng Qian, Tianyi Xu, Yanling Sun, Yiming Bao, Jingfa Xiao, CCLHunter: An efficient toolkit for cancer cell line authentication, Computational and Structural Biotechnology Journal, 2023, https://doi.org/10.1016/j.csbj.2023.09.040.
