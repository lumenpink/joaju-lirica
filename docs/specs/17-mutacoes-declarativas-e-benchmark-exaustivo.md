# Especificação Técnica: Mutações Declarativas e Benchmark Exaustivo

**Ticket:** #17  
**Branch:** `feat/17-mutacoes-declarativas-e-benchmark-exaustivo`  
**Escopo:** Motor de Mutações, Schema Serde, Corpus Canônico e Suíte de Testes Exaustivos

---

## 1. Visão Geral

Este documento define o contrato formal para transformar o corpus de escansão poética (`gabarito_canonico.json`) em uma suíte canônica de testes de estresse por mutações declarativas.

A raiz do gabarito é composta estritamente por obras 100% aderentes (ground truth). Cada entrada pode conter uma coleção de `mutacoes`, com deltas cirúrgicos de linha, comutações de flags fonéticas e expectativas diagnósticas.

---

## 2. Modelos de Dados (Serde)

### 2.1. `MutacaoPoema`
- `id: String` (identificador único da mutação, ex.: `profana_quebra_distico`)
- `tipo: String` (`"metrica"`, `"rima"`, `"estrutura"`, `"configuracao"`, `"combinada"`)
- `descricao: String` (explicação pedagógica da mutação)
- `operacoes: Option<Vec<OperacaoLinha>>` (operações ordenadas de linha/texto)
- `flags: Option<FlagsMutacao>` (sobreposição de flags de escansão)
- `expectativa: ExpectativaMutacao` (critérios esperados de assertividade)

### 2.2. `OperacaoLinha`
- `op: String` (`"substituir"`, `"remover"`, `"inserir"`, `"remover_espacos_estroficos"`)
- `linha: Option<usize>` (linha 1-indexada relevante para substituir, remover ou inserir)
- `texto: Option<String>` (texto substituto ou a ser inserido)

### 2.3. `FlagsMutacao`
- `contagem_castilho: Option<bool>` (padrão do motor é `true`; mutação testa `false`)
- `elidir_pontuacao: Option<bool>` (padrão é `false`; mutação testa `true`)
- `forcar_rima_estrita: Option<bool>`

### 2.4. `ExpectativaMutacao`
- `forma_esperada: Option<String>` (ex.: `"Soneto Inglês (Shakespeariano)"`)
- `nota_min: Option<u32>`
- `nota_max: Option<u32>`
- `observacoes_obrigatorias: Option<Vec<String>>` (substrings que DEVEM constar nas observações do diagnóstico)

---

## 3. Função do Aplicador de Deltas

```rust
pub fn aplicar_mutacoes_texto(
    texto_original: &str,
    operacoes: &[OperacaoLinha],
) -> String;
```

A função deve:
1. Dividir o texto em linhas preservando quebras.
2. Aplicar `remover_espacos_estroficos` caso solicitado (eliminando linhas vazias).
3. Aplicar substituições, inserções e remoções respeitando os números de linha informados.
4. Reconstruir a string do poema para envio ao analisador de escansão.

---

## 4. Critérios de Aceite e Validação

1. Todas as contra-provas avulsas anteriores (`profana_contraprova_distico`) são absorvidas pelo campo `mutacoes` do respectivo poema matriz.
2. Todos os poemas raiz de `gabarito_canonico.json` atingem rigorosamente 100% de conformidade.
3. A suíte automatizada `mutacoes_corpus_tests.rs` executa cada mutação de cada poema e valida conformidade estrita com a `ExpectativaMutacao`.
4. Quality gate com zero warnings e 100% Gosplan (`cccp check --strict`).
