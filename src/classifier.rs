//! Rhythm classification and traditional Portuguese meter identification.

/// Classifies a verse according to its metric syllable count and tonic scheme.
pub fn classificar_ritmo(total_metricas: usize, esquema_tonico: &[usize]) -> Option<String> {
    if total_metricas == 0 {
        return None;
    }

    let class = match total_metricas {
        1 => "Monossílabo",
        2 => "Dissílabo",
        3 => "Trissílabo",
        4 => "Tetrassílabo",
        5 => "Redondilha Menor",
        6 => "Hexassílabo",
        7 => "Redondilha Maior",
        8 => "Octossílabo",
        9 => "Eneassílabo",
        10 => {
            if esquema_tonico.contains(&6) && esquema_tonico.contains(&10) {
                "Decassílabo Heroico"
            } else if (esquema_tonico.contains(&4)
                && esquema_tonico.contains(&8)
                && esquema_tonico.contains(&10))
                || (esquema_tonico.contains(&4) && esquema_tonico.contains(&10))
            {
                "Decassílabo Sáfico"
            } else if esquema_tonico.contains(&3)
                && esquema_tonico.contains(&6)
                && esquema_tonico.contains(&10)
            {
                "Decassílabo Martelo"
            } else {
                "Decassílabo"
            }
        }
        11 => "Hendecassílabo",
        12 => {
            if esquema_tonico.contains(&6) && esquema_tonico.contains(&12) {
                "Alexandrino Clássico"
            } else {
                "Dodecassílabo"
            }
        }
        _ => "Forma Livre",
    };

    Some(class.to_string())
}

/// Returns a distinctive Unicode icon for a given metric classification.
pub fn icone_metro(classificacao: &str) -> &'static str {
    match classificacao {
        "Decassílabo Heroico" => "⚔️",
        "Decassílabo Sáfico" => "🏛️",
        "Decassílabo Martelo" => "🔨",
        "Decassílabo" => "📜",
        "Redondilha Maior" => "🌿",
        "Redondilha Menor" => "🍃",
        "Alexandrino Clássico" => "👑",
        "Dodecassílabo" => "⚜️",
        "Octossílabo" => "🎵",
        "Eneassílabo" => "🌊",
        "Hendecassílabo" => "🕊️",
        _ => "🪶",
    }
}
