args:=

doc:
	cargo watch -- cargo doc --features web $(args)

web:
	gxib -d examples/web web $(args)

desktop:
	gxib -d examples/desktop desktop $(args)

test:
	cd tests && cargo test $(args)