# Especificação Técnica: Expansão do Gabarito Canônico com Formas Regionais, Versos-Sentinela e Mutações de Contraprova

**Ticket:** #22  
**Branch:** `feat/22-expansao-do-gabarito-canonico-regional-e-fonotatico`  
**Escopo:** `joaju-lirica` (`tests/corpus/gabarito_canonico.json`, `tests/gabarito_canonico_tests.rs`, `tests/mutacoes_corpus_tests.rs`)  
**Status:** Concluído e Validado (Fase 9: Close)

---

## 1. Contexto e Motivação

O **Gabarito Canônico (Ground Truth)** do `joaju-lirica` operava com 55 obras-primas e 102 mutações declarativas. Embora cobrisse com excelência o cânone clássico (Camões, Bocage, Bilac, Gonçalves Dias, Vinícius), revelou duas fragilidades metodológicas durante o ciclo do Ticket #21:
1. **Zonas Cegas em Casos de Fronteira Fonotática:** Colisões vocálicas de 3 núcleos contíguos (crases triplas homófonas intermediadas por clíticos átonos, como em *"O horizonte da serra a alvorecer"*) não possuíam versos-sentinela dedicados no gabarito, permitindo que a quebra passasse despercebida.
2. **Sub-representação das Formas Populares Brasileiras e do Cordel:** Embora o cliente WebAssembly (`lirica`) já reconhecesse formas como Pajada Gaúcha, Galope à Beira-Mar e Folhetos de Cordel in-octavo, essas matrizes históricas ainda não estavam incorporadas no Ground Truth auditado do motor principal.

---

## 2. Árvore de Decisão e Racional Técnico (Para Natasha)

### Decisão 1: Expansão Qualitativa Orientada a Fenômenos vs. Expansão Quantitativa Bruta
- **Opção A (Descartada):** Inserir dezenas de sonetos parnasianos comuns sem critérios fonotáticos.
  - *Problema:* Volume inflado com versos métrica e fonotaticamente redundantes (sinalefas binárias triviais) que consomem tempo de CI sem aumentar a assertividade nas zonas de estresse.
- **Opção B (Escolhida):** Expansão estruturada em 3 frentes:
  1. *Formas Populares e Regionais Brasileiras:* Pavão Misterioso (cordel in-octavo em sextilhas de Redondilha Maior), Pajada Gaúcha (décima espineliana campeira) e Galope à Beira-Mar (martelo decassilábico nordestino).
  2. *Cânone de Alta Tensão Prosódica:* Augusto dos Anjos (vocabulário tenso, proparoxítonas e acidentes métricos) e Cruz e Sousa (alexandrinos simétricos e encontros vocálicos densos).
  3. *Banco de Versos-Sentinela Fonotáticos:* Casos cirúrgicos de crase tripla homófona, cadeias de clíticos e hiatos obrigatórios por tônica.

### Decisão 2: Contraprovas Ativas com Mutações Declarativas
- Todo novo poema incluído no gabarito deve possuir um conjunto de **mutações declarativas** (`tipo: "rima"`, `tipo: "metrica"`, `tipo: "estrofe"`, `tipo: "refrao"`), testando se o motor diagnostica com precisão:
  - Invasão de versos hiper/hipométricos.
  - Corrupção de rimas obrigatórias (ex.: rimas pares `xAxAxA` do cordel e ponte `c` da décima espineliana).
  - Supressão ou quebra do refrão canônico no Galope à Beira-Mar.
  - Deslocamento de acentos métricos característicos.

---

## 3. Matriz de Obras e Mutações Planejadas

| ID da Peça | Título / Autor | Forma / Metro | Variantes / Mutações Declarativas |
|:---|:---|:---|:---|
| `cordel_pavao_misterioso` | *Romance do Pavão Misterioso* / José Camelo de Melo Rezende | Sextilhas (Redondilha Maior, `xAxAxA`) | Quebra de rima par; verso octossílabo hipermetrico; quebra de topologia estrófica |
| `pajada_gaucha_pampa` | *Pajada do Gaúcho Missioneiro* / Tradição Sulina | Décima Espineliana (`abbaaccddc`, Redondilha Maior) | Quebra da rima-ponte 5; verso decassílabo infiltrado |
| `galope_beira_mar_calunga` | *Galope à Beira-Mar* / Tradição da Cantoria | Estrofe de 10 versos em Martelo Decassílabo | Ausência do refrão canônico; quebra do ritmo de 3ª (anapéstico) |
| `augusto_anjos_vencido` | *Psicologia de um Vencido* / Augusto dos Anjos | Soneto Italiano (Decassílabo Heroico) | Invasão de rima clichê pobre; quebra métrica em proparoxítona |
| `cruz_sousa_carceres` | *Cárceres das Almas* / Cruz e Sousa | Soneto Italiano (Decassílabo Heroico) | Desvio de cesura; quebra de sinalefa |
| `sentinelas_fonotaticos_fronteira` | *Matriz de Estresse Fonotático* / Casos Canônicos | Métrica Variada (Decassílabos e Redondilhas) | Mutações de barra `/` forçando hiato e pontuação travando crase |

---

## 4. Matriz de Validação
- `tests/gabarito_canonico_tests.rs`: Todos os novos poemas devem ser verificados verso a verso (100% de precisão silábica esperada).
- `tests/mutacoes_corpus_tests.rs`: 100% das novas mutações declarativas devem disparar os diagnósticos e rebaixamentos de nota esperados.
- Não-regressão: As 55 obras e 102 mutações anteriores devem permanecer 100% aprovadas.
