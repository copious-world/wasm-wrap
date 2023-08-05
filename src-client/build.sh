cargo build --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/wasm_nopackage.wasm ../assets/
cp js/* ../assets/
