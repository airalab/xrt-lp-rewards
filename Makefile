wasm:
	cargo build --release --target wasm32-unknown-unknown --no-default-features --features browser
	wasm-bindgen target/wasm32-unknown-unknown/release/xrt_lp_rewards.wasm --out-dir pkg --target web

clean:
	rm -rf pkg

web:
	python -m http.server 8000
