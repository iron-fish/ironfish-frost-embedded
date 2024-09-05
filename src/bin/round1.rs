#![no_std]
#![no_main]

use cortex_m_rt::entry;

use cortex_m_semihosting::hprintln;
use ironfish_frost::{dkg::round1, participant::Identity};
use ironfish_frost_embedded::{init_heap, HEAP};
use rand::{rngs::OsRng, CryptoRng, RngCore};

#[entry]
fn main() -> ! {
    init_heap();
    let mut rng = OsRng;
    let identity_bytes1 = [
        114, 86, 240, 217, 3, 5, 5, 151, 109, 35, 242, 2, 173, 214, 229, 169, 53, 81, 158, 56, 11,
        208, 196, 125, 160, 214, 61, 34, 131, 114, 104, 159, 249, 68, 105, 129, 152, 91, 62, 174,
        188, 2, 234, 183, 130, 166, 165, 242, 242, 96, 199, 49, 16, 147, 238, 81, 209, 111, 155,
        102, 62, 105, 147, 111, 21, 182, 195, 205, 53, 190, 9, 56, 2, 162, 146, 165, 153, 152, 227,
        114, 151, 222, 188, 228, 143, 177, 101, 235, 99, 28, 190, 223, 54, 137, 15, 157, 161, 231,
        209, 43, 237, 221, 144, 4, 48, 180, 164, 190, 92, 249, 126, 210, 126, 196, 65, 44, 157,
        137, 147, 7, 73, 246, 81, 116, 70, 52, 180, 34, 15,
    ];
    let identity1 = Identity::deserialize_from(&identity_bytes1[..]).unwrap();

    let identity_bytes2 = [
        114, 124, 157, 239, 202, 84, 148, 135, 52, 15, 62, 107, 173, 82, 253, 13, 117, 75, 6, 135,
        56, 156, 243, 44, 3, 170, 171, 36, 76, 131, 197, 32, 211, 118, 184, 117, 116, 149, 169,
        123, 189, 69, 107, 185, 175, 126, 77, 86, 61, 231, 87, 234, 76, 58, 70, 225, 255, 199, 58,
        137, 90, 6, 145, 115, 10, 253, 17, 225, 57, 67, 76, 154, 49, 98, 200, 53, 35, 189, 10, 66,
        53, 228, 151, 246, 8, 223, 92, 111, 40, 239, 197, 196, 67, 219, 255, 86, 72, 149, 61, 78,
        66, 192, 23, 15, 56, 192, 51, 193, 1, 41, 48, 18, 113, 30, 103, 158, 180, 122, 243, 29,
        127, 167, 124, 139, 63, 140, 89, 57, 11,
    ];
    let identity2 = Identity::deserialize_from(&identity_bytes2[..]).unwrap();

    let identity_bytes3 = [
        114, 47, 118, 27, 67, 116, 82, 234, 125, 80, 5, 23, 107, 34, 74, 188, 122, 185, 128, 46,
        84, 194, 85, 11, 196, 124, 42, 106, 198, 207, 178, 239, 125, 211, 153, 172, 15, 45, 165,
        83, 238, 112, 106, 39, 38, 123, 27, 222, 78, 247, 136, 167, 178, 166, 93, 241, 204, 143,
        54, 49, 94, 198, 146, 12, 2, 235, 110, 74, 14, 199, 134, 138, 59, 77, 169, 1, 110, 37, 162,
        251, 43, 189, 252, 47, 247, 111, 184, 115, 0, 100, 222, 32, 198, 254, 196, 5, 112, 235,
        166, 222, 145, 16, 158, 63, 146, 80, 140, 77, 2, 95, 159, 242, 57, 82, 138, 247, 33, 155,
        8, 11, 62, 221, 227, 105, 197, 113, 214, 249, 7,
    ];
    let identity3 = Identity::deserialize_from(&identity_bytes3[..]).unwrap();

    let identities = [&identity1, &identity2, &identity3];

    execute_round1(1, &identity1, identities, rng);
    execute_round1(2, &identity2, identities, rng);
    execute_round1(3, &identity3, identities, rng);

    panic!("End of main");
}

fn execute_round1<'a>(
    identity_id: u8,
    identity: &Identity,
    identities: impl IntoIterator<Item = &'a Identity>,
    rng: impl RngCore + CryptoRng,
) {
    let (round1_secret_package, package) = round1::round1(identity, 2, identities, rng).unwrap();
    hprintln!("Heap used {}", HEAP.used()).unwrap();
    hprintln!(
        "let round1_secret_package_bytes{} = {:?};",
        identity_id,
        round1_secret_package
    )
    .unwrap();
    hprintln!(
        "let package1_bytes{} = {:?};\n\n",
        identity_id,
        package.serialize()
    )
    .unwrap();
}
