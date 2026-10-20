//! Phonotactic utilities and character-level classifications for Portuguese.

/// Normalizes a character to lowercase.
#[inline]
pub fn to_lower(c: char) -> char {
    c.to_lowercase().next().unwrap_or(c)
}

/// Checks whether a character is a Portuguese vowel (accented or unaccented).
#[inline]
pub fn is_vowel(c: char) -> bool {
    matches!(
        to_lower(c),
        'a' | 'e'
            | 'i'
            | 'o'
            | 'u'
            | 'á'
            | 'é'
            | 'í'
            | 'ó'
            | 'ú'
            | 'â'
            | 'ê'
            | 'ô'
            | 'ã'
            | 'õ'
            | 'à'
            | 'ü'
    )
}

/// Checks whether a character has an explicit graphic accent (acute, circumflex, or tilde).
#[inline]
pub fn is_accented_vowel(c: char) -> bool {
    matches!(
        to_lower(c),
        'á' | 'é' | 'í' | 'ó' | 'ú' | 'â' | 'ê' | 'ô' | 'ã' | 'õ' | 'à' | 'ü'
    )
}

/// Checks whether a character carries an acute or circumflex accent.
#[inline]
pub fn is_acute_or_circumflex(c: char) -> bool {
    matches!(to_lower(c), 'á' | 'é' | 'í' | 'ó' | 'ú' | 'â' | 'ê' | 'ô')
}

/// Checks whether a character carries a tilde accent.
#[inline]
pub fn is_tilde(c: char) -> bool {
    matches!(to_lower(c), 'ã' | 'õ')
}

/// Checks whether a character acts as a semivowel (`i` or `u` unaccented).
#[inline]
pub fn is_semivowel(c: char) -> bool {
    matches!(to_lower(c), 'i' | 'u')
}

/// Checks whether a character is a strong/low vowel (`a`, `e`, `o` or accented variants).
#[inline]
pub fn is_strong_vowel(c: char) -> bool {
    matches!(
        to_lower(c),
        'a' | 'e' | 'o' | 'á' | 'é' | 'ó' | 'â' | 'ê' | 'ô' | 'ã' | 'õ' | 'à'
    )
}

/// Checks if two adjacent consonants form an inseparable onset cluster.
#[inline]
pub fn is_inseparable_onset(c1: char, c2: char) -> bool {
    let c1 = to_lower(c1);
    let c2 = to_lower(c2);
    matches!((c1, c2), ('c', 'h') | ('l', 'h') | ('n', 'h'))
        || (matches!(c1, 'b' | 'c' | 'd' | 'f' | 'g' | 'p' | 't' | 'v') && matches!(c2, 'l' | 'r'))
}

/// Checks if a lowercase word is an unstressed monosyllable (clitic).
pub fn is_unstressed_monosyllable(word: &str) -> bool {
    matches!(
        word.to_lowercase().as_str(),
        "o" | "a"
            | "os"
            | "as"
            | "um"
            | "uns"
            | "de"
            | "do"
            | "da"
            | "dos"
            | "das"
            | "em"
            | "no"
            | "na"
            | "nos"
            | "nas"
            | "por"
            | "pelo"
            | "pela"
            | "pelos"
            | "pelas"
            | "com"
            | "sem"
            | "sob"
            | "para"
            | "pra"
            | "pro"
            | "me"
            | "te"
            | "se"
            | "lhe"
            | "lhes"
            | "vos"
            | "que"
            | "e"
            | "mas"
            | "ou"
            | "como"
    )
}
