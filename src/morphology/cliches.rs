//! Catálogo e detector determinístico de rimas clichê (banais / hiper-saturadas).
//!
//! Identifica pares e feixes semântico-fonéticos clássicos que, embora possam ser
//! formalmente ricos ou consoantes perfeitos, carregam desgaste estilístico histórico
//! na tradição poética da língua portuguesa (Olavo Bilac, Guimarães Passos, Said Ali).

/// Verifica se duas palavras formam uma rima clichê / hiper-saturada na lírica em português.
///
/// Retorna `Some(motivo_descritivo)` caso o par conste no catálogo canônico de clichês,
/// ou `None` caso não seja clichê.
pub fn verificar_cliche(palavra1: &str, palavra2: &str) -> Option<&'static str> {
    let w1 = normalizar_termo_cliche(palavra1);
    let w2 = normalizar_termo_cliche(palavra2);

    if w1 == w2 || w1.is_empty() || w2.is_empty() {
        return None;
    }

    // Ordenação lexicográfica canônica para busca simétrica
    let (a, b) = if w1 < w2 {
        (w1.as_str(), w2.as_str())
    } else {
        (w2.as_str(), w1.as_str())
    };

    match (a, b) {
        // === Feixe AMOR / DOR / FLOR / CLAMOR / ARDOR / FERVOR / SENHOR / FAVOR / PASTOR ===
        ("amor", "dor") => Some("Clichê arquetípico: binômio amor x dor"),
        ("amor", "flor") => Some("Clichê romântico: amor x flor"),
        ("dor", "flor") => Some("Clichê romântico: dor x flor"),
        ("amor", "clamor") => Some("Clichê poético: amor x clamor"),
        ("clamor", "dor") => Some("Clichê poético: clamor x dor"),
        ("amor", "ardor") => Some("Clichê poético: amor x ardor"),
        ("ardor", "dor") => Some("Clichê poético: ardor x dor"),
        ("amor", "fervor") => Some("Clichê poético: amor x fervor"),
        ("dor", "fervor") => Some("Clichê lírico: dor x fervor"),
        ("amor", "senhor") => Some("Clichê lírico: amor x senhor"),
        ("favor", "senhor") => Some("Clichê lírico: favor x senhor"),
        ("amor", "favor") => Some("Clichê lírico: amor x favor"),
        ("pastor", "senhor") => Some("Clichê bucólico: pastor x senhor"),

        // === Feixe MAR / AMAR / LUAR / OLHAR / CANTAR / CHORAR / SONHAR / BEIJAR ===
        ("amar", "mar") => Some("Clichê clássico: mar x amar (rica, porém hiper-saturada)"),
        ("luar", "mar") => Some("Clichê lírico: mar x luar"),
        ("mar", "navegar") => Some("Clichê temático: mar x navegar"),
        ("amar", "olhar") => Some("Clichê romântico: amar x olhar"),
        ("amar", "cantar") => Some("Clichê cancioneiro: cantar x amar"),
        ("amar", "chorar") => Some("Clichê sentimental: amar x chorar"),
        ("amar", "sonhar") => Some("Clichê romântico: sonhar x amar"),
        ("amar", "beijar") => Some("Clichê sentimental: amar x beijar"),
        ("beijar", "olhar") => Some("Clichê lírico: olhar x beijar"),
        ("chorar", "olhar") => Some("Clichê sentimental: olhar x chorar"),
        ("cantar", "luar") => Some("Clichê trovadoresco: cantar x luar"),
        ("cantar", "sonhar") => Some("Clichê romântico: cantar x sonhar"),
        ("chorar", "penar") => Some("Clichê elegíaco: chorar x penar"),
        ("lembrar", "sonhar") => Some("Clichê memorialista: lembrar x sonhar"),
        ("olhar", "sonhar") => Some("Clichê lírico: olhar x sonhar"),

        // === Feixe CORAÇÃO / PAIXÃO / ILUSÃO / SOLIDÃO / CANÇÃO / RAZÃO / PERDÃO / EMOÇÃO ===
        ("coracao", "paixao") => Some("Clichê lírico em -ão: coração x paixão"),
        ("coracao", "ilusao") => Some("Clichê lírico em -ão: coração x ilusão"),
        ("coracao", "solidao") => Some("Clichê lírico em -ão: coração x solidão"),
        ("cancao", "coracao") => Some("Clichê lírico em -ão: coração x canção"),
        ("coracao", "razao") => Some("Clichê lírico em -ão: coração x razão"),
        ("coracao", "perdao") => Some("Clichê lírico em -ão: coração x perdão"),
        ("coracao", "emocao") => Some("Clichê lírico em -ão: coração x emoção"),
        ("ilusao", "paixao") => Some("Clichê lírico em -ão: paixão x ilusão"),
        ("paixao", "solidao") => Some("Clichê lírico em -ão: paixão x solidão"),
        ("ilusao", "solidao") => Some("Clichê lírico em -ão: ilusão x solidão"),
        ("cancao", "paixao") => Some("Clichê lírico em -ão: canção x paixão"),
        ("cancao", "solidao") => Some("Clichê lírico em -ão: canção x solidão"),
        ("cancao", "ilusao") => Some("Clichê lírico em -ão: canção x ilusão"),
        ("emocao", "paixao") => Some("Clichê lírico em -ão: paixão x emoção"),
        ("paixao", "razao") => Some("Clichê lírico em -ão: razão x paixão"),
        ("perdao", "solidao") => Some("Clichê lírico em -ão: solidão x perdão"),
        ("paixao", "perdao") => Some("Clichê lírico em -ão: paixão x perdão"),
        ("perdao", "razao") => Some("Clichê lírico em -ão: razão x perdão"),
        ("razao", "solidao") => Some("Clichê lírico em -ão: solidão x razão"),
        ("ilusao", "razao") => Some("Clichê lírico em -ão: ilusão x razão"),
        ("escuridao", "solidao") => Some("Clichê lírico em -ão: solidão x escuridão"),
        ("desolacao", "solidao") => Some("Clichê lírico em -ão: solidão x desolação"),

        // === Feixe CÉU / VÉU / TROFÉU / RÉU ===
        ("ceu", "veu") => Some("Clichê parnasiano/romântico: céu x véu"),
        ("ceu", "trofeu") => Some("Clichê lírico: céu x troféu"),
        ("ceu", "reu") => Some("Clichê dramático: céu x réu"),
        ("reu", "veu") => Some("Clichê dramático: réu x véu"),

        // === Feixe VENTO / PENSAMENTO / SENTIMENTO / TORMENTO / MOMENTO / LAMENTO / ALENTO ===
        ("pensamento", "vento") => Some("Clichê meditativo: vento x pensamento"),
        ("pensamento", "sentimento") => Some("Clichê meditativo: pensamento x sentimento"),
        ("sentimento", "tormento") => Some("Clichê romântico: sentimento x tormento"),
        ("momento", "pensamento") => Some("Clichê meditativo: pensamento x momento"),
        ("lamento", "pensamento") => Some("Clichê lírico: pensamento x lamento"),
        ("tormento", "vento") => Some("Clichê dramático: tormento x vento"),
        ("lamento", "tormento") => Some("Clichê dramático: lamento x tormento"),
        ("lamento", "vento") => Some("Clichê elegíaco: lamento x vento"),
        ("momento", "sentimento") => Some("Clichê reflexivo: momento x sentimento"),
        ("momento", "tormento") => Some("Clichê trágico: momento x tormento"),
        ("momento", "vento") => Some("Clichê lírico: momento x vento"),
        ("alento", "tormento") => Some("Clichê lírico: alento x tormento"),
        ("alento", "vento") => Some("Clichê lírico: alento x vento"),
        ("pensamento", "sofrimento") => Some("Clichê romântico: pensamento x sofrimento"),
        ("sentimento", "sofrimento") => Some("Clichê romântico: sentimento x sofrimento"),

        // === Feixe VIDA / QUERIDA / PERDIDA / DESPEDIDA / FERIDA / LIDA / ILUDIDA / ESQUECIDA ===
        ("querida", "vida") => Some("Clichê romântico: vida x querida"),
        ("perdida", "vida") => Some("Clichê elegíaco: vida x perdida"),
        ("despedida", "vida") => Some("Clichê lírico: vida x despedida"),
        ("ferida", "vida") => Some("Clichê dramático: vida x ferida"),
        ("lida", "vida") => Some("Clichê existencial: vida x lida"),
        ("esquecida", "vida") => Some("Clichê elegíaco: vida x esquecida"),
        ("iludida", "vida") => Some("Clichê romântico: vida x iludida"),
        ("despedida", "ferida") => Some("Clichê dramático: despedida x ferida"),
        ("ferida", "perdida") => Some("Clichê dramático: ferida x perdida"),
        ("esquecida", "perdida") => Some("Clichê melancólico: perdida x esquecida"),
        ("esquecida", "querida") => Some("Clichê sentimental: querida x esquecida"),
        ("iludida", "perdida") => Some("Clichê lírico: perdida x iludida"),

        // === Feixe LUZ / SEDUZ / CONDUZ / CRUZ / RELUZ ===
        ("luz", "seduz") => Some("Clichê poético: luz x seduz"),
        ("cruz", "luz") => Some("Clichê sacral/dramático: luz x cruz"),
        ("conduz", "luz") => Some("Clichê poético: luz x conduz"),
        ("luz", "reluz") => Some("Clichê sensorial: luz x reluz"),
        ("conduz", "seduz") => Some("Clichê lírico: seduz x conduz"),
        ("reluz", "seduz") => Some("Clichê estético: seduz x reluz"),

        // === Feixe MORTE / SORTE / FORTE / NORTE ===
        ("morte", "sorte") => Some("Clichê trágico: morte x sorte"),
        ("forte", "morte") => Some("Clichê dramático: morte x forte"),
        ("forte", "sorte") => Some("Clichê lírico: sorte x forte"),
        ("morte", "norte") => Some("Clichê lírico: morte x norte"),

        // === Feixe GLÓRIA / VITÓRIA / MEMÓRIA / HISTÓRIA ===
        ("gloria", "vitoria") => Some("Clichê cívico/heroico: glória x vitória"),
        ("gloria", "memoria") => Some("Clichê lírico: glória x memória"),
        ("memoria", "vitoria") => Some("Clichê memorialista: vitória x memória"),
        ("gloria", "historia") => Some("Clichê solene: glória x história"),
        ("historia", "memoria") => Some("Clichê narrativo: história x memória"),

        // === Feixe NOITE / AÇOITE ===
        ("acoite", "noite") => Some("Clichê dramático condoreiro: açoite x noite"),

        // === Feixe ALMA / CALMA / PALMA ===
        ("alma", "calma") => Some("Clichê meditativo: alma x calma"),
        ("alma", "palma") => Some("Clichê simbólico: alma x palma"),
        ("calma", "palma") => Some("Clichê lírico: calma x palma"),

        // === Feixe TRISTEZA / BELEZA / NATUREZA / CERTEZA / PUREZA ===
        ("beleza", "tristeza") => Some("Clichê romântico: beleza x tristeza"),
        ("beleza", "natureza") => Some("Clichê bucólico: beleza x natureza"),
        ("certeza", "tristeza") => Some("Clichê lírico: tristeza x certeza"),
        ("natureza", "tristeza") => Some("Clichê romântico: natureza x tristeza"),
        ("certeza", "natureza") => Some("Clichê reflexivo: certeza x natureza"),
        ("pureza", "tristeza") => Some("Clichê lírico: pureza x tristeza"),
        ("beleza", "pureza") => Some("Clichê lírico: beleza x pureza"),

        // === Feixe PAZ / CAPAZ / TRAZ ===
        ("capaz", "paz") => Some("Clichê reflexivo: paz x capaz"),
        ("paz", "traz") => Some("Clichê lírico: paz x traz"),

        // === Feixe SONHO / RISONHO / MEDONHO ===
        ("risonho", "sonho") => Some("Clichê bucólico: sonho x risonho"),
        ("medonho", "sonho") => Some("Clichê gótico/romântico: sonho x medonho"),

        // === Feixe CHAMA / AMA / DRAMA / CAMA / FAMA ===
        ("ama", "chama") => Some("Clichê lírico: chama x ama"),
        ("chama", "drama") => Some("Clichê dramático: chama x drama"),
        ("cama", "chama") => Some("Clichê romântico: chama x cama"),
        ("chama", "fama") => Some("Clichê solene: chama x fama"),

        // === Feixe MUNDO / PROFUNDO / FUNDO ===
        ("mundo", "profundo") => Some("Clichê existencial: mundo x profundo"),
        ("fundo", "mundo") => Some("Clichê existencial: mundo x fundo"),
        ("fundo", "profundo") => Some("Clichê reflexivo: fundo x profundo"),

        // === Feixe ESPERANÇA / LEMBRANÇA / CRIANÇA / BONANÇA / MUDANÇA ===
        ("esperanca", "lembranca") => Some("Clichê romântico: esperança x lembrança"),
        ("crianca", "lembranca") => Some("Clichê memorialista: lembrança x criança"),
        ("bonanca", "esperanca") => Some("Clichê lírico: esperança x bonança"),
        ("lembranca", "mudanca") => Some("Clichê lírico: lembrança x mudança"),
        ("esperanca", "mudanca") => Some("Clichê lírico: esperança x mudança"),

        // === Feixe DIA / ALEGRIA / POESIA / HARMONIA / MAGIA / NOSTALGIA / FRIA ===
        ("alegria", "dia") => Some("Clichê lírico: dia x alegria"),
        ("alegria", "poesia") => Some("Clichê metapoético: alegria x poesia"),
        ("dia", "fria") => Some("Clichê atmosférico: dia x fria"),
        ("alegria", "harmonia") => Some("Clichê harmônico: alegria x harmonia"),
        ("alegria", "magia") => Some("Clichê poético: alegria x magia"),
        ("harmonia", "poesia") => Some("Clichê metapoético: harmonia x poesia"),
        ("nostalgia", "poesia") => Some("Clichê lírico: nostalgia x poesia"),
        ("magia", "poesia") => Some("Clichê lírico: magia x poesia"),

        // === Feixe CANTO / PRANTO / ENCANTO / ESPANTO / DESENCANTO ===
        ("canto", "pranto") => Some("Clichê romântico: canto x pranto"),
        ("encanto", "pranto") => Some("Clichê sentimental: pranto x encanto"),
        ("canto", "encanto") => Some("Clichê estético: canto x encanto"),
        ("espanto", "pranto") => Some("Clichê elegíaco: pranto x espanto"),
        ("desencanto", "encanto") => Some("Clichê lírico: encanto x desencanto"),
        ("desencanto", "pranto") => Some("Clichê elegíaco: desencanto x pranto"),

        // === Feixe DESTINO / MENINO / DESATINO / DIVINO / PEREGRINO ===
        ("desatino", "destino") => Some("Clichê trágico: destino x desatino"),
        ("destino", "menino") => Some("Clichê cancioneiro: destino x menino"),
        ("divino", "menino") => Some("Clichê sacro-cancioneiro: menino x divino"),
        ("destino", "divino") => Some("Clichê místico: destino x divino"),
        ("destino", "peregrino") => Some("Clichê existencial: destino x peregrino"),

        // === Feixe ROSA / FORMOSA / MIMOSA / PRECIOSA / SAUDOSA ===
        ("formosa", "rosa") => Some("Clichê lírico: rosa x formosa"),
        ("mimosa", "rosa") => Some("Clichê bucólico: rosa x mimosa"),
        ("preciosa", "rosa") => Some("Clichê estético: rosa x preciosa"),
        ("rosa", "saudosa") => Some("Clichê lírico: rosa x saudosa"),
        ("formosa", "mimosa") => Some("Clichê lírico: formosa x mimosa"),

        // === Feixe OLHOS / ABROLHOS ===
        ("abrolho", "olho") | ("abrolhos", "olhos") => {
            Some("Clichê arcádico/romântico: olhos x abrolhos")
        }

        // === Feixe GUERRA / TERRA ===
        ("guerra", "terra") => Some("Clichê épico/lírico: terra x guerra"),

        // === Feixe SOMBRA / ASSOMBRA ===
        ("assombra", "sombra") => Some("Clichê romântico: sombra x assombra"),

        // === Feixe FOGO / JOGO ===
        ("fogo", "jogo") => Some("Clichê lírico: fogo x jogo"),

        // === Feixe RISO / PARAÍSO / AVISO / PRECISO ===
        ("paraiso", "riso") => Some("Clichê romântico: riso x paraíso"),
        ("aviso", "riso") => Some("Clichê lírico: riso x aviso"),

        _ => None,
    }
}

/// Normaliza o termo da rima descolando pontuação, ênclise e convertendo para minúsculas sem diacríticos.
fn normalizar_termo_cliche(p: &str) -> String {
    let limpo = p.trim().to_lowercase();
    let base = if let Some((prefix, _)) = limpo.split_once('-') {
        prefix
    } else {
        &limpo
    };
    base.chars()
        .filter(|c| c.is_alphabetic())
        .map(|c| match c {
            'á' | 'à' | 'ã' | 'â' => 'a',
            'é' | 'ê' => 'e',
            'í' => 'i',
            'ó' | 'ô' | 'õ' => 'o',
            'ú' => 'u',
            'ç' => 'c',
            other => other,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verificar_cliches_principais() {
        assert!(verificar_cliche("amor", "dor").is_some());
        assert!(verificar_cliche("dor", "amor").is_some());
        assert!(verificar_cliche("mar", "amar").is_some());
        assert!(verificar_cliche("coração", "ilusão").is_some());
        assert!(verificar_cliche("vento", "pensamento").is_some());
        assert!(verificar_cliche("vida", "querida").is_some());
        assert!(verificar_cliche("beleza", "tristeza").is_some());
        assert!(verificar_cliche("luz", "seduz").is_some());
        assert!(verificar_cliche("morte", "sorte").is_some());
        assert!(verificar_cliche("glória", "vitória").is_some());
        assert!(verificar_cliche("olhos", "abrolhos").is_some());
        assert!(verificar_cliche("terra", "guerra").is_some());
        assert!(verificar_cliche("estrela", "vê-la").is_none());
        assert!(verificar_cliche("canto", "santo").is_none());
    }
}
