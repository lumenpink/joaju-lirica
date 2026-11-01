# Spec: Ticket #13 - Alinhar ao Perfil Lilya e ci.yml

## 1. Contexto e Motivação
Alinhamento do repositório `joaju-lirica` às normas GOST v2.0 com perfil `lilya` (`https://lilya.2lp.in/profile.md`), adoção de `.forgejo/workflows/ci.yml`, atualização de testes de cascata GitOps e higienização vocabular.

## 2. Requisitos Técnicos
- Renomeação de `.forgejo/workflows/ci.yaml` para `.forgejo/workflows/ci.yml`.
- Atualização de `tests/audit_gitops_cascade.sh` e `.forgejo/workflows/ci.yml` para direcionar dispatches a `ci.yml`.
- Higienização de terminologia obsoleta nos scripts de teste.
- Conformidade total com `cccp check --strict`.

## 3. Critérios de Aceitação
- Testes unitários e de integração lírica 100% aprovados.
- 100% de conformidade com `cccp check --strict`.


## 4. Conclusão da Fase 9
- Workflow renomeado para `.forgejo/workflows/ci.yml`.
- Auditoria de cascata GitOps 100% verde.
- Perfil GOST alinhado para Lilya e vocabulário higienizado.
