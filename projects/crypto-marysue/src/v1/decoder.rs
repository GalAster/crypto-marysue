//! doc me
use crate::utils::Dense;
use encoding_rs::GB18030;
use flate2::write::DeflateDecoder;
use std::io::Write;

pub fn cycle_xor(vec: &mut Vec<u8>) -> Vec<u8> {
    let s = vec.pop().unwrap();
    for i in vec.iter_mut() {
        *i ^= s;
    }
    vec.reverse();
    vec.to_owned()
}

pub fn decompress<'a>(input: Vec<u8>) -> Result<String, String> {
    let mut compressed = input;
    cycle_xor(&mut compressed);
    cycle_xor(&mut compressed);

    let mut writer = Vec::new();
    let mut deflater = DeflateDecoder::new(writer);
    if let Err(e) = deflater.write_all(&compressed[..]) {
        return Err(e.to_string());
    }
    writer = match deflater.finish() {
        Ok(o) => o,
        Err(e) => return Err(e.to_string()),
    };
    let (cow, _, _) = GB18030.decode(&writer);
    Ok(String::from(cow))
}

/// doc me
pub fn decode(s: &str) -> String {
    let mut r = s.to_string();
    r.retain(|c| !"·".contains(c));
    let mapped = Dense.decode(&r);
    match decompress(mapped) {
        Ok(s) => s,
        Err(e) => e,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[rustfmt::skip]
    fn short() {
        //[219, 185, 237, 192, 239, 157, 191, 246, 92, 58, 249, 235, 244, 129, 75, 87, 116, 20, 142, 125, 188, 244, 105, 247, 183, 29, 139, 55, 222, 57, 254, 253, 218, 58, 123, 0]
        let r = decode("莺姣·萍凌玥寒情裳·艾寇玫丹·缈翼缈晗·璃菲梦悦姆凪洁·茜韵佳萍拉·妍乐蓉吉茜·温融·叶玲仪");
        debug_assert_eq!(r, "苟利国家生死以, 岂因祸福避趋之?");
        let r = decode("滢珠妖心·柔如寂·琬爱颜洛蔷·仪塔菁·亚妲吉伤·烟琪芳玥秋月融·瑗娅盘飘兮·雅薇丹落陌斯·凝");
        debug_assert_eq!(r, "苟利国家生死以, 岂因祸福避趋之?");
    }
}
