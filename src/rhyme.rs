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

/// Qualitative evaluation and acoustic classification of poem rhymes.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InfoAvaliacaoRima {
    /// Overall rhyme scheme string (e.g. "ABBA", "ABBAACCDDC").
    pub esquema: String,
    /// Spatial arrangement category (e.g. "Interpoladas (Abraçadas)", "Alternadas", "Emparelhadas").
    pub tipo_disposicao: String,
    /// Acoustic purity classification (e.g. "Consoante (Perfeita)", "Toante (Assonante)", "Branca").
    pub pureza_acustica: String,
    /// Percentage of rhymed verses (0 to 100).
    pub percentual_rimado: u8,
}

/// Evaluates rhyme quality, acoustic purity, and disposition pattern across a poem.
pub fn avaliar_qualidade_rimas(poema: &crate::block::Poema) -> InfoAvaliacaoRima {
    let total = poema.versos.len();
    if total == 0 {
        return InfoAvaliacaoRima {
            esquema: String::new(),
            tipo_disposicao: "Sem versos".to_string(),
            pureza_acustica: "Indeterminada".to_string(),
            percentual_rimado: 0,
        };
    }

    let esquema = poema.esquema_rima.clone().unwrap_or_default();

    // Contagem de frequência de cada letra
    let mut frequencia: HashMap<char, usize> = HashMap::new();
    for c in esquema.chars() {
        *frequencia.entry(c).or_insert(0) += 1;
    }

    // Versos que rimam (com letra repetida ao menos 2 vezes)
    let versos_rimados = poema
        .versos
        .iter()
        .filter(|v| {
            if let Some(r) = &v.rima {
                if let Some(c) = r.chars().next() {
                    return frequencia.get(&c).copied().unwrap_or(0) >= 2;
                }
            }
            false
        })
        .count();

    let percentual_rimado = if total > 0 {
        ((versos_rimados as f32 / total as f32) * 100.0)
            .round()
            .clamp(0.0, 100.0) as u8
    } else {
        0
    };

    // Disposição
    let tipo_disposicao = match esquema.as_str() {
        "ABBA" => "Interpoladas (Abraçadas)".to_string(),
        "ABAB" => "Alternadas (Cruzadas)".to_string(),
        "AABB" => "Emparelhadas (Geminadas)".to_string(),
        "AAAA" => "Contínuas (Monorrimas)".to_string(),
        "ABCB" => "Alternadas Populares".to_string(),
        "ABBAACCDDC" => "Espinela (Interpoladas e Geminadas)".to_string(),
        "ABABABCC" => "Oitava Real (Alternadas com Dístico)".to_string(),
        s if s.starts_with("ABBAABBA") => "Mista Clássica (Interpoladas nas Quadras)".to_string(),
        s if s.starts_with("ABABABAB") => "Mista Clássica (Alternadas nas Quadras)".to_string(),
        s if s.len() == 6
            && s.chars().nth(1) == s.chars().nth(3)
            && s.chars().nth(3) == s.chars().nth(5) =>
        {
            "Sextilha de Cordel (Versos Pares)".to_string()
        }
        _ => {
            if percentual_rimado == 0 {
                "Versos Brancos (Sem Rima)".to_string()
            } else if frequencia.values().any(|&count| count >= 2) {
                "Mista / Livre".to_string()
            } else {
                "Sem Esquema Definido".to_string()
            }
        }
    };

    // Pureza acústica
    let mut mapa_terminacoes: HashMap<char, Vec<String>> = HashMap::new();
    for v in &poema.versos {
        if let (Some(r), Some(t)) = (&v.rima, &v.rima_terminacao) {
            if let Some(c) = r.chars().next() {
                if frequencia.get(&c).copied().unwrap_or(0) >= 2 {
                    mapa_terminacoes.entry(c).or_default().push(t.clone());
                }
            }
        }
    }

    let pureza_acustica = if mapa_terminacoes.is_empty() {
        "Branca (Versos Soltos)".to_string()
    } else {
        let mut todas_perfeitas = true;
        for terms in mapa_terminacoes.values() {
            if terms.len() >= 2 {
                let primeira = &terms[0];
                for t in &terms[1..] {
                    if t != primeira {
                        todas_perfeitas = false;
                        break;
                    }
                }
            }
        }
        if todas_perfeitas {
            "Consoante (Perfeita)".to_string()
        } else {
            "Toante (Assonante)".to_string()
        }
    };

    InfoAvaliacaoRima {
        esquema,
        tipo_disposicao,
        pureza_acustica,
        percentual_rimado,
    }
}
