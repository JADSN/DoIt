all: build run

build:
	cargo build --release

run:
	./target/release/app