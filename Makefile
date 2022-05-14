args:=

expand_web_example:
	cd examples/web && cargo expand | bat -l rust

create_container:
	podman create \
		--name gxi \
		-p 8080:8080 \
		-v $(shell pwd):/gxi:Z \
       	-it ghcr.io/gxi-rs/gxib:latest

start_container:
	podman start gxi -a

doc:
	cargo watch -- cargo doc --features web $(args)

web:
	gxib -d examples/web web $(args)

serve:
	make web args="-p examples/web/public -wrs 0.0.0.0:8080 $(args)"

desktop:
	gxib -d examples/desktop desktop $(args)

test:
	RUST_BACKTRACE=full RUST_LIB_BACKTRACE=0 cargo test --workspace --lib $(args)
	cd examples/web && cargo check

release:
	cargo release --workspace --sign --exclude example $(args) 

clean:
	cargo clean
