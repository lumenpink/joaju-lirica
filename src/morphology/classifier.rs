//! Classificador qualitativo de rimas e topologia de pares estróficos.

use serde::{Deserialize, Serialize};

use super::clitics::descolar_enclise;
use super::dafsa::consultar_lexico;
use super::lookback::desambiguar_por_contexto;
use super::pos_mask::PosMask;
use super::suffixes::inferir_por_sufixo;
use crate::rhyme::analisar_esquema_rimas;

/// Token morfológico no final do verso para avaliação de rima.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TokenRimante {
    /// Texto original bruto do token final.
    pub texto: String,
    /// Base normalizada (sem ênclise, convertida para infinitivo ou lema).
    pub base: String,
    /// Indica se o token continha pronome enclítico ou mesoclítico.
    pub tem_enclise: bool,
    /// Máscara morfológica inferida para o token.
    pub pos: PosMask,
    /// Identificador da regra determinística que resolveu a classe do token.
    pub regra_resolucao: Option<String>,
}

/// Classificação qualitativa determinística da rima.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum QualidadeRima {
    /// Rima entre vocábulos da mesma classe gramatical (ex.: Substantivo x Substantivo).
    Pobre,
    /// Rima entre vocábulos de classes gramaticais disjuntas (ex.: Substantivo x Adjetivo).
    Rica,
    /// Rima com assimetria de ênclise/mesóclise (ex.: "vê-la" x "estrela").
    Preciosa,
    /// Vocábulos homógrafos idênticos rimando entre si no mesmo sentido ou grafia rara.
    Rara,
    /// Impossibilidade determinística de resolução da classe ou vocábulo desconhecido.
    Indeterminada {
        /// Razão objetiva da indeterminação.
        motivo: IndeterminadoMotivo,
    },
}

/// Razão explícita para impossibilidade determinística de resolução da rima.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum IndeterminadoMotivo {
    /// O vocábulo não consta na base léxica canônica e nenhum sufixo produtivo permitiu inferência.
    VocabuloNaoCatalogado {
        /// Palavra desconhecida.
        palavra: String,
    },
    /// A palavra reteve múltiplos bits de classe morfológica e o contexto imediato não ativou regras.
    AmbiguidadeIrresoluvel {
        /// Palavra ambígua.
        palavra: String,
        /// Máscara morfológica com as classes concorrentes.
        mascara: PosMask,
    },
}

/// Justificativa textual e auditável para o diagnóstico do par de rima.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum JustificativaAuditoria {
    /// Ambos os termos compartilham a mesma classe morfológica.
    MesmaClasse(PosMask),
    /// Os termos pertencem a classes morfológicas distintas.
    ClassesDistintas(PosMask, PosMask),
    /// Par assimétrico envolvendo pronome enclítico/mesoclítico.
    AssimetriaEnclise,
    /// Ambiguidade sintática não resolvida pelo contexto local.
    AmbiguidadeSintatica(String),
    /// Vocábulo ausente na base léxica e nos autômatos de derivação.
    FalhaLexical(String),
}

/// Avaliação completa de um par rimante.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParRimaAvaliado {
    /// Número de linha dos versos rimantes (1-based).
    pub versos: (usize, usize),
    /// Palavras finais dos dois versos.
    pub tokens: (String, String),
    /// Classificação qualitativa atribuída ao par.
    pub classificacao: QualidadeRima,
    /// Justificativa analítica para auditoria.
    pub auditoria: JustificativaAuditoria,
}

/// Diagnóstico estruturado por estrofe.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EstrofeRimas {
    /// Índice sequencial da estrofe (0-based).
    pub indice: usize,
    /// Esquema rímico detectado para a estrofe (ex.: "ABBA").
    pub esquema: String,
    /// Lista de pares rimantes avaliados conforme a topologia estrófica.
    pub pares_avaliados: Vec<ParRimaAvaliado>,
}

/// Relatório global de classificação de rimas de uma peça poética.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelatorioRimasPoema {
    /// Diagnósticos estróficos individuais.
    pub estrofes: Vec<EstrofeRimas>,
}

/// Extrai e resolve o token rimante terminal de uma linha de verso.
pub fn extrair_token_rimante(linha: &str, _num_linha: usize) -> TokenRimante {
    let limpo = linha.trim();
    let palavras: Vec<&str> = limpo
        .split_whitespace()
        .map(|w| w.trim_matches(|c: char| !c.is_alphabetic() && c != '-'))
        .filter(|w| !w.is_empty())
        .collect();

    if palavras.is_empty() {
        return TokenRimante {
            texto: String::new(),
            base: String::new(),
            tem_enclise: false,
            pos: PosMask::UNKNOWN,
            regra_resolucao: None,
        };
    }

    let t0 = palavras.last().unwrap();
    let anteriores = &palavras[..palavras.len() - 1];

    // Estágio 1: Descolamento de Ênclise
    let (base_verbal, clitico, tem_enclise) = descolar_enclise(t0);
    if tem_enclise {
        return TokenRimante {
            texto: (*t0).to_string(),
            base: base_verbal,
            tem_enclise: true,
            pos: PosMask::VERBO,
            regra_resolucao: Some(format!("ENCLISE({})", clitico.unwrap_or_default())),
        };
    }

    // Estágio 2: Consulta Léxica
    let mut pos = consultar_lexico(&base_verbal).unwrap_or(PosMask::UNKNOWN);
    let mut regra = None;

    if pos != PosMask::UNKNOWN {
        if pos.is_resolved() {
            regra = Some("DAFSA_EXACT".to_string());
        }
    } else {
        // Estágio 2b: Sufixação Produtiva
        if let Some(pos_sufixo) = inferir_por_sufixo(&base_verbal) {
            pos = pos_sufixo;
            regra = Some("SUFIXO_PRODUTIVO".to_string());
        }
    }

    // Estágio 3: Desambiguação por Retroanálise (Lookback)
    if pos.is_ambiguous() {
        let (pos_resolvida, regra_ctx) = desambiguar_por_contexto(&base_verbal, pos, anteriores);
        if let Some(r) = regra_ctx {
            pos = pos_resolvida;
            regra = Some(r.to_string());
        }
    }

    TokenRimante {
        texto: (*t0).to_string(),
        base: base_verbal,
        tem_enclise: false,
        pos,
        regra_resolucao: regra,
    }
}

/// Avalia deterministricamente a qualidade de um par de rima entre dois tokens.
pub fn avaliar_par_rima(
    t1: &TokenRimante,
    t2: &TokenRimante,
) -> (QualidadeRima, JustificativaAuditoria) {
    // 1. Checagem de Vocábulos Inexistentes / Desconhecidos
    if t1.pos == PosMask::UNKNOWN {
        return (
            QualidadeRima::Indeterminada {
                motivo: IndeterminadoMotivo::VocabuloNaoCatalogado {
                    palavra: t1.texto.clone(),
                },
            },
            JustificativaAuditoria::FalhaLexical(t1.texto.clone()),
        );
    }
    if t2.pos == PosMask::UNKNOWN {
        return (
            QualidadeRima::Indeterminada {
                motivo: IndeterminadoMotivo::VocabuloNaoCatalogado {
                    palavra: t2.texto.clone(),
                },
            },
            JustificativaAuditoria::FalhaLexical(t2.texto.clone()),
        );
    }

    // 2. Rimas Preciosas por Assimetria de Ênclise
    if t1.tem_enclise != t2.tem_enclise {
        return (
            QualidadeRima::Preciosa,
            JustificativaAuditoria::AssimetriaEnclise,
        );
    }

    // 3. Checagem de Ambiguidade Irresolvida
    if t1.pos.is_ambiguous() {
        return (
            QualidadeRima::Indeterminada {
                motivo: IndeterminadoMotivo::AmbiguidadeIrresoluvel {
                    palavra: t1.texto.clone(),
                    mascara: t1.pos,
                },
            },
            JustificativaAuditoria::AmbiguidadeSintatica(t1.texto.clone()),
        );
    }
    if t2.pos.is_ambiguous() {
        return (
            QualidadeRima::Indeterminada {
                motivo: IndeterminadoMotivo::AmbiguidadeIrresoluvel {
                    palavra: t2.texto.clone(),
                    mascara: t2.pos,
                },
            },
            JustificativaAuditoria::AmbiguidadeSintatica(t2.texto.clone()),
        );
    }

    // 4. Comparação de Máscaras Resolvidas
    if (t1.pos & t2.pos).is_empty() {
        (
            QualidadeRima::Rica,
            JustificativaAuditoria::ClassesDistintas(t1.pos, t2.pos),
        )
    } else {
        (
            QualidadeRima::Pobre,
            JustificativaAuditoria::MesmaClasse(t1.pos),
        )
    }
}

/// Analisa e classifica deterministricamente as rimas de todas as estrofes de um poema.
pub fn classificar_rimas_poema(texto: &str) -> RelatorioRimasPoema {
    let blocos: Vec<&str> = texto.split("\n\n").collect();
    let mut estrofes_relatorio = Vec::new();
    let mut offset_verso = 1;

    for (idx, bloco) in blocos.iter().enumerate() {
        let linhas: Vec<&str> = bloco.lines().filter(|l| !l.trim().is_empty()).collect();
        if linhas.is_empty() {
            continue;
        }

        let num_versos = linhas.len();
        let tokens: Vec<TokenRimante> = linhas
            .iter()
            .enumerate()
            .map(|(i, l)| extrair_token_rimante(l, offset_verso + i))
            .collect();

        let linhas_vec: Vec<String> = linhas.iter().map(|s| s.to_string()).collect();
        let (_infos, opt_esquema) = analisar_esquema_rimas(&linhas_vec);
        let esquema = opt_esquema.unwrap_or_else(|| "-".repeat(num_versos));
        let chars_esquema: Vec<char> = esquema.chars().collect();

        let mut pares_avaliados = Vec::new();

        // Topologia Canônica para Quartetos
        if num_versos == 4 && chars_esquema.len() == 4 {
            let s: String = chars_esquema.iter().collect();
            match s.as_str() {
                "ABBA" => {
                    // Interpolada: (1, 4) e (2, 3)
                    let (q1, j1) = avaliar_par_rima(&tokens[0], &tokens[3]);
                    pares_avaliados.push(ParRimaAvaliado {
                        versos: (offset_verso, offset_verso + 3),
                        tokens: (tokens[0].texto.clone(), tokens[3].texto.clone()),
                        classificacao: q1,
                        auditoria: j1,
                    });

                    let (q2, j2) = avaliar_par_rima(&tokens[1], &tokens[2]);
                    pares_avaliados.push(ParRimaAvaliado {
                        versos: (offset_verso + 1, offset_verso + 2),
                        tokens: (tokens[1].texto.clone(), tokens[2].texto.clone()),
                        classificacao: q2,
                        auditoria: j2,
                    });
                }
                "ABAB" => {
                    // Cruzada: (1, 3) e (2, 4)
                    let (q1, j1) = avaliar_par_rima(&tokens[0], &tokens[2]);
                    pares_avaliados.push(ParRimaAvaliado {
                        versos: (offset_verso, offset_verso + 2),
                        tokens: (tokens[0].texto.clone(), tokens[2].texto.clone()),
                        classificacao: q1,
                        auditoria: j1,
                    });

                    let (q2, j2) = avaliar_par_rima(&tokens[1], &tokens[3]);
                    pares_avaliados.push(ParRimaAvaliado {
                        versos: (offset_verso + 1, offset_verso + 3),
                        tokens: (tokens[1].texto.clone(), tokens[3].texto.clone()),
                        classificacao: q2,
                        auditoria: j2,
                    });
                }
                "AABB" => {
                    // Emparelhada: (1, 2) e (3, 4)
                    let (q1, j1) = avaliar_par_rima(&tokens[0], &tokens[1]);
                    pares_avaliados.push(ParRimaAvaliado {
                        versos: (offset_verso, offset_verso + 1),
                        tokens: (tokens[0].texto.clone(), tokens[1].texto.clone()),
                        classificacao: q1,
                        auditoria: j1,
                    });

                    let (q2, j2) = avaliar_par_rima(&tokens[2], &tokens[3]);
                    pares_avaliados.push(ParRimaAvaliado {
                        versos: (offset_verso + 2, offset_verso + 3),
                        tokens: (tokens[2].texto.clone(), tokens[3].texto.clone()),
                        classificacao: q2,
                        auditoria: j2,
                    });
                }
                _ => {
                    // Agrupamento genérico por feixe de caracteres
                    pares_avaliados.extend(avaliar_grupos_genericos(&tokens, &chars_esquema, offset_verso));
                }
            }
        } else {
            // Grupos e estrofes genéricas (tercetos, sextilhas, oitavas, etc.)
            pares_avaliados.extend(avaliar_grupos_genericos(&tokens, &chars_esquema, offset_verso));
        }

        estrofes_relatorio.push(EstrofeRimas {
            indice: idx,
            esquema,
            pares_avaliados,
        });

        offset_verso += num_versos;
    }

    RelatorioRimasPoema {
        estrofes: estrofes_relatorio,
    }
}

fn avaliar_grupos_genericos(
    tokens: &[TokenRimante],
    chars_esquema: &[char],
    offset_verso: usize,
) -> Vec<ParRimaAvaliado> {
    let mut map_grupos: std::collections::BTreeMap<char, Vec<usize>> = std::collections::BTreeMap::new();
    for (i, &ch) in chars_esquema.iter().enumerate() {
        if i < tokens.len() {
            map_grupos.entry(ch).or_default().push(i);
        }
    }

    let mut resultado = Vec::new();

    for (&ch, indices) in &map_grupos {
        if ch == '-' || indices.len() < 2 {
            continue;
        }

        // Cadeia sequencial de adjacência linear
        for w in indices.windows(2) {
            let i1 = w[0];
            let i2 = w[1];
            let (qual, just) = avaliar_par_rima(&tokens[i1], &tokens[i2]);
            resultado.push(ParRimaAvaliado {
                versos: (offset_verso + i1, offset_verso + i2),
                tokens: (tokens[i1].texto.clone(), tokens[i2].texto.clone()),
                classificacao: qual,
                auditoria: just,
            });
        }
    }

    resultado
}
