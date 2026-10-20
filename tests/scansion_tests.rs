use joaju_lirica::{escandir_verso, parse_lirica_block};

#[test]
fn test_camoes_decassilabo_heroico() {
    let verso = "Amor é fogo que arde sem se ver";
    let escansao = escandir_verso(verso);

    assert_eq!(escansao.total_metricas, 10);
    assert_eq!(escansao.esquema_tonico, vec![2, 3, 4, 6, 10]);
    assert_eq!(
        escansao.classificacao.as_deref(),
        Some("Decassílabo Heroico")
    );

    // Verify sinalefa between "que" and "arde"
    let fusao_opt = escansao.silabas.iter().find(|s| s.texto.contains('_'));
    assert!(fusao_opt.is_some(), "Expected elision between que and arde");
    let fusao = fusao_opt.unwrap();
    assert_eq!(fusao.texto, "que_ar");
    assert!(fusao.fusao);
    assert!(fusao.tonica);
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
