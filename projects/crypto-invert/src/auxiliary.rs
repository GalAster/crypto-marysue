



fn invert_map(c: char) -> char {
    match c {
        'a' => 'ɐ',
        'b' => 'q',
        'c' => 'ɔ',
        'd' => 'p',
        'e' => 'ǝ',
        'f' => 'ɟ',
        'g' => 'ᵷ',
        'h' => 'ɥ',
        'i' => 'ᴉ',
        'j' => 'f',
        'k' => 'ʞ',
        'l' => 'ꞁ',
        'm' => 'ɯ',
        'n' => 'u',
        'o' => 'o',
        'p' => 'd',
        'q' => 'b',
        'r' => 'ɹ',
        's' => 's',
        't' => 'ʇ',
        'u' => 'n',
        'v' => 'ʌ',
        'w' => 'ʍ',
        'x' => 'x',
        'y' => 'ʎ',
        'z' => 'z',
        //
        'A' => 'Ɐ',
        'B' => 'B',
        'C' => 'Ɔ',
        'D' => 'D',
        'E' => 'Ǝ',
        'F' => 'Ⅎ',
        'G' => '⅁',
        'H' => 'H',
        'I' => 'I',
        'J' => 'ſ',
        'K' => 'Ʞ',
        'L' => 'Ꞁ',
        'M' => 'Ɯ',
        'N' => 'N',
        'O' => 'O',
        'P' => 'Ԁ',
        'Q' => 'Ò',
        'R' => 'ᴚ',
        'S' => 'S',
        'T' => 'Ʇ',
        'U' => '∩',
        'V' => 'Ʌ',
        'W' => 'ʍ',
        'X' => 'X',
        'Y' => '⅄',
        'Z' => 'Z',
        //
        ',' => '⸲',
        '.' => '˙',
        ';' => '⸵',
        '!' => '¡',
        '?' => '¿',
        '(' => ')',
        ')' => '(',
        '[' => ']',
        ']' => '[',
        '{' => '}',
        '}' => '{',
        '&' => '⅋',
        '†' => '⸸',
        _ => c
    }
}