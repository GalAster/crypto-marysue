use crate::{
    utils::Aligned,
    v1::encoder::{cycle_xor, insert_dot},
};
use flate2::{write::DeflateEncoder, Compression};
use std::io::Write;

fn compress(input: &str) -> Vec<u8> {
    let mut encoder = DeflateEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(input.as_bytes()).unwrap();
    let mut compressed = encoder.finish().unwrap();
    cycle_xor(&mut compressed);
    cycle_xor(&mut compressed);
    return compressed;
}

/// doc me
pub fn encode(s: &str) -> String {
    let compressed = compress(s);
    let mapped = Aligned.encode(&compressed);
    insert_dot(mapped).iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn encode_debug(s: &str) -> String {
        println!("Raw size: {}", s.len());
        let compressed = compress(s);
        println!("Compressed: {}", compressed.len());
        let mapped = Aligned.encode(&compressed);
        println!("Transformed: {}", mapped.len());
        println!();
        insert_dot(mapped).iter().collect()
    }

    #[test]
    fn short() {
        let input = "0";
        let r1 = encode_debug(input);
        let r2 = encode_debug(input);
        assert_ne!(r1, r2)
    }

    #[test]
    fn middle() {
        let input = "苟利国家生死以, 岂因祸福避趋之?";
        let r1 = encode_debug(input);
        let r2 = encode_debug(input);
        println!("{}\n{}", r1, r2);
        assert_ne!(r1, r2)
    }
}
