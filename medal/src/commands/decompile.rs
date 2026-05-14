use std::path::Path;

use base64::prelude::*;
use tracing::warn;

// Large default worker stack to avoid stack overflows on deeply nested/decompiled inputs.
const DEFAULT_DECOMPILE_STACK_SIZE: usize = 64 * 1024 * 1024;

fn configured_decompile_stack_size() -> usize {
    std::env::var("MEDAL_DECOMPILE_STACK_SIZE_MB")
        .ok()
        .and_then(|value| value.parse::<usize>().ok())
        .filter(|mb| *mb > 0)
        .map(|mb| mb.saturating_mul(1024 * 1024))
        .unwrap_or(DEFAULT_DECOMPILE_STACK_SIZE)
}

fn decompile_inline(bytecode: &[u8], encode_key: u8, lua51: bool) -> String {
    if lua51 {
        lua51_lifter::decompile_bytecode(bytecode)
    } else {
        luau_lifter::decompile_bytecode(bytecode, encode_key)
    }
}

fn decompile_with_large_stack(bytecode: Vec<u8>, encode_key: u8, lua51: bool) -> String {
    let stack_size = configured_decompile_stack_size();
    let spawn_result = std::thread::scope(|scope| {
        std::thread::Builder::new()
            .name("medal-decompile".to_string())
            .stack_size(stack_size)
            .spawn_scoped(scope, || decompile_inline(&bytecode, encode_key, lua51))
            .map(|handle| handle.join())
    });

    match spawn_result {
        Ok(Ok(output)) => output,
        Ok(Err(payload)) => std::panic::resume_unwind(payload),
        Err(err) => {
            warn!(
                %err,
                stack_size,
                "failed to spawn decompile worker thread; falling back to current thread"
            );
            decompile_inline(&bytecode, encode_key, lua51)
        }
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
