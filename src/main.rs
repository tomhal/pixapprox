#![allow(dead_code)]
#![allow(unused_variables)]

use std::{error::Error, thread};
#[macro_use]
extern crate smallvec;

mod eval;
mod expr;
mod mutate;
mod myimage;
mod pixapprox;
mod population;
mod stack;
mod state;

fn main() -> Result<(), Box<dyn Error>> {
    const STACK_SIZE: usize = 32 * 1024 * 1024;

    // Spawn thread with explicit stack size
    let child = thread::Builder::new()
        .stack_size(STACK_SIZE)
        .spawn(pixapprox::approx_pic)
        .unwrap();

    // Wait for thread to join
    child.join().unwrap();

    Ok(())
}
