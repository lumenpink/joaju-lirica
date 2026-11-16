# Especificação Técnica: Normalização de Ponteiro Git no CI para Eventos de Tag

**Ticket:** #23  
**Branch:** `fix/23-normalizar-branch-antes-do-cccp-check-em-eventos-de-tag`  
**Escopo:** `joaju-lirica` (`.forgejo/workflows/ci.yml`, `tests/version_spec.rs`)  
**Status:** Concluído e Validado (Fase 9: Close)

---

## 1. Contexto e Motivação

Durante a release v1.0.0, o push da tag disparou a Run #2181 no Forgejo Actions. Todos os testes unitários, formatação e linters (`cargo fmt`, `cargo clippy`, 70 testes) passaram. No entanto, o step `CCCP Quality Audit` falhou com:
`[FAIL] GOST-05-BRANCH-TICKET : Nome da branch 'HEAD' fora do padrão GOST`

Isso ocorreu porque a action `actions/checkout@v4` realiza o checkout de tags em modo *detached HEAD*, fazendo com que o Git retorne `'HEAD'` como nome de branch.

---

## 2. Árvore de Decisão e Racional Técnico (Para Natasha)

### Decisão 1: Anexar Branch no Runner vs. Silenciar via `|| true`
- **Opção A (Descartada):** Modificar a chamada para `cccp check --strict || true`.
  - *Problema:* Mascara qualquer erro futuro. Se Natasha ou outra pessoa cometer uma quebra real das normas GOST (ausência de LICENSE, binário compilado rastreado no git, SemVer malformado no VERSION), o CI continuará passando silenciosamente, acumulando dívida técnica invisível.
- **Opção B (Escolhida):** Normalizar a branch antes da auditoria caso o Git esteja em estado desanexado (`HEAD`).
  - *Implementação:* Se `git rev-parse --abbrev-ref HEAD` retornar `'HEAD'`, reanexar via `git checkout -B "${GITHUB_BASE_REF:-main}"`.
  - *Vantagem:* Mantém o rigor compulsório do `--strict`. Se houver qualquer violação legítima da norma GOST, o portão de qualidade barra o build imediatamente.

---

## 3. Matriz de Validação
- `tests/version_spec.rs`: Adicionar asserção garantindo que o workflow `.forgejo/workflows/ci.yml` contém a proteção de reanexação em detached HEAD antes de invocar `cccp check --strict`.
- Teste local de simulação: Em estado detached HEAD (`git checkout --detach HEAD`), verificar que a rotina normaliza para a branch principal antes da verificação.
