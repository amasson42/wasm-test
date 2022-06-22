
all: build

build:
	wasm-pack build

run:
	cd www && npm run start
