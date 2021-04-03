fix:
	cargo +nightly fix && fmt

fmt:
	cargo +nightly fmt

macro_trace:
	RUSTFLAGS="-Z macro-backtrace" cargo +nightly build

macro_expand:
	cargo +nightly expand > macro_expand_output.rs

run_release:
	cargo run --release

run:
	GTK_DEBUG=interactive cargo run