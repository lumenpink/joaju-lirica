//! Lexical stress and tonic detection for Portuguese words.

use crate::phonotactics::*;
use std::ops::Range;

/// Identifies the index of the tonic syllable within a list of syllables, or `None` if
/// the word is an atonic clitic monosyllable.
pub fn descobrir_tonica(silabas: &[(String, Range<usize>)], palavra_limpa: &str) -> Option<usize> {
    if silabas.is_empty() {
        return None;
    }

    // 0. Explicit atonic marker prefix '~' (e.g. "~de", "~fogo") forces all syllables to be atonic
    if palavra_limpa.starts_with('~') {
        return None;
    }

    // 1. Explicit apostrophe stress marker `'` (e.g. "que'", "a'mor", "fogo'")
    for (idx, (s, _)) in silabas.iter().enumerate() {
        if s.contains('\'') {
            return Some(idx);
        }
    }

    // 2. Explicit acute or circumflex accent takes absolute precedence
    for (idx, (s, _)) in silabas.iter().enumerate() {
        if s.chars().any(is_acute_or_circumflex) {
            return Some(idx);
        }
    }

    // 3. Explicit tilde accent
    for (idx, (s, _)) in silabas.iter().enumerate() {
        if s.chars().any(is_tilde) {
            return Some(idx);
        }
    }

    // 4. Monosyllables without graphic accent or explicit stress marker
    if silabas.len() == 1 {
        let clean_lower: String = palavra_limpa
            .to_lowercase()
            .chars()
            .filter(|&c| c != '\'')
            .collect();
        return if is_unstressed_monosyllable(&clean_lower) {
            None
        } else {
            Some(0)
        };
    }

    // 5. Regular termination rules for unaccented polysyllables
    let clean_lower: String = palavra_limpa
        .to_lowercase()
        .chars()
        .filter(|&c| c != '\'')
        .collect();
    let paroxitona = clean_lower.ends_with('a')
        || clean_lower.ends_with("as")
        || clean_lower.ends_with('e')
        || clean_lower.ends_with("es")
        || clean_lower.ends_with('o')
        || clean_lower.ends_with("os")
        || clean_lower.ends_with("em")
        || clean_lower.ends_with("ens");

    if paroxitona {
        if silabas.len() >= 2 {
            Some(silabas.len() - 2)
        } else {
            Some(0)
        }
    } else {
        Some(silabas.len() - 1)
    }
}
