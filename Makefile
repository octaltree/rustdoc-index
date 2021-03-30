release:
	cargo build --release

dev: format lint test doc

d:
	cargo watch -c -s 'make dev'

format:
	cargo fmt

lint:
	cargo clippy --all-targets

test:
	cargo tarpaulin

doc:
	cargo doc
