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

Quinto retoma o bom ritmo a cantar,
Sexto introduz um clima de fervor,
Sétimo busca o leitor encantar,
Oitavo fecha a quadra com louvor.

Nono prossegue a marcha musical,
Décimo traz a chama do saber,
O verso onze rebrilha no final,
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
    assert_eq!(
        diag.confianca_percentual, 100,
        "Soneto Shakespeariano canônico deve atingir 100%, obteve {}",
        diag.confianca_percentual
    );
}

#[test]
fn test_soneto_shakespeariano_profana_100_por_cento() {
    let texto = "\
Cobiça a carne, que depois recusa,
Exige o molde, pronta a condenar;
Disfarça a culpa no pudor da musa
Enquanto o vício prende o seu olhar.

Despreza a mão que levantou seu muro,
No lindo linho limpa o nojo e ri;
Consome o braço no trabalho duro,
Negando a quem mantém seu trono ali.

Condena o gozo na oração que tece,
Reduz a vida a tráfico venal;
Compra o silêncio na vendida prece,
E julga o mundo em cego tribunal.

Se o teu altar é palco de pecado,
Sangue, suor ou sêmen é sagrado?";

    let poema = escandir_poema(texto, false, false, true, true);
    let diag = avaliar_forma_poetica(&poema).expect("Deve detectar Soneto Inglês (Shakespeariano)");
    assert_eq!(diag.nome, "Soneto Inglês (Shakespeariano)");
    assert_eq!(
        diag.confianca_percentual, 100,
        "Profana deve atingir 100% de conformidade, obteve {}%",
        diag.confianca_percentual
    );
    assert!(
        diag.observacoes.is_empty(),
        "Profana não deve ter observações de desvio, obteve: {:?}",
        diag.observacoes
    );
}

#[test]
fn test_soneto_spenseriano_100_por_cento() {
    // Soneto Spenseriano: 14 versos decassílabos em 4+4+4+2 com rimas entrelaçadas ABAB BCBC CDCD EE
    let texto = "\
Um dia escrevi seu nome na areia,
Porém a onda logo o arrebatou;
Mais uma vez a minha mão alteia,
Mas outra vaga o rastro devorou.

Mulher mortal em vão já pelejou,
Disse ela então com seu tão triste olhar:
Pois o meu ser a cinza já levou,
E o tempo o meu perfil há de apagar.

Não há de ser, respondo ao teu cantar,
Pois na canção teu brilho viverá,
E os céus verão a música ecoar,
Onde minh'alma sempre brilhará.

Quando este chão for puro e bem perfeito,
Nosso amor dormirá em santo leito.";

    let poema = escandir_poema(texto, false, false, true, true);
    let diag = avaliar_forma_poetica(&poema).expect("Deve detectar Soneto Inglês (Shakespeariano / Spenseriano)");
    assert!(diag.nome.contains("Soneto Inglês"));
    assert_eq!(
        diag.confianca_percentual, 100,
        "Spenseriano deve atingir 100% de conformidade, obteve {}%",
        diag.confianca_percentual
    );
}

#[test]
fn test_soneto_shakespeariano_com_desvio_de_rima_e_observacoes() {
    // Soneto Shakespeariano cujo dístico final quebra a rima (versos 13 e 14 não rimam entre si)
    let texto = "\
Cobiça a carne, que depois recusa,
Exige o molde, pronta a condenar;
Disfarça a culpa no pudor da musa
Enquanto o vício prende o seu olhar.

Despreza a mão que levantou seu muro,
No lindo linho limpa o nojo e ri;
Consome o braço no trabalho duro,
Negando a quem mantém seu trono ali.

Condena o gozo na oração que tece,
Reduz a vida a tráfico venal;
Compra o silêncio na vendida prece,
E julga o mundo em cego tribunal.

Se o teu altar é palco de pecado,
Todo este pranto agora é vão lamento.";

    let poema = escandir_poema(texto, false, false, true, true);
    let diag = avaliar_forma_poetica(&poema).expect("Deve detectar aproximação de Soneto Inglês");
    assert!(diag.nome.contains("Soneto Inglês"));
    assert!(
        diag.confianca_percentual < 100,
        "Dístico com rima quebrada não pode pontuar 100%"
    );
    assert!(
        !diag.observacoes.is_empty(),
        "Deve conter observações apontando desvio nas rimas"
    );
}

#[test]
fn test_sonetilha_100_por_cento() {
    // 14 versos em Redondilha Maior (7 sílabas métricas) em 4+4+3+3
    let texto = "\
Na margem do rio manso,
Vejo a tarde desmaiar;
Em vão busco o meu descanso,
Cansado de navegar.

O vento frio me avisa
Que a noite vem ao redor;
Num leve sopro de brisa,
Cresce no peito esta dor.

As sombras cobrem a terra,
O céu perde a claridade;
A escuridão tudo encerra.

Fica no peito a saudade,
Toda a dor que a alma aferra,
Chora na calma cidade.";

    let poema = escandir_poema(texto, false, false, true, true);
    let diag = avaliar_forma_poetica(&poema).expect("Deve detectar Sonetilha");
    assert_eq!(diag.nome, "Sonetilha (Sonetino)");
    assert_eq!(
        diag.confianca_percentual, 100,
        "Sonetilha canônica deve atingir 100%"
    );
}

#[test]
fn test_septilha_cordel_100_por_cento() {
    // 7 versos em Redondilha Maior (7 sílabas métricas), rimas nos versos pares e fecho emparelhado (XAXABBA)
    let texto = "\
O verso de sete linhas
É bonito e compassado;
Quem lê com muita atenção
Fica o leitor encantado;
Tem beleza e perfeição,
Tem nobreza no sertão,
E o verso é bem respeitado.";

    let poema = escandir_poema(texto, false, false, true, true);
    let diag = avaliar_forma_poetica(&poema).expect("Deve detectar Septilha de Cordel");
    assert_eq!(diag.nome, "Septilha de Cordel");
    assert_eq!(
        diag.confianca_percentual, 100,
        "Septilha canônica deve atingir 100%"
    );
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
    let clean_pkg_version = pkg_version
        .split('-')
        .next()
        .unwrap_or("")
        .split('+')
        .next()
        .unwrap_or("");
    assert_eq!(
        clean_pkg_version, clean_version,
        "A versão no Cargo.toml ({}) deve corresponder à versão canônica do CCCP ({})",
        pkg_version, clean_version
    );
}
