#!/usr/bin/env python3
import argparse
def main(args):
    with open(args.barcode) as f, open(args.output, 'w') as out:
        prev_pos = None
        block = 0
        block_ended = False
        block_line = []
        for line in f:
            pos = int(line.split()[1])
            if prev_pos is not None:
                if (pos - prev_pos) > int(args.min_dist):
                        prev_pos = pos
                        block_ended = True
                else:

                    block_line.append(line.strip())

                    if block_ended:
                        if len(block_line) >= int(args.min_snps):
                            for bline in block_line:
                                out.write(bline+"|block="+str(block)+"\n")
                            block += 1
                        block_line.clear()
                        block_ended = False
                        
                    prev_pos = pos
            else:
                prev_pos = pos



if __name__ == "__main__":
    parser = argparse.ArgumentParser(description='Process some integers.')
    parser.add_argument('-b','--barcode',type=str, help='path to the contamination barcode file')
    parser.add_argument('-o','--output',type=str, help='path to the output file')
    parser.add_argument('-m','--min_dist',type=str, help='minimal distance between the prev and next SNP',default='200')
    parser.add_argument('-n','--min_snps',type=str, help='minimal number of SNPs in a block', default='5')
    args = parser.parse_args()
    main(args)