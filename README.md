# Efficient binary data transfer between a host and WebAssembly application: SHA256 PoC
This uses Wasm component model Resources (which internally use safe externrefs) as handles to files in host-memory.
Wasm applications can then call methods such as `read_byte() -> u8` or `read_max_n_bytes(n) -> list<u8>` on the resource.

This proof of concept uses Wasm to calculate the SHA256 hash of a file provided by the host application.

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
- `sha256sum`: 10.49s
- Rust native(opt-level=O3): 8.17s
- Wasm (opt-level=S): 25.65s
- Wasm (opt-level=O3): 26.08s

_Note: Both Rust and Wasm programs read data in 2^24 byte chunks from the file to make both of them comparable_

