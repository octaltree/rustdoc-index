release:
	cargo build --release

denite: release
	sh -c 'cp -r denite/* .'

dev: format lint test doc

d:
	cargo watch -c -s 'make dev'

format:
	cargo fmt

lint:
	cargo clippy --all-targets

test:
	cargo test

doc:
	cargo doc
