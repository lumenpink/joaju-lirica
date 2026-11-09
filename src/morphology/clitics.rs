//! Módulo para identificação e descolamento de pronomes enclíticos e mesoclíticos.

const CLITICOS: &[&str] = &[
    "me", "te", "se", "lhe", "lhes", "nos", "vos", "o", "a", "os", "as", "lo", "la", "los", "las",
    "no", "na", "nos", "nas",
];

/// Descola o pronome enclítico de uma palavra verbal, normalizando a base.
/// Retorna `(base_normalizada, Some(clitico), true)` ou `(palavra_original, None, false)`.
pub fn descolar_enclise(palavra: &str) -> (String, Option<String>, bool) {
    let lower = palavra.to_lowercase();
    let parts: Vec<&str> = lower.split('-').collect();

    if parts.len() < 2 {
        return (palavra.to_string(), None, false);
    }

    let clitico = parts.last().unwrap();
    if !CLITICOS.contains(clitico) {
        return (palavra.to_string(), None, false);
    }

    let prefixo = parts[..parts.len() - 1].join("-");
    let base_normalizada = normalizar_radical_verbal(&prefixo);

    (base_normalizada, Some((*clitico).to_string()), true)
}

fn normalizar_radical_verbal(radical: &str) -> String {
    // Casos especiais de formas e lemas irregulares frequentes
    match radical {
        "diz" => return "dizer".to_string(),
        "faz" => return "fazer".to_string(),
        "tem" => return "ter".to_string(),
        "vai" => return "ir".to_string(),
        "vê" => return "ver".to_string(),
        "dá" => return "dar".to_string(),
        "pô" | "pô-lo" => return "pôr".to_string(),
        _ => {}
    }

    // Regras de apócope r/s/z perante lo/la/los/las
    if radical.ends_with('á') {
        let mut s = radical[..radical.len() - 'á'.len_utf8()].to_string();
        s.push_str("ar");
        return s;
    }
    if radical.ends_with('ê') {
        let mut s = radical[..radical.len() - 'ê'.len_utf8()].to_string();
        s.push_str("er");
        return s;
    }
    if radical.ends_with('í') {
        let mut s = radical[..radical.len() - 'í'.len_utf8()].to_string();
        s.push_str("ir");
        return s;
    }

    radical.to_string()
}
