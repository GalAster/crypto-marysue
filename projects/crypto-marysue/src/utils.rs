//! doc me
use crypto_random_map::{SecretAligned, SecretDense};
use lazy_static::lazy_static;
use std::ops::{Div, Rem};

/// doc me
pub fn div_rem<T: Div<Output = T> + Rem<Output = T> + Copy>(x: T, y: T) -> (T, T) {
    let quotient = x / y;
    let remainder = x % y;
    (quotient, remainder)
}

/// doc me
#[rustfmt::skip]
pub const CHAR_SET: &str = "\
丝丹丽之乐云亚仪伊优伤佳依俏倩倾兮兰冰凌凝凡凤凪利千华卿可叶吉君咏哀嘉园城基塔墨夏多奥如妍妖妙妮妲姆\
姣姬娅娜娣娥娴婉婵婷媛嫩宁安宜寂寇寒岚巧希幻幽弥彩影御心思怡恋恩悠悦情慕慧拉文斯春昭晓晗晶曦曼月朵枝\
枫柒柔格桂梅梦樱欢欣殇残毓沫泪洁洛浅海涅淑清温渺滢澜澪灵烟然燕燢爱爽玉玖玛玥玫环玲珊珍珠琉琦琪琬琰琳\
琴琼瑗瑞瑟瑰瑶瑷璃璎璐璧白百盘眉真碎离秀秋筱米素紫红纨纯纱绯缈美羽翠翼育舒舞艳艺艾芊芝芬花芳芸苏苑英\
茉茗茜茹荔荷莉莎莲莳莹莺菁菲萌萍萝萦萨落蒂蓉蓓蓝蔷蕊蕴蕾薇薰蝶融血裳语贞迷邪铃银锦阳陌雁雅雨雪霄霜霞\
霭露青静音韵颖颜风飘香馥馨魂魅魑鸢黎黛";

lazy_static! {
/// doc me
    pub static ref Dense: SecretDense = SecretDense::new(CHAR_SET);
    /// doc me
    pub static ref Aligned: SecretAligned = SecretAligned::new(CHAR_SET);
}
