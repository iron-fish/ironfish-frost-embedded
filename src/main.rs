#![no_std]
#![no_main]

use ironfish_frost::dkg::group_key::GroupSecretKeyShard;
use ironfish_frost::dkg::{round1, round2, round3};
use ironfish_frost::participant::{Secret, Secret2};
// pick a panicking behavior
// use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m_rt::entry;
use getrandom::Error;
use rand::rngs::OsRng;
use core::result::Result;
use crate::Result::Ok;


// extern crate alloc;
// use core::prelude::rust_2024::*;
// use core::write;
// use core::stringify;

use core::mem::MaybeUninit;
use cortex_m_semihosting::hprintln;

extern crate alloc;
use alloc::vec::Vec;

use embedded_alloc::LlffHeap as Heap;
// use rand::SeedableRng;
// use rand::rngs::SmallRng;

use getrandom::register_custom_getrandom;

static mut SEED: u32 = 0x12345678; // Initial seed value

pub fn custom_getrandom(buf: &mut [u8]) -> Result<(), Error> {
    for byte in buf.iter_mut() {
        unsafe {
            // Linear Congruential Generator (LCG) parameters
            SEED = SEED.wrapping_mul(1664525).wrapping_add(1013904223);
            *byte = (SEED >> 16) as u8; // Use the upper 8 bits of the generated value
        }
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
use panic_semihosting as _; // features = ["exit"]


#[entry]
fn main() -> ! {
    init_heap();
    let mut vec1 = Vec::from([1,2,3]);
    let mut vec2 = Vec::from([1,2,3]);
    let mut rng = OsRng;
    // let secret2 = Secret::random(&mut rng);
    let secret2 = Secret2::random(&mut rng);

    hprintln!("used {}", HEAP.used()).unwrap();

    // hprintln!("Hello, world!").unwrap();
    // let secret2 = Secret::random(&mut rng);
    // let secret3 = Secret::random(&mut rng);
    // let identity1 = secret1.to_identity();
    // let identity2 = secret2.to_identity();
    // let identity3 = secret3.to_identity();
    // hprintln!("used {}", HEAP.used()).unwrap();

    // let vec = secret3.to_identity().serialize();
    // assert_eq!([0u8; 129], vec);
    // let (round1_secret_package_1, package1) = round1::round1(
    //     &identity1,
    //     2,
    //     [&identity1, &identity2, &identity3],
    //     &mut rng,
    // )
    // .unwrap();
    // assert_eq!(round1_secret_package_1, Vec::new());

    // let (round1_secret_package_2, package2) = round1::round1(
    //     &identity2,
    //     2,
    //     [&identity1, &identity2, &identity3],
    //     rng,
    // )
    // .unwrap();

    // let (round1_secret_package_3, package3) = round1::round1(
    //     &identity3,
    //     2,
    //     [&identity1, &identity2, &identity3],
    //     rng,
    // )
    // .unwrap();

    // let (encrypted_secret_package_1, round2_public_packages_1) = round2::round2(
    //     &secret1,
    //     &round1_secret_package_1,
    //     [&package1, &package2, &package3],
    //     rng,
    // )
    // .unwrap();

    // let (encrypted_secret_package_2, round2_public_packages_2) = round2::round2(
    //     &secret2,
    //     &round1_secret_package_2,
    //     [&package1, &package2, &package3],
    //     rng,
    // )
    // .unwrap();

    // let (encrypted_secret_package_3, round2_public_packages_3) = round2::round2(
    //     &secret3,
    //     &round1_secret_package_3,
    //     [&package1, &package2, &package3],
    //     rng,
    // )
    // .unwrap();

    // let (key_package_1, public_key_package, group_secret_key) = round3::round3(
    //     &secret1,
    //     &encrypted_secret_package_1,
    //     [&package1, &package2, &package3],
    //     [&round2_public_packages_2, &round2_public_packages_3],
    // )
    // .unwrap();


    // let (key_package_2, _, _) = round3::round3(
    //     &secret2,
    //     &encrypted_secret_package_2,
    //     [&package1, &package2, &package3],
    //     [&round2_public_packages_1, &round2_public_packages_3],
    // )
    // .unwrap();


    // let (key_package_3, _, _) = round3::round3(
    //     &secret3,
    //     &encrypted_secret_package_3,
    //     [&package1, &package2, &package3],
    //     [&round2_public_packages_1, &round2_public_packages_2],
    // )    .unwrap();
    panic!("End of main");
    loop {}
}
