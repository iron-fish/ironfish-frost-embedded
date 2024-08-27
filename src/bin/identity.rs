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
    let mut rng = OsRng;
    for i in 0..4 {
        let secret = Secret::random(&mut rng);
        let identity = secret.to_identity();
        hprintln!("let secret_bytes_{} = {:?};", i+1, secret.serialize()).unwrap();
        hprintln!("secrets.push(Secret::deserialize_from(&secret_bytes_{}[..]).unwrap());", i+1).unwrap();
        hprintln!("let identity_bytes_{} = {:?};", i+1, identity.serialize()).unwrap();
        hprintln!("identities.push(Identity::deserialize_from(&identity_bytes_{}[..]).unwrap());", i+1).unwrap();
    }
    hprintln!("Heap used {}", HEAP.used()).unwrap();
    panic!("End of main");
}
