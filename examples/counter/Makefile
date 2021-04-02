fix:
	cargo +nightly fix && fmt

fmt:
	cargo +nightly fmt

macro_trace:
	RUSTFLAGS="-Z macro-backtrace" cargo +nightly build

macro_expand:
	cargo expand > macro_expand_output.rs

release:
	cargo build --release

run_release:
	./target/release/counter
