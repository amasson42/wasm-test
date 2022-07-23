
all: run

setup:
    @cargo version || curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    @wasm-pack -V || curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
	@trunk --version || cargo install --locked trunk
	@wasm-bindgen -V || cargo install wasm-bindgen-cli

run:
	trunk serve

build:
	trunk build --release

clean:
	cargo clean
	trunk clean

.PHONY: setup run build
