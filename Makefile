args:=

doc:
	cargo watch -- cargo doc --features web $(args)

web:
	gxib -d examples/web web $(args)

serve:
	make web args="-wrs localhost:8080"

desktop:
	gxib -d examples/desktop desktop $(args)

test:
	cd gxi_macro/tests && cargo test $(args)