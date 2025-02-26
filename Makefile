
build:
	cargo build --target=x86_64-unknown-linux-musl --release
	cargo build --release --target=x86_64-unknown-linux-gnu

push:
	git add . && git commit -m "update" && git push