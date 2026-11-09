# Especificação Técnica: DAFSA Ampla de Vocabulário Geral do Português

**Ticket:** #19  
**Branch:** `feat/19-dafsa-ampla-de-lexico-geral-embutido`  
**Status:** Implementado / Concluído  
**Escopo:** `joaju-lirica` (DAFSA / FST em bytes comprimidos, base léxica de $\ge 100.000$ termos, lookup estático e validação no gabarito canônico)

---

## 1. Visão Geral e Objetivos

Este ciclo implementa uma base léxica ampla do português no subsistema morfológico do `joaju-lirica`:
1. **Transdutor FST Compacto (`fst`):** Mapeamento direto `&str` $\to$ `u64` (armazenando `PosMask: u16`) em blob binário estático `data/lexicon_pt.bin`.
2. **Zero Alocações na Heap em Runtime:** O blob binário é embutido diretamente no executável/WASM via `include_bytes!`, com inicialização via `fst::Map::new(...)` em tempo $O(1)$.
3. **Limite de Memória:** O arquivo binário comprimido não deve exceder $1.2\text{ MB}$.
4. **Resolução no Gabarito Canônico:** Redução do percentual de rimas indeterminadas de $79,4\%$ para menos de $15\%$.

---

## 2. Estrutura do DAFSA e Dependências

- Dependência adicionada em `Cargo.toml`: `fst = { version = "0.4", default-features = false }`.
- Estrutura de armazenamento: arquivo `src/morphology/data/lexicon_pt.bin` gerado por script/builder determinístico a partir de dados morfológicos lematizados do português (DELAS/DELAF-PB e dicionários livres).
- Interface de consulta em `src/morphology/dafsa.rs`:
  - `consultar_lexico(palavra: &str) -> Option<PosMask>`:
    - Consulta no `fst::Map` estático em tempo $O(k)$ sobre o tamanho da palavra.
    - Se presente, retorna `Some(PosMask(val as u16))`.
    - Se ausente, recorre à tabela canônica de fallback e ao autômato de sufixos produtivos.

---

## 3. Critérios de Aceite

1. `fst::Map` instanciado e consultado com sucesso em tempo de execução sem alocações dinâmicas na heap.
2. Tamanho de `lexicon_pt.bin` $\le 1.2\text{ MB}$.
3. Todas as 382 palavras rimantes do gabarito canônico agora resolvidas com suas classes gramaticais atestadas.
4. Suíte de testes passando com 100% de sucesso.
5. Conformidade Gosplan estrita.
