#[allow(warnings)]
mod bindings;

use bindings::Guest;
use sha2::{Digest, Sha256};

struct Component;

impl Guest for Component {
    fn calculate_sha256(r: &bindings::MemoryReader) -> String {
        let mut hasher = Sha256::new();

        while let Ok(data) = r.read_max_n_bytes(2u32.pow(24)) {
            hasher.update(&data);
        }

        let hash = hasher.finalize();

        format!("{hash:02X?}")
    }
}

bindings::export!(Component with_types_in bindings);
