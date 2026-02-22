[X] add sample name when generating barcode file
[X] support read bam from URL
[ ] support multiple distance methods
[X] swap columns in the barcode.tsv files
[X] assign the NN to missing GT
[X] deal with the low coverage(set a parameter)
[X] estimate genotype instead of using hard threshold
[ ] html report 
[ ] change pval to -log10p
[ ] change the log likelihood ratio with sigmod or tanh function.
use multiple distance methods to meature the distance between the barcodes



# contaimination detection:


一个fingerprint 首先查询到hp，hp之间允许重叠

所有的hp 检查finterprint， 找到最佳匹配的hp；规则是： 1 overlap 必须完全一样， overlap越大越好； 2 如果没有overlap，则生成新的hp

======================================================
 *******************************hp1
    ***********************hp2 
                                     ***************hp3
                     ******************hp4



运行所有的overlap之后，进行haplotype的合并，根据其相似性合并，最终进行purge

