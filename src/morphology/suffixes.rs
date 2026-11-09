//! Autômato determinístico de derivação e sufixação produtiva do português.

use super::pos_mask::PosMask;

/// Autômato determinístico de sufixação produtiva do português para palavras desconhecidas.
pub fn inferir_por_sufixo(palavra: &str) -> Option<PosMask> {
    let p = palavra.to_lowercase();

    // Advérbios em -mente
    if p.ends_with("mente") && p.len() > 6 {
        return Some(PosMask::ADVERBIO);
    }

    // Substantivos abstratos e derivados
    let sufixos_subst = [
        "ção", "ções", "são", "sões", "dão", "dões", "eza", "ezas", "ura", "uras", "ice", "ices",
        "ismo", "ismos", "mento", "mentos", "tude", "tudes", "ância", "ência", "edouro", "edouros",
        "aria", "arias", "agem", "agens",
    ];
    for suf in sufixos_subst {
        if p.ends_with(suf) && p.len() > suf.len() + 2 {
            return Some(PosMask::SUBSTANTIVO);
        }
    }

    // Adjetivos derivados
    let sufixos_adj = [
        "oso", "osa", "osos", "osas", "vel", "veis", "al", "ais", "ico", "ica", "icos", "icas",
        "il", "is", "eiro", "eira", "ento", "enta", "ento", "enta",
    ];
    for suf in sufixos_adj {
        if p.ends_with(suf) && p.len() > suf.len() + 2 {
            return Some(PosMask::ADJETIVO);
        }
    }

    // Terminações regulares de particípio passivo (Substantivo | Adjetivo)
    if (p.ends_with("ado") || p.ends_with("ada") || p.ends_with("ido") || p.ends_with("ida"))
        && p.len() > 5
    {
        return Some(PosMask::ADJETIVO | PosMask::SUBSTANTIVO);
    }

    None
}
