all: format lint test doc

d:
	cargo watch -c -s 'make all'

format:
	cargo fmt

lint:
	cargo clippy --all-targets

test:
	cargo tarpaulin

doc:
	cargo doc
