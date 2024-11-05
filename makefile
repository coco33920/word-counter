dev:
	cargo build
release:
	cargo build --release
run:
	cargo run
watch:
	cargo watch -x check -x fmt -x run
test:
	cargo test
