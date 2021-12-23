args:=

docker:
	docker run \
		-p 8080:8080 \
       	-v $(shell pwd):/app \
       	-it ghcr.io/gxi-rs/gxib:latest

doc:
	cargo watch -- cargo doc --features web $(args)

web:
	gxib -d examples/web web $(args)

serve:
	make web args="-p examples/web/public -wrs 0.0.0.0:8080 $(args)"

desktop:
	gxib -d examples/desktop desktop $(args)

test:
	cargo test --workspace --lib $(args)

release:
	cargo release --workspace --sign --exclude example $(args)

clean:
	cargo clean
