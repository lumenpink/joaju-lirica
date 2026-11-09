//! Regras determinísticas de retroanálise contextual (lookback) para desambiguação sintática.

use super::dafsa::consultar_lexico;
use super::pos_mask::PosMask;

/// Aplica as regras determinísticas de retroanálise (lookback) em T_0 a partir de T_{-1} e T_{-2}.
/// Retorna `(pos_resolvida, Some("ID_REGRA"))` ou a máscara original inalterada e `None`.
pub fn desambiguar_por_contexto(
    palavra_t0: &str,
    mascara_t0: PosMask,
    tokens_anteriores: &[&str],
) -> (PosMask, Option<&'static str>) {
    if mascara_t0.is_resolved() || mascara_t0.is_empty() {
        return (mascara_t0, None);
    }

    if tokens_anteriores.is_empty() {
        return (mascara_t0, None);
    }

    let t_minus_1 = tokens_anteriores.last().unwrap().to_lowercase();
    let t_minus_2 = if tokens_anteriores.len() >= 2 {
        Some(tokens_anteriores[tokens_anteriores.len() - 2].to_lowercase())
    } else {
        None
    };

    // R-DET: Substantivação Determinada
    // Se T_{-1} pertence a {Artigos, Possessivos, Demonstrativos}
    if is_determinante(&t_minus_1) && mascara_t0.contains(PosMask::SUBSTANTIVO) {
        return (PosMask::SUBSTANTIVO, Some("R-DET"));
    }

    // R-PRON: Predicação Pessoal Imediata
    // Se T_{-1} pertence a {eu, tu, ele, ela, nós, vós, eles, elas}
    if is_pronome_reto(&t_minus_1) && mascara_t0.contains(PosMask::VERBO) {
        return (PosMask::VERBO, Some("R-PRON"));
    }
    // Suporte a clítico ou advérbio intercalado: "eu já não canto" ou "ele me ama"
    if let Some(ref t2) = t_minus_2 {
        if is_pronome_reto(t2) && (is_negacao(&t_minus_1) || is_clitico_intercalado(&t_minus_1)) {
            if mascara_t0.contains(PosMask::VERBO) {
                return (PosMask::VERBO, Some("R-PRON"));
            }
        }
    }

    // R-NEG: Negação Verbal Imediata
    if is_negacao(&t_minus_1) && mascara_t0.contains(PosMask::VERBO) {
        return (PosMask::VERBO, Some("R-NEG"));
    }

    // R-PREP: Conexão Preposicional Infinitiva
    if is_preposicao_infinitiva(&t_minus_1) && is_infinitivo(palavra_t0) {
        if mascara_t0.contains(PosMask::VERBO) {
            return (PosMask::VERBO, Some("R-PREP"));
        }
    }

    // R-MOD: Modificação Nominal Pós-Substantivo Inequívoco
    if let Some(mask_t1) = consultar_lexico(&t_minus_1) {
        if mask_t1 == PosMask::SUBSTANTIVO && mascara_t0.contains(PosMask::ADJETIVO) {
            return (PosMask::ADJETIVO, Some("R-MOD"));
        }
    }

    (mascara_t0, None)
}

fn is_determinante(t: &str) -> bool {
    matches!(
        t,
        "o" | "a"
            | "os"
            | "as"
            | "um"
            | "uma"
            | "uns"
            | "umas"
            | "meu"
            | "minha"
            | "meus"
            | "minhas"
            | "teu"
            | "tua"
            | "teus"
            | "tuas"
            | "seu"
            | "sua"
            | "seus"
            | "suas"
            | "nosso"
            | "nossa"
            | "nossos"
            | "nossas"
            | "este"
            | "esta"
            | "estes"
            | "estas"
            | "esse"
            | "essa"
            | "esses"
            | "essas"
            | "aquele"
            | "aquela"
            | "aqueles"
            | "aquelas"
    )
}

fn is_pronome_reto(t: &str) -> bool {
    matches!(
        t,
        "eu" | "tu" | "ele" | "ela" | "nós" | "vós" | "eles" | "elas"
    )
}

fn is_negacao(t: &str) -> bool {
    matches!(t, "não" | "nunca" | "jamais")
}

fn is_clitico_intercalado(t: &str) -> bool {
    matches!(t, "me" | "te" | "se" | "lhe" | "nos" | "vos" | "lhes")
}

fn is_preposicao_infinitiva(t: &str) -> bool {
    matches!(t, "para" | "de" | "a" | "sem" | "por" | "com" | "até")
}

fn is_infinitivo(p: &str) -> bool {
    let lower = p.to_lowercase();
    lower.ends_with("ar") || lower.ends_with("er") || lower.ends_with("ir") || lower.ends_with("ôr")
}
