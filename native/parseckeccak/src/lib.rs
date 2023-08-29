use rustler::{NifMap, NifResult, Error};

use tiny_keccak::Keccak;
use tiny_keccak::Hasher;

use base16;

#[derive(NifMap)]
struct Opts {
    decode: bool
}

#[rustler::nif]
fn hash(string: &str, opts: Opts) -> NifResult<String> {
    if opts.decode {
        match base16::decode(string) {
            Ok(decoded) => Ok(do_hash(&decoded)),
            Err(_err)   => Err(Error::Atom("error")),
        }
        
    } else {
        Ok(do_hash(string.as_bytes()))
    }
}

fn do_hash(bytes: &[u8]) -> String {
    let mut sha3   = Keccak::v256();
    let mut output = [0u8; 32];

    sha3.update(bytes);
    sha3.finalize(&mut output);

    base16::encode_lower(&output)
}

rustler::init!("Elixir.ParsecKeccak.Wrapper", [hash]);
