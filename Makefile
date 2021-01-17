
lint:
	cargo fmt
test:
	RUST_BACKTRACE=1 RUST_LOG=map_store=trace cargo test -- --nocapture --test-threads=1

