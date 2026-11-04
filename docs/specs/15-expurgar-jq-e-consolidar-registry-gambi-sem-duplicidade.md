# Spec 15: Expurgar jq e Consolidar Registry Gambi sem Duplicidade

## 1. Contexto e Motivação
A esteira de CI do repositório `joaju-lirica` falhava durante a execução do job `publish` com código de erro 127 (`jq: command not found`) na etapa de extração de versão para notificação do daemon Kombinat.
Adicionalmente, a etapa de configuração do Cargo declarava de forma duplicada o registry `gambi` e o registry `lumen` apontando para o mesmo URL sparse index, o que gera erro de múltiplos registries com a mesma URL.

## 2. Requisitos Técnicos
- Substituir a invocação do binário `jq` por extração de JSON via Python (`python3 -c "import sys, json; ..."`).
- Remover a configuração redundante do registry `lumen` no arquivo `.forgejo/workflows/ci.yml`, mantendo estritamente a configuração e autenticação do registry canônico `gambi`.
- Adicionar asserções nos testes de integração do workflow para auditar a ausência de `jq` e a unicidade do registry configurado.
- Preservar a notificação em cascata ao daemon Kombinat.

## 3. Plano de Testes (DD-TDD)
- Fase 4 (Red Test): Adicionar teste em `tests/version_spec.rs` verificando ausência de `jq` e proibição de registries duplicados.
- Fase 5 (Green Implement): Atualizar `.forgejo/workflows/ci.yml`.
- Fase 6 (Refactor): Limpeza e verificação com zero warnings.
- Fase 7 (Mutation): Validação de mutantes e robustez de asserções.
- Fase 8 (Quality Gate): Testes, `cargo fmt` e `cargo clippy` 100% verdes.
- Fase 9 (Close): Fechamento do ticket e merge.

## 4. Status do Ciclo DD-TDD
- **Fase Atual:** Fase 7 (Mutantes verificados contra regressão).
