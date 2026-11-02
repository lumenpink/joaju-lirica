# Spec 14: Remover Disparos ao Codeberg e Notificar Kombinat no ci.yml

## 1. Contexto e Motivação
A esteira de CI (`.forgejo/workflows/ci.yml`) ainda possui resquícios de disparos manuais de workflow via API do Codeberg para o repositório `joaju`. Com a centralização das notificações no daemon `kombinat` (`http://127.0.0.1:4040/api/cascade/dispatch`), tais chamadas diretas ao Codeberg devem ser expurgadas.

Adicionalmente, a crate deve expor a constante canônica `pub const VERSION: &str` em `src/lib.rs`.

## 2. Requisitos Técnicos
- Remover o step de dispatch para a API do Codeberg em `.forgejo/workflows/ci.yml`.
- Adicionar o step de notificação ao daemon Kombinat no `.forgejo/workflows/ci.yml`.
- Atualizar `tests/audit_gitops_cascade.sh` para refletir o novo modelo de cascata.
- Exportar `pub const VERSION: &str = env!("CARGO_PKG_VERSION");` em `src/lib.rs`.
- Validar via teste de integração `tests/version_spec.rs`.

## 3. Plano de Testes (DD-TDD)
- Fase 4: Teste comprovadamente falho verificando a existência de `VERSION` e a notificação ao Kombinat.
- Fase 5: Implementação funcional.
- Fase 8: Testes verdes, clippy limpo, `cccp check --strict`.

## 4. Status de Conclusão
- [x] Step de disparo ao Codeberg removido de `.forgejo/workflows/ci.yml`.
- [x] Step de notificação do Kombinat configurado em `.forgejo/workflows/ci.yml`.
- [x] Script `tests/audit_gitops_cascade.sh` auditado e aprovado.
- [x] `pub const VERSION: &str` verificado em `src/lib.rs`.
- [x] Suíte de testes 100% verde (`cargo test`).
- [x] 100% de conformidade Gosplan (`cccp check --strict`).
