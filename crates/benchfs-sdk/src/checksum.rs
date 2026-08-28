/// Computes the canonical BenchFS CRC32C (Castagnoli) checksum.
///
/// The accumulator starts with all bits set and is finalized by inversion, as
/// required by `agent-docs/on-disk-format.md`. The implementation is intentionally
/// portable and does not depend on host CPU feature detection or a crate.
#[must_use]
pub fn crc32c(bytes: &[u8]) -> u32 {
    const REVERSED_CASTAGNOLI: u32 = 0x82f6_3b78;

    let mut crc = u32::MAX;
    for &byte in bytes {
        crc ^= u32::from(byte);
        for _ in 0..8 {
            let mask = 0_u32.wrapping_sub(crc & 1);
            crc = (crc >> 1) ^ (REVERSED_CASTAGNOLI & mask);
        }
    }
    !crc
}

#[cfg(test)]
mod tests {
    use super::crc32c;

    #[test]
    fn standard_check_value() {
        assert_eq!(crc32c(b"123456789"), 0xe306_9283);
    }

    #[test]
    fn empty_input() {
        assert_eq!(crc32c(&[]), 0);
    }
}
