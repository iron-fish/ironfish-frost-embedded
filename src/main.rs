#![no_std]
#![no_main]

use ironfish_frost::dkg::{round1, round2, round3};
use ironfish_frost::participant::Secret;
// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m::asm;
use cortex_m_rt::entry;
use cortex_m::peripheral::DWT;
use getrandom::Error;
use rand::rngs::OsRng;
use rand::RngCore;
use core::result::Result;
use crate::Result::Ok;


// extern crate alloc;
// use core::prelude::rust_2024::*;
// use core::write;
// use core::stringify;

use core::mem::MaybeUninit;
use cortex_m_semihosting::{debug, hprintln};

use embedded_alloc::Heap;
// use rand::SeedableRng;
// use rand::rngs::SmallRng;

use getrandom::register_custom_getrandom;

pub fn custom_getrandom(buf: &mut [u8]) -> Result<(), Error> {
    let mut dwt = unsafe { &*DWT::ptr() };

    for byte in buf.iter_mut() {
        // Generate a random value using DWT cycle counter
        *byte = (dwt.cyccnt.read() & 0xFF) as u8;

        // Optionally, add some delay to ensure different values
        unsafe { dwt.cyccnt.write(0) };
        while dwt.cyccnt.read() < 1000 {}
    }

    Ok(())
}

register_custom_getrandom!(custom_getrandom);

#[global_allocator]
static HEAP: Heap = Heap::empty();

pub fn init_heap() {
    const HEAP_SIZE: usize = 14336;
    static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
    unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }
}


#[entry]
fn main() -> ! {
    // init_heap();
    asm::nop(); // To not have main optimize to abort in release mode, remove when you add code
    hprintln!("Hello, world!").unwrap();
    // let mut rng = OsRng;
    // let secret1 = Secret::random(rng);
    // let secret2 = Secret::random(rng);
    // let secret3 = Secret::random(rng);
    // let identity1 = secret1.to_identity();
    // let identity2 = secret2.to_identity();
    // let identity3 = secret3.to_identity();

    // let (round1_secret_package_1, package1) = match round1::round1(
    //     &identity1,
    //     2,
    //     [&identity1, &identity2, &identity3],
    //     rng,
    // ) {
    //     Ok(result) => result,
    //     Err(e) => {
    //         // Handle the error appropriately
    //         panic!("Error in round1 for identity1: {:?}", e);
    //     }
    // };

    // let (round1_secret_package_2, package2) = match round1::round1(
    //     &identity2,
    //     2,
    //     [&identity1, &identity2, &identity3],
    //     rng,
    // ) {
    //     Ok(result) => result,
    //     Err(e) => {
    //         // Handle the error appropriately
    //         panic!("Error in round1 for identity2: {:?}", e);
    //     }
    // };

    // let (round1_secret_package_3, package3) = match round1::round1(
    //     &identity3,
    //     2,
    //     [&identity1, &identity2, &identity3],
    //     rng,
    // ) {
    //     Ok(result) => result,
    //     Err(e) => {
    //         // Handle the error appropriately
    //         panic!("Error in round1 for identity3: {:?}", e);
    //     }
    // };

    // let (encrypted_secret_package, _) = match round2::round2(
    //     &secret1,
    //     &round1_secret_package_1,
    //     [&package1, &package2, &package3],
    //     rng,
    // ) {
    //     Ok(result) => result,
    //     Err(e) => {
    //         // Handle the error appropriately
    //         panic!("Error in round2 for secret1: {:?}", e);
    //     }
    // };

    // let (_, round2_public_packages_2) = match round2::round2(
    //     &secret2,
    //     &round1_secret_package_2,
    //     [&package1, &package2, &package3],
    //     rng,
    // ) {
    //     Ok(result) => result,
    //     Err(e) => {
    //         // Handle the error appropriately
    //         panic!("Error in round2 for secret2: {:?}", e);
    //     }
    // };

    // let (_, round2_public_packages_3) = match round2::round2(
    //     &secret3,
    //     &round1_secret_package_3,
    //     [&package1, &package2, &package3],
    //     rng,
    // ) {
    //     Ok(result) => result,
    //     Err(e) => {
    //         // Handle the error appropriately
    //         panic!("Error in round2 for secret3: {:?}", e);
    //     }
    // };

    // let _ = match round3::round3(
    //     &secret1,
    //     &encrypted_secret_package,
    //     [&package1, &package2, &package3],
    //     [&round2_public_packages_2, &round2_public_packages_3],
    // ) {
    //     Ok(result) => result,
    //     Err(e) => {
    //         // Handle the error appropriately
    //         panic!("Error in round3: {:?}", e);
    //     }
    // };

    loop {}
}
