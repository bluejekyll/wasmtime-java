#![allow(
    clippy::needless_lifetimes,
    clippy::needless_borrow,
    clippy::clone_on_copy
)]

mod opaque_ptr;
mod ty;
mod wasm_engine;
mod wasm_exception;
mod wasm_function;
mod wasm_instance;
mod wasm_linker;
mod wasm_module;
mod wasm_store;
mod wasm_value;
mod wasmtime;

mod net_bluejekyll_wasmtime {
    #![allow(
        non_camel_case_types,
        dead_code,
        non_snake_case,
        improper_ctypes_definitions,
        clippy::let_unit_value,
        clippy::unused_unit
    )]

    include!(concat!(env!("OUT_DIR"), "/generated_jaffi.rs"));
}

use wasm_engine::WasmEngineRsImpl;
