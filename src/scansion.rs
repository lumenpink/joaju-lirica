//! Verse scansion, tokenization, poetic elision (sinalefa), and Castilho metric truncation.

use crate::classifier::classificar_ritmo;
use crate::phonotactics::*;
use crate::syllabary::separar_silabas_com_spans;
use crate::tonic::descobrir_tonica;
use std::ops::Range;

/// Representation of a single poetic syllable.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SilabaPoetica {
    /// Visual text of the syllable (e.g. "que_ar" if fused).
    pub texto: String,
    /// Byte span (start..end) in the original input string.
    pub span: Range<usize>,
    /// Whether this syllable receives lexical or metric stress.
    pub tonica: bool,
    /// 1-based metric position (Some(1..=N)) up to the final tonic, or None for post-tonics.
    pub numero_metrico: Option<usize>,
    /// Whether this syllable is a result of cross-word elision (sinalefa).
    pub fusao: bool,
}

/// Complete scansion result for a single verse.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EscansaoVerso {
    /// Ordered sequence of poetic syllables.
    pub silabas: Vec<SilabaPoetica>,
    /// Total count of metric syllables according to the active rule.
    pub total_metricas: usize,
    /// Positions of all tonic syllables (e.g. [2, 6, 10]).
    pub esquema_tonico: Vec<usize>,
    /// Identified poetic rhythm or meter (e.g. "Decassílabo Heroico").
    pub classificacao: Option<String>,
}

#[derive(Debug, Clone)]
struct SilabaBruta {
    texto: String,
    span: Range<usize>,
    tonica: bool,
    fim_de_palavra: bool,
    bloqueia_sinalefa: bool,
    forcar_sinalefa_proxima: bool,
}

/// Scans a single line/verse with full custom options.
pub fn escandir_verso_com_opcoes(
    linha: &str,
    elidir_pontuacao: bool,
    dierese_crescente: bool,
    contagem_castilho: bool,
) -> EscansaoVerso {
    let mut silabas_brutas: Vec<SilabaBruta> = Vec::new();
    let n = linha.len();
    let mut i = 0;

    while i < n {
        // Skip leading whitespace
        while i < n {
            let c = linha[i..].chars().next().unwrap();
            if !c.is_whitespace() {
                break;
            }
            i += c.len_utf8();
        }
        if i >= n {
            break;
        }

        // Collect word characters (alphabetic, '/', or '-')
        let inicio_palavra = i;
        while i < n {
            let c = linha[i..].chars().next().unwrap();
            if !c.is_alphabetic() && c != '/' && c != '-' {
                break;
            }
            i += c.len_utf8();
        }
        let fim_palavra = i;
        let fatia_palavra = &linha[inicio_palavra..fim_palavra];

        // Inspect following characters for punctuation / underscores
        let mut bloqueia_sinalefa = false;
        let mut forcar_sinalefa = false;
        let mut j = fim_palavra;
        while j < n {
            let c = linha[j..].chars().next().unwrap();
            if c.is_alphabetic() || c == '/' {
                break;
            }
            if c == '_' {
                forcar_sinalefa = true;
            }
            if matches!(c, ',' | ';' | '.' | ':' | '!' | '?' | '—' | ')') && !elidir_pontuacao {
                bloqueia_sinalefa = true;
            }
            j += c.len_utf8();
        }

        let palavra_limpa: String = fatia_palavra.chars().filter(|&c| c != '/').collect();
        if !palavra_limpa.is_empty() {
            let sils = separar_silabas_com_spans(fatia_palavra, inicio_palavra, dierese_crescente);
            let idx_tonica = descobrir_tonica(&sils, &palavra_limpa);
            let total_sils = sils.len();

            for (idx, (texto_sil, span_sil)) in sils.into_iter().enumerate() {
                let eh_fim = idx + 1 == total_sils;
                silabas_brutas.push(SilabaBruta {
                    texto: texto_sil,
                    span: span_sil,
                    tonica: idx_tonica == Some(idx),
                    fim_de_palavra: eh_fim,
                    bloqueia_sinalefa: eh_fim && bloqueia_sinalefa && !forcar_sinalefa,
                    forcar_sinalefa_proxima: eh_fim && forcar_sinalefa,
                });
            }
        }

        i = j;
    }

    // Sinalefa / Elision pass
    let mut silabas_poeticas: Vec<SilabaPoetica> = Vec::new();
    let mut idx = 0;
    while idx < silabas_brutas.len() {
        let mut atual = silabas_brutas[idx].clone();

        while idx + 1 < silabas_brutas.len() {
            let prox = &silabas_brutas[idx + 1];

            let termina_vogal = atual.texto.chars().last().is_some_and(is_vowel);
            let mut chars_prox = prox.texto.chars();
            let p1 = chars_prox.next();
            let p2 = chars_prox.next();
            let comeca_som_vocalico = p1.is_some_and(is_vowel)
                || (matches!(p1, Some('h' | 'H')) && p2.is_some_and(is_vowel));

            // Vogal tônica final (oxítona ou monossílabo tônico) não forma sinalefa
            // com a vogal seguinte (hiato obrigatório), a menos que explicitamente forçada com '_'.
            let pode_fundir = (atual.fim_de_palavra
                && !atual.bloqueia_sinalefa
                && termina_vogal
                && comeca_som_vocalico
                && !atual.tonica)
                || atual.forcar_sinalefa_proxima;

            if pode_fundir {
                atual.texto = format!("{}_{}", atual.texto, prox.texto);
                atual.span = atual.span.start..prox.span.end;
                atual.tonica = atual.tonica || prox.tonica;
                atual.fim_de_palavra = prox.fim_de_palavra;
                atual.bloqueia_sinalefa = prox.bloqueia_sinalefa;
                atual.forcar_sinalefa_proxima = prox.forcar_sinalefa_proxima;
                idx += 1; // Consume next syllable in fusion
            } else {
                break;
            }
        }

        let houve_fusao = atual.texto.contains('_');
        silabas_poeticas.push(SilabaPoetica {
            texto: atual.texto,
            span: atual.span,
            tonica: atual.tonica,
            numero_metrico: None,
            fusao: houve_fusao,
        });

        idx += 1;
    }

    // Metric numbering pass (Castilho rule or full count)
    let mut esquema_tonico = Vec::new();
    let mut total_metricas = 0;

    if contagem_castilho {
        let ultima_tonica_idx = silabas_poeticas.iter().rposition(|s| s.tonica);
        if let Some(ult_idx) = ultima_tonica_idx {
            total_metricas = ult_idx + 1;
            for (i, sil) in silabas_poeticas.iter_mut().enumerate() {
                if i <= ult_idx {
                    let num = i + 1;
                    sil.numero_metrico = Some(num);
                    if sil.tonica {
                        esquema_tonico.push(num);
                    }
                }
            }
        }
    } else {
        total_metricas = silabas_poeticas.len();
        for (i, sil) in silabas_poeticas.iter_mut().enumerate() {
            let num = i + 1;
            sil.numero_metrico = Some(num);
            if sil.tonica {
                esquema_tonico.push(num);
            }
        }
    }

    let classificacao = classificar_ritmo(total_metricas, &esquema_tonico);

    EscansaoVerso {
        silabas: silabas_poeticas,
        total_metricas,
        esquema_tonico,
        classificacao,
    }
}

/// Scans a single line/verse using standard classical defaults (Castilho rule, cesura on punctuation).
pub fn escandir_verso(linha: &str) -> EscansaoVerso {
    escandir_verso_com_opcoes(linha, false, false, true)
}
