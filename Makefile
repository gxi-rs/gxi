fix:
	cargo fix && fix

fmt:
	cargo +nightly fmt

macro_trace:
	cargo +nightly -- -Zmacro-backtrace run