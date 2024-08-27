#![no_std]

use cortex_m::peripheral::SYST;
use embedded_alloc::LlffHeap as Heap;
use getrandom::Error;
use seed::SEED;
use core::mem::MaybeUninit;

pub mod seed;

static mut SEED_VAR: u32 = SEED;


use getrandom::register_custom_getrandom;

// Function to initialize the seed using the SysTick timer
fn init_seed() {
    unsafe {
        let syst = &*SYST::PTR;
        SEED_VAR = syst.cvr.read() ^ SEED; // Combine timer value with the environment seed
    }
}

pub fn custom_getrandom(buf: &mut [u8]) -> Result<(), Error> {
    unsafe {
        if SEED_VAR == 0 {
            init_seed();
        }
    }

    for byte in buf.iter_mut() {
        unsafe {
            // Linear Congruential Generator (LCG) parameters
            SEED_VAR = SEED_VAR.wrapping_mul(1664525).wrapping_add(1013904223);
            *byte = (SEED_VAR >> 16) as u8; // Use the upper 8 bits of the generated value
        }
    }

    Ok(())
}
register_custom_getrandom!(custom_getrandom);

#[global_allocator]
pub static HEAP: Heap = Heap::empty();

pub fn init_heap() {
    const HEAP_SIZE: usize = 12500;
    static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
    unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }
}
use panic_semihosting as _; // features = ["exit"]