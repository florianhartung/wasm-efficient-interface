use std::{
    any::Any,
    borrow::{Borrow, BorrowMut},
    collections::HashMap,
};

use bindings::myinterface::myinterface::types::HostMemoryReader;
use memory_reader::MemoryReader;
use memory_reader_import_impl::MemoryReaders;
use wasmtime::{
    component::{Component, Linker, Resource},
    AsContext, Engine, Store, StoreLimits,
};

mod bindings;
mod memory_reader;
mod memory_reader_import_impl;

struct StoreData {
    memory_readers: MemoryReaders,
}

fn main() -> anyhow::Result<()> {
    // First we load our Wasm component and setup its interfaces.
    let (mut store, bindings) = init_wasm_app()?;

    // Now we can finally start to call our Wasm component and give it some data to process.
    // In this simple example we load a large file from disk (`../dummy-data.bin`) and then let Wasm calculate its SHA256 hash.

    // First create a resource
    println!("Reading data...");
    let data = std::fs::read("../dummy-data.bin")?;
    let memory_reader = MemoryReader::from_vec(data);
    let memory_reader_resource = store.data_mut().memory_readers.new_reader(memory_reader);

    // Then call the component with a borrowed resource
    println!("Calling Wasm...");
    let result = bindings.call_calculate_sha256(
        &mut store,
        Resource::new_borrow(memory_reader_resource.rep()),
    )?;
    println!("result is {result}");

    // And then drop the host resource to make space
    store
        .data_mut()
        .memory_readers
        .drop(memory_reader_resource)?;

    Ok(())
}

fn init_wasm_app() -> anyhow::Result<(Store<StoreData>, bindings::CommonWorld)> {
    // Create a linker and export all functions that are available for importing for Wasm components
    let mut linker = Linker::new(&Engine::default());
    bindings::CommonWorld::add_to_linker(&mut linker, |store_data: &mut StoreData| {
        &mut store_data.memory_readers
    })?;

    // Load our `app` Wasm component
    let component = {
        // let component_bytes = include_bytes!("../../app/target/wasm32-wasip1/release/app.wasm");
        let component_bytes =
            include_bytes!("../../app/target/wasm32-unknown-unknown/release/app.wasm");
        Component::new(linker.engine(), component_bytes)?
    };

    // Create a store and bindings for the components exports
    let mut store = Store::new(
        linker.engine(),
        StoreData {
            memory_readers: MemoryReaders::new(),
        },
    );
    let bindings = {
        let instance = linker.instantiate(&mut store, &component)?;
        bindings::CommonWorld::new(&mut store, &instance)?
    };

    Ok((store, bindings))
}
