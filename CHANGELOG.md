# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
