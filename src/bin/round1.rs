#![no_std]
#![no_main]

use cortex_m_rt::entry;

use cortex_m_semihosting::hprintln;
use ironfish_frost::{dkg::round1, participant::Identity};
use ironfish_frost_embedded::{init_heap, HEAP};
use rand::rngs::OsRng;

extern crate alloc;
use alloc::vec::Vec;


#[entry]
fn main() -> ! {
    init_heap();
    let mut rng = OsRng;
    let mut identities = Vec::new();
    let identity_bytes_1 = [114, 252, 166, 249, 254, 182, 154, 244, 83, 249, 107, 83, 140, 114, 0, 150, 147, 139, 40, 52, 178, 185, 20, 254, 147, 121, 74, 47, 94, 164, 128, 190, 240, 175, 244, 104, 98, 2, 253, 35, 208, 19, 163, 96, 96, 28, 160, 144, 194, 87, 217, 205, 125, 153, 242, 142, 63, 212, 218, 84, 22, 128, 27, 253, 119, 181, 226, 203, 123, 115, 37, 202, 144, 184, 36, 153, 179, 80, 125, 190, 159, 82, 233, 53, 6, 89, 153, 86, 122, 77, 39, 8, 0, 23, 137, 169, 160, 111, 111, 41, 54, 243, 23, 56, 129, 142, 244, 248, 52, 189, 79, 1, 238, 157, 242, 96, 232, 163, 74, 49, 75, 165, 210, 180, 165, 87, 128, 168, 4];
    identities.push(Identity::deserialize_from(&identity_bytes_1[..]).unwrap());
    let identity_bytes_2 = [114, 97, 154, 23, 138, 59, 9, 33, 165, 233, 156, 69, 240, 37, 103, 57, 46, 95, 42, 198, 219, 213, 94, 99, 203, 227, 221, 23, 78, 44, 163, 150, 14, 152, 225, 5, 254, 73, 52, 14, 173, 14, 8, 46, 130, 119, 148, 210, 96, 53, 201, 196, 177, 208, 41, 76, 148, 231, 140, 247, 105, 220, 204, 155, 1, 202, 13, 61, 61, 231, 38, 58, 31, 212, 211, 67, 31, 61, 126, 169, 225, 79, 184, 10, 94, 234, 43, 52, 77, 77, 92, 123, 76, 12, 68, 161, 66, 156, 2, 243, 56, 139, 125, 88, 115, 51, 231, 95, 140, 94, 114, 158, 18, 169, 239, 173, 115, 123, 70, 70, 96, 52, 143, 95, 187, 228, 142, 1, 6];
    identities.push(Identity::deserialize_from(&identity_bytes_2[..]).unwrap());
    let identity_bytes_3 = [114, 191, 69, 141, 0, 28, 209, 64, 134, 145, 198, 134, 15, 44, 43, 123, 141, 15, 142, 167, 0, 19, 159, 200, 155, 156, 25, 121, 251, 106, 99, 24, 253, 15, 186, 145, 34, 6, 218, 193, 111, 60, 68, 34, 212, 235, 82, 170, 45, 102, 0, 64, 199, 80, 41, 26, 24, 194, 132, 147, 119, 156, 142, 109, 22, 252, 113, 48, 23, 164, 123, 119, 132, 229, 12, 41, 154, 183, 193, 38, 139, 217, 158, 159, 53, 129, 111, 255, 165, 7, 174, 63, 121, 57, 46, 189, 255, 53, 55, 155, 31, 199, 47, 240, 179, 44, 249, 84, 56, 227, 128, 165, 18, 184, 208, 34, 220, 4, 232, 17, 219, 33, 235, 155, 0, 99, 101, 216, 5];
    identities.push(Identity::deserialize_from(&identity_bytes_3[..]).unwrap());
    let identity_bytes_4 = [114, 135, 138, 49, 165, 59, 16, 213, 197, 75, 246, 6, 53, 57, 78, 156, 184, 149, 151, 131, 0, 76, 26, 214, 74, 189, 36, 32, 148, 24, 6, 12, 206, 29, 141, 81, 192, 74, 180, 57, 90, 206, 202, 252, 214, 93, 170, 27, 235, 106, 13, 204, 111, 54, 7, 202, 136, 141, 60, 77, 227, 55, 148, 99, 37, 35, 90, 241, 13, 117, 143, 38, 222, 115, 47, 169, 131, 73, 211, 25, 68, 130, 186, 57, 226, 83, 202, 130, 88, 112, 68, 101, 183, 96, 12, 241, 58, 76, 246, 48, 5, 138, 22, 178, 32, 8, 134, 117, 219, 216, 27, 120, 97, 152, 6, 100, 64, 106, 154, 186, 137, 90, 191, 211, 159, 186, 14, 154, 7];
    identities.push(Identity::deserialize_from(&identity_bytes_4[..]).unwrap());
    for (i, identity) in identities.iter().enumerate() {
        let (round1_secret_package, package) = round1::round1(
            &identity,
            2,
            &identities,
            &mut rng,
        ).unwrap();
        hprintln!("let secret_package_{} = {:?};", i+1, round1_secret_package).unwrap();
        hprintln!("let package_bytes_{} = {:?};", i+1, package.serialize()).unwrap();
        hprintln!("let package_{} = PublicPackage::deserialize_from(&package_bytes_{}[..]).unwrap();",i+1,  i+1).unwrap();
    }
    panic!("End of main");
}
