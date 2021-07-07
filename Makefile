args:=

doc:
	cargo watch -- cargo doc --features web $(args)

web:
	gxib -d gxi-web/example web $(args)

serve:
	make web args="-p gxi-web/example/public -wrs localhost:8080"

desktop:
	gxib -d examples/desktop desktop $(args)

test:
	cargo test $(args)

release:
	cargo release $(args) --workspace --exclude desktop --exclude tests --exclude web

clean:
	cargo clean