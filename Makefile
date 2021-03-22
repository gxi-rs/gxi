fix:
	cargo fix && fix

fmt:
	cargo +nightly fmt

macro_trace:
	RUSTFLAGS="-Z macro-backtrace" cargo +nightly build

macro_expand:
	cargo expand
