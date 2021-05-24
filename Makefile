doc:
	cargo watch -- cargo doc --features web

web:
	gxib -d examples/web web

desktop:
	gxib -d examples/desktop desktop

test:
	cd tests && cargo test