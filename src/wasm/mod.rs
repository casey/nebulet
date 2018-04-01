//! The wasm compiler and runtime
//! Uses Cretonne as the compiler
//! 
//! Some code is taken from wasmstandalone
//! - https://github.com/sunfishcode/wasmstandalone

pub mod runtime;

use cton_wasm::translate_module;
use cton_native;
use self::runtime::{Instance, Module, ModuleEnvironment};
use cretonne::{result::CtonError, isa::TargetIsa};
use cretonne::settings::{self, Configurable};
use memory::Code;

use nabi::{Result, Error};

use alloc::Vec;

pub fn wasm_test() {
    let codes: Vec<Result<Code>> = WASM_TESTS
        .iter()
        .map(|wasm| compile(*wasm))
        .collect();

    for (i, code_opt) in codes.iter().enumerate() {
        match code_opt {
            Ok(code) => {
                println!("Executing wasm test #{}", i);
                code.execute();
            },
            Err(err) => {
                println!("Wasm test #{} failed to compile: {:?}", i, err);
            }
        }
    }
    println!("Didn't crash!");
}

pub fn compile(wasm: &[u8]) -> Result<Code> {
    let (mut flag_builder, isa_builder) = cton_native::builders()
        .expect("Host machine not supported.");

    flag_builder.set("opt_level", "best")
        .map_err(|_| Error::INTERNAL)?;

    let isa = isa_builder.finish(settings::Flags::new(&flag_builder));

    let mut module = Module::new();
    let mut environ = ModuleEnvironment::new(isa.flags(), module);

    translate_module(wasm, &mut environ)
        .map_err(|_| Error::INTERNAL)?;
    
    let translation = environ.finish_translation();
    let compliation = translation.compile(&*isa)?;

    Ok(compliation.emit())
}

static WASM_TESTS: [&'static [u8]; 6] = [
    include_bytes!("wasmtests/arith.wasm"),
    include_bytes!("wasmtests/call.wasm"),
    include_bytes!("wasmtests/fibonacci.wasm"),
    include_bytes!("wasmtests/globals.wasm"),
    include_bytes!("wasmtests/memory.wasm"),
    &[0x42, 0x43, 0x44, 0x45],
];