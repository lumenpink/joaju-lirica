//! Syllabification algorithm preserving exact byte spans in the original text.

use crate::phonotactics::*;
use std::ops::Range;

/// Separates a single word into syllables, returning each syllable's text and its exact
/// byte range relative to `offset_base`.
pub fn separar_silabas_com_spans(
    palavra: &str,
    offset_base: usize,
    dierese_crescente: bool,
) -> Vec<(String, Range<usize>)> {
    let chars: Vec<(usize, char)> = palavra.char_indices().collect();
    if chars.is_empty() {
        return Vec::new();
    }

    // Contração átona indivisível "ao" / "aos" (ditongo decrescente oral [aw]),
    // a menos que haja forçamento explícito de hiato com barra "/" (ex.: "a/o").
    let lower_limpa = palavra
        .to_lowercase()
        .chars()
        .filter(|&c| c != '\'' && c != '/')
        .collect::<String>();
    if !palavra.contains('/') && (lower_limpa == "ao" || lower_limpa == "aos") {
        return vec![(
            palavra.to_string(),
            offset_base..offset_base + palavra.len(),
        )];
    }

    let mut silabas = Vec::new();
    let mut inicio_byte = offset_base + chars[0].0;
    let mut texto_atual = String::new();
    let n = chars.len();
    let mut i = 0;

    while i < n {
        let (_byte_idx, c) = chars[i];

        // 1. Manual hiatus / dieresis operator '/' (e.g. "ru/a")
        if c == '/' {
            if !texto_atual.is_empty() {
                let fim_byte = offset_base + chars[i].0;
                silabas.push((texto_atual.clone(), inicio_byte..fim_byte));
                texto_atual.clear();
            }
            if i + 1 < n {
                inicio_byte = offset_base + chars[i + 1].0;
            }
            i += 1;
            continue;
        }

        texto_atual.push(c);
        let prox = if i + 1 < n {
            Some(chars[i + 1].1)
        } else {
            None
        };
        let prox2 = if i + 2 < n {
            Some(chars[i + 2].1)
        } else {
            None
        };
        let ant = if i > 0 { Some(chars[i - 1].1) } else { None };

        if is_vowel(c) {
            // Mute / semivowel 'u' in qu/gu followed by a vowel does not close a syllable
            if matches!(to_lower(c), 'u' | 'ü')
                && matches!(ant.map(to_lower), Some('q' | 'g'))
                && prox.is_some_and(is_vowel)
            {
                i += 1;
                continue;
            }

            // 2. Vowel Encounter (Diphthong vs Hiatus)
            if let Some(p) = prox {
                if is_vowel(p) {
                    // Nasal diphthongs (ão, õe, ãe) never split
                    if is_tilde(c) && matches!(to_lower(p), 'a' | 'e' | 'o') {
                        i += 1;
                        continue;
                    }

                    // Hiatus if next vowel has graphic accent (e.g. sa-í-da, pa-ís)
                    if is_accented_vowel(p) {
                        let fim_byte = offset_base + chars[i + 1].0;
                        silabas.push((texto_atual.clone(), inicio_byte..fim_byte));
                        texto_atual.clear();
                        inicio_byte = fim_byte;
                        i += 1;
                        continue;
                    }

                    // Two strong vowels (e.g. po-e-ta, ca-os, vo-o) -> Hiatus
                    if is_strong_vowel(c) && is_strong_vowel(p) {
                        let fim_byte = offset_base + chars[i + 1].0;
                        silabas.push((texto_atual.clone(), inicio_byte..fim_byte));
                        texto_atual.clear();
                        inicio_byte = fim_byte;
                        i += 1;
                        continue;
                    }

                    // Semivowel (i, u) + Strong Vowel (a, e, o):
                    // In Portuguese, if there was NO graphic accent earlier in the word (e.g. po-e-si-a, ru-a, di-a, fri-o),
                    // it is phonologically guaranteed that the semivowel carries lexical stress and forms an unstressed hiatus!
                    // If there WAS a prior graphic accent (e.g. his-tó-ria, sé-rie), it forms a rising diphthong,
                    // unless `dierese_crescente` is explicitly enabled.
                    if is_semivowel(c) && is_strong_vowel(p) {
                        let ja_teve_acento = palavra[..chars[i].0].chars().any(is_accented_vowel);
                        let forcar_hiato = !ja_teve_acento || dierese_crescente;
                        if forcar_hiato {
                            let fim_byte = offset_base + chars[i + 1].0;
                            silabas.push((texto_atual.clone(), inicio_byte..fim_byte));
                            texto_atual.clear();
                            inicio_byte = fim_byte;
                            i += 1;
                            continue;
                        }
                    }

                    // Strong Vowel + Semivowel (i, u): forms falling diphthong (e.g. pai, meu),
                    // UNLESS followed by 'nh' or coda consonant (ra-iz, ru-im)
                    if is_semivowel(p) {
                        let p2_lower = prox2.map(to_lower);
                        let p3_lower = if i + 3 < n {
                            Some(to_lower(chars[i + 3].1))
                        } else {
                            None
                        };
                        let forma_hiato_nh = p2_lower == Some('n') && p3_lower == Some('h');
                        let forma_hiato_coda = matches!(p2_lower, Some('z' | 'r' | 'l' | 'm'))
                            && !p3_lower.is_some_and(is_vowel);

                        if forma_hiato_nh || forma_hiato_coda {
                            let fim_byte = offset_base + chars[i + 1].0;
                            silabas.push((texto_atual.clone(), inicio_byte..fim_byte));
                            texto_atual.clear();
                            inicio_byte = fim_byte;
                            i += 1;
                            continue;
                        }

                        // Otherwise, continues in diphthong
                        i += 1;
                        continue;
                    }
                }
            }

            // 3. Next character is Consonant: determine boundary
            if let Some(p) = prox {
                if !is_vowel(p) && p != '/' {
                    let mut j = i + 1;
                    while j < n && !is_vowel(chars[j].1) && chars[j].1 != '/' {
                        j += 1;
                    }

                    // If no more vowels remain, the rest is coda
                    if j == n {
                        for (_, ch) in chars.iter().take(n).skip(i + 1) {
                            texto_atual.push(*ch);
                        }
                        break;
                    }

                    let qtd_consoantes = j - (i + 1);
                    if qtd_consoantes == 1 {
                        // V - C - V -> Consonant moves to next syllable
                        let fim_byte = offset_base + chars[i + 1].0;
                        silabas.push((texto_atual.clone(), inicio_byte..fim_byte));
                        texto_atual.clear();
                        inicio_byte = fim_byte;
                    } else if qtd_consoantes == 2 {
                        let c1 = chars[i + 1].1;
                        let c2 = chars[i + 2].1;
                        if is_inseparable_onset(c1, c2) {
                            let fim_byte = offset_base + chars[i + 1].0;
                            silabas.push((texto_atual.clone(), inicio_byte..fim_byte));
                            texto_atual.clear();
                            inicio_byte = fim_byte;
                        } else {
                            // Split in middle
                            texto_atual.push(c1);
                            i += 1;
                            let fim_byte = offset_base + chars[i + 1].0;
                            silabas.push((texto_atual.clone(), inicio_byte..fim_byte));
                            texto_atual.clear();
                            inicio_byte = fim_byte;
                        }
                    } else {
                        // >= 3 consonants (e.g. com-prar, trans-por-te)
                        let penultima = chars[j - 2].1;
                        let ultima = chars[j - 1].1;
                        let ficam_na_coda = if is_inseparable_onset(penultima, ultima) {
                            qtd_consoantes - 2
                        } else {
                            qtd_consoantes - 1
                        };

                        for k in 0..ficam_na_coda {
                            texto_atual.push(chars[i + 1 + k].1);
                        }
                        i += ficam_na_coda;
                        let fim_byte = offset_base + chars[i + 1].0;
                        silabas.push((texto_atual.clone(), inicio_byte..fim_byte));
                        texto_atual.clear();
                        inicio_byte = fim_byte;
                    }
                }
            }
        }

        i += 1;
    }

    if !texto_atual.is_empty() {
        let fim_total = offset_base + palavra.len();
        silabas.push((texto_atual, inicio_byte..fim_total));
    }

    silabas
}
