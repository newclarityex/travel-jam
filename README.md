
# Build for wasm using

1. `rustup target install wasm32-unknown-unknown`
2. `cargo install -f wasm-bindgen-cli`
3. `cargo build --target wasm32-unknown-unknown --release`
4. `wasm-bindgen --out-dir ./out/ --target web ./target/wasm32-unknown-unknown/release/travel-jam.wasm`

cargo run --features bevy/dynamic_linking
