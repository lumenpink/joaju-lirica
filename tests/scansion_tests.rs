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

#[test]
fn test_forcar_hiato_entre_palavras() {
    // Sem barra: elide naturalmente ("que_ar" é a única métrica sob Castilho)
    let com_elisao = escandir_verso("que arde");
    assert!(
        com_elisao.silabas.iter().any(|s| s.fusao),
        "Sem barra deve haver sinalefa"
    );
    assert_eq!(com_elisao.total_metricas, 1);

    // Com barra entre palavras: forçar hiato impede elisão ("que" = 1, "ar" = 2)
    let hiato_espacos = escandir_verso("que / arde");
    assert!(
        !hiato_espacos.silabas.iter().any(|s| s.fusao),
        "'que / arde' deve forçar hiato"
    );
    assert_eq!(hiato_espacos.total_metricas, 2);

    let hiato_colado_esq = escandir_verso("que/ arde");
    assert!(
        !hiato_colado_esq.silabas.iter().any(|s| s.fusao),
        "'que/ arde' deve forçar hiato"
    );
    assert_eq!(hiato_colado_esq.total_metricas, 2);

    let hiato_colado_dir = escandir_verso("que /arde");
    assert!(
        !hiato_colado_dir.silabas.iter().any(|s| s.fusao),
        "'que /arde' deve forçar hiato"
    );
    assert_eq!(hiato_colado_dir.total_metricas, 2);
}

#[test]
fn test_mudanca_manual_de_tonica() {
    // 'que' é normalmente átono
    let que_normal = escandir_verso("que");
    assert_eq!(que_normal.esquema_tonico.len(), 0);

    // 'que'' com apóstrofo vira tônico
    let que_tonico = escandir_verso("que'");
    assert_eq!(que_tonico.esquema_tonico, vec![1]);

    // 'fogo' é paroxítona natural: [fo]-go
    let fogo_normal = escandir_verso("fogo");
    assert_eq!(fogo_normal.esquema_tonico, vec![1]);

    // 'fogo'' com apóstrofo na última sílaba vira oxítona: fo-[go]
    let fogo_oxitona = escandir_verso("fogo'");
    assert_eq!(fogo_oxitona.esquema_tonico, vec![2]);

    // Sendo oxítona, a tônica final bloqueia elisão seguinte e recalcula o verso
    let verso_oxitona = escandir_verso("fogo' arde");
    assert!(
        !verso_oxitona.silabas.iter().any(|s| s.fusao),
        "fogo' oxítona não deve elidir com arde"
    );
}

#[test]
fn test_analise_rimas_e_icones() {
    use joaju_lirica::{analisar_esquema_rimas, icone_metro};

    let linhas = vec![
        "Amor é fogo que arde sem se ver,".to_string(),
        "É ferida que dói e não se sente;".to_string(),
        "É um contentamento descontente,".to_string(),
        "É dor que desatina sem doer.".to_string(),
    ];

    let (rimas, esquema) = analisar_esquema_rimas(&linhas);
    assert_eq!(esquema.as_deref(), Some("ABBA"));
    assert_eq!(rimas[0].as_ref().unwrap().letra, "A");
    assert_eq!(rimas[1].as_ref().unwrap().letra, "B");
    assert_eq!(rimas[2].as_ref().unwrap().letra, "B");
    assert_eq!(rimas[3].as_ref().unwrap().letra, "A");

    assert_eq!(icone_metro("Decassílabo Heroico"), "⚔️");
    assert_eq!(icone_metro("Decassílabo Sáfico"), "🏛️");
    assert_eq!(icone_metro("Redondilha Maior"), "🌿");
    assert_eq!(icone_metro("Alexandrino Clássico"), "👑");
}

#[test]
fn test_verso14_camoes_decassilabo() {
    let esc = escandir_verso("Se tão contrário a si é o mesmo Amor?");
    println!(
        "Verso 14 escansao: {:?}",
        esc.silabas.iter().map(|s| &s.texto).collect::<Vec<_>>()
    );
    println!("Total metricas: {}", esc.total_metricas);
    assert_eq!(
        esc.total_metricas, 10,
        "Verso 14 de Camões deve ser decassílabo exato"
    );
}

#[test]
fn test_ditongo_ao_e_soneto_fidelidade() {
    use joaju_lirica::escandir_verso_com_opcoes;

    // "ao" e "aos" devem ser uma única sílaba átona
    let esc_ao = escandir_verso("ao");
    assert_eq!(esc_ao.silabas.len(), 1, "ao deve ser 1 sílaba só");
    assert_eq!(esc_ao.silabas[0].texto, "ao");
    assert!(!esc_ao.silabas[0].tonica, "ao deve ser átono");

    let esc_aos = escandir_verso("aos");
    assert_eq!(esc_aos.silabas.len(), 1, "aos deve ser 1 sílaba só");
    assert_eq!(esc_aos.silabas[0].texto, "aos");
    assert!(!esc_aos.silabas[0].tonica, "aos deve ser átono");

    // Verso 8 do Soneto de Fidelidade de Vinícius de Moraes
    let v8 = "Ao seu pesar ou seu contentamento.";
    let esc_v8 = escandir_verso_com_opcoes(v8, true, false, true);
    assert_eq!(
        esc_v8.total_metricas, 10,
        "Verso 8 de Vinícius deve ser decassílabo exato, teve {}",
        esc_v8.total_metricas
    );
    assert!(
        esc_v8
            .classificacao
            .as_deref()
            .unwrap()
            .contains("Decassílabo"),
        "Verso 8 deve ser classificado como Decassílabo"
    );

    // Verso 1 do Soneto de Fidelidade
    let v1 = "De tudo, ao meu amor serei atento";
    let esc_v1 = escandir_verso_com_opcoes(v1, true, false, true);
    assert_eq!(
        esc_v1.total_metricas, 10,
        "Verso 1 de Vinícius deve ser decassílabo exato, teve {}",
        esc_v1.total_metricas
    );
}
