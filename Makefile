args:=

doc:
	cargo watch -- cargo doc --features web $(args)

web:
	gxib -d examples/web web $(args)

serve:
	make web args="-p examples/web/public -wrs localhost:8080 $(args)"

desktop:
	gxib -d examples/desktop desktop $(args)

test:
	cargo test $(args)

release:
	cargo release $(args) --workspace --exclude desktop --exclude tests --exclude web

clean:
	cargo clean
