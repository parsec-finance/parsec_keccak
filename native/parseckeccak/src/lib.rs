use tiny_keccak::Keccak;
use tiny_keccak::Hasher;

use base16;

#[rustler::nif]
fn hash(string: &str) -> String {
    let mut sha3   = Keccak::v256();
    let mut output = [0u8; 32];

    sha3.update(string.as_bytes());
    sha3.finalize(&mut output);

    base16::encode_lower(&output)
}

rustler::init!("Elixir.ParsecKeccak", [hash]);
