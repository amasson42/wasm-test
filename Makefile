
all: build

setup:
# apt install build-essential nodejs npm
	@cargo version || curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
	@wasm-pack -V || curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
	@cargo generate -V || cargo install cargo-generate

build:
	wasm-pack build
	# cd www && npm install

run:
	cd www && npm run start

test:
	wasm-pack test --firefox --headless
