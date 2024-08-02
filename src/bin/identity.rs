#![no_std]
#![no_main]

use cortex_m_rt::entry;

use cortex_m_semihosting::hprintln;
use ironfish_frost::participant::Secret;
use ironfish_frost_embedded::{init_heap, HEAP};
use rand::rngs::OsRng;

#[entry]
fn main() -> ! {
    init_heap();
    let rng = OsRng;
    let secret = Secret::random(rng);
    let identity1 = secret.to_identity();
    hprintln!("Heap used {}", HEAP.used()).unwrap();

    hprintln!("secret {:?}", secret.serialize()).unwrap();
    hprintln!("identity {:?}", identity1.serialize()).unwrap();
    panic!("End of main");
}
