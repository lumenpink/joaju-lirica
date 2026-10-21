//! Tests executing the Golden Benchmark Corpus of 50+ classical poems across Portuguese literary history.

use joaju_lirica::{avaliar_forma_poetica, escandir_poema};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct PoemaBenchmark {
    id: String,
    titulo: String,
    autor: String,
    epoca: String,
    metro_esperado: Option<usize>,
    forma_esperada: Option<String>,
    #[serde(default)]
    elidir_pontuacao: bool,
    texto: String,
}

#[test]
fn test_golden_corpus_50_poemas() {
    let json_data = include_str!("corpus/golden_50.json");
    let corpus: Vec<PoemaBenchmark> =
        serde_json::from_str(json_data).expect("Deve desserializar corpus/golden_50.json");

    assert!(
        corpus.len() >= 50,
        "Corpus deve conter pelo menos 50 poemas, contém {}",
        corpus.len()
    );

    let mut total_versos_analisados = 0;

    for poema in &corpus {
        let p = escandir_poema(
            &poema.texto,
            poema.elidir_pontuacao,
            false,
            true, // Castilho
            true, // Rimas
        );

        assert!(
            !p.versos.is_empty(),
            "Poema '{}' ({}) de {} não gerou versos",
            poema.id,
            poema.titulo,
            poema.autor
        );

        total_versos_analisados += p.versos.len();

        // Validar que cada verso tem métrica válida
        for (i, v) in p.versos.iter().enumerate() {
            assert!(
                v.total_metricas > 0,
                "Verso {} de '{}' teve 0 sílabas métricas",
                i + 1,
                poema.id
            );
        }

        // Se há metro esperado canônico (ex.: 10 para decassílabos de Camões, 7 para redondilha)
        if let Some(alvo) = poema.metro_esperado {
            let conformes = p.versos.iter().filter(|v| v.total_metricas == alvo).count();
            let percentual = (conformes as f32 / p.versos.len() as f32) * 100.0;
            assert!(
                percentual >= 50.0,
                "Poema '{}' ({}) deveria ter metro dominante {}, mas apenas {}/{} versos atingiram ({:.1}%)",
                poema.id,
                poema.titulo,
                alvo,
                conformes,
                p.versos.len(),
                percentual
            );
        }

        // Se há forma poética esperada (ex.: Soneto Italiano)
        if let Some(ref forma_nome) = poema.forma_esperada {
            let diag = avaliar_forma_poetica(&p);
            assert!(
                diag.is_some(),
                "Poema '{}' ({}) deveria ser diagnosticado com forma '{}', mas retornou None",
                poema.id,
                poema.titulo,
                forma_nome
            );
            let d = diag.unwrap();
            assert!(
                d.confianca_percentual >= 50,
                "Poema '{}' ({}) diagnosticado como '{}' teve confiança muito baixa ({}%)",
                poema.id,
                poema.titulo,
                d.nome,
                d.confianca_percentual
            );
        }
    }

    println!(
        "✓ Golden Corpus validado com sucesso: {} poemas e {} versos escandidos.",
        corpus.len(),
        total_versos_analisados
    );
}

#[test]
fn test_camoes_soneto_completo_14_decassilabos() {
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

    // Com elidir_pontuacao = true (padrão clássico contínuo)
    let poema = escandir_poema(texto, true, false, true, true);
    assert_eq!(poema.versos.len(), 14);

    for (i, v) in poema.versos.iter().enumerate() {
        assert_eq!(
            v.total_metricas,
            10,
            "Verso {} de Camões ('{}') deve ter exatamente 10 sílabas, teve {}",
            i + 1,
            poema.linhas_originais[i],
            v.total_metricas
        );
        assert!(
            v.classificacao.as_deref().unwrap().contains("Decassílabo"),
            "Verso {} deve ser Decassílabo",
            i + 1
        );
    }

    let diag = avaliar_forma_poetica(&poema).expect("Deve detectar Soneto Petrarquiano");
    assert_eq!(diag.nome, "Soneto Italiano (Petrarquiano)");
    assert_eq!(diag.confianca_percentual, 100);
}

#[test]
fn test_vinicius_soneto_fidelidade_14_decassilabos() {
    let texto = "\
De tudo, ao meu amor serei atento
Antes, e com tal zelo, e sempre tanto
Que mesmo em face do maior encanto
Dele se encante mais meu pensamento.

Quero vivê-lo em cada vão momento
E em seu louvor hei de espalhar meu canto
E rir meu riso e derramar meu pranto
Ao seu pesar ou seu contentamento.

E assim, quando mais tarde me procure
Quem sabe a morte, angústia de quem vive
Quem sabe a solidão, fim de quem ama

Eu possa me dizer do amor (que tive):
Que não seja imortal, posto que é chama
Mas que seja infinito enquanto dure.";

    let poema = escandir_poema(texto, true, false, true, true);
    assert_eq!(poema.versos.len(), 14);

    for (i, v) in poema.versos.iter().enumerate() {
        assert_eq!(
            v.total_metricas,
            10,
            "Verso {} de Vinícius ('{}') deve ter exatamente 10 sílabas, teve {}",
            i + 1,
            poema.linhas_originais[i],
            v.total_metricas
        );
        assert!(
            v.classificacao.as_deref().unwrap().contains("Decassílabo"),
            "Verso {} deve ser Decassílabo",
            i + 1
        );
    }

    let diag = avaliar_forma_poetica(&poema).expect("Deve detectar Soneto Petrarquiano");
    assert_eq!(diag.nome, "Soneto Italiano (Petrarquiano)");
    assert_eq!(diag.confianca_percentual, 100);
}
