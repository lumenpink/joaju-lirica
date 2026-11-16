# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Fixed
- Normalização de branch Git em modo detached HEAD no runner de CI antes de executar o CCCP Quality Audit, eliminando falso positivo de GOST-05-BRANCH-TICKET em eventos de tags (closes #23).

## [1.0.0] - 2026-11-16

### Features
- expansao do gabarito canonico e mutacoes (closes #22 - ph9)
- formatacao e quality gate aprovados (ref #22 - ph8)
- testes de mutacao para expansao do gabarito (ref #22 - ph7)
- expansao do gabarito canonico com 6 pecas (ref #22 - ph5)
- red test para expansao do gabarito canonico (ref #22 - ph4)
- especificar expansao do gabarito e mutacoes (ref #22 - ph3)
- sinalefa terciaria com clitico atono ponte (closes #21 - ph9)
- formatacao e quality gate aprovados (ref #21 - ph8)
- testes de mutacao para fusao terciaria (ref #21 - ph7)
- sinalefa terciaria com clitico atono ponte (ref #21 - ph5)
- red test para sinalefa terciaria com clitico (ref #21 - ph4)
- especificar sinalefa terciaria com clitico (ref #21 - ph3)
- graus epistemicos e detector cliches (closes #20 - ph9)
- **morphology:** graus epistemicos e cliches (ref #20 - ph5)
- feat(morphology): dafsa ampla de lexico geral (closes #19 - ph9)
- **morphology:** integrar fst e lexico amplo (ref #19 - ph5)
- classificador deterministico de rimas (closes #18 - ph9)
- **morphology:** classificador de rimas deterministico (ref #18 - ph5)
- mutacoes declarativas e benchmark exaustivo (closes #17 - ph9)
- **corpus:** mutacoes declarativas no gabarito (ref #17 - ph5)

### Bug Fixes
- concluir blindagem de sonetos e cordel (closes #16 - ph9)
- implement sonetos, sonetilha and cordel (ref #16 - ph5)
- concluir expurgo de jq no ci (closes #15 - ph9)
- concluir validacoes de qualidade e testes (ref #15 - ph8)
- validar assercoes de teste contra mutantes (ref #15 - ph7)
- validar formatacao e linter no joaju-lirica (ref #15 - ph6)
- expurgar jq e consolidar registry lumen (ref #15 - ph5)
- adicionar teste falho para jq no ci (ref #15 - ph4)
- especificar expurgo de jq no ci (ref #15 - ph3)
- **ci:** configurar token Bearer para cargo publish (ref #12)
- **cargo:** sincronizar versao 0.5.1 no Cargo.toml (closes #11)
- **ci:** atualizar endpoint para gambi e auditoria (closes #9)
- **ci:** remover campo permissions do workflow (ref #10)

### Refactoring
- **morphology:** formatacao e polimento de graus (ref #20 - ph6)
- **morphology:** migrar extensao de lexico para fst (ref #19 - ph6)
- **poetic_form:** polimento de formatacao e testes (ref #17 - ph6)
- polish poetic forms and formatting (ref #16 - ph6)
- **ci:** padronizar pipeline com Bearer auth (ref #12)

### Documentation
- **morphology:** especificar graus e cliches de rima (ref #20 - ph3)
- **morphology:** especificar dafsa ampla embutida (ref #19 - ph3)
- **morphology:** especificar classificacao de rimas (ref #18 - ph3)
- **specs:** especificar mutacoes declarativas do corpus (ref #17 - ph3)
- specify sonetos ingleses, sonetilha e cordel (ref #16 - ph3)
- **spec:** concluir fase 9 de autenticacao Bearer (closes #12)
- **ci:** documentar autenticacao Bearer para cargo (ref #12)
- **spec:** documentar remocao de permissions no ci (#10)

### Testing
- **morphology:** validacao por mutacao e gabarito (ref #20 - ph7)
- **morphology:** red test para graus e cliches de rima (ref #20 - ph4)
- **morphology:** red test de dafsa ampla embutida (ref #19 - ph4)
- **morphology:** par canonico estrela e ve-la (ref #18 - ph8)
- **morphology:** red test de classificacao de rimas (ref #18 - ph4)
- **corpus:** red test de mutacoes declarativas (ref #17 - ph4)
- add red tests for sonetos and cordel (ref #16 - ph4)
- **ci:** exigir autenticacao Bearer no publish (ref #12)
- **ci:** adicionar teste red para remocao de permissions (ref #10)

### Maintenance
- **release:** quality gate de graus e cliches (ref #20 - ph8)
- **release:** sincronizar lockfile e quality gate (ref #19 - ph8)
- **morphology:** formatar e corrigir clippy (ref #18 - ph8)
- sync Cargo.lock dev version (ref #17 - ph8)
- sync Cargo.lock dev version (ref #16 - ph8)
- concluir expurgo codeberg e notificacao ci (closes #14 - ph9)
- expurgar codeberg e notificar kombinat no ci (ref #14 - ph5)
- adicionar teste falho para expurgo codeberg (ref #14 - ph4)
- especificar expurgo codeberg e notificacao ci (ref #14 - ph3)
- concluir padronizacao lilya e ci.yml (closes #13 - ph9)
- ci(workflow): padronizar dispatch ci.yml (ref #13 - ph5)
- test(ci): atualizar auditoria para ci.yml (ref #13 - ph4)
- docs(spec): definir spec lilya e ci.yml (ref #13 - ph3)
- **version:** sincronizar VERSION para 0.5.1-dev.16+20261029.4f23012
- **version:** sincronizar VERSION para 0.5.1-dev.10+20261025.ffb44d9
- re-trigger pipeline apos ajuste no runner
- **version:** sincronizar VERSION para 0.5.1-dev.7+20261025.03b9d4c
- **version:** sincronizar VERSION para 0.5.1-dev.5+20261024.d00464b
- **changelog:** registrar remocao de permissions (closes #10)

## [Unreleased]

### Added
- Expansão do Gabarito Canônico para 61 poemas e 573 versos auditados, incorporando formas regionais brasileiras (Cordel in-octavo, Pajada Gaúcha, Galope à Beira-Mar), simbolismo e parnasianismo de alta tensão prosódica, matriz de sentinelas fonotáticos e 106 mutações declarativas negativas (closes #22).
- Suporte a sinalefa terciária por crase homófona através de clítico átono intermediário (ex.: 'serra a alvorecer') com proteção estrita contra absorção de tônicas (closes #21).

### Changed
- Remoção de campo `permissions:` incompatível com Forgejo Actions (closes #10).

## [0.5.0] - 2026-10-21

### Features
- **test:** gabarito canonico verso a verso de 50 poemas (closes #8)

### Maintenance
- **version:** sincronizar VERSION para 0.5.0-dev.2+20261022.3e6877d

## [0.4.0] - 2026-10-21

### Features
- **metrics:** ditongo atono ao/aos e soneto de fidelidade (closes #7)

### Maintenance
- **version:** sincronizar VERSION para 0.4.0-dev.2+20261021.399bde4

## [0.3.0] - 2026-10-21

### Features
- **corpus:** benchmark de 50 poemas e elisão universal (closes #6)

### Maintenance
- **version:** sincronizar Cargo.toml para 0.3.0
- **version:** sincronizar VERSION para 0.3.0-dev.2+20261021.713ff53

## [0.3.0] - 2026-10-21

### Features
- **corpus:** benchmark de 50 poemas e elisão universal (closes #6)

### Maintenance
- **version:** sincronizar Cargo.toml para 0.3.0
- **version:** sincronizar VERSION para 0.3.0-dev.2+20261021.713ff53

## [0.2.0] - 2026-10-21

### Features
- **form:** detecção de formas poéticas e rimas (closes #4)
- detecção de rimas, forçar hiato com barra e apóstrofo tônico
- **ci:** padronizar pipeline gitops e registry lumen (closes #2)
- **engine:** implement core poetic scansion and directives (#1)

### Bug Fixes
- **scansion:** impedir sinalefa sobre vogais e ditongos tônicos finais
- **ci:** instalar rustfmt e clippy no setup toolchain (closes #3)
- **cli:** resolve ambiguous short flag collision for typst (#1)

### Documentation
- **changelog:** record poetic scansion engine features (closes #1)
- **specs:** add specification for poetic scansion engine (#1)

### Testing
- **scansion:** add edge cases, mutation, and renderer tests (#1)
- **scansion:** add failing unit and integration tests (#1)

### Maintenance
- **version:** sincronizar VERSION para 0.1.0-dev.20+20261021.d4c4c0c
- **release:** automação de sync de versão da crate (closes #5)
- update Cargo.lock to 0.1.3
- **version:** sincronizar VERSION para 0.1.0-dev.17+20261021.02540c9
- **release:** bump joaju-lirica to 0.1.3 (#4)
- **version:** sincronizar VERSION para 0.1.0-dev.12+20261020.59a55b9
- **version:** sincronizar VERSION para 0.1.0-dev.10+20261020.98786a6
- **version:** sincronizar VERSION para 0.1.0-dev.8+20261020.c81b1a0
- initial repository scaffold under GOST standard

## [Unreleased]

### Added
- Core poetic scansion and metric analysis engine in Rust (`escandir_verso`).
- Exact UTF-8 byte span preservation (`SilabaPoetica.span: Range<usize>`) for real-time editor integration.
- Lexical and metric stress detection with graphic accent precedence, clitic monosyllables, and paroxytone/oxytone ending rules.
- Poetic elision (sinalefa) engine respecting punctuation cesura and stress clash.
- Support for inline dieresis `/` and forced elision `_` operators.
- Support for ````lirica code blocks with `%%` configuration directives (`%% metro`, `%% elidir_pontuacao`, `%% dierese_crescente`, `%% saida`, `%% titulo`, `%% autor`).
- Dual-panel HedgeDoc style viewer and native Typst renderer.
- Standalone CLI utility (`joaju-lirica`) supporting interactive panel view and `--json` structured output.
