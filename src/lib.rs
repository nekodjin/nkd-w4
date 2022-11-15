#![no_std]

#[cfg(not(target_family = "wasm"))]
compile_error!("This crate cannot be built for a non-WASM target.");

mod custom_alloc;

pub mod prim;
