use std::mem;

use anyhow::Result;
use wasmtime::*;

fn main() -> Result<()> {
    // let engine = Engine::default();

    let mut store = Store::<()>::default();

    let module = Module::from_file(
        store.engine(),
        "../data-exp/target/wasm32-unknown-unknown/release/data_exp.wasm",
    )?;

    let instance = Instance::new(&mut store, &module, &[])?;

    let sum_func = instance.get_typed_func::<(i32, i32), i64, _>(&mut store, "sum")?;

    let memory = instance
        .get_memory(&mut store, "memory")
        .ok_or(anyhow::format_err!("failed to find `memory` export"))?;

    
    // this will introduce unexpected problems! 
    // let offset = 0;

    let offset = memory.data_size(&store) as _;

    println!("DEBUG: memory size in byte: {:?}", offset);


    // if we put the data on "growed" memory, this region should not be compromised
    memory.grow(&mut store, 200)?;

    let array = [1u8; 1048576];


    memory.write(&mut store, offset, &array)?;

    let res = sum_func.call(&mut store, (offset as _, array.len() as _ ))?;
    println!("Reuslt: {:?}", res);

    Ok(())
}
