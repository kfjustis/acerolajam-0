Example:

```
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --target web target/wasm32-unknown-unknown/release/bevy-jam-4.wasm --out-dir wasm-out
basic-http-server wasm-out/
```

Custom:

```
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --no-typescript --target web --out-dir ./out/ --out-name "acerolajam-0" ./target/wasm32-unknown-unknown/release/acerolajam-0.wasm
basic-http-server out/
```

Notes:

 * `index.html` template here: https://bevy-cheatbook.github.io/platforms/wasm/webpage.html
 * official 0.13.0 template here: https://github.com/bevyengine/bevy/blob/release-0.13.0/examples/wasm/index.html
 * update `index.html` to load `my_game.js`
 * copy `./assets` into `./out/assets`
 * only target web/wasm with `webgl2` flag, otherwise it defaults to WebGPU