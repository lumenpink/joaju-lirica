//! # joaju-lirica
//!
//! A fast, deterministic Rust library for Portuguese poetic scansion, rhythm analysis,
//! metric classification, and rhyme schemes.

#![warn(missing_docs)]

pub mod block;
pub mod classifier;
pub mod phonotactics;
pub mod rhyme;
pub mod scansion;
pub mod syllabary;
pub mod tonic;

pub use block::{
    parse_lirica_block, render_painel, render_typst, LiricaError, ModoSaida, Poema, PoemaConfig,
};
pub use classifier::{classificar_ritmo, icone_metro};
pub use rhyme::{analisar_esquema_rimas, extrair_terminacao_rima, InfoRima};
pub use scansion::{escandir_verso, escandir_verso_com_opcoes, EscansaoVerso, SilabaPoetica};
pub use syllabary::separar_silabas_com_spans;
pub use tonic::descobrir_tonica;

/// Scans a multi-line poem text block with full options, automatically executing scansion
/// and rhyme detection across all verses.
pub fn escandir_poema(
    texto: &str,
    elidir_pontuacao: bool,
    dierese_crescente: bool,
    contagem_castilho: bool,
    analisar_rima: bool,
) -> Poema {
    let mut linhas = Vec::new();
    let mut versos = Vec::new();

    for linha in texto.lines() {
        let trimmed = linha.trim();
        if !trimmed.is_empty() && !trimmed.starts_with("%%") {
            let esc = escandir_verso_com_opcoes(
                linha,
                elidir_pontuacao,
                dierese_crescente,
                contagem_castilho,
            );
            versos.push(esc);
            linhas.push(linha.to_string());
        }
    }

    let mut poema = Poema {
        versos,
        linhas_originais: linhas.clone(),
        esquema_rima: None,
    };

    if analisar_rima && !linhas.is_empty() {
        let (info_rimas, esquema) = analisar_esquema_rimas(&linhas);
        poema.esquema_rima = esquema;
        for (v, rima_opt) in poema.versos.iter_mut().zip(info_rimas) {
            if let Some(r) = rima_opt {
                v.rima = Some(r.letra);
                v.rima_terminacao = Some(r.terminacao);
            }
        }
    }

    poema
}

/// Module version identifier.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
