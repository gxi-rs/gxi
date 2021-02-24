all: rust_lib main run

rust_lib:
	cd rust_lib && cargo build --release

main:
	cargo build

run:
	cargo run