
# export CC_x86_64_unknown_linux_gnu=clang

format:
	cargo fmt --all

style-check:
	cargo fmt --all -- --check

debug:
	cargo build
	
release:
	cargo build --release

package:
	cargo build --release --target x86_64-unknown-linux-gnu
	cargo build --release --target x86_64-unknown-linux-musl
	cargo build --release --target x86_64-pc-windows-gnu
	cargo build --release --target x86_64-apple-darwin
	cargo build --release --target aarch64-apple-darwin

publish:
	go-semantic-release --hooks goreleaser --show-progress
