//! Rhyme detection, phonetic suffix extraction, and poem rhyme scheme analysis.

use crate::phonotactics::*;
use crate::syllabary::separar_silabas_com_spans;
use crate::tonic::descobrir_tonica;
use std::collections::HashMap;

/// Result of rhyme analysis for a verse.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InfoRima {
    /// Rhyme scheme letter (e.g. "A", "B", "C").
    pub letra: String,
    /// Phonetic rhyme ending suffix (e.g. "-er", "-ente", "-á").
    pub terminacao: String,
}

/// Extracts the normalized phonetic rhyme suffix from the final tonic vowel onward.
pub fn extrair_terminacao_rima(verso: &str) -> Option<String> {
    let clean = verso
        .trim()
        .trim_end_matches(|c: char| !c.is_alphabetic() && c != '/' && c != '\'');
    let last_word = clean.split_whitespace().last()?;
    let palavra_limpa: String = last_word.chars().filter(|&c| c.is_alphabetic()).collect();
    if palavra_limpa.is_empty() {
        return None;
    }

    let sils = separar_silabas_com_spans(&palavra_limpa, 0, false);
    if sils.is_empty() {
        return None;
    }

    let idx_tonica = descobrir_tonica(&sils, &palavra_limpa).unwrap_or(sils.len() - 1);

    let mut term = String::new();
    for (i, (s, _)) in sils.iter().enumerate().skip(idx_tonica) {
        if i == idx_tonica {
            let chars: Vec<char> = s.chars().collect();
            let mut v_idx = 0;
            for (ci, &c) in chars.iter().enumerate() {
                if is_vowel(c) {
                    v_idx = ci;
                    break;
                }
            }
            term.push_str(&chars[v_idx..].iter().collect::<String>());
        } else {
            term.push_str(s);
        }
    }

    if term.is_empty() {
        return None;
    }

    let normalizada = term
        .to_lowercase()
        .replace(['á', 'â', 'à', 'ã'], "a")
        .replace(['é', 'ê'], "e")
        .replace('í', "i")
        .replace(['ó', 'ô', 'õ'], "o")
        .replace(['ú', 'ü'], "u");

    Some(normalizada)
}

/// Analyzes an array of verse lines and assigns scheme letters (A, B, C...) and overall scheme.
pub fn analisar_esquema_rimas(linhas: &[String]) -> (Vec<Option<InfoRima>>, Option<String>) {
    let mut mapa_rimas: HashMap<String, char> = HashMap::new();
    let mut letra_atual = 'A';
    let mut resultado = Vec::new();
    let mut letras_esquema = Vec::new();

    for linha in linhas {
        if let Some(term) = extrair_terminacao_rima(linha) {
            let letra = *mapa_rimas.entry(term.clone()).or_insert_with(|| {
                let l = letra_atual;
                if letra_atual < 'Z' {
                    letra_atual = (letra_atual as u8 + 1) as char;
                }
                l
            });
            letras_esquema.push(letra);
            resultado.push(Some(InfoRima {
                letra: letra.to_string(),
                terminacao: format!("-{}", term),
            }));
        } else {
            resultado.push(None);
        }
    }

    let esquema = if !letras_esquema.is_empty() {
        Some(letras_esquema.into_iter().collect())
    } else {
        None
    };

    (resultado, esquema)
}
