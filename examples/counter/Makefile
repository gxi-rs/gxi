fix:
	cargo +nightly fix && fmt

fmt:
	cargo +nightly fmt

macro_trace:
	RUSTFLAGS="-Z macro-backtrace" cargo +nightly build

macro_expand:
	cargo expand > my_app_clone.rs
