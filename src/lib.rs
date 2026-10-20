//! # joaju-lirica
//!
//! A fast, deterministic Rust library for Portuguese poetic scansion, rhythm analysis,
//! and metric classification.
//!
//! > ⚠️ **Early Development Notice (pre-1.0 / alpha)**: This library is currently in early
//! > active development (`0.1.0`). APIs are subject to change prior to a stable release.
//!
//! ## Features
//! - **Phonotactic Syllabification**: Exact UTF-8 byte span preservation (`Range<usize>`)
//!   for editor integrations (Ratatui, egui, Dioxus, Neovim extmarks).
//! - **Lexical & Poetic Stress**: Automatic detection of acute/circumflex accents, tildes,
//!   atonic clitics, and paroxytone/oxytone endings.
//! - **Poetic Elision (Sinalefa)**: Classical vowel merging across word boundaries respecting
//!   punctuation cesura.
//! - **Metric Truncation (Castilho Rule)**: Automatic metric syllable numbering ending at
//!   the verse's final tonic syllable.
//! - **Manual Overrides**: Inline `/` (forced hiatus/dieresis) and `_` (forced elision).
//! - **Joaju Code Block Integration**: Supports ````lirica code blocks with `%%` configuration
//!   directives (`%% metro`, `%% elidir_pontuacao`, `%% dierese_crescente`, `%% saida`).
//!
//! ## Quick Example
//!
//! ```rust
//! use joaju_lirica::escandir_verso;
//!
//! let verso = "Amor é fogo que arde sem se ver";
//! let escansao = escandir_verso(verso);
//!
//! assert_eq!(escansao.total_metricas, 10);
//! assert_eq!(escansao.classificacao.as_deref(), Some("Decassílabo Heroico"));
//! ```

#![warn(missing_docs)]

pub mod block;
pub mod classifier;
pub mod phonotactics;
pub mod scansion;
pub mod syllabary;
pub mod tonic;

pub use block::{
    parse_lirica_block, render_painel, render_typst, LiricaError, ModoSaida, Poema, PoemaConfig,
};
pub use classifier::classificar_ritmo;
pub use scansion::{escandir_verso, escandir_verso_com_opcoes, EscansaoVerso, SilabaPoetica};
pub use syllabary::separar_silabas_com_spans;
pub use tonic::descobrir_tonica;

/// Module version identifier.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
