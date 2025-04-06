use std::io::{BufWriter, Write};

fn main() {
    let mut file = BufWriter::new(std::fs::File::create("dummy-data.bin").unwrap());

    let mut buf = [0u8; 2usize.pow(16)];
    for _ in 0..(2i64.pow(16)) {
        rand::fill(&mut buf);
        file.write(&buf).unwrap();
    }
}
