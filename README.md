
# Build for wasm using

1. `rustup target install wasm32-unknown-unknown`
2. `cargo install -f wasm-bindgen-cli`
3. `cargo build --target wasm32-unknown-unknown --release`
4. `wasm-bindgen --out-dir ./out/ --target web ./target/wasm32-unknown-unknown/release/travel-jam.wasm`

# Run with 
cargo run --features bevy/dynamic_linking

Built for the Mini Jam 154: Travel.

I worked on creating a reusable template for bevy games before the jam, but all game specific code was created within 72 hours