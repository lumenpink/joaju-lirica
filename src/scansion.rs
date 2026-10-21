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
    /// Scheme rhyme letter if analyzed (e.g. "A", "B").
    pub rima: Option<String>,
    /// Rhyme termination phonemes (e.g. "-er", "-ente").
    pub rima_terminacao: Option<String>,
}

#[derive(Debug, Clone)]
struct SilabaBruta {
    texto: String,
    span: Range<usize>,
    tonica: bool,
    fim_de_palavra: bool,
    eh_monossilabo_sem_acento: bool,
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
        // 1. Skip leading whitespace
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

        // 2. Standalone '/' between words forces hiatus on the preceding syllable
        let c_char = linha[i..].chars().next().unwrap();
        if c_char == '/' {
            if let Some(last) = silabas_brutas.last_mut() {
                last.bloqueia_sinalefa = true;
                last.forcar_sinalefa_proxima = false;
            }
            i += c_char.len_utf8();
            continue;
        }

        // 3. Collect word characters (alphabetic, '\'', or internal '/')
        let inicio_palavra = i;
        while i < n {
            let c = linha[i..].chars().next().unwrap();
            if c.is_alphabetic() || c == '\'' || c == '-' {
                i += c.len_utf8();
            } else if c == '/'
                && i + 1 < n
                && linha[i + 1..]
                    .chars()
                    .next()
                    .is_some_and(|nc| nc.is_alphabetic())
            {
                // Internal '/' between letters in a word (e.g. sa/úva)
                i += c.len_utf8();
            } else {
                break;
            }
        }
        let fim_palavra = i;
        let fatia_palavra = &linha[inicio_palavra..fim_palavra];

        // 4. Inspect following characters for punctuation, underscores, or slashes
        let mut bloqueia_sinalefa = false;
        let mut forcar_sinalefa = false;
        let mut j = fim_palavra;
        while j < n {
            let c = linha[j..].chars().next().unwrap();
            if c.is_alphabetic() {
                break;
            }
            if c == '/' {
                bloqueia_sinalefa = true;
                forcar_sinalefa = false;
            } else if c == '_' {
                forcar_sinalefa = true;
                bloqueia_sinalefa = false;
            } else if matches!(
                c,
                ',' | ';'
                    | '.'
                    | ':'
                    | '!'
                    | '?'
                    | '—'
                    | '-'
                    | ')'
                    | ']'
                    | '…'
                    | '\"'
                    | '«'
                    | '»'
                    | '”'
            ) && !elidir_pontuacao
            {
                bloqueia_sinalefa = true;
            }
            j += c.len_utf8();
        }

        let palavra_limpa: String = fatia_palavra.chars().filter(|&c| c != '/').collect();
        if !palavra_limpa.is_empty() {
            let sils = separar_silabas_com_spans(fatia_palavra, inicio_palavra, dierese_crescente);
            let idx_tonica = descobrir_tonica(&sils, &palavra_limpa);
            let total_sils = sils.len();
            let eh_monossilabo_sem_acento =
                total_sils == 1 && !palavra_limpa.chars().any(is_acute_or_circumflex);

            for (idx, (texto_sil, span_sil)) in sils.into_iter().enumerate() {
                let eh_fim = idx + 1 == total_sils;
                let texto_limpo: String = texto_sil.chars().filter(|&c| c != '\'').collect();
                silabas_brutas.push(SilabaBruta {
                    texto: texto_limpo,
                    span: span_sil,
                    tonica: idx_tonica == Some(idx),
                    fim_de_palavra: eh_fim,
                    eh_monossilabo_sem_acento,
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

            // Vogais tônicas finais (oxítonas e monossílabos acentuados) impõem hiato obrigatório,
            // exceto se for monossílabo sem acento gráfico (ex: "si", "ti") ou com forçamento expresso '_'.
            let bloqueia_por_tonica = atual.tonica && !atual.eh_monossilabo_sem_acento;
            let pode_fundir = (atual.fim_de_palavra
                && !atual.bloqueia_sinalefa
                && termina_vogal
                && comeca_som_vocalico
                && !bloqueia_por_tonica)
                || atual.forcar_sinalefa_proxima;

            if pode_fundir {
                atual.texto = format!("{}_{}", atual.texto, prox.texto);
                atual.span = atual.span.start..prox.span.end;
                atual.tonica = atual.tonica || prox.tonica;
                atual.fim_de_palavra = prox.fim_de_palavra;
                atual.bloqueia_sinalefa = prox.bloqueia_sinalefa;
                let forcar_prox = prox.forcar_sinalefa_proxima;
                idx += 1; // Consume next syllable in fusion

                // A sinalefa canônica na métrica lusófona é binária (une 2 sílabas vocálicas contíguas).
                // Não absorve uma 3ª palavra sucessiva em tritongo, a menos que haja '_' forçado.
                if !forcar_prox {
                    break;
                }
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
            for (i, s) in silabas_poeticas.iter_mut().enumerate() {
                if i <= ult_idx {
                    total_metricas += 1;
                    s.numero_metrico = Some(total_metricas);
                    if s.tonica {
                        esquema_tonico.push(total_metricas);
                    }
                } else {
                    s.numero_metrico = None;
                }
            }
        } else {
            // No tonic found, count all
            for s in &mut silabas_poeticas {
                total_metricas += 1;
                s.numero_metrico = Some(total_metricas);
            }
        }
    } else {
        for s in &mut silabas_poeticas {
            total_metricas += 1;
            s.numero_metrico = Some(total_metricas);
            if s.tonica {
                esquema_tonico.push(total_metricas);
            }
        }
    }

    let classificacao = classificar_ritmo(total_metricas, &esquema_tonico);

    EscansaoVerso {
        silabas: silabas_poeticas,
        total_metricas,
        esquema_tonico,
        classificacao,
        rima: None,
        rima_terminacao: None,
    }
}

/// Scans a single line/verse using standard default options.
pub fn escandir_verso(linha: &str) -> EscansaoVerso {
    escandir_verso_com_opcoes(linha, false, false, true)
}
