# Especificação Técnica: Módulo Determinístico de Classificação de Rimas

**Ticket:** #18  
**Branch:** `feat/18-modulo-deterministico-de-classificacao-de-rimas`  
**Escopo:** `joaju-lirica` (Morfologia, DAFSA, Topologia de Pares e Rastreabilidade)  

---

## 1. Visão Geral e Princípios de Projeto

O objetivo deste ciclo é dotar o motor *joaju-lirica* de um subsistema determinístico de classificação qualitativa de rimas, sem heurísticas ocultas ou chutes probabilísticos, com consumo total de memória $< 3\text{ MB}$.

### 1.1 Princípios Fundamentais
1. **Determinismo Estrito:** A mesma entrada textual sob o mesmo esquema rímico produz idêntica saída estruturada em qualquer ambiente (CLI nativo ou WebAssembly).
2. **Auditabilidade Granular:** Toda classificação registra a justificativa textual explícita da resolução de cada token.
3. **Falha Honesta:** Casos com vocábulos não catalogados ou ambiguidades sintáticas irresolúveis retornam `QualidadeRima::Indeterminada` acompanhada da razão objetiva.
4. **Isolamento de Memória:** Estrutura binária de léxico estático embutida via `include_bytes!` com $< 1.2\text{ MB}$.

---

## 2. Tipagem e Álgebra de Estados

### 2.1 Máscaras de Bits Morfológicas (`PosMask`)

Representado por `u16` com os seguintes bits:
- `UNKNOWN`: `0`
- `SUBSTANTIVO`: `1 << 0` (0x0001)
- `VERBO`: `1 << 1` (0x0002)
- `ADJETIVO`: `1 << 2` (0x0004)
- `ADVERBIO`: `1 << 3` (0x0008)
- `PRONOME`: `1 << 4` (0x0010)
- `ARTIGO`: `1 << 5` (0x0020)
- `PREPOSICAO`: `1 << 6` (0x0040)
- `CONJUNCAO`: `1 << 7` (0x0080)
- `INTERJEICAO`: `1 << 8` (0x0100)
- `NUMERAL`: `1 << 9` (0x0200)

Métodos:
- `is_ambiguous()`: `self.bits().count_ones() > 1`
- `is_resolved()`: `self.bits().count_ones() == 1`

### 2.2 Classificação Qualitativa (`QualidadeRima`)
- `Pobre`: Tokens rimantes compartilham a mesma classe gramatical (ex.: Substantivo x Substantivo).
- `Rica`: Tokens com classes gramaticais disjuntas (ex.: Substantivo x Adjetivo).
- `Preciosa`: Rima com assimetria de ênclise/mesóclise (ex.: `vê-la` x `estrela`).
- `Rara`: Casos de vocábulos homógrafos idênticos com o mesmo valor ou combinações anômalas.
- `Indeterminada { motivo: IndeterminadoMotivo }`:
  - `VocabuloNaoCatalogado { palavra: String }`
  - `AmbiguidadeIrresoluvel { palavra: String, mascara: PosMask }`

---

## 3. Topologia de Pares e Escopo Estrófico

1. **Esquemas Canônicos Fechados (Quartetos):**
   - Oposta/Interpolada ($ABBA$): $(V_1, V_4)$ e $(V_2, V_3)$
   - Cruzada/Alternada ($ABAB$): $(V_1, V_3)$ e $(V_2, V_4)$
   - Emparelhada ($AABB$): $(V_1, V_2)$ e $(V_3, V_4)$
2. **Grupos $\ge 3$ Versos (Monorrimas e Encadeamentos):**
   - Cadeia linear adjacente: para $\{V_1, V_2, V_3\}$, avalia-se $(V_1, V_2)$ e $(V_2, V_3)$.

---

## 4. Esteira Morfológica e Desambiguação (Lookback)

1. **Estágio 1: Descolamento de Ênclise:**
   Regex/autômato reconhecendo pronomes clíticos (`-me`, `-te`, `-se`, `-lhe`, `-o`, `-a`, `-lo`, `-la`, etc.) e normalizando a base verbal para o infinitivo.
2. **Estágio 2: Consulta Léxica DAFSA:**
   Tabela binária compilada indexando raízes e flexões essenciais.
3. **Estágio 2b: Autômato de Sufixação Produtiva:**
   Sufixos nominais (`-ção`, `-ura`, `-mento`), adjetivais (`-oso`, `-vel`, `-al`), adverbiais (`-mente`) e verbais (`-ar`, `-er`, `-ir`).
4. **Estágio 3: Retroanálise Contextual (Lookback):**
   - `R-DET`: Artigo / Demonstrativo / Possessivo $\implies$ `SUBSTANTIVO`.
   - `R-PRON`: Pronome pessoal reto imediato $\implies$ `VERBO`.
   - `R-NEG`: Partícula negativa (`não`, `nunca`, `jamais`) $\implies$ `VERBO`.
   - `R-MOD`: Substantivo inequívoco anterior $\implies$ `ADJETIVO`.
   - `R-PREP`: Preposição pura anterior a infinitivo $\implies$ `VERBO`.

---

## 5. Critérios de Aceite

1. Suíte unitária em `tests/` cobrindo todas as regras de retroanálise ($R\text{-DET}$ a $R\text{-PREP}$).
2. Testes de ênclise reconhecendo rimas preciosas clássicas.
3. Validação dos pares no gabarito canônico auditado.
4. Tamanho do léxico binário $\le 1.2\text{ MB}$.
5. Conformidade Gosplan 100% via `cccp check --strict`.

---

## 6. Laudo de Conclusão (Fase 9)

- **Status:** Implementado e Validado
- **Módulos:** `pos_mask`, `clitics`, `dafsa`, `suffixes`, `lookback`, `classifier`
- **Testes Unitários e Integração:** 47/47 testes aprovados
- **Quality Gate:** `cargo fmt --check` e `cargo clippy --all-targets -- -D warnings` aprovados com zero avisos.

