use std::io::{BufReader, Read};

use sha2::{Digest, Sha256};

fn main() {
    let mut data = std::fs::File::open("../dummy-data.bin").unwrap();

    let mut hasher = Sha256::new();

    let mut buffer = vec![0u8; 2usize.pow(24)];
    loop {
        let n = data.read(&mut buffer).unwrap();
        if n == 0 {
            break;
        }

        hasher.update(&buffer[..n]);
    }

    let hash = hasher.finalize();

    println!("{hash:02X?}");
}
