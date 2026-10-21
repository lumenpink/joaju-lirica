use joaju_lirica::{avaliar_forma_poetica, avaliar_qualidade_rimas, escandir_poema};

#[test]
fn test_decima_espinela_perfeita() {
    // Exemplo clássico de Décima Espinela em Redondilha Maior (7 sílabas, ABBAACCDDC):
    let texto = "\
Ninguém viva descurado,
Que a fortuna é mui mudável;
Não há glória tão louvável
Que não pare em triste estado.
O bem é pouco durado,
O mal dura e cresce tanto,
Que torna o prazer em pranto;
E quem vive mais contente
Acha no fim mui patente
Causa de perpétuo espanto.";

    let poema = escandir_poema(texto, false, false, true, true);
    let diag = avaliar_forma_poetica(&poema).expect("Deve detectar Décima Espinela");
    assert_eq!(diag.nome, "Décima Espinela");
    assert!(
        diag.confianca_percentual >= 90,
        "Confiança deve ser >= 90%, foi {}",
        diag.confianca_percentual
    );
    assert!(diag.descricao.contains("Vicente Espinel") || diag.descricao.contains("Espinela"));
}

#[test]
fn test_soneto_italiano_camoes_4_4_3_3() {
    // Soneto clássico de Camões com separação em estrofes 4, 4, 3, 3:
    let texto = "\
Amor é fogo que arde sem se ver,
É ferida que dói e não se sente;
É um contentamento descontente,
É dor que desatina sem doer.

É um não querer mais que bem querer;
É um andar solitário entre a gente;
É nunca contentar-se de contente;
É um cuidar que ganha em se perder.

É querer estar preso por vontade;
É servir a quem vence, o vencedor;
É ter com quem nos mata, lealdade.

Mas como causar pode seu favor
Nos corações humanos amizade,
Se tão contrário a si é o mesmo Amor?";

    let poema = escandir_poema(texto, false, false, true, true);
    assert_eq!(poema.estrofes.len(), 4, "Deve reconhecer 4 estrofes");
    assert_eq!(poema.estrofes[0].versos.len(), 4);
    assert_eq!(poema.estrofes[1].versos.len(), 4);
    assert_eq!(poema.estrofes[2].versos.len(), 3);
    assert_eq!(poema.estrofes[3].versos.len(), 3);

    let diag = avaliar_forma_poetica(&poema).expect("Deve detectar Soneto Italiano");
    assert_eq!(diag.nome, "Soneto Italiano (Petrarquiano)");
    assert!(
        diag.confianca_percentual >= 90,
        "Confiança deve ser >= 90%, foi {}",
        diag.confianca_percentual
    );
}

#[test]
fn test_soneto_shakespeariano_4_4_4_2() {
    // 14 versos em 4, 4, 4, 2 decassílabos com esquema ABAB CDCD EFEF GG
    let texto = "\
Verso primeiro para começar,
Outro segundo segue com rigor,
Terceiro verso vem para rimar,
Quarto termina a quadra com valor.

Quinto retoma o ritmo a cantar,
Sexto introduz um clima de fervor,
Sétimo busca o leitor encantar,
Oitavo fecha a quadra com louvor.

Nono prossegue a marcha musical,
Décimo traz a chama do saber,
O verso onze brilha no final,
Doze completa o quadro a resplender.

Por fim o par resume a perfeição,
Fechando a peça com nobre canção.";

    let poema = escandir_poema(texto, false, false, true, true);
    assert_eq!(poema.estrofes.len(), 4);
    assert_eq!(
        poema.estrofes[3].versos.len(),
        2,
        "Última estrofe é dístico"
    );

    let diag = avaliar_forma_poetica(&poema).expect("Deve detectar Soneto Inglês/Shakespeariano");
    assert_eq!(diag.nome, "Soneto Inglês (Shakespeariano)");
    assert!(diag.confianca_percentual >= 85);
}

#[test]
fn test_soneto_com_aproximacao_e_diagnostico_de_desvio() {
    // Soneto onde um verso tem 8 sílabas em vez de 10:
    let texto = "\
Amor é fogo que arde sem se ver,
É ferida que dói e não se sente;
É um contentamento descontente,
É dor que não dói.

É um não querer mais que bem querer;
É um andar solitário entre a gente;
É nunca contentar-se de contente;
É um cuidar que ganha em se perder.

É querer estar preso por vontade;
É servir a quem vence, o vencedor;
É ter com quem nos mata, lealdade.

Mas como causar pode seu favor
Nos corações humanos amizade,
Se tão contrário a si é o mesmo Amor?";

    let poema = escandir_poema(texto, false, false, true, true);
    let diag = avaliar_forma_poetica(&poema).expect("Deve detectar aproximação de Soneto");
    assert!(diag.nome.contains("Soneto"));
    // Deve ficar entre 75% e 94% de confiança
    assert!(diag.confianca_percentual < 95 && diag.confianca_percentual >= 70);
    // Deve apontar a observação do desvio métrico
    assert!(
        diag.observacoes.iter().any(|obs| obs.contains("Verso 4")),
        "Deve diagnosticar o verso 4 fora da métrica"
    );
}

#[test]
fn test_haicai() {
    let texto = "\
Noite de verão
A lua brilha serena
Voz de um sabiá";

    let poema = escandir_poema(texto, false, false, true, true);
    let diag = avaliar_forma_poetica(&poema).expect("Deve detectar Haicai");
    assert_eq!(diag.nome, "Haicai");
    assert!(
        diag.confianca_percentual >= 80,
        "Confiança deve ser >= 80%, foi {}",
        diag.confianca_percentual
    );
}

#[test]
fn test_avaliacao_qualidade_rimas() {
    // 4 versos com rimas perfeitas consoantes
    let texto = "\
Amor é fogo que arde sem se ver,
É ferida que dói e não se sente;
É um contentamento descontente,
É dor que desatina sem doer.";

    let poema = escandir_poema(texto, false, false, true, true);
    let rima_info = avaliar_qualidade_rimas(&poema);
    assert_eq!(rima_info.esquema, "ABBA");
    assert_eq!(rima_info.pureza_acustica, "Consoante (Perfeita)");
    assert_eq!(rima_info.tipo_disposicao, "Interpoladas (Abraçadas)");
    assert_eq!(rima_info.percentual_rimado, 100);
}

#[test]
fn test_versao_cargo_consistente_com_cccp() {
    let pkg_version = env!("CARGO_PKG_VERSION");
    let version_file_content =
        std::fs::read_to_string("VERSION").unwrap_or_else(|_| pkg_version.to_string());
    let clean_version = version_file_content
        .trim()
        .split('-')
        .next()
        .unwrap_or("")
        .split('+')
        .next()
        .unwrap_or("");
    assert_eq!(
        pkg_version, clean_version,
        "A versão no Cargo.toml ({}) deve corresponder à versão canônica do CCCP ({})",
        pkg_version, clean_version
    );
}
