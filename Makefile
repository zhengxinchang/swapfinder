
build:
	cargo build --target=x86_64-unknown-linux-musl --release
	cargo build --release --target=x86_64-unknown-linux-gnu

push:
	git add . && git commit -m "update" && git push

hg002:
	cargo run -- estcon  -b test/chr22.cont_barcode.txt  -i test/HG002_HIFI.GRCh38.sorted.bam -o test/hg002_chr22

hg005:
	cargo run -- estcon  -b test/chr22.cont_barcode.txt  -i test/HG005_GRCh38_ONT-UL_chr22.bam  -o test/hg005_chr22