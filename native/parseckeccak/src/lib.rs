use rustler::{OwnedBinary, Binary};

use tiny_keccak::Keccak;
use tiny_keccak::Hasher;

#[rustler::nif]
fn hash(binary: Binary) -> OwnedBinary {
    // do_hash(string.as_bytes())
    let mut sha3   = Keccak::v256();
    let mut output = [0u8; 32];

    sha3.update(binary.as_slice());
    sha3.finalize(&mut output);

    let bytes = output;
    let mut binary: OwnedBinary = OwnedBinary::new(bytes.len()).unwrap();

    binary.as_mut_slice().copy_from_slice(&bytes);

    binary
}

// fn do_hash(bytes: &[u8]) -> OwnedBinary {
//     let mut sha3   = Keccak::v256();
//     let mut output = [0u8; 32];

//     sha3.update(bytes);
//     sha3.finalize(&mut output);

//     let bytes = output;
//     let mut binary: OwnedBinary = OwnedBinary::new(bytes.len()).unwrap();

//     binary.as_mut_slice().copy_from_slice(&bytes);

//     binary
// }

rustler::init!("Elixir.ParsecKeccak", [hash]);
