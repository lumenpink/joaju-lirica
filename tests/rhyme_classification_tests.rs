//! Suíte de testes de estresse para o Módulo Determinístico de Classificação de Rimas.

use joaju_lirica::morphology::{
    avaliar_par_rima, classificar_rimas_poema, descolar_enclise, extrair_token_rimante,
    IndeterminadoMotivo, JustificativaAuditoria, PosMask, QualidadeRima, TokenRimante,
};

#[test]
fn test_pos_mask_operacoes_basicas() {
    let subst = PosMask::SUBSTANTIVO;
    assert!(subst.is_resolved());
    assert!(!subst.is_ambiguous());

    let amb = PosMask::SUBSTANTIVO | PosMask::VERBO;
    assert!(!amb.is_resolved());
    assert!(amb.is_ambiguous());
    assert!(amb.contains(PosMask::SUBSTANTIVO));
    assert!(amb.contains(PosMask::VERBO));
    assert!(!amb.contains(PosMask::ADJETIVO));

    let intersec = amb & PosMask::SUBSTANTIVO;
    assert_eq!(intersec, PosMask::SUBSTANTIVO);

    let disjunto = PosMask::ADJETIVO & PosMask::SUBSTANTIVO;
    assert!(disjunto.is_empty());
}

#[test]
fn test_descolar_enclise_e_mesoclise() {
    let (base1, clitico1, tem1) = descolar_enclise("amá-lo");
    assert!(tem1);
    assert_eq!(base1, "amar");
    assert_eq!(clitico1.as_deref(), Some("lo"));

    let (base2, clitico2, tem2) = descolar_enclise("diz-se");
    assert!(tem2);
    assert_eq!(base2, "dizer");
    assert_eq!(clitico2.as_deref(), Some("se"));

    let (base3, clitico3, tem3) = descolar_enclise("estrela");
    assert!(!tem3);
    assert_eq!(base3, "estrela");
    assert_eq!(clitico3, None);
}

#[test]
fn test_regras_lookback_desambiguacao() {
    // R-DET: "o meu canto" -> SUBSTANTIVO
    let t1 = extrair_token_rimante("Na noite ouço o meu canto", 0);
    assert_eq!(t1.pos, PosMask::SUBSTANTIVO);
    assert_eq!(t1.regra_resolucao.as_deref(), Some("R-DET"));

    // R-PRON: "quando eu canto" -> VERBO
    let t2 = extrair_token_rimante("Na solidão quando eu canto", 0);
    assert_eq!(t2.pos, PosMask::VERBO);
    assert_eq!(t2.regra_resolucao.as_deref(), Some("R-PRON"));

    // R-NEG: "pois já não canto" -> VERBO
    let t3 = extrair_token_rimante("Pois já não canto", 0);
    assert_eq!(t3.pos, PosMask::VERBO);
    assert_eq!(t3.regra_resolucao.as_deref(), Some("R-NEG"));

    // R-MOD: "sopra um vento lento" -> ADJETIVO
    let t4 = extrair_token_rimante("Pela montanha sopra um vento lento", 0);
    assert_eq!(t4.pos, PosMask::ADJETIVO);
    assert_eq!(t4.regra_resolucao.as_deref(), Some("R-MOD"));

    // R-PREP: "nasci para cantar" -> VERBO
    let t5 = extrair_token_rimante("Nasci somente para cantar", 0);
    assert_eq!(t5.pos, PosMask::VERBO);
    assert_eq!(t5.regra_resolucao.as_deref(), Some("R-PREP"));
}

#[test]
fn test_algebra_pares_de_rima() {
    // 1. Rima Pobre: Substantivo x Substantivo
    let t_coracao = TokenRimante {
        texto: "coração".to_string(),
        base: "coração".to_string(),
        tem_enclise: false,
        pos: PosMask::SUBSTANTIVO,
        regra_resolucao: Some("DAFSA_EXACT".to_string()),
    };
    let t_ilusao = TokenRimante {
        texto: "ilusão".to_string(),
        base: "ilusão".to_string(),
        tem_enclise: false,
        pos: PosMask::SUBSTANTIVO,
        regra_resolucao: Some("DAFSA_EXACT".to_string()),
    };
    let (qual_pobre, _) = avaliar_par_rima(&t_coracao, &t_ilusao);
    assert_eq!(qual_pobre, QualidadeRima::Pobre);

    // 2. Rima Rica: Substantivo x Adjetivo
    let t_vento = TokenRimante {
        texto: "vento".to_string(),
        base: "vento".to_string(),
        tem_enclise: false,
        pos: PosMask::SUBSTANTIVO,
        regra_resolucao: Some("DAFSA_EXACT".to_string()),
    };
    let t_lento = TokenRimante {
        texto: "lento".to_string(),
        base: "lento".to_string(),
        tem_enclise: false,
        pos: PosMask::ADJETIVO,
        regra_resolucao: Some("DAFSA_EXACT".to_string()),
    };
    let (qual_rica, _) = avaliar_par_rima(&t_vento, &t_lento);
    assert_eq!(qual_rica, QualidadeRima::Rica);

    // 3. Rima Preciosa: Assimetria de Ênclise (estrela x vê-la)
    let t_estrela = TokenRimante {
        texto: "estrela".to_string(),
        base: "estrela".to_string(),
        tem_enclise: false,
        pos: PosMask::SUBSTANTIVO,
        regra_resolucao: Some("DAFSA_EXACT".to_string()),
    };
    let t_vela = TokenRimante {
        texto: "vê-la".to_string(),
        base: "ver".to_string(),
        tem_enclise: true,
        pos: PosMask::VERBO,
        regra_resolucao: Some("ENCLISE".to_string()),
    };
    let (qual_preciosa, just_prec) = avaliar_par_rima(&t_estrela, &t_vela);
    assert_eq!(qual_preciosa, QualidadeRima::Preciosa);
    assert_eq!(just_prec, JustificativaAuditoria::AssimetriaEnclise);

    // 4. Rima Indeterminada: Vocábulo Desconhecido
    let t_desconhecido = TokenRimante {
        texto: "albatrozesco".to_string(),
        base: "albatrozesco".to_string(),
        tem_enclise: false,
        pos: PosMask::UNKNOWN,
        regra_resolucao: None,
    };
    let (qual_indet, just_indet) = avaliar_par_rima(&t_desconhecido, &t_lento);
    assert!(matches!(
        qual_indet,
        QualidadeRima::Indeterminada {
            motivo: IndeterminadoMotivo::VocabuloNaoCatalogado { .. }
        }
    ));
    assert!(matches!(
        just_indet,
        JustificativaAuditoria::FalhaLexical(_)
    ));
}

#[test]
fn test_topologia_estrofica_e_classificacao_poema() {
    let soneto = "\
Amor é fogo que arde sem se ver,
É ferida que dói e não se sente;
É um contentamento descontente,
É dor que desatina sem doer.";

    let relatorio = classificar_rimas_poema(soneto);
    assert_eq!(relatorio.estrofes.len(), 1);
    let e0 = &relatorio.estrofes[0];
    assert_eq!(e0.esquema, "ABBA");

    // ABBA interpolação: pares (V1, V4) e (V2, V3)
    assert_eq!(e0.pares_avaliados.len(), 2);

    let par1 = &e0.pares_avaliados[0];
    assert_eq!(par1.versos, (1, 4));
    assert_eq!(par1.tokens, ("ver".to_string(), "doer".to_string()));
    assert_eq!(par1.classificacao, QualidadeRima::Pobre);

    let par2 = &e0.pares_avaliados[1];
    assert_eq!(par2.versos, (2, 3));
    assert_eq!(
        par2.tokens,
        ("sente".to_string(), "descontente".to_string())
    );
    assert_eq!(par2.classificacao, QualidadeRima::Rica);
}

#[test]
fn test_camoes_jacob_rima_preciosa_ve_la() {
    let estrofe2 = "Os dias, na esperança de um só dia,\nPassava, contentando-se com vê-la;\nPorém o pai, usando de cautela,\nEm lugar de Raquel lhe dava Lia.";
    let rel = classificar_rimas_poema(estrofe2);
    let e0 = &rel.estrofes[0];
    assert_eq!(e0.esquema, "ABBA");

    // Par (2, 3) = vê-la x cautela -> Preciosa por assimetria de ênclise!
    let par = &e0.pares_avaliados[1];
    assert_eq!(par.versos, (2, 3));
    assert_eq!(par.tokens, ("vê-la".to_string(), "cautela".to_string()));
    assert_eq!(par.classificacao, QualidadeRima::Preciosa);
}
