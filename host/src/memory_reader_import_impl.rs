use std::collections::HashMap;

use crate::bindings;
use wasmtime::component::Resource;

use crate::memory_reader::MemoryReader;

#[derive(Default)]
pub struct MemoryReaders {
    readers: HashMap<u32, MemoryReader>,
    next_key: u32,
}

impl MemoryReaders {
    pub fn new() -> Self {
        Self {
            readers: HashMap::new(),
            next_key: 0,
        }
    }

    pub fn new_reader(&mut self, memory_reader: MemoryReader) -> Resource<bindings::MemoryReader> {
        let key_representation = self.next_key;
        self.next_key += 1;

        self.readers.insert(key_representation, memory_reader);
        Resource::new_own(key_representation)
    }
}

impl bindings::myinterface::myinterface::types::Host for MemoryReaders {}

impl bindings::myinterface::myinterface::types::HostMemoryReader for MemoryReaders {
    fn read_byte(
        &mut self,
        resource: wasmtime::component::Resource<bindings::MemoryReader>,
    ) -> Result<u8, ()> {
        self.readers
            .get_mut(&resource.rep())
            .expect("reader to exist, as it was referenced by a resource's representation")
            .read()
            .ok_or(())
    }

    fn drop(
        &mut self,
        self_: wasmtime::component::Resource<bindings::MemoryReader>,
    ) -> wasmtime::Result<()> {
        let _memory_reader = self
            .readers
            .remove(&self_.rep())
            .expect("reader with this rep to exist, because it is dropped now");

        Ok(())
    }

    fn read_max_n_bytes(
        &mut self,
        resource: wasmtime::component::Resource<bindings::MemoryReader>,
        n: u32,
    ) -> Result<Vec<u8>, ()> {
        self.readers
            .get_mut(&resource.rep())
            .expect("reader to exist, as it was referenced by a resource's representation")
            .read_n(n.try_into().unwrap())
            .ok_or(())
    }
}
