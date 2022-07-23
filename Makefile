
all: build

build:
	wasm-pack build

run:
	cd www && npm run start

test:
	wasm-pack test --firefox --headless
