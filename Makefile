.PHONY: release doc d dev denite test

release:
	cargo build --release

denite: release
	ln -sf denite/* .

dev: format lint doc test

d:
	cargo watch -c -s 'make dev'

format:
	cargo fmt

lint:
	cargo clippy --all-targets

test:
	cargo build
	cargo test --all-targets

doc:
	cargo doc
