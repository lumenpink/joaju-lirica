//! Lexical stress and tonic detection for Portuguese words.

use crate::phonotactics::*;
use std::ops::Range;

/// Identifies the index of the tonic syllable within a list of syllables, or `None` if
/// the word is an atonic clitic monosyllable.
pub fn descobrir_tonica(silabas: &[(String, Range<usize>)], palavra_limpa: &str) -> Option<usize> {
    if silabas.is_empty() {
        return None;
    }

    // 1. Explicit acute or circumflex accent takes absolute precedence
    for (idx, (s, _)) in silabas.iter().enumerate() {
        if s.chars().any(is_acute_or_circumflex) {
            return Some(idx);
        }
    }

    // 2. Explicit tilde accent
    for (idx, (s, _)) in silabas.iter().enumerate() {
        if s.chars().any(is_tilde) {
            return Some(idx);
        }
    }

    // 3. Monosyllables without graphic accent
    if silabas.len() == 1 {
        return if is_unstressed_monosyllable(palavra_limpa) {
            None
        } else {
            Some(0)
        };
    }

    // 4. Regular termination rules for unaccented polysyllables
    let lower = palavra_limpa.to_lowercase();
    let paroxitona = lower.ends_with('a')
        || lower.ends_with("as")
        || lower.ends_with('e')
        || lower.ends_with("es")
        || lower.ends_with('o')
        || lower.ends_with("os")
        || lower.ends_with("em")
        || lower.ends_with("ens");

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
