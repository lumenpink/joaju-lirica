//! Parser for `lirica` code blocks and `%%` directives with dual-panel and Typst renderers.

use crate::rhyme::analisar_esquema_rimas;
use crate::scansion::{escandir_verso_com_opcoes, EscansaoVerso};
use std::collections::HashMap;
use thiserror::Error;

/// Errors during lirica block parsing or processing.
#[derive(Error, Debug)]
pub enum LiricaError {
    /// The provided block contains no verse lines.
    #[error("Empty poem block")]
    EmptyBlock,
}

/// Output rendering format.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ModoSaida {
    /// Human-readable dual-panel HedgeDoc style.
    #[default]
    Painel,
    /// Typst academic/typographical markup.
    Typst,
    /// Structured JSON data.
    Json,
    /// Plain text without annotations.
    Texto,
}

/// Configuration directives parsed from `%%` headers.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PoemaConfig {
    /// Poem title.
    pub titulo: Option<String>,
    /// Poem author.
    pub autor: Option<String>,
    /// Target meter expected (e.g. "decassilabo").
    pub metro_alvo: Option<String>,
    /// Whether punctuation creates a cesura that blocks elision (default: false).
    pub elidir_pontuacao: bool,
    /// Whether rising diphthongs are treated as hiatuses (default: false).
    pub dierese_crescente: bool,
    /// Whether to terminate counting at the final tonic (Castilho rule, default: true).
    pub contagem_castilho: bool,
    /// Whether to extract and analyze end rhymes (default: true).
    pub analisar_rima: bool,
    /// Output mode.
    pub modo_saida: ModoSaida,
}

impl Default for PoemaConfig {
    fn default() -> Self {
        Self {
            titulo: None,
            autor: None,
            metro_alvo: None,
            elidir_pontuacao: false,
            dierese_crescente: false,
            contagem_castilho: true,
            analisar_rima: true,
            modo_saida: ModoSaida::Painel,
        }
    }
}

/// Analyzed poem containing scansion results for each line.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Poema {
    /// Scansion results for each verse.
    pub versos: Vec<EscansaoVerso>,
    /// Raw original lines.
    pub linhas_originais: Vec<String>,
    /// Overall detected rhyme scheme (e.g. "ABBA", "ABCB").
    pub esquema_rima: Option<String>,
}

/// Parses a `lirica` block containing optional `%%` headers followed by verses.
pub fn parse_lirica_block(bloco: &str) -> Result<(PoemaConfig, Poema), LiricaError> {
    let mut directives = HashMap::new();
    let mut verse_lines = Vec::new();

    for line in bloco.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("%%") {
            let directive_str = trimmed.trim_start_matches('%').trim();
            if let Some((k, v)) = directive_str.split_once('=') {
                directives.insert(
                    k.trim().to_lowercase(),
                    v.trim().trim_matches('"').trim_matches('\'').to_string(),
                );
            }
        } else if !trimmed.is_empty() {
            verse_lines.push(line.to_string());
        }
    }

    let mut config = PoemaConfig::default();

    if let Some(t) = directives.get("titulo").or_else(|| directives.get("title")) {
        config.titulo = Some(t.clone());
    }
    if let Some(a) = directives.get("autor").or_else(|| directives.get("author")) {
        config.autor = Some(a.clone());
    }
    if let Some(m) = directives.get("metro") {
        config.metro_alvo = Some(m.to_lowercase());
    }
    if let Some(el) = directives.get("elidir_pontuacao") {
        config.elidir_pontuacao = matches!(el.as_str(), "true" | "1" | "sim" | "yes");
    }
    if let Some(dc) = directives.get("dierese_crescente") {
        config.dierese_crescente = matches!(dc.as_str(), "true" | "1" | "sim" | "yes");
    }
    if let Some(rc) = directives
        .get("regra_contagem")
        .or_else(|| directives.get("contagem"))
    {
        config.contagem_castilho = rc.to_lowercase() != "absoluta";
    }
    if let Some(rima) = directives.get("rima") {
        config.analisar_rima = matches!(rima.as_str(), "true" | "1" | "sim" | "yes");
    }
    if let Some(s) = directives.get("saida") {
        config.modo_saida = match s.to_lowercase().as_str() {
            "typst" => ModoSaida::Typst,
            "json" => ModoSaida::Json,
            "texto" => ModoSaida::Texto,
            _ => ModoSaida::Painel,
        };
    }

    let mut versos = Vec::new();
    for linha in &verse_lines {
        let escansao = escandir_verso_com_opcoes(
            linha,
            config.elidir_pontuacao,
            config.dierese_crescente,
            config.contagem_castilho,
        );
        versos.push(escansao);
    }

    let mut poem = Poema {
        versos,
        linhas_originais: verse_lines.clone(),
        esquema_rima: None,
    };

    if config.analisar_rima {
        let (info_rimas, esquema) = analisar_esquema_rimas(&verse_lines);
        poem.esquema_rima = esquema;
        for (v, rima_opt) in poem.versos.iter_mut().zip(info_rimas) {
            if let Some(r) = rima_opt {
                v.rima = Some(r.letra);
                v.rima_terminacao = Some(r.terminacao);
            }
        }
    }

    Ok((config, poem))
}

/// Renders a poem in dual-panel HedgeDoc style.
pub fn render_painel(poema: &Poema) -> String {
    let mut out = String::new();
    for (orig, esc) in poema.linhas_originais.iter().zip(poema.versos.iter()) {
        let partes: Vec<String> = esc
            .silabas
            .iter()
            .map(|s| {
                let txt = if s.tonica {
                    format!("[{}]", s.texto)
                } else {
                    s.texto.clone()
                };
                if let Some(num) = s.numero_metrico {
                    format!("{}:{}", num, txt)
                } else {
                    format!("·{}", txt)
                }
            })
            .collect();

        let class_str = esc.classificacao.as_deref().unwrap_or("Indeterminado");
        let rima_str = if let Some(r) = &esc.rima {
            format!("[{}] ", r)
        } else {
            String::new()
        };

        out.push_str(&format!(
            "{:<40} │ {} │ {}{} [Tônicas: {:?}]\n",
            orig,
            partes.join("-"),
            rima_str,
            class_str,
            esc.esquema_tonico
        ));
    }

    if let Some(esq) = &poema.esquema_rima {
        out.push_str(&format!("\nEsquema Rímico: {}\n", esq));
    }

    out
}

/// Renders a poem into native Typst markup.
pub fn render_typst(config: &PoemaConfig, poema: &Poema) -> String {
    let mut typst = String::new();
    if let Some(titulo) = &config.titulo {
        typst.push_str(&format!("== {}\n", titulo));
    }
    if let Some(autor) = &config.autor {
        typst.push_str(&format!("_por {}_\n\n", autor));
    }

    typst.push_str("#align(center)[\n#block(width: auto)[\n");
    for (orig, esc) in poema.linhas_originais.iter().zip(poema.versos.iter()) {
        let class = esc.classificacao.as_deref().unwrap_or("");
        let rima = esc.rima.as_deref().unwrap_or("");
        typst.push_str(&format!(
            "  #grid(columns: (1fr, auto), align: (left, right), [{}], text(size: 8pt, fill: luma(120))[{} {} ({}s)])\n",
            orig, rima, class, esc.total_metricas
        ));
    }
    typst.push_str("]\n]\n");
    typst
}
