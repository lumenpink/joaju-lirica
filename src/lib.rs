//! # joaju-lirica
//!
//! A fast, deterministic Rust library for Portuguese poetic scansion, rhythm analysis,
//! metric classification, and rhyme schemes.

#![warn(missing_docs)]

pub mod block;
pub mod classifier;
pub mod phonotactics;
pub mod poetic_form;
pub mod rhyme;
pub mod scansion;
pub mod syllabary;
pub mod tonic;

pub use block::{
    parse_lirica_block, render_painel, render_typst, Estrofe, LiricaError, ModoSaida, Poema,
    PoemaConfig,
};
pub use classifier::{classificar_ritmo, icone_metro};
pub use poetic_form::{avaliar_forma_poetica, DiagnosticoForma};
pub use rhyme::{
    analisar_esquema_rimas, avaliar_qualidade_rimas, extrair_terminacao_rima, InfoAvaliacaoRima,
    InfoRima,
};
pub use scansion::{escandir_verso, escandir_verso_com_opcoes, EscansaoVerso, SilabaPoetica};
pub use syllabary::separar_silabas_com_spans;
pub use tonic::descobrir_tonica;

/// Scans a multi-line poem text block with full options, automatically executing scansion,
/// stanza grouping, and rhyme detection across all verses.
pub fn escandir_poema(
    texto: &str,
    elidir_pontuacao: bool,
    dierese_crescente: bool,
    contagem_castilho: bool,
    analisar_rima: bool,
) -> Poema {
    let mut linhas = Vec::new();
    let mut versos = Vec::new();
    let mut stanza_lines: Vec<Vec<String>> = Vec::new();
    let mut current_stanza: Vec<String> = Vec::new();

    for linha in texto.lines() {
        let trimmed = linha.trim();
        if trimmed.starts_with("%%") {
            continue;
        } else if trimmed.is_empty() {
            if !current_stanza.is_empty() {
                stanza_lines.push(current_stanza);
                current_stanza = Vec::new();
            }
        } else {
            let esc = escandir_verso_com_opcoes(
                linha,
                elidir_pontuacao,
                dierese_crescente,
                contagem_castilho,
            );
            versos.push(esc);
            linhas.push(linha.to_string());
            current_stanza.push(linha.to_string());
        }
    }
    if !current_stanza.is_empty() {
        stanza_lines.push(current_stanza);
    }

    let esquema_rima = if analisar_rima && !linhas.is_empty() {
        let (info_rimas, esquema) = analisar_esquema_rimas(&linhas);
        for (v, rima_opt) in versos.iter_mut().zip(info_rimas) {
            if let Some(r) = rima_opt {
                v.rima = Some(r.letra);
                v.rima_terminacao = Some(r.terminacao);
            }
        }
        esquema
    } else {
        None
    };

    let mut estrofes = Vec::new();
    let mut offset = 0;
    for (i, s_lines) in stanza_lines.iter().enumerate() {
        let count = s_lines.len();
        let s_versos = versos[offset..offset + count].to_vec();
        offset += count;
        estrofes.push(Estrofe {
            indice: i,
            versos: s_versos,
            linhas_originais: s_lines.clone(),
        });
    }

    Poema {
        versos,
        linhas_originais: linhas,
        estrofes,
        esquema_rima,
    }
}

/// Module version identifier.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
