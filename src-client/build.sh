cargo build --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/blake3_wasm_nopackage.wasm ../assets/
cp js/* ../assets/
