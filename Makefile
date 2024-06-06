.PHONY: debug release

format:
	cargo fmt --all

style-check:
	cargo fmt --all -- --check

debug:
	cargo build
	
release:
	cargo build --release

publish:
	npx semantic-release

install: release
	cp target/release/aws-assume-role ~/.cargo/bin/aws-assume-role
