extern crate wasmer_runtime;

use std::str;
use std::fs;

use wasmer_runtime::{
    imports,
    instantiate,
    func,
    Ctx,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let import_object = imports! {
        "env" => {
            "print_str" => func!(print_str),
        },
    };

    let wasm = &fs::read("../wasm-sample-app/target/wasm32-unknown-unknown/release/wasm_sample_app.wasm")?;
    let instance = instantiate(wasm, &import_object)?;

    instance.call("hello_wasm", &[])?;

    Ok(())
}

// Let's define our "print_str" function.
//
// The declaration must start with "extern" or "extern "C"".
fn print_str(ctx: &mut Ctx, ptr: u32, len: u32) {
    // Get a slice that maps to the memory currently used by the webassembly
    // instance.
    //
    // Webassembly only supports a single memory for now,
    // but in the near future, it'll support multiple.
    //
    // Therefore, we don't assume you always just want to access first
    // memory and force you to specify the first memory.
    let memory = ctx.memory(0);

    // Get a subslice that corresponds to the memory used by the string.
    let str_vec: Vec<_> = memory.view()[ptr as usize..(ptr + len) as usize].iter().map(|cell| cell.get()).collect();

    // Convert the subslice to a `&str`.
    let string = str::from_utf8(&str_vec).unwrap();

    // Print it!
    println!("Printed from wasm: {}", string);
}
