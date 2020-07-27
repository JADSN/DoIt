all: build run

build:
	cargo build --release

run:
	./target/release/app

dist:
	mkdir -p dist
	cp ./target/release/app ./dist/.
	cp -r ./client/static*  ./dist/.
	cd ./dist ./app