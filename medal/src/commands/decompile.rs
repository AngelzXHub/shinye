use std::path::Path;

use base64::prelude::*;

const DECOMPILE_STACK_SIZE: usize = 512 * 1024 * 1024;

fn decompile_with_large_stack(bytecode: Vec<u8>, encode_key: u8, lua51: bool) -> String {
    let handle = std::thread::Builder::new()
        .name("medal-decompile".to_string())
        .stack_size(DECOMPILE_STACK_SIZE)
        .spawn(move || {
            if lua51 {
                lua51_lifter::decompile_bytecode(&bytecode)
            } else {
                luau_lifter::decompile_bytecode(&bytecode, encode_key)
            }
        })
        .expect("failed to spawn decompile worker thread");

    match handle.join() {
        Ok(output) => output,
        Err(payload) => std::panic::resume_unwind(payload),
    }
}

pub fn decompile_no_io<T>(bytecode: T, encode_key: u8, lua51: bool) -> String
where
    T: Into<Vec<u8>>,
{
    let mut bytecode = bytecode.into();
    if let Ok(decoded) = BASE64_STANDARD.decode(&bytecode) {
        bytecode = decoded;
    }

    decompile_with_large_stack(bytecode, encode_key, lua51)
}

pub fn decompile(
    input: &Path,
    output: &Path,
    encode_key: u8,
    lua51: bool,
) -> Result<(), std::io::Error> {
    let bytecode = std::fs::read(input)?;
    let out = decompile_no_io(bytecode, encode_key, lua51);
    std::fs::write(output, out)
}
