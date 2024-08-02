#![no_std]
#![no_main]

use cortex_m_rt::entry;

use cortex_m_semihosting::hprintln;
use ironfish_frost::{dkg::round1, participant::Identity};
use ironfish_frost_embedded::{init_heap, HEAP};
use rand::rngs::OsRng;

#[entry]
fn main() -> ! {
    init_heap();
    let mut rng = OsRng;
    let identity_bytes1: [u8; 129] = [
        114, 74, 119, 222, 130, 99, 78, 38, 205, 60, 41, 201, 219, 43, 52, 110, 44, 77, 173, 209,
        211, 24, 150, 205, 114, 137, 152, 201, 0, 183, 124, 109, 217, 186, 199, 249, 57, 92, 215,
        245, 13, 84, 15, 12, 2, 145, 174, 16, 115, 127, 131, 134, 108, 3, 187, 108, 223, 118, 252,
        46, 179, 12, 114, 174, 6, 33, 84, 161, 211, 175, 30, 62, 150, 14, 99, 245, 180, 206, 227,
        15, 89, 135, 196, 8, 48, 174, 82, 34, 131, 224, 227, 229, 236, 53, 45, 160, 195, 219, 83,
        102, 188, 78, 188, 109, 127, 118, 109, 22, 158, 77, 185, 84, 134, 139, 247, 42, 127, 100,
        22, 154, 224, 89, 50, 178, 221, 238, 78, 14, 1,
    ];

    let identity1 = Identity::deserialize_from(&identity_bytes1[..]).unwrap();

    let identity_bytes2: [u8; 129] = [
        114, 47, 118, 27, 67, 116, 82, 234, 125, 80, 5, 23, 107, 34, 74, 188, 122, 185, 128, 46,
        84, 194, 85, 11, 196, 124, 42, 106, 198, 207, 178, 239, 125, 211, 153, 172, 15, 45, 165,
        83, 238, 112, 106, 39, 38, 123, 27, 222, 78, 247, 136, 167, 178, 166, 93, 241, 204, 143,
        54, 49, 94, 198, 146, 12, 2, 235, 110, 74, 14, 199, 134, 138, 59, 77, 169, 1, 110, 37, 162,
        251, 43, 189, 252, 47, 247, 111, 184, 115, 0, 100, 222, 32, 198, 254, 196, 5, 112, 235,
        166, 222, 145, 16, 158, 63, 146, 80, 140, 77, 2, 95, 159, 242, 57, 82, 138, 247, 33, 155,
        8, 11, 62, 221, 227, 105, 197, 113, 214, 249, 7,
    ];
    let identity2 = Identity::deserialize_from(&identity_bytes2[..]).unwrap();

    let (round1_secret_package, package) =
        round1::round1(&identity1, 2, [&identity1, &identity2], rng).unwrap();
    hprintln!("Heap used {}", HEAP.used()).unwrap();
    hprintln!("round1_secret_package {:?}", round1_secret_package).unwrap();
    hprintln!("package {:?}", package.serialize()).unwrap();

    panic!("End of main");
}
