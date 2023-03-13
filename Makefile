
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
