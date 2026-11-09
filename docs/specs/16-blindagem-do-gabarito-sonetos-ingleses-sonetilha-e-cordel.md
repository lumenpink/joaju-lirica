# Spec 16: Blindagem do Gabarito, Sonetos Ingleses, Sonetilha e Cordel

## 1. Contexto e Motivação
A avaliação do soneto *Profana* no motor poético atingiu 94% em vez de 100% devido a um erro tipográfico no padrão canônico de rimas do Soneto Inglês (`"ABABCDCDXEXFGG"` em vez de `"ABABCDCDEFEFGG"`), além de ausência de alertas diagnósticos para desvios em `avaliar_soneto_ingles`.
Adicionalmente, o Gabarito Canônico carecia de poemas shakespearianos e spenserianos, bem como de contra-provas imperfeitas (testes negativos) para validar o rebaixamento de notas e diagnósticos.
Por fim, formas tradicionais como a **Sonetilha** (soneto em redondilhas) e modalidades de **Cordel** (Septilha) não possuíam avaliadores nem representação canônica.

## 2. Requisitos Técnicos
1. **Correção do Soneto Inglês (Shakespeariano e Spenseriano):**
   - Corrigir a string de comparação de rimas para `"ABABCDCDEFEFGG"`.
   - Adicionar suporte canônico à variação spenseriana com entrelaçamento: `"ABABBCBCCDCDEE"`.
   - Inserir observações detalhadas em `obs` quando houver desvio no número de versos, na divisão estrófica (`4+4+4+2`) ou no esquema rímico.
2. **Nova Forma Poética — Sonetilha (Sonetino):**
   - Criar `avaliar_sonetilha`: 14 versos distribuídos em quartetos e tercetos (ou quartetos e dístico) compostos em Redondilha Maior (7 sílabas métricas) ou Menor (5 sílabas métricas).
3. **Nova Forma Poética — Septilha de Cordel:**
   - Criar `avaliar_septilha`: estrofe de 7 versos em Redondilha Maior (7 sílabas métricas) com rimas tradicionais nos versos pares e fecho emparelhado (`XAXABBX` ou `ABABCCA`).
4. **Enriquecimento Diagnóstico Global:**
   - Garantir que todos os avaliadores de formas poéticas emitam alertas precisos em `observacoes` sempre que a nota for inferior a 100%.
5. **Gabarito Canônico Estrito e Matriz de Contra-provas:**
   - Adicionar ao `tests/corpus/gabarito_canonico.json` o soneto *Profana* (Shakespeariano 100%), um soneto Spenseriano (100%), Sonetilha (100%), Septilha de Cordel (100%), além de contra-provas imperfeitas com desvios conhecidos para validação cruzada.
   - Atualizar `tests/gabarito_canonico_tests.rs` para asserir `confianca_percentual == 100` e ausência de observações nos casos perfeitos, bem como redução de nota e presença de observações nos imperfeitos.

## 3. Plano de Testes (DD-TDD)
- **Fase 4 (Red Test):** Testes em `tests/poetic_forms_tests.rs` e `tests/gabarito_canonico_tests.rs` esperando 100% de pontuação para o Soneto Shakespeariano *Profana*, soneto spenseriano, sonetilha, septilha e alertas nas contra-provas.
- **Fase 5 (Green Implement):** Correções em `src/poetic_form.rs` e dados em `tests/corpus/gabarito_canonico.json`.
- **Fase 6 (Refactor):** Otimizações nas rotinas de pontuação e tolerância zero a warnings.
- **Fase 7 (Mutation):** Testes de mutação com `cccp mutate` no diff.
- **Fase 8 (Quality Gate):** Checagens estritas (`cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test --test poetic_forms_tests`).
- **Fase 9 (Close):** Laudo e fechamento da issue #16.

## 4. Status do Ciclo DD-TDD
- **Status:** Concluído com sucesso (Fases 1 a 9). Fechamento homologado e integrado.
- **Laudo:** Aprovado em todos os testes do gabarito canônico verso a verso e formas poéticas.
