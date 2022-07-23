
all: run

setup:
	@cargo version || curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
	@wasm-pack -V || curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
	@trunk --version || cargo install --locked trunk
	@wasm-bindgen -V || cargo install wasm-bindgen-cli

run:
	sass --watch scss:styles --sourcemap=none & trunk serve && fg

build:
	sass scss/styles.scss styles/styles.css --style compressed
#--sourcemap=none
	trunk build --release

clean:
	cargo clean
	trunk clean
	rm -fr styles .sass-cache

.PHONY: setup run build
