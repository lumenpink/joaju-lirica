//! Poetic forms evaluation, structural pattern matching, and fuzzy adherence scoring.

use crate::block::Poema;

/// Structural diagnosis and adherence score for a poetic form.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DiagnosticoForma {
    /// Form name (e.g. "Décima Espinela", "Soneto Italiano (Petrarquiano)").
    pub nome: String,
    /// Percentage score of structural and metric adherence (0 to 100).
    pub confianca_percentual: u8,
    /// Explanatory description of the poetic form.
    pub descricao: String,
    /// Canonical metric pattern expected.
    pub padrao_esperado: String,
    /// Detected stanza and verse layout (e.g. "4 + 4 + 3 + 3 versos").
    pub estrutura_detectada: String,
    /// Detailed observations and diagnostic deviations found.
    pub observacoes: Vec<String>,
}

/// Evaluates a poem against classical forms and returns the best matching diagnostic.
pub fn avaliar_forma_poetica(poema: &Poema) -> Option<DiagnosticoForma> {
    let total_versos = poema.versos.len();
    if total_versos == 0 {
        return None;
    }

    let esquema = poema.esquema_rima.clone().unwrap_or_default();
    let num_estrofes = poema.estrofes.len();
    let estrofes_layout: Vec<usize> = if num_estrofes > 0 {
        poema.estrofes.iter().map(|e| e.versos.len()).collect()
    } else {
        vec![total_versos]
    };
    let layout_str = estrofes_layout
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join(" + ")
        + &format!(" = {} versos", total_versos);

    let mut candidatos: Vec<DiagnosticoForma> = Vec::new();

    // 1. Décima Espinela (10 versos, Heptassílabo ou Decassílabo, ABBAACCDDC)
    if let Some(diag) = avaliar_decima_espinela(poema, &esquema, &estrofes_layout, &layout_str) {
        candidatos.push(diag);
    }

    // 2. Soneto Italiano / Petrarquiano (14 versos, 4+4+3+3, Decassílabo ou Alexandrino)
    if let Some(diag) = avaliar_soneto_italiano(poema, &esquema, &estrofes_layout, &layout_str) {
        candidatos.push(diag);
    }

    // 3. Soneto Inglês / Shakespeariano (14 versos, 4+4+4+2, Decassílabo, ABAB CDCD EFEF GG)
    if let Some(diag) = avaliar_soneto_ingles(poema, &esquema, &estrofes_layout, &layout_str) {
        candidatos.push(diag);
    }

    // 4. Martelo Agalopado (10 versos, Decassílabos com 3,6,10)
    if let Some(diag) = avaliar_martelo_agalopado(poema, &esquema, &estrofes_layout, &layout_str) {
        candidatos.push(diag);
    }

    // 5. Oitava Camoniana / Rima Real (8 versos, Decassílabo, ABABABCC)
    if let Some(diag) = avaliar_oitava_camoniana(poema, &esquema, &estrofes_layout, &layout_str) {
        candidatos.push(diag);
    }

    // 6. Sextilha de Cordel (6 versos, Heptassílabo)
    if let Some(diag) = avaliar_sextilha(poema, &esquema, &estrofes_layout, &layout_str) {
        candidatos.push(diag);
    }

    // 7. Quadra / Trova (4 versos, Heptassílabo)
    if let Some(diag) = avaliar_quadra_trova(poema, &esquema, &estrofes_layout, &layout_str) {
        candidatos.push(diag);
    }

    // 8. Haicai (3 versos, 5-7-5)
    if let Some(diag) = avaliar_haicai(poema, &estrofes_layout, &layout_str) {
        candidatos.push(diag);
    }

    // Retorna o candidato com maior confiança
    candidatos
        .into_iter()
        .max_by_key(|c| c.confianca_percentual)
}

fn avaliar_decima_espinela(
    poema: &Poema,
    esquema: &str,
    estrofes_layout: &[usize],
    layout_str: &str,
) -> Option<DiagnosticoForma> {
    let total = poema.versos.len();
    // Relevante se tiver entre 8 e 12 versos ou esquema começar com ABBAA
    if !(8..=12).contains(&total) {
        return None;
    }

    let mut score: f32 = 0.0;
    let mut obs = Vec::new();

    // 1. Contagem de versos (peso 35%)
    if total == 10 {
        score += 35.0;
    } else {
        score += 35.0 * (1.0 - (total as f32 - 10.0).abs() / 10.0).max(0.0);
        obs.push(format!("Possui {} versos em vez dos 10 exigidos.", total));
    }

    // Estrofes: Espinela canônica é estrofe única de 10 ou 4+6
    if estrofes_layout.len() == 1 || estrofes_layout == [4, 6] {
        // bônus estrutural
    } else if estrofes_layout.len() > 1 {
        obs.push(format!("Estrutura dividida em {} estrofes (o padrão clássico de Espinel é estrofe contínua de 10 com cesura no 4º verso).", estrofes_layout.len()));
    }

    // 2. Métrica (peso 35%): Redondilha Maior (7s) ou Decassílabo (10s)
    let heptassilabos = poema
        .versos
        .iter()
        .filter(|v| v.total_metricas == 7)
        .count();
    let decassilabos = poema
        .versos
        .iter()
        .filter(|v| v.total_metricas == 10)
        .count();
    let alvo_metro = if heptassilabos >= decassilabos { 7 } else { 10 };
    let nome_metro = if alvo_metro == 7 {
        "Redondilha Maior (7 sílabas)"
    } else {
        "Decassílabo (10 sílabas)"
    };

    let conformes = poema
        .versos
        .iter()
        .filter(|v| v.total_metricas == alvo_metro)
        .count();
    score += 35.0 * (conformes as f32 / total as f32);

    for (idx, v) in poema.versos.iter().enumerate() {
        if v.total_metricas != alvo_metro {
            obs.push(format!(
                "Verso {} possui {} sílabas métricas em vez de {}.",
                idx + 1,
                v.total_metricas,
                alvo_metro
            ));
        }
    }

    // 3. Rima (peso 30%): ABBAACCDDC
    let rima_alvo = "ABBAACCDDC";
    let match_rimas = comparar_esquemas(esquema, rima_alvo);
    score += 30.0 * match_rimas;

    if match_rimas < 0.95 {
        obs.push(format!(
            "Esquema de rimas '{}' difere do padrão espineliano estrito 'ABBAACCDDC'.",
            esquema
        ));
    }

    let final_score = score.round().clamp(0.0, 100.0) as u8;
    if final_score >= 40 {
        Some(DiagnosticoForma {
            nome: "Décima Espinela".to_string(),
            confianca_percentual: final_score,
            descricao: format!("Forma estrófica clássica criada por Vicente Espinel (1591), composta por 10 versos em {} com rimas estritas ABBAACCDDC e pausa obrigatória no 4º verso.", nome_metro),
            padrao_esperado: format!("10 versos em {} | Rimas ABBAACCDDC", nome_metro),
            estrutura_detectada: layout_str.to_string(),
            observacoes: obs,
        })
    } else {
        None
    }
}

fn avaliar_soneto_italiano(
    poema: &Poema,
    esquema: &str,
    estrofes_layout: &[usize],
    layout_str: &str,
) -> Option<DiagnosticoForma> {
    let total = poema.versos.len();
    if !(11..=16).contains(&total) {
        return None;
    }

    let mut score: f32 = 0.0;
    let mut obs = Vec::new();

    // 1. Contagem e Estrofes (peso 35%)
    if total == 14 {
        score += 20.0;
    } else {
        score += 20.0 * (1.0 - (total as f32 - 14.0).abs() / 14.0).max(0.0);
        obs.push(format!("Possui {} versos em vez dos 14 canônicos.", total));
    }

    if estrofes_layout == [4, 4, 3, 3] {
        score += 15.0;
    } else if estrofes_layout.len() == 1 {
        score += 7.0;
        obs.push("Versos agrupados em bloco único sem separação por linhas em branco em quartetos e tercetos.".to_string());
    } else {
        score += 5.0;
        obs.push(format!("Divisão estrófica ({}) desvia do padrão 4 + 4 + 3 + 3 (dois quartetos e dois tercetos).", layout_str));
    }

    // 2. Métrica (peso 35%): Decassílabos (10s) ou Alexandrinos (12s)
    let decassilabos = poema
        .versos
        .iter()
        .filter(|v| v.total_metricas == 10)
        .count();
    let alexandrinos = poema
        .versos
        .iter()
        .filter(|v| v.total_metricas == 12)
        .count();
    let alvo_metro = if alexandrinos > decassilabos { 12 } else { 10 };
    let nome_metro = if alvo_metro == 12 {
        "Alexandrino (12 sílabas)"
    } else {
        "Decassílabo (10 sílabas)"
    };

    let conformes = poema
        .versos
        .iter()
        .filter(|v| v.total_metricas == alvo_metro)
        .count();
    score += 35.0 * (conformes as f32 / total as f32);

    for (idx, v) in poema.versos.iter().enumerate() {
        if v.total_metricas != alvo_metro {
            obs.push(format!(
                "Verso {} possui {} sílabas métricas em vez de {}.",
                idx + 1,
                v.total_metricas,
                alvo_metro
            ));
        }
    }

    // 3. Rimas (peso 30%): Quadras ABBA ABBA (ou ABAB ABAB) e Tercetos CDC DCD / CDE CDE
    let padroes_soneto = [
        "ABBAABBACDCDCD",
        "ABBAABBACDECDE",
        "ABBAABBACDEDCE",
        "ABBAABBACDEDEC",
        "ABABABABCDCDCD",
        "ABABABABCDECDE",
    ];

    let mut melhor_match: f32 = 0.0;
    for &padrao in &padroes_soneto {
        let m = comparar_esquemas(esquema, padrao);
        if m > melhor_match {
            melhor_match = m;
        }
    }
    score += 30.0 * melhor_match;

    if melhor_match < 0.90 && !esquema.is_empty() {
        obs.push(format!("Esquema de rimas '{}' apresenta variações nos tercetos ou quadras em relação aos modelos clássicos petrarquianos.", esquema));
    }

    let final_score = score.round().clamp(0.0, 100.0) as u8;
    if final_score >= 40 {
        Some(DiagnosticoForma {
            nome: "Soneto Italiano (Petrarquiano)".to_string(),
            confianca_percentual: final_score,
            descricao: format!("Forma lírica suprema introduzida por Francesco Petrarca e consolidada em português por Camões. 14 versos em {} estruturados em dois quartetos e dois tercetos (4+4+3+3).", nome_metro),
            padrao_esperado: format!("14 versos em {} (4+4+3+3) | Rimas ABBA ABBA CDC DCD", nome_metro),
            estrutura_detectada: layout_str.to_string(),
            observacoes: obs,
        })
    } else {
        None
    }
}

fn avaliar_soneto_ingles(
    poema: &Poema,
    esquema: &str,
    estrofes_layout: &[usize],
    layout_str: &str,
) -> Option<DiagnosticoForma> {
    let total = poema.versos.len();
    if !(11..=16).contains(&total) {
        return None;
    }

    let mut score: f32 = 0.0;
    let mut obs = Vec::new();

    // 1. Contagem e Estrofes (peso 35%)
    if total == 14 {
        score += 20.0;
    } else {
        score += 20.0 * (1.0 - (total as f32 - 14.0).abs() / 14.0).max(0.0);
    }

    if estrofes_layout == [4, 4, 4, 2] {
        score += 15.0;
    } else if estrofes_layout.len() == 1 {
        score += 7.0;
    } else {
        score += 4.0;
    }

    // 2. Métrica (peso 35%): Decassílabo
    let conformes = poema
        .versos
        .iter()
        .filter(|v| v.total_metricas == 10)
        .count();
    score += 35.0 * (conformes as f32 / total as f32);

    for (idx, v) in poema.versos.iter().enumerate() {
        if v.total_metricas != 10 {
            obs.push(format!(
                "Verso {} possui {} sílabas métricas em vez de 10.",
                idx + 1,
                v.total_metricas
            ));
        }
    }

    // 3. Rimas (peso 30%): ABAB CDCD EFEF GG
    let match1 = comparar_esquemas(esquema, "ABABCDCDXEXFGG");
    let match2 = comparar_esquemas(esquema, "ABABCDCDCDCDEE");
    let match_final = match1.max(match2);
    score += 30.0 * match_final;

    let final_score = score.round().clamp(0.0, 100.0) as u8;
    if final_score >= 45 {
        Some(DiagnosticoForma {
            nome: "Soneto Inglês (Shakespeariano)".to_string(),
            confianca_percentual: final_score,
            descricao: "Soneto elisabetano consagrado por William Shakespeare: 14 versos decassílabos distribuídos em três quartetos com rimas alternadas e um dístico de ouro final (4+4+4+2, ABAB CDCD EFEF GG).".to_string(),
            padrao_esperado: "14 versos decassílabos (4+4+4+2) | Rimas ABAB CDCD EFEF GG".to_string(),
            estrutura_detectada: layout_str.to_string(),
            observacoes: obs,
        })
    } else {
        None
    }
}

fn avaliar_martelo_agalopado(
    poema: &Poema,
    esquema: &str,
    _estrofes_layout: &[usize],
    layout_str: &str,
) -> Option<DiagnosticoForma> {
    let total = poema.versos.len();
    if !(8..=12).contains(&total) {
        return None;
    }

    let martelos = poema
        .versos
        .iter()
        .filter(|v| {
            v.total_metricas == 10
                && v.esquema_tonico.contains(&3)
                && v.esquema_tonico.contains(&6)
                && v.esquema_tonico.contains(&10)
        })
        .count();

    if martelos == 0 && !esquema.starts_with("AABB") {
        return None;
    }

    let mut score = 30.0 * (1.0 - (total as f32 - 10.0).abs() / 10.0).max(0.0);
    score += 40.0 * (martelos as f32 / total as f32);
    let match_rima =
        comparar_esquemas(esquema, "AABBAABBCC").max(comparar_esquemas(esquema, "ABBAACCDDC"));
    score += 30.0 * match_rima;

    let final_score = score.round().clamp(0.0, 100.0) as u8;
    if final_score >= 50 {
        Some(DiagnosticoForma {
            nome: "Martelo Agalopado".to_string(),
            confianca_percentual: final_score,
            descricao: "Modalidade tradicional do repente nordestino e cordel: décima em decassílabos cantados em ritmo acelerado com acentuações marcadas na 3ª, 6ª e 10ª sílabas métricas.".to_string(),
            padrao_esperado: "10 versos decassílabos martelo (3, 6, 10) | Rimas AABBAABBCC ou ABBAACCDDC".to_string(),
            estrutura_detectada: layout_str.to_string(),
            observacoes: Vec::new(),
        })
    } else {
        None
    }
}

fn avaliar_oitava_camoniana(
    poema: &Poema,
    esquema: &str,
    _estrofes_layout: &[usize],
    layout_str: &str,
) -> Option<DiagnosticoForma> {
    let total = poema.versos.len();
    if !(7..=9).contains(&total) {
        return None;
    }

    let mut score = 35.0 * (1.0 - (total as f32 - 8.0).abs() / 8.0).max(0.0);
    let decassilabos = poema
        .versos
        .iter()
        .filter(|v| v.total_metricas == 10)
        .count();
    score += 35.0 * (decassilabos as f32 / total as f32);
    let match_rima = comparar_esquemas(esquema, "ABABABCC");
    score += 30.0 * match_rima;

    let final_score = score.round().clamp(0.0, 100.0) as u8;
    if final_score >= 50 {
        Some(DiagnosticoForma {
            nome: "Oitava Camoniana (Rima Real)".to_string(),
            confianca_percentual: final_score,
            descricao: "A estrofe épica renascentista imortalizada por Camões em 'Os Lusíadas': 8 versos decassílabos heroicos com seis versos alternados e um dístico de fecho (ABABABCC).".to_string(),
            padrao_esperado: "8 versos decassílabos heroicos | Rimas ABABABCC".to_string(),
            estrutura_detectada: layout_str.to_string(),
            observacoes: Vec::new(),
        })
    } else {
        None
    }
}

fn avaliar_sextilha(
    poema: &Poema,
    esquema: &str,
    _estrofes_layout: &[usize],
    layout_str: &str,
) -> Option<DiagnosticoForma> {
    let total = poema.versos.len();
    if !(5..=7).contains(&total) {
        return None;
    }

    let mut score = 35.0 * (1.0 - (total as f32 - 6.0).abs() / 6.0).max(0.0);
    let redondilhas = poema
        .versos
        .iter()
        .filter(|v| v.total_metricas == 7)
        .count();
    score += 35.0 * (redondilhas as f32 / total as f32);
    // Versos pares rimam (2, 4, 6)
    let match_rima = if esquema.len() >= 6 {
        let chars: Vec<char> = esquema.chars().collect();
        if chars[1] == chars[3] && chars[3] == chars[5] {
            1.0
        } else if chars[1] == chars[3] || chars[3] == chars[5] {
            0.6
        } else {
            0.2
        }
    } else {
        0.3
    };
    score += 30.0 * match_rima;

    let final_score = score.round().clamp(0.0, 100.0) as u8;
    if final_score >= 50 {
        Some(DiagnosticoForma {
            nome: "Sextilha de Cordel".to_string(),
            confianca_percentual: final_score,
            descricao: "Estrutura mestra da poesia popular e literatura de cordel brasileira: 6 versos em Redondilha Maior (7 sílabas) com rima obrigatória nos versos pares (2, 4 e 6).".to_string(),
            padrao_esperado: "6 versos em Redondilha Maior (7s) | Rimas XAXAXA / ABCBDB".to_string(),
            estrutura_detectada: layout_str.to_string(),
            observacoes: Vec::new(),
        })
    } else {
        None
    }
}

fn avaliar_quadra_trova(
    poema: &Poema,
    esquema: &str,
    _estrofes_layout: &[usize],
    layout_str: &str,
) -> Option<DiagnosticoForma> {
    let total = poema.versos.len();
    if total != 4 {
        return None;
    }

    let redondilhas = poema
        .versos
        .iter()
        .filter(|v| v.total_metricas == 7)
        .count();
    let mut score: f32 = 40.0;
    score += 35.0 * (redondilhas as f32 / 4.0);
    let match_rima = comparar_esquemas(esquema, "ABCB").max(comparar_esquemas(esquema, "ABAB"));
    score += 25.0 * match_rima;

    let final_score = score.round().clamp(0.0, 100.0) as u8;
    if final_score >= 50 {
        Some(DiagnosticoForma {
            nome: "Quadra / Trova Popular".to_string(),
            confianca_percentual: final_score,
            descricao: "Forma poética fundamental e lírica de quatro versos em Redondilha Maior (7 sílabas métricas), com rimas alternadas ou no segundo e quarto versos.".to_string(),
            padrao_esperado: "4 versos em Redondilha Maior (7s) | Rimas ABCB ou ABAB".to_string(),
            estrutura_detectada: layout_str.to_string(),
            observacoes: Vec::new(),
        })
    } else {
        None
    }
}

fn avaliar_haicai(
    poema: &Poema,
    _estrofes_layout: &[usize],
    layout_str: &str,
) -> Option<DiagnosticoForma> {
    let total = poema.versos.len();
    if total != 3 {
        return None;
    }

    let m1 = poema.versos[0].total_metricas;
    let m2 = poema.versos[1].total_metricas;
    let m3 = poema.versos[2].total_metricas;

    let mut score: f32 = 40.0;
    if m1 == 5 {
        score += 20.0;
    }
    if m2 == 7 {
        score += 20.0;
    }
    if m3 == 5 {
        score += 20.0;
    }

    let final_score = score.round().clamp(0.0, 100.0) as u8;
    if final_score >= 55 {
        Some(DiagnosticoForma {
            nome: "Haicai".to_string(),
            confianca_percentual: final_score,
            descricao: "Forma poética de origem japonesa fixada em três versos de 5, 7 e 5 sílabas métricas (17 sílabas no total), capturando um instante poético da natureza ou sensação pura.".to_string(),
            padrao_esperado: "3 versos (5, 7, 5 sílabas métricas)".to_string(),
            estrutura_detectada: layout_str.to_string(),
            observacoes: Vec::new(),
        })
    } else {
        None
    }
}

/// Compares two rhyme scheme strings and returns a similarity factor between 0.0 and 1.0.
fn comparar_esquemas(esquema: &str, padrao: &str) -> f32 {
    if esquema.is_empty() || padrao.is_empty() {
        return 0.0;
    }

    let min_len = esquema.len().min(padrao.len());
    let max_len = esquema.len().max(padrao.len());

    let mut matches = 0;
    let s_chars: Vec<char> = esquema.chars().collect();
    let p_chars: Vec<char> = padrao.chars().collect();

    for i in 0..min_len {
        if s_chars[i] == p_chars[i] {
            matches += 1;
        }
    }

    (matches as f32 / max_len as f32).clamp(0.0, 1.0)
}
