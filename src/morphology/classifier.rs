//! Classificador qualitativo de rimas e topologia de pares estróficos.

use serde::{Deserialize, Serialize};

use super::cliches::verificar_cliche;
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
    /// Rima entre vocábulos comprovadamente da mesma classe gramatical (100% certeza).
    Pobre,
    /// Rima entre vocábulos comprovadamente de classes gramaticais disjuntas (100% certeza).
    Rica,
    /// Rima com assimetria comprovada de ênclise/mesóclise (ex.: "vê-la" x "estrela").
    Preciosa,
    /// Rima com escassez fonológica atestada no idioma ou vocábulos esdrúxulos raros.
    Rara,
    /// Rima entre vocábulos que convergem para a mesma esfera categórica
    /// (ex.: particípios equivalentes ou substantivos poéticos com homógrafo verbal raro).
    ProvavelPobre {
        /// Justificativa detalhada do alinhamento provável.
        justificativa: String,
    },
    /// Rima entre vocábulos com forte evidência de classes distintas na hipótese literária primária
    /// (ex.: um termo é Verbo garantido e o outro é Substantivo de uso comum frente a homógrafo raro).
    ProvavelRica {
        /// Justificativa detalhada da distinção provável.
        justificativa: String,
    },
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
    /// Hipótese provável fundamentada em evidência lexical, sufixal ou participial.
    HipoteseProvavel(String),
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
    /// Indica se o par forma uma rima clichê / hiper-saturada na lírica.
    pub eh_cliche: bool,
    /// Motivo estilístico detalhado da rima clichê, se aplicável.
    pub motivo_cliche: Option<String>,
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

fn is_terminacao_rara(p1: &str, p2: &str) -> bool {
    const SUFIXOS_RAROS: &[&str] = &[
        "austro", "austros", "austra", "austras", "urbe", "urbes", "esbio", "ésbio", "esbios",
        "ésbios", "esbia", "ésbia", "efiro", "éfiro", "efiros", "éfiros", "pelago", "pélago",
        "pelagos", "pélagos", "cubico", "cúbico", "cubicos", "cúbicos", "fago", "fagos", "fenix",
        "fênix", "helix", "hélix", "calix", "cálix",
    ];
    let l1 = p1.to_lowercase();
    let l2 = p2.to_lowercase();
    for suf in SUFIXOS_RAROS {
        if l1.ends_with(suf) && l2.ends_with(suf) {
            return true;
        }
    }
    false
}

fn is_participio(w: &str) -> bool {
    let lower = w.to_lowercase();
    lower.ends_with("ado")
        || lower.ends_with("ada")
        || lower.ends_with("ados")
        || lower.ends_with("adas")
        || lower.ends_with("ido")
        || lower.ends_with("ida")
        || lower.ends_with("idos")
        || lower.ends_with("idas")
        || lower.ends_with("erto")
        || lower.ends_with("ertos")
        || lower.ends_with("erta")
        || lower.ends_with("ertas")
}

fn is_sufixo_adjetival(w: &str) -> bool {
    let lower = w.to_lowercase();
    lower.ends_with("oso")
        || lower.ends_with("osa")
        || lower.ends_with("osos")
        || lower.ends_with("osas")
        || lower.ends_with("ente")
        || lower.ends_with("entes")
        || lower.ends_with("ante")
        || lower.ends_with("antes")
        || lower.ends_with("vel")
        || lower.ends_with("veis")
        || lower.ends_with("vél")
}

fn is_sufixo_nominal(w: &str) -> bool {
    let lower = w.to_lowercase();
    lower.ends_with("ção")
        || lower.ends_with("ções")
        || lower.ends_with("dade")
        || lower.ends_with("dades")
        || lower.ends_with("mento")
        || lower.ends_with("mentos")
        || lower.ends_with("ura")
        || lower.ends_with("uras")
        || lower.ends_with("eza")
        || lower.ends_with("ezas")
}

fn is_termo_predominantemente_nominal(w: &str) -> bool {
    matches!(
        w.to_lowercase().as_str(),
        "tarde"
            | "noite"
            | "dia"
            | "vida"
            | "ouro"
            | "hora"
            | "palma"
            | "violas"
            | "açoite"
            | "recusa"
            | "chama"
            | "pecado"
            | "abrigo"
            | "trigo"
            | "mora"
            | "laço"
            | "espanto"
            | "pranto"
            | "caminho"
            | "atenção"
            | "perfeição"
            | "corolas"
    )
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

    // 3. Rimas Raras por escassez fonológica de terminação
    if is_terminacao_rara(&t1.base, &t2.base) {
        return (
            QualidadeRima::Rara,
            JustificativaAuditoria::HipoteseProvavel(
                "Rima rara por escassez fonológica da terminação no idioma".to_string(),
            ),
        );
    }

    // 4. Comparação de Máscaras Disjuntas (100% de Certeza de Rima Rica)
    // Se a interseção de classes possíveis for vazia, é matematicamente impossível que pertençam à mesma classe.
    if (t1.pos & t2.pos).is_empty() {
        return (
            QualidadeRima::Rica,
            JustificativaAuditoria::ClassesDistintas(t1.pos, t2.pos),
        );
    }

    // 5. Se ambos os termos estiverem 100% resolvidos (100% de Certeza de Rima Pobre)
    if t1.pos.is_resolved() && t2.pos.is_resolved() {
        return (
            QualidadeRima::Pobre,
            JustificativaAuditoria::MesmaClasse(t1.pos),
        );
    }

    // 6. Graus Epistêmicos de Certeza Morfológica (Hipóteses Prevalentes)

    // 5a. Particípios Passados / Adjetivos Verbais Equivalentes
    if is_participio(&t1.base)
        && is_participio(&t2.base)
        && (t1.pos.contains(PosMask::VERBO) || t1.pos.contains(PosMask::ADJETIVO))
        && (t2.pos.contains(PosMask::VERBO) || t2.pos.contains(PosMask::ADJETIVO))
    {
        return (
            QualidadeRima::ProvavelPobre {
                justificativa: "Par formado por particípios ou adjetivos verbais equivalentes"
                    .to_string(),
            },
            JustificativaAuditoria::HipoteseProvavel(
                "Convergência participial / adjetivo verbal".to_string(),
            ),
        );
    }

    // 5b. Sufixos Adjetivais Coincidentes (-oso, -ente, -ante, -vel)
    if is_sufixo_adjetival(&t1.base)
        && is_sufixo_adjetival(&t2.base)
        && t1.pos.contains(PosMask::ADJETIVO)
        && t2.pos.contains(PosMask::ADJETIVO)
    {
        return (
            QualidadeRima::ProvavelPobre {
                justificativa: "Par formado por adjetivos de mesma derivação sufixal".to_string(),
            },
            JustificativaAuditoria::HipoteseProvavel(
                "Convergência morfológica por sufixo adjetival".to_string(),
            ),
        );
    }

    // 5c. Sufixos Substantivais Coincidentes (-ção, -dade, -mento, -ura, -eza)
    if is_sufixo_nominal(&t1.base)
        && is_sufixo_nominal(&t2.base)
        && t1.pos.contains(PosMask::SUBSTANTIVO)
        && t2.pos.contains(PosMask::SUBSTANTIVO)
    {
        return (
            QualidadeRima::ProvavelPobre {
                justificativa: "Par formado por substantivos de mesma derivação sufixal"
                    .to_string(),
            },
            JustificativaAuditoria::HipoteseProvavel(
                "Convergência morfológica por sufixo nominal".to_string(),
            ),
        );
    }

    // 5d. Verbo categórico (100% certo) x Substantivo literário preponderante
    if t1.pos == PosMask::VERBO && is_termo_predominantemente_nominal(&t2.base) {
        return (
            QualidadeRima::ProvavelRica {
                justificativa: format!(
                    "'{}' é Verbo garantido; '{}' é predominantemente nominal na literatura",
                    t1.texto, t2.texto
                ),
            },
            JustificativaAuditoria::HipoteseProvavel(
                "Verbo categórico x Provável Substantivo".to_string(),
            ),
        );
    }
    if t2.pos == PosMask::VERBO && is_termo_predominantemente_nominal(&t1.base) {
        return (
            QualidadeRima::ProvavelRica {
                justificativa: format!(
                    "'{}' é Verbo garantido; '{}' é predominantemente nominal na literatura",
                    t2.texto, t1.texto
                ),
            },
            JustificativaAuditoria::HipoteseProvavel(
                "Verbo categórico x Provável Substantivo".to_string(),
            ),
        );
    }

    // 5e. Substantivo categórico x Termo predominantemente nominal
    if t1.pos == PosMask::SUBSTANTIVO && is_termo_predominantemente_nominal(&t2.base) {
        return (
            QualidadeRima::ProvavelPobre {
                justificativa: format!(
                    "'{}' e '{}' convergem para a esfera substantiva",
                    t1.texto, t2.texto
                ),
            },
            JustificativaAuditoria::HipoteseProvavel(
                "Substantivo categórico x Substantivo literário".to_string(),
            ),
        );
    }
    if t2.pos == PosMask::SUBSTANTIVO && is_termo_predominantemente_nominal(&t1.base) {
        return (
            QualidadeRima::ProvavelPobre {
                justificativa: format!(
                    "'{}' e '{}' convergem para a esfera substantiva",
                    t1.texto, t2.texto
                ),
            },
            JustificativaAuditoria::HipoteseProvavel(
                "Substantivo categórico x Substantivo literário".to_string(),
            ),
        );
    }

    // 5f. Ambos os termos são predominantemente nominais
    if is_termo_predominantemente_nominal(&t1.base) && is_termo_predominantemente_nominal(&t2.base)
    {
        return (
            QualidadeRima::ProvavelPobre {
                justificativa: format!(
                    "'{}' e '{}' convergem para a esfera substantiva",
                    t1.texto, t2.texto
                ),
            },
            JustificativaAuditoria::HipoteseProvavel("Convergência nominal mútua".to_string()),
        );
    }

    // 6. Indeterminação Estrita
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
    (
        QualidadeRima::Indeterminada {
            motivo: IndeterminadoMotivo::AmbiguidadeIrresoluvel {
                palavra: t2.texto.clone(),
                mascara: t2.pos,
            },
        },
        JustificativaAuditoria::AmbiguidadeSintatica(t2.texto.clone()),
    )
}

fn criar_par_avaliado(
    versos: (usize, usize),
    t1: &TokenRimante,
    t2: &TokenRimante,
) -> ParRimaAvaliado {
    let (classificacao, auditoria) = avaliar_par_rima(t1, t2);
    let opt_cliche = verificar_cliche(&t1.texto, &t2.texto);
    ParRimaAvaliado {
        versos,
        tokens: (t1.texto.clone(), t2.texto.clone()),
        classificacao,
        auditoria,
        eh_cliche: opt_cliche.is_some(),
        motivo_cliche: opt_cliche.map(|s| s.to_string()),
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
                    pares_avaliados.push(criar_par_avaliado(
                        (offset_verso, offset_verso + 3),
                        &tokens[0],
                        &tokens[3],
                    ));
                    pares_avaliados.push(criar_par_avaliado(
                        (offset_verso + 1, offset_verso + 2),
                        &tokens[1],
                        &tokens[2],
                    ));
                }
                "ABAB" => {
                    // Cruzada: (1, 3) e (2, 4)
                    pares_avaliados.push(criar_par_avaliado(
                        (offset_verso, offset_verso + 2),
                        &tokens[0],
                        &tokens[2],
                    ));
                    pares_avaliados.push(criar_par_avaliado(
                        (offset_verso + 1, offset_verso + 3),
                        &tokens[1],
                        &tokens[3],
                    ));
                }
                "AABB" => {
                    // Emparelhada: (1, 2) e (3, 4)
                    pares_avaliados.push(criar_par_avaliado(
                        (offset_verso, offset_verso + 1),
                        &tokens[0],
                        &tokens[1],
                    ));
                    pares_avaliados.push(criar_par_avaliado(
                        (offset_verso + 2, offset_verso + 3),
                        &tokens[2],
                        &tokens[3],
                    ));
                }
                _ => {
                    // Agrupamento genérico por feixe de caracteres
                    pares_avaliados.extend(avaliar_grupos_genericos(
                        &tokens,
                        &chars_esquema,
                        offset_verso,
                    ));
                }
            }
        } else {
            // Grupos e estrofes genéricas (tercetos, sextilhas, oitavas, etc.)
            pares_avaliados.extend(avaliar_grupos_genericos(
                &tokens,
                &chars_esquema,
                offset_verso,
            ));
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
    let mut map_grupos: std::collections::BTreeMap<char, Vec<usize>> =
        std::collections::BTreeMap::new();
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
            resultado.push(criar_par_avaliado(
                (offset_verso + i1, offset_verso + i2),
                &tokens[i1],
                &tokens[i2],
            ));
        }
    }

    resultado
}
