# Efficient binary data transfer between a host and WebAssembly application: SHA256 PoC
This uses Wasm component model Resources (which internally use safe externrefs) as handles to files in memory.

1. Generate data from inside `./random-file` (by default 4GB):
  ```sh
  cargo run --release
  mv dummy-data.bin ..
  ```
2. Build Wasm app inside `./app` with:
  ```sh
  cargo component build --release --target wasm32-unknown-unknown
  ```
3. Run host app inside `./host` with:
  ```sh
  cargo run --release
  ```


## Some results on my system
Rust native: 8.17
`sha256sum`: 10.49
Wasm S: 25.65
Wasm O3: 26.08

Note: Both Rust and Wasm programs read data in 2^24 byte chunks from the file

