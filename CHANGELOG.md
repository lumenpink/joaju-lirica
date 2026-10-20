# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
