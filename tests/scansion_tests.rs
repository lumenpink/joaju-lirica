use joaju_lirica::{escandir_verso, parse_lirica_block};

#[test]
fn test_camoes_decassilabo_heroico() {
    let versos = [
        "Amor é fogo que arde sem se ver,",
        "É ferida que dói e não se sente;",
        "É um contentamento descontente,",
        "É dor que desatina sem doer.",
    ];

    for v in versos {
        let esc = escandir_verso(v);
        assert_eq!(
            esc.total_metricas, 10,
            "Verso '{}' deve ter exatamente 10 sílabas métricas, teve {}",
            v, esc.total_metricas
        );
        assert!(
            esc.classificacao
                .as_deref()
                .unwrap()
                .contains("Decassílabo"),
            "Verso '{}' deve ser classificado como Decassílabo",
            v
        );
    }

    // Verify sinalefa between "que" and "arde" on verse 1
    let escansao = escandir_verso("Amor é fogo que arde sem se ver");
    let fusao_opt = escansao.silabas.iter().find(|s| s.texto.contains('_'));
    assert!(fusao_opt.is_some(), "Expected elision between que and arde");
    let fusao = fusao_opt.unwrap();
    assert_eq!(fusao.texto, "que_ar");
    assert!(fusao.fusao);
    assert!(fusao.tonica);

    // Verify hiatus on tonic monosyllable "É" and "dói"
    let esc2 = escandir_verso("É ferida que dói e não se sente;");
    assert!(
        !esc2.silabas.iter().any(|s| s.texto.contains("dói_e")),
        "dói não deve elidir com e"
    );

    let esc3 = escandir_verso("É um contentamento descontente,");
    assert!(
        !esc3.silabas.iter().any(|s| s.texto.contains("É_um")),
        "É não deve elidir com um"
    );
}

#[test]
fn test_hiatos_deterministas_sem_acento_vs_com_acento() {
    let esc_poesia = escandir_verso("poesia");
    let textos_poesia: Vec<&str> = esc_poesia
        .silabas
        .iter()
        .map(|s| s.texto.as_str())
        .collect();
    assert_eq!(textos_poesia, vec!["po", "e", "si", "a"]);
    assert_eq!(esc_poesia.total_metricas, 3); // Castilho stops at "si"

    let esc_historia = escandir_verso("história");
    let textos_historia: Vec<&str> = esc_historia
        .silabas
        .iter()
        .map(|s| s.texto.as_str())
        .collect();
    assert_eq!(textos_historia, vec!["his", "tó", "ria"]);
    assert_eq!(esc_historia.total_metricas, 2); // Castilho stops at "tó"

    let esc_rua = escandir_verso("rua");
    let textos_rua: Vec<&str> = esc_rua.silabas.iter().map(|s| s.texto.as_str()).collect();
    assert_eq!(textos_rua, vec!["ru", "a"]);
    assert_eq!(esc_rua.total_metricas, 1);
}

#[test]
fn test_pontuacao_bloqueia_sinalefa() {
    let verso_virgula = "Fogo, ardente sem se ver";
    let esc = escandir_verso(verso_virgula);

    // "Fogo," ends with comma, so it should not merge with "ar"
    let fusoes: Vec<&str> = esc
        .silabas
        .iter()
        .filter(|s| s.fusao)
        .map(|s| s.texto.as_str())
        .collect();
    assert!(
        !fusoes.iter().any(|s| s.contains("go_ar")),
        "Comma should block sinalefa"
    );
}

#[test]
fn test_overrides_manuais() {
    let verso_split = "A ru/a clara";
    let esc = escandir_verso(verso_split);
    let sils: Vec<&str> = esc.silabas.iter().map(|s| s.texto.as_str()).collect();
    assert!(sils.contains(&"ru"));
    assert!(sils.contains(&"a"));

    let verso_fusao = "alma_pura";
    let esc_f = escandir_verso(verso_fusao);
    assert!(esc_f.silabas.iter().any(|s| s.fusao));
}

#[test]
fn test_spans_de_bytes_utf8() {
    let verso = "é ferida que dói";
    let esc = escandir_verso(verso);

    for s in &esc.silabas {
        let slice = &verso[s.span.clone()];
        assert!(!slice.is_empty());
        if !s.fusao {
            assert_eq!(slice, s.texto);
        }
    }
}

#[test]
fn test_parse_bloco_lirica_com_diretivas() {
    let bloco = r#"%% titulo = Soneto
%% metro = decassilabo
%% elidir_pontuacao = true

Fogo, ardente sem se ver
"#;

    let (config, poem) = parse_lirica_block(bloco).expect("Failed to parse block");
    assert_eq!(config.titulo.as_deref(), Some("Soneto"));
    assert_eq!(config.metro_alvo.as_deref(), Some("decassilabo"));
    assert!(config.elidir_pontuacao);
    assert_eq!(poem.versos.len(), 1);
    // With elidir_pontuacao = true, the comma does NOT block sinalefa between "go" and "ar":
    assert!(poem.versos[0]
        .silabas
        .iter()
        .any(|s| s.fusao && s.texto.contains("go_ar")));
}

#[test]
fn test_redondilha_maior_goncalves_dias() {
    // "Minha terra tem palmeiras" -> Mi-nha ter-ra tem pal-mei-(ras) = 7 syllables
    let verso = "Minha terra tem palmeiras";
    let esc = escandir_verso(verso);
    assert_eq!(esc.total_metricas, 7);
    assert_eq!(esc.classificacao.as_deref(), Some("Redondilha Maior"));
}

#[test]
fn test_monossilabos_atonos_vs_tonicos() {
    let verso = "O sol brilha no mar";
    let esc = escandir_verso(verso);
    // 'O' atonic, 'sol' tonic, 'bri' tonic, 'lha' atonic, 'no' atonic, 'mar' tonic
    assert!(esc.esquema_tonico.contains(&2)); // sol
    assert!(esc.esquema_tonico.contains(&3)); // bri
    assert!(esc.esquema_tonico.contains(&6)); // mar
}

#[test]
fn test_render_painel_e_typst() {
    use joaju_lirica::{render_painel, render_typst};

    let bloco = r#"%% titulo = Canção do Exílio
%% autor = Gonçalves Dias
%% metro = redondilha_maior

Minha terra tem palmeiras
Onde canta o Sabiá
"#;

    let (config, poem) = parse_lirica_block(bloco).unwrap();
    let painel = render_painel(&poem);
    assert!(painel.contains("Minha terra tem palmeiras"));
    assert!(painel.contains("Redondilha Maior"));

    let typst = render_typst(&config, &poem);
    assert!(typst.contains("== Canção do Exílio"));
    assert!(typst.contains("_por Gonçalves Dias_"));
    assert!(typst.contains("#grid"));
}

#[test]
fn test_edge_cases_espacos_e_pontuacao() {
    let vazio = escandir_verso("");
    assert_eq!(vazio.total_metricas, 0);

    let so_espacos = escandir_verso("   \t  \n  ");
    assert_eq!(so_espacos.total_metricas, 0);

    let so_pontuacao = escandir_verso("... !? —");
    assert_eq!(so_pontuacao.total_metricas, 0);
}
