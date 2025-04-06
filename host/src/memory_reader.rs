pub struct MemoryReader {
    data: Vec<u8>,
    current_index: usize,
}

impl MemoryReader {
    pub fn from_vec(vec: Vec<u8>) -> Self {
        Self {
            data: vec,
            current_index: 0,
        }
    }

    pub fn get_num_remaining_bytes(&self) -> usize {
        self.data.len() - self.current_index
    }

    pub fn read(&mut self) -> Option<u8> {
        let byte = *self.data.get(self.current_index)?;
        self.current_index += 1;
        Some(byte)
    }

    pub fn read_into(&mut self, buf: &mut [u8]) -> usize {
        let num_bytes_to_copy = self.get_num_remaining_bytes().min(buf.len());
        buf[..num_bytes_to_copy].copy_from_slice(
            &self.data[self.current_index..(self.current_index + num_bytes_to_copy)],
        );
        self.current_index += num_bytes_to_copy;

        num_bytes_to_copy
    }

    pub fn read_n(&mut self, n: usize) -> Option<Vec<u8>> {
        let num_bytes_to_read = self.get_num_remaining_bytes().min(n);

        if num_bytes_to_read == 0 {
            return None;
        }

        println!("Sending {num_bytes_to_read}b to Wasm...");
        let data = Some(
            self.data[self.current_index..(self.current_index + num_bytes_to_read)].to_owned(),
        );
        self.current_index += num_bytes_to_read;
        data
    }
}
