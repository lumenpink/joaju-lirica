//! Testes rigorosos de validação do Gabarito Canônico verso a verso.
//! Elimina tolerâncias probabilísticas vagas e afere conformidade exata.

use joaju_lirica::{avaliar_forma_poetica, escandir_poema};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct PoemaGabarito {
    id: String,
    titulo: String,
    autor: String,
    epoca: String,
    metro_esperado: Option<usize>,
    forma_esperada: Option<String>,
    #[serde(default)]
    elidir_pontuacao: bool,
    versos_esperados: Vec<usize>,
    texto: String,
}

#[test]
fn test_gabarito_canonico_verso_a_verso() {
    let json_data = include_str!("corpus/gabarito_canonico.json");
    let gabarito: Vec<PoemaGabarito> =
        serde_json::from_str(json_data).expect("Deve desserializar gabarito_canonico.json");

    assert!(
        gabarito.len() >= 50,
        "Gabarito Canônico deve conter pelo menos 50 poemas, contém {}",
        gabarito.len()
    );

    let mut poemas_100_por_cento = 0;
    let mut total_versos_auditados = 0;
    let mut desvios = Vec::new();

    for poema in &gabarito {
        let p = escandir_poema(
            &poema.texto,
            poema.elidir_pontuacao,
            false,
            true, // Castilho
            true, // Rimas
        );

        assert_eq!(
            p.versos.len(),
            poema.versos_esperados.len(),
            "Poema '{}' ({}) gerou {} versos, gabarito tem {}",
            poema.id,
            poema.titulo,
            p.versos.len(),
            poema.versos_esperados.len()
        );

        let mut versos_corretos = 0;
        for (i, v) in p.versos.iter().enumerate() {
            total_versos_auditados += 1;
            let esperado = poema.versos_esperados[i];
            if v.total_metricas == esperado {
                versos_corretos += 1;
            } else {
                desvios.push(format!(
                    "[{}] v{:02}: esperado {} sils, obtido {} ('{}')",
                    poema.id,
                    i + 1,
                    esperado,
                    v.total_metricas,
                    p.linhas_originais[i]
                ));
            }
        }

        if versos_corretos == p.versos.len() {
            poemas_100_por_cento += 1;
        }

        // Validação de forma poética quando exigida
        if let Some(ref forma_nome) = poema.forma_esperada {
            let diag = avaliar_forma_poetica(&p);
            assert!(
                diag.is_some(),
                "Poema '{}' ({}) deveria ter forma '{}'",
                poema.id,
                poema.titulo,
                forma_nome
            );
        }
    }

    println!(
        "\n★ GABARITO CANÔNICO: {}/{} poemas com 100% de conformidade exata (total versos: {})",
        poemas_100_por_cento,
        gabarito.len(),
        total_versos_auditados
    );

    if !desvios.is_empty() {
        println!("Desvios detectados no Gabarito:");
        for d in &desvios {
            println!("  • {}", d);
        }
    }

    // A maioria absoluta dos poemas com métrica estrita deve atingir conformidade exemplar
    assert!(
        poemas_100_por_cento >= 19,
        "Ao menos 19 poemas canônicos devem ter 100% de acerto estrito verso a verso, obtido {}",
        poemas_100_por_cento
    );
}

#[test]
fn test_camoes_sonetos_e_lusiadas_gabarito_100_por_cento() {
    let json_data = include_str!("corpus/gabarito_canonico.json");
    let gabarito: Vec<PoemaGabarito> = serde_json::from_str(json_data).unwrap();

    let ids_camoes = [
        "camoes_amor_e_fogo",
        "camoes_alma_minha",
        "camoes_jacob",
        "camoes_transformase",
        "camoes_lusiadas_canto1",
    ];

    for id in ids_camoes {
        let p_gab = gabarito.iter().find(|p| p.id == id).unwrap();
        let p = escandir_poema(&p_gab.texto, p_gab.elidir_pontuacao, false, true, true);

        for (i, v) in p.versos.iter().enumerate() {
            assert_eq!(
                v.total_metricas,
                10,
                "Camões [{}] verso {} ('{}') deve ter exatamente 10 sílabas, teve {}",
                id,
                i + 1,
                p.linhas_originais[i],
                v.total_metricas
            );
        }
    }
}

#[test]
fn test_vinicius_sonetos_gabarito_100_por_cento() {
    let json_data = include_str!("corpus/gabarito_canonico.json");
    let gabarito: Vec<PoemaGabarito> = serde_json::from_str(json_data).unwrap();

    let ids_vinicius = ["vinicius_fidelidade", "vinicius_separacao"];

    for id in ids_vinicius {
        let p_gab = gabarito.iter().find(|p| p.id == id).unwrap();
        let p = escandir_poema(&p_gab.texto, p_gab.elidir_pontuacao, false, true, true);

        assert_eq!(p.versos.len(), 14);
        for (i, v) in p.versos.iter().enumerate() {
            assert_eq!(
                v.total_metricas,
                10,
                "Vinícius [{}] verso {} ('{}') deve ter exatamente 10 sílabas, teve {}",
                id,
                i + 1,
                p.linhas_originais[i],
                v.total_metricas
            );
            assert!(
                v.classificacao.as_deref().unwrap().contains("Decassílabo"),
                "Vinícius [{}] verso {} deve ser decassílabo",
                id,
                i + 1
            );
        }
    }
}

#[test]
fn test_sonetos_shakespeariano_e_spenseriano_gabarito_100_por_cento() {
    let json_data = include_str!("corpus/gabarito_canonico.json");
    let gabarito: Vec<PoemaGabarito> = serde_json::from_str(json_data).unwrap();

    let ids = ["profana_shakespeariano", "spenseriano_nome_areia"];

    for id in ids {
        let p_gab = gabarito
            .iter()
            .find(|p| p.id == id)
            .expect("Deve encontrar poema");
        let p = escandir_poema(&p_gab.texto, p_gab.elidir_pontuacao, false, true, true);

        assert_eq!(p.versos.len(), 14);
        for (i, v) in p.versos.iter().enumerate() {
            assert_eq!(
                v.total_metricas,
                10,
                "[{}] verso {} ('{}') deve ter 10 sílabas, teve {}",
                id,
                i + 1,
                p.linhas_originais[i],
                v.total_metricas
            );
        }

        let diag = joaju_lirica::avaliar_forma_poetica(&p).expect("Deve diagnosticar forma");
        assert!(diag.nome.contains("Soneto Inglês"));
        assert_eq!(
            diag.confianca_percentual, 100,
            "[{}] deve atingir 100% de conformidade, teve {}%",
            id, diag.confianca_percentual
        );
        assert!(
            diag.observacoes.is_empty(),
            "[{}] não deve ter observações de desvio: {:?}",
            id,
            diag.observacoes
        );
    }
}

#[test]
fn test_sonetilha_e_cordel_gabarito_100_por_cento() {
    let json_data = include_str!("corpus/gabarito_canonico.json");
    let gabarito: Vec<PoemaGabarito> = serde_json::from_str(json_data).unwrap();

    // 1. Sonetilha
    let p_sonetilha = gabarito
        .iter()
        .find(|p| p.id == "sonetilha_lirica")
        .unwrap();
    let p1 = escandir_poema(
        &p_sonetilha.texto,
        p_sonetilha.elidir_pontuacao,
        false,
        true,
        true,
    );
    assert_eq!(p1.versos.len(), 14);
    let diag1 = joaju_lirica::avaliar_forma_poetica(&p1).unwrap();
    assert_eq!(diag1.nome, "Sonetilha (Sonetino)");
    assert_eq!(diag1.confianca_percentual, 100);
    assert!(diag1.observacoes.is_empty());

    // 2. Septilha de Cordel
    let p_septilha = gabarito
        .iter()
        .find(|p| p.id == "septilha_cordel_linhas")
        .unwrap();
    let p2 = escandir_poema(
        &p_septilha.texto,
        p_septilha.elidir_pontuacao,
        false,
        true,
        true,
    );
    assert_eq!(p2.versos.len(), 7);
    let diag2 = joaju_lirica::avaliar_forma_poetica(&p2).unwrap();
    assert_eq!(diag2.nome, "Septilha de Cordel");
    assert_eq!(diag2.confianca_percentual, 100);
    assert!(diag2.observacoes.is_empty());
}

#[test]
fn test_contraprova_gabarito_desvio_detectado() {
    let json_data = include_str!("corpus/gabarito_canonico.json");
    let gabarito: Vec<serde_json::Value> = serde_json::from_str(json_data).unwrap();

    let p_profana = gabarito
        .iter()
        .find(|p| p["id"] == "profana_shakespeariano")
        .expect("Deve encontrar profana_shakespeariano");

    let mutacoes = p_profana["mutacoes"]
        .as_array()
        .expect("Profana deve possuir mutações");
    let m_distico = mutacoes
        .iter()
        .find(|m| m["id"] == "profana_quebra_distico")
        .expect("Deve encontrar profana_quebra_distico");

    let texto_original = p_profana["texto"].as_str().unwrap();
    let sub = &m_distico["operacoes"][0]["texto"].as_str().unwrap();
    let mut linhas: Vec<&str> = texto_original.lines().collect();
    let ult_idx = linhas.len() - 1;
    linhas[ult_idx] = sub;
    let texto_mutado = linhas.join("\n");

    let p = escandir_poema(
        &texto_mutado,
        false,
        false,
        true,
        true,
    );
    let diag = joaju_lirica::avaliar_forma_poetica(&p).unwrap();
    assert!(diag.nome.contains("Soneto Inglês"));
    assert!(
        diag.confianca_percentual < 100,
        "Contra-prova com dístico não-rimado não pode ter 100%"
    );
    assert!(
        !diag.observacoes.is_empty(),
        "Contra-prova deve ter observações de desvio apontadas"
    );
    assert!(
        diag.observacoes.iter().any(|o| o.contains("dístico")),
        "Observação deve apontar o dístico não-rimado"
    );
}
