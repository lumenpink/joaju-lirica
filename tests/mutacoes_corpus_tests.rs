//! Suíte de testes de estresse por Mutações Declarativas no Gabarito Canônico.
//! Aplica deltas cirúrgicos aos poemas matrizes e valida os desvios esperados.

use joaju_lirica::{avaliar_forma_poetica, escandir_poema};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PoemaGabarito {
    pub id: String,
    pub titulo: String,
    pub autor: String,
    pub epoca: String,
    pub metro_esperado: Option<usize>,
    pub forma_esperada: Option<String>,
    #[serde(default)]
    pub elidir_pontuacao: bool,
    pub versos_esperados: Vec<usize>,
    pub texto: String,
    #[serde(default)]
    pub mutacoes: Vec<MutacaoPoema>,
}

#[derive(Debug, Deserialize)]
pub struct MutacaoPoema {
    pub id: String,
    pub tipo: String,
    pub descricao: String,
    #[serde(default)]
    pub operacoes: Vec<OperacaoLinha>,
    #[serde(default)]
    pub flags: FlagsMutacao,
    pub expectativa: ExpectativaMutacao,
}

#[derive(Debug, Deserialize, Clone)]
pub struct OperacaoLinha {
    pub op: String,
    pub linha: Option<usize>,
    pub texto: Option<String>,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct FlagsMutacao {
    pub contagem_castilho: Option<bool>,
    pub elidir_pontuacao: Option<bool>,
    pub forcar_rima_estrita: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct ExpectativaMutacao {
    pub forma_esperada: Option<String>,
    pub nota_min: Option<u8>,
    pub nota_max: Option<u8>,
    pub observacoes_obrigatorias: Option<Vec<String>>,
}

/// Aplica as operações de mutação sobre o texto do poema original.
pub fn aplicar_mutacoes_texto(texto_original: &str, operacoes: &[OperacaoLinha]) -> String {
    let mut linhas: Vec<String> = texto_original.lines().map(|s| s.to_string()).collect();

    for op in operacoes {
        match op.op.as_str() {
            "remover_espacos_estroficos" => {
                linhas.retain(|l| !l.trim().is_empty());
            }
            "substituir" => {
                if let Some(num_linha) = op.linha {
                    let mut contador = 0;
                    for l in linhas.iter_mut() {
                        if !l.trim().is_empty() {
                            contador += 1;
                            if contador == num_linha {
                                if let Some(novo_texto) = &op.texto {
                                    *l = novo_texto.clone();
                                }
                                break;
                            }
                        }
                    }
                }
            }
            "remover" => {
                if let Some(num_linha) = op.linha {
                    let mut contador = 0;
                    let mut indice_remover = None;
                    for (idx, l) in linhas.iter().enumerate() {
                        if !l.trim().is_empty() {
                            contador += 1;
                            if contador == num_linha {
                                indice_remover = Some(idx);
                                break;
                            }
                        }
                    }
                    if let Some(idx) = indice_remover {
                        linhas.remove(idx);
                    }
                }
            }
            "inserir" => {
                if let Some(num_linha) = op.linha {
                    let mut contador = 0;
                    let mut indice_inserir = linhas.len();
                    for (idx, l) in linhas.iter().enumerate() {
                        if !l.trim().is_empty() {
                            contador += 1;
                            if contador == num_linha {
                                indice_inserir = idx;
                                break;
                            }
                        }
                    }
                    if let Some(novo_texto) = &op.texto {
                        linhas.insert(indice_inserir, novo_texto.clone());
                    }
                }
            }
            outro => panic!("Operação de mutação desconhecida: {}", outro),
        }
    }

    linhas.join("\n")
}

#[test]
fn test_execucao_todas_mutacoes_declarativas_corpus() {
    let json_data = include_str!("corpus/gabarito_canonico.json");
    let gabarito: Vec<PoemaGabarito> =
        serde_json::from_str(json_data).expect("Deve desserializar gabarito com mutações");

    let total_mutacoes: usize = gabarito.iter().map(|p| p.mutacoes.len()).sum();
    assert!(
        total_mutacoes >= 50,
        "O gabarito deve conter pelo menos 50 mutações declarativas exaustivas, encontrou {}",
        total_mutacoes
    );

    for poema in &gabarito {
        for mutacao in &poema.mutacoes {
            let texto_mutado = aplicar_mutacoes_texto(&poema.texto, &mutacao.operacoes);

            let castilho = mutacao.flags.contagem_castilho.unwrap_or(true); // Padrão é true
            let elidir_pont = mutacao
                .flags
                .elidir_pontuacao
                .unwrap_or(poema.elidir_pontuacao);
            let rima = mutacao.flags.forcar_rima_estrita.unwrap_or(true);

            let p = escandir_poema(&texto_mutado, elidir_pont, false, castilho, rima);
            let diag = avaliar_forma_poetica(&p)
                .unwrap_or_else(|| panic!("Falha ao avaliar forma da mutação '{}'", mutacao.id));

            if let Some(ref forma_esp) = mutacao.expectativa.forma_esperada {
                assert!(
                    diag.nome.contains(forma_esp),
                    "Mutação '{}': forma esperada contendo '{}', obtida '{}'",
                    mutacao.id,
                    forma_esp,
                    diag.nome
                );
            }

            if let Some(nota_min) = mutacao.expectativa.nota_min {
                assert!(
                    diag.confianca_percentual >= nota_min,
                    "Mutação '{}': nota {} abaixo da mínima esperada {}",
                    mutacao.id,
                    diag.confianca_percentual,
                    nota_min
                );
            }

            if let Some(nota_max) = mutacao.expectativa.nota_max {
                assert!(
                    diag.confianca_percentual <= nota_max,
                    "Mutação '{}': nota {} acima da máxima esperada {}",
                    mutacao.id,
                    diag.confianca_percentual,
                    nota_max
                );
            }

            if let Some(ref obs_obrig) = mutacao.expectativa.observacoes_obrigatorias {
                for obs_esperada in obs_obrig {
                    assert!(
                        diag.observacoes
                            .iter()
                            .any(|o| o.to_lowercase().contains(&obs_esperada.to_lowercase())),
                        "Mutação '{}': observação obrigatória '{}' não encontrada em: {:?}",
                        mutacao.id,
                        obs_esperada,
                        diag.observacoes
                    );
                }
            }
        }
    }
}
