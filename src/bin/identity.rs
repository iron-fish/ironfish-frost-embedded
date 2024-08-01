#![no_std]
#![no_main]

use cortex_m_rt::entry;

use cortex_m_semihosting::hprintln;
use ironfish_frost::participant::Secret;
use ironfish_frost_embedded::init_heap;
use rand::rngs::OsRng;


#[entry]
fn main() -> ! {
    init_heap();
    let mut rng = OsRng;
    let secret = Secret::random(&mut rng);
    hprintln!("secret {:?}", secret.serialize()).unwrap();
    // hprintln!("secret {}", HEAP.used()).unwrap();
    let identity1 = secret.to_identity();
    hprintln!("identity {:?}", identity1.serialize()).unwrap();
    // hprintln!("used {}", HEAP.used()).unwrap();
    panic!("End of main");
}
