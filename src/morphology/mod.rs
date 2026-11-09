//! Módulo determinístico de morfologia, consulta léxica e classificação de rimas.

pub mod classifier;
pub mod clitics;
pub mod dafsa;
pub mod lookback;
pub mod pos_mask;
pub mod suffixes;

pub use classifier::{
    avaliar_par_rima, classificar_rimas_poema, extrair_token_rimante, EstrofeRimas,
    IndeterminadoMotivo, JustificativaAuditoria, ParRimaAvaliado, QualidadeRima,
    RelatorioRimasPoema, TokenRimante,
};
pub use clitics::descolar_enclise;
pub use dafsa::consultar_lexico;
pub use lookback::desambiguar_por_contexto;
pub use pos_mask::PosMask;
pub use suffixes::inferir_por_sufixo;
