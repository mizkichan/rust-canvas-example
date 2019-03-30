.PHONY: debug release

debug: src/lib.rs
	wasm-pack build --debug --target web

release: src/lib.rs
	wasm-pack build --target web
