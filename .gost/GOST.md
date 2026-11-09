# ☭ ГОСТ / GOSPLAN — Especificação Canônica Consolidada

> **Aviso de Integridade Gosplan:**
> Este documento foi montado automaticamente pelo CCCP (`cccp profile assemble`).
> A precedência é estrita de baixo para cima: cada camada subsequente sobrepõe as definições anteriores.

### 🌳 Árvore de Herança e Precedência Resolvida:
1. Camada 1: `https://gost.2lp.in/3.0.0/`
2. Camada 2: `https://gosplan.2lp.in/1.0.0/`
3. Camada 3: `https://kollontai.2lp.in/2.1.0/`
4. Camada 4: `https://lilya.2lp.in/profile.md`
5. Camada Local Autônoma: `CUSTOM.md`

---

<!-- ===================================================================== -->
<!-- CAMADA 1: https://gost.2lp.in/3.0.0/ -->
<!-- ===================================================================== -->

---
standard: "gost"
tier: "fundamental"
version: "3.0.0"
locale: "pt-br"
authority: "Standards Committee"
published_at: "2026-11-07"
---

# GOST — Metanorma Fundamental para Repositórios Abertos
**Versão da Norma:** `3.0.0`  
**Status:** Oficial / Metanorma Fundamental  
**Referência:** ГОСТ (Государственный Стандарт / General Open Source Standard)

---

## 1. Escopo e Propósito

Esta especificação define a **Metanorma Fundamental** do ecossistema, estabelecendo a teoria geral das normas, a hierarquia de camadas, a gramática de requisitos, a anatomia documental canônica e as garantias de governança e integridade aplicáveis universalmente a repositórios de software aberto.

A GOST opera sob o princípio da neutralidade tecnológica perene: ela positiva as regras de validade, delegação e controle sem vincular-se a ferramentas utilitárias efêmeras ou tecnologias particulares.

As palavras-chave **DEVE** (*MUST*), **NÃO DEVE** (*MUST NOT*), **DEVERIA** (*SHOULD*), **NÃO DEVERIA** (*SHOULD NOT*) e **PODE** (*MAY*) neste documento devem ser interpretadas conforme descrito na [RFC 2119](https://datatracker.ietf.org/doc/html/rfc2119).

---

## 2. Teoria Geral das Normas e Hierarquia em Quatro Camadas

O ordenamento regulatório do ecossistema estrutura-se em quatro esferas subordinadas e interdependentes:

1. **Camada Fundamental (`tier: fundamental`):**
   - Papel constitucional e meta-regulatório do ecossistema (GOST).
   - Estabelece as regras sobre a criação, vigência, formato, tipicidade e controle de todas as normas subordinadas.
2. **Camada Estratégica (`tier: strategic`):**
   - Define o planejamento plurianual, filosofia de desenvolvimento, autonomia popular e metodologia de ciclo de vida (ex.: ciclo DD-TDD de 9 fases e protocolo do Dia Zero).
3. **Camada Tática (`tier: tactical`):**
   - Fixa as toolchains homologadas de engenharia, interfaces de interoperabilidade e contratos universais de CLI.
4. **Camada Operacional (`tier: operational`):**
   - Determina a infraestrutura concreta, topologia de storage, cotas de retenção, esteiras de deploy e serviços de suprimento/estado.
5. **Autonomia Local (`tier: local`):**
   - Regras e especificações customizadas do próprio repositório (`CUSTOM.md` e `custom-rules.toml`), respeitando os limites da hierarquia superior.

### 2.1 Princípio da Tríade Normativa Fechada
* Todo projeto regulado vincula-se formalmente a uma tríade imutável de normas: `Operacional X.Y.Z -> Tática A.B.C -> Estratégica D.E.F -> Fundamental G.H.I`.
* É vedada a vinculação a versões instáveis (`-dev.XX`), tags flutuantes (`latest`) ou o uso do prefixo `v` nas declarações de versão.
* A alteração de uma norma superior exige a emissão de nova versão imutável nas normas inferiores correspondentes, garantindo determinismo histórico e auditabilidade perene.

### 2.2 Anatomia Documental e Frontmatter Canônico
Todo documento normativo DEVE ser redigido em Markdown UTF-8 com Frontmatter YAML obrigatório em inglês declarando:
`standard`, `tier`, `version`, `locale`, `authority`, `published_at` e o bloco `inherits_from` com `base_url` canônica apontando para o portal público (`https://<norma>.2lp.in/<versao>/`).

---

## 3. Filosofia e Licenciamento

Todo projeto em conformidade com o GOST **DEVE** declarar sua licença de forma explícita na raiz do repositório através de um arquivo `LICENSE`. A especificação técnica detalhada, perfis e matriz normativa encontram-se em [specs/legal/licenses.md](./specs/legal/licenses.md).

### 2.1 Princípio da Menor Permissividade como Padrão Raiz
* **Regra da Raiz:** O arquivo `LICENSE` na raiz **DEVE** ser a licença padrão e geral de todo o repositório.
* Em repositórios com múltiplos tipos de artefatos (como monorepos ou aplicações híbridas), o arquivo `LICENSE` **DEVE** conter a licença menos permissiva (mais restritiva) adotada (ex: `GPL-3.0`).
* Quaisquer licenças mais permissivas (ex: `Apache-2.0` para sub-bibliotecas ou `CC-BY-SA-4.0` para mídias) funcionam como exceções modulares declaradas.

### 2.2 Tipologia por Perfil de Repositório
1. **Aplicações Completas / Monorepos Híbridos:** Adotam `GPL-3.0` (ou `AGPL-3.0` para serviços web) na raiz (`LICENSE`).
2. **Utilitários, Ferramentas Autônomas e Microapps:** Adotam `Apache-2.0` na raiz (`LICENSE`) diretamente, sem necessidade de arquivos satélites supérfluos.
3. **Bibliotecas, Protocolos e SDKs:** Adotam `Apache-2.0` na raiz (`LICENSE`) para máxima interoperabilidade.
4. **Especificações e Normas:** Adotam `CC-BY-SA-4.0` para os textos normativos na raiz (`LICENSE`), com esquemas e templates em `CC0-1.0` e marcas protegidas via satélite.

### 2.3 Arquivos Satélites Padronizados
Quando houver artefatos que exijam tratamentos jurídicos específicos, os seguintes arquivos canônicos **DEVEM** ser utilizados:
* `LICENSE-MEDIA`: Creative Commons Atribuição-CompartilhaIgual 4.0 Internacional (`CC-BY-SA-4.0`) para mídias, áudios, vídeos, fontes e ilustrações.
* `LICENSE-LOGO`: Creative Commons Atribuição-SemDerivações 4.0 Internacional (`CC-BY-ND-4.0`) para marcas, insígnias, ícones oficiais e logotipos.
* `LICENSE-APACHE`: Apache License 2.0 (`Apache-2.0`) para módulos, sub-crates ou bibliotecas desacopladas contidas em monorepos de aplicações.

### 2.4 Mapeamento Obrigatório no `README.md`
A presença de qualquer arquivo satélite (`LICENSE-MEDIA`, `LICENSE-LOGO` ou `LICENSE-APACHE`) **OBRIGA** a inclusão de uma seção explícita de mapeamento de diretórios e arquivos cobertos por cada licença no `README.md`.

### 2.5 Princípio Anti-Boilerplate
Cada repositório implementa exclusivamente as licenças condizentes com seu perfil real. É vedado copiar blocos genéricos de licenciamento e arquivos satélites vazios para projetos simples.

---

## 3. Anatomia Mínima da Raiz e Metadados

Todo repositório GOST **DEVE** conter em sua raiz a seguinte estrutura mínima independente da stack ou linguagem utilizada:

```text
.
├── GOSTVER                     # Declaração canônica de versão da norma e perfil
├── .gost/
│   └── metadata.json           # Manifesto SSOT de metadados do projeto (Regra GOST-26)
├── LICENSE                     # Termo legal de licenciamento padrão (Seção 2)
├── LICENSE-MEDIA               # (Opcional) Licença CC-BY-SA para mídias e assets
├── LICENSE-LOGO                # (Opcional) Licença CC-BY-ND para marcas e logos
├── LICENSE-APACHE              # (Opcional) Licença Apache-2.0 para bibliotecas internas
├── VERSION                     # Versão semântica em texto plano (Seção 4)
├── CHANGELOG.md                # Registro cronológico de mudanças (Seção 4)
├── README.md                   # Apresentação executiva e portal enxuto (Seção 9)
├── .gitignore                  # Higiene do repositório e arquivos ignorados
├── .editorconfig               # Configuração canônica de formatação para editores
├── site/                       # Portal estático de documentação pública (Seção 9)
│   ├── index.html              # Ponto de entrada da documentação
│   └── _headers                # Cabeçalhos HTTP de segurança (ou wrangler.jsonc)
└── .forgejo/workflows/ci.yml   # Esteira hermética de CI/CD em arquivo único (Seção 10)
```

### 3.1 Proibição de Poluição na Raiz
É terminantemente proibida a presença de arquivos avulsos de linters (`.prettierrc`, `.prettierrc.json`, `.eslintrc`, `.eslintrc.js`) ou arquivos de orquestração legados (`compose.yml`, `docker-compose.yml`, `docker-compose.yaml`). Arquivos de configuração devem residir em subdiretórios apropriados ou aderir estritamente aos nomes canônicos (`compose.yaml`).

### 3.2 Manifesto Canônico de Metadados (`.gost/metadata.json` — Regra `GOST-26`)
O arquivo `.gost/metadata.json` é a autoridade central (SSOT) para identidade, vitrine e catálogos:
* `slug`: Identificador único minúsculo em kebab-case ($\le 24$ caracteres, `^[a-z0-9-]+$`).
* `title`: Nome próprio legível do aplicativo ou biblioteca ($\le 25$ caracteres).
* `site_type`: Tipo de vitrine pública (`"doc"` para documentação/CLI ou `"app"` para aplicação web interativa).
* `tagline`: Subtítulo resumido em uma frase clara ($\le 60$ caracteres), respeitando `len(title + " — " + tagline) <= 65`.
* `summary`: Resumo executivo objetivo ($50$ a $140$ caracteres), consumido em meta tags Open Graph e feeds.
* `abstract`: Resumo expandido ($150$ a $500$ caracteres) utilizado no lead do `README.md` e hero do site.
* `tags`: Lista contendo entre 3 e 8 tags em kebab-case ($\le 20$ caracteres cada).
* `icon`: Ícone representativo ou caractere visual.
* `theme`: Objeto com `bg_color` e `accent_color` em formato hexadecimal.
* **Filtro Anti-Jargão:** As descrições em prosa **NÃO DEVEM** conter termos técnicos vazios ou autorreferenciais (`"autônomo"`, `"autônomo"`, `"hexagonal"`, `"ssot"`, `"rust puro"`, `"em rust"`, `"em php"`, `"wasm"`).

---

## 4. Versionamento Semântico e Histórico

### 4.1 Arquivo `VERSION` e Conformidade SemVer 2.0.0
* O repositório **DEVE** manter um arquivo `VERSION` na raiz contendo exclusivamente a versão atual do projeto em formato de texto puro UTF-8, finalizado por quebra de linha.
* O formato de versionamento **DEVE** aderir estritamente ao [Semantic Versioning 2.0.0](https://semver.org/) (`MAJOR.MINOR.PATCH[-PRERELEASE][+BUILD]`).
* **Exibição Dual Padronizada:**
  - *Versão Short:* `v{MAJOR}.{MINOR}.{PATCH}` (truncada antes do `-` ou `+`) para cabeçalhos, banners e títulos.
  - *Versão Long:* String integral idêntica ao arquivo `VERSION` para rodapés e CLI `--version`.
* **Sincronia Multi-Arquivo Mandatória:** A versão base declarada em `VERSION` deve coincidir rigorosamente com a versão declarada no topo do `CHANGELOG.md`, no arquivo `site/index.html` e nos manifestos de pacotes correspondentes (`Cargo.toml`, `package.json`, `composer.json`, `DESCRIPTION`).

### 4.2 Arquivo `CHANGELOG.md`
* O repositório **DEVE** manter um histórico de modificações legível por humanos e baseado no padrão [Keep a Changelog](https://keepachangelog.com/).
* As seções de cada versão **DEVEM** categorizar alterações em: `Added`, `Changed`, `Deprecated`, `Removed`, `Fixed`, `Security` ou subdivisões semânticas convencionais.

---

## 5. Ciclo DD-TDD, Branches e o Protocolo de 9 Fases

O fluxo de trabalho em repositórios GOST **DEVE** seguir o ciclo rigoroso de **Documentation-Driven & Test-Driven Development (DD-TDD)** em 9 fases sequenciais e indivisíveis, governado pela regra de rastreabilidade estrita de tickets e isolamento em branches.

### 5.1 Cláusulas Pétreas do Ciclo

1. **Inviolabilidade das 9 Fases e Inferência Automática:** Todas as 9 etapas **DEVEM** ser seguidas em ordem sequencial estrita. O desenvolvedor não informa a fase manualmente; o Maestro de Governança deduz a fase (`ph3` a `ph9`) a partir do estado do histórico e dos arquivos alterados no commit.
2. **Zero Código sem Ticket e Documentação Prévia:** Nenhuma alteração em código de produção é permitida sem ticket aberto e especificação prévia de contrato/documentação.
3. **Isolamento Estrito da Fase de Documentação (Fase 3):** Na Fase 3, é terminantemente proibido alterar qualquer linha de código executável em `src/`, `lib/`, `bin/` ou `tests/`. Apenas especificações textuais, contratos e esquemas são aceitos.
4. **Rastreabilidade Estrita de Commits e Marcadores de Fase:**
   - Commits intermediários **DEVEM** seguir o padrão: `<tipo>: <resumo> (ref #<id> - ph<N>)` (onde `N` varia de 3 a 8).
   - O commit final de encerramento da Fase 9 **DEVE** seguir o padrão: `<tipo>: <resumo> (closes #<id> - ph9)`.
5. **Laudo Textual de Resolução Obrigatório na Fase 9:** O commit de fechamento da Fase 9 (`closes #<id> - ph9`) **DEVE** conter corpo textual ($\ge 80$ caracteres) com os cabeçalhos formais:
   - `[Resumo]:` Síntese objetiva do escopo tratado.
   - `[Solução]:` Descrição das modificações arquiteturais e técnicas efetuadas.
   - `[Validação]:` Relação dos testes determinísticos e gates executados para comprovação.  
   *No momento do merge, o Maestro extrai automaticamente esse laudo e o registra como comentário de fechamento no issue tracker.*
6. **Fast-Tracks Declarados:**
   - `docs`: Isenção formal das Fases 4 a 7 (exige estritamente as Fases 1, 2, 3, 8 e 9).
   - `chore`: Isenção formal das Fases 4 e 7 para manutenções de infraestrutura (exige estritamente as Fases 1, 2, 3, 5, 8 e 9).
7. **Proibição Absoluta de Commit Direto na `main`:** É estritamente proibido commitar diretamente na branch base `main`. Todo trabalho reside em branch de ticket isolada.

### 5.2 Estratégia de Branches e Governança Local-First
* **Branches de Trabalho:** Devem seguir o padrão `^(feat|fix|docs|refactor|test|chore)/([0-9]+)-([a-z0-9-]+)$`.
* **Governança Local-First:** As branches de trabalho existem exclusivamente no ambiente local. O envio de branches de trabalho para a forja remota é proibido, exceto se autorizado via flag explícita (`--multidev`).
* **Branches Persistentes Autorizadas:** Além da `main` (e `pages`), são autorizadas no remote `origin`:
  - Branches de manutenção de versão maior: `v<MAJOR>` ou `release/v<MAJOR>.x`.
  - Branches de épicos multi-ticket: `epic/<slug>`.
* **Protocolo de Gênese (Ticket #1):** Novos repositórios são inicializados pelo Maestro (`init`), abrindo a Issue #1 (`chore`), gerando a anatomia básica, emitindo o Laudo de Gênese no commit inaugural da `main` e disparando a publicação inicial.

### 5.3 As 9 Fases Canônicas do Ciclo DD-TDD

| Fase | Nome | Obrigatoriedade | Padrão do Commit |
| :---: | :--- | :---: | :--- |
| **1** | **Ticket / Issue** | ✅ **OBRIGATÓRIO** | *(Abertura no tracker — sem commit)* |
| **2** | **New Branch** | ✅ **OBRIGATÓRIO** | *(Criação da branch — sem commit)* |
| **3** | **Doc / Spec Contract** | ✅ **OBRIGATÓRIO** | `docs: specify <comportamento> (ref #<id> - ph3)` |
| **4** | **Red: Failing Test** | 🔴 **OBRIGATÓRIO** *(exceto fast-track)* | `test: add failing test for <requisito> (ref #<id> - ph4)` |
| **5** | **Green: Implement** | ✅ **OBRIGATÓRIO** | `feat: implement <funcionalidade> (ref #<id> - ph5)` ou `fix: resolve <bug> (ref #<id> - ph5)` |
| **6** | **Refactor to Excellence** | 🟡 **CONDICIONAL** | `refactor: optimize and polish <escopo> (ref #<id> - ph6)` |
| **7** | **Mutation Test & Fortify** | 🟡 **CONDICIONAL** *(ordem estrita)* | `test: eliminate surviving mutants in <escopo> (ref #<id> - ph7)` *(testes primeiro)*;<br>`refactor: eliminate dead code in <escopo> (ref #<id> - ph7)` *(código depois)* |
| **8** | **Fast Local Hygiene** | ✅ **OBRIGATÓRIO** | `fix: lint and formatting hygiene (ref #<id> - ph8)` *(se houver ajustes)* |
| **9** | **Finish Docs & Close** | ✅ **OBRIGATÓRIO** | `<tipo>: finalize <escopo> (closes #<id> - ph9)` *(com Laudo no corpo)* |

---

## 6. O Maestro de Governança

### 6.1 Política de Ponto Único de Entrada (*Single Entrypoint*)
O gerenciamento do ciclo de vida, git, tickets e conformidade deve ser operado exclusivamente através do Maestro de Governança CLI. É terminantemente proibido o uso de comandos manuais do Git (`git commit`, `git merge`, `git push`, `git pull`, `git tag`) ou ferramentas ad-hoc de terceiros (`tea`, `gh`, `glab`).

### 6.2 Contrato Universal de Subcomandos do Maestro
Qualquer implementação do Maestro de Governança compatível com a GOST deve expor a seguinte interface determinística:
* `init`: Inicializa repositório, estrutura e protocolo de gênese.
* `ticket`: Consulta, abertura, triagem e fechamento de tickets.
* `divan`: Assistente interativo de triagem e planejamento.
* `start <id>`: Cria e faz checkout da branch de trabalho padronizada.
* `status`: Audita a conformidade atual do repositório, branch e hooks.
* `commit`: Valida e cria commits com inserção determinística de tags de ticket e fase (`ph3` a `ph8`).
* `mutate`: Executa testes de mutação restritos ao diff local.
* `hygiene`: Executa os formatadores determinísticos e linters rápidos (Fase 8).
* `close`: Registra o laudo e gera o commit canônico de encerramento (`closes #<id> - ph9`).
* `merge`: Realiza o merge validando o laudo e a tag de fechamento.
* `push`: Executa pré-checagens e realiza o envio para o remote autorizado.
* `release`: Conduz o processo de tag semântica e auditoria pré-release.
* `gost sync/check`: Sincroniza e audita a conformidade das regras técnicas.

### 6.3 Protocolo de Comandos para Agentes de IA
Agentes autônomos de IA devem seguir o vocabulário de comandos operacionais determinísticos:
* `DCPD`: Despacho completo com planejamento e documentação prévia.
* `DCP`: Despacho padrão de implementação e correção.
* `PLAN`: Elaboração e detalhamento de plano de ação sem escrita de código.
* `TDD`: Execução estrita do ciclo Red-Green-Refactor.
* `AUDIT`: Auditoria estática de conformidade e integridade.
* `FIX`: Correção de bugs pontuais com teste de regressão comprovado.
* `RELEASE`: Preparação e condução de versão estável.

---

## 7. Estratégia de Testes e Cobertura

### 7.1 Taxonomia de Testes
1. **Testes Unitários:** Isolamento absoluto, sem dependência de I/O externo, execução em milissegundos.
2. **Testes Funcionais:** Validação de regras de negócio e transformações de dados.
3. **Testes de Integração:** Validação entre subsistemas internos e adaptadores concretos.
4. **Testes Ponta a Ponta (E2E):** Simulação de interação real de usuários em navegadores headless via Playwright quando aplicável.

### 7.2 Delegação de Cobertura para a Esteira
* **Piso Mínimo Inviolável:** A cobertura global de testes automatizados **DEVE** ser de no mínimo **`90%`**, com meta de **`100%`** no domínio.
* **Aferição na Esteira Remota:** A verificação estrita de cobertura com quebra de build é critério bloqueante delegado categoricamente à esteira remota hermética (`ci.yml`), liberando o fluxo de desenvolvimento local de overhead excessivo.

### 7.3 Mutação Restrita ao Diff
Na Fase 7 local, as ferramentas de teste de mutação devem operar restritas ao diff de alterações (`--in-diff` ou `--git-diff-lines`), deixando baterias globais de mutação para esteiras de auditoria periódica.

---

## 8. Análise Estática e Tolerância Zero a Avisos (*Zero Warnings*)

### 8.1 Tolerância Zero a Warnings
Linters e analisadores estáticos devem operar com severidade máxima (ex: `-D warnings`). Avisos são tratados como erros fatais e bloqueiam a integração. É proibida a permanência de tokens de depuração (`dbg!`, `debugger;`, `var_dump`, `dd()`, `print_r`) em código final.

### 8.2 Formatação Determinística e `.editorconfig`
Todo projeto deve adotar um formatador oficial determinístico verificado com flags `--check`. O arquivo `.editorconfig` é obrigatório na raiz e padroniza UTF-8, LF, newline final e remoção de espaços em branco.

---

## 9. Vitrine Pública, SSOT e Banners

### 9.1 Topologia de Vitrine e Arquétipos de Aplicação (`APP_TYPE` / `site_type`)
Todo repositório deve declarar sua natureza estrutural através do metadado `APP_TYPE` (ou `site_type`) no `GOSTVER` ou frontmatter de projeto, categorizado sob quatro arquétipos canônicos:
1. **`web` (Webapps Autônomas e Autorais):** O componente principal é uma aplicação gráfica interativa web (*ex.: Navalha, Lírica*). A frontpage é a própria aplicação ou manifesto vivo, com liberdade estética e de design absoluta. Documentação técnica e de apoio é embutida sob rota `/docs` ou diretório `docs/`.
2. **`hybrid` (Aplicações Híbridas):** O foco principal reside no utilitário binário ou serviço compilado, acompanhado de uma interface web por conveniência (*ex.: Lacraia, Lauda*). A frontpage apresenta o produto e documentação de usuário (`docs/user/`), reservando subdiretório dedicado (`web/` ou subrota `/app/`) para a webapp interativa.
3. **`binary` (Utilitários Binários Puros):** Ferramentas de linha de comando (CLI) ou daemons sem interface web interativa (*ex.: CCCP, Kombinat, Lunokhod*). O site institucional é 100% voltado à apresentação, manuais de usuário, documentação técnica e notas de versão, gerado de forma determinística por motor SSG padronizado.
4. **`manifesto` (Documentações e Manifestos Puros):** Projetos estritamente textuais, normativos ou literários sem compilação binária (*ex.: GOST, Travesti*). O site é uma publicação editorial de alta legibilidade, índice remissivo e preservação integral dos textos.

### 9.2 SSOT em Markdown e README Enxuto
* **SSOT:** Documentações, manuais e guias residem estritamente em Markdown. É proibido duplicar textos extensos manualmente em HTML.
* **README Enxuto:** O arquivo `README.md` na raiz deve ter um teto rígido de **50 linhas**, servindo como portal rápido para a vitrine oficial.

### 9.3 Banner Open Graph Minimalista
* **Resolução:** `1200 × 630 px` com Safe Zone centralizada 4:3 (`720–840px` $\times$ `500px`).
* **Conteúdo Visual:** Apenas o ícone/logo sobre as cores do tema e o texto `{title} — {tagline}` ($\le 65$ caracteres). Veto a badges de linguagem ou links impressos diretamente na imagem.
* **Sincronia de Meta Tags:** `<meta property="og:title">` deve equivaler a `{title} — {tagline}`, e `<meta property="og:description">` ao `{summary}` ($\le 140$ caracteres).

### 9.4 Rodapé Canônico em Duas Linhas
Aplicações e documentações estáticas devem exibir rodapé em duas linhas:
* **Linha 1 — Afirmação Identitária:** Definida e parametrizada conforme o perfil adotado.
* **Linha 2 — Metadados de Versão e Licença:** `{Nome do App} v{Versão} • [{Licença}](LICENSE)`.

### 9.5 Cabeçalhos HTTP de Segurança
A vitrine estática deve declarar cabeçalhos `X-Content-Type-Options: nosniff`, `X-Frame-Options: DENY` e `Referrer-Policy: strict-origin-when-cross-origin` em `site/_headers` ou `wrangler.jsonc`.

---

## 10. Esteira Hermética de CI/CD

### 10.1 Arquivo Único e Hermeticidade
O pipeline de automação reside estritamente em `.forgejo/workflows/ci.yml`. É proibida a existência da pasta `.github/workflows/`. São proibidos comandos de download ou instalação em runtime (`apt-get`, `apk add`, `pip install`, `npm i -g`, `cargo install`, `wget`, `curl`). Todas as ferramentas necessárias devem estar pré-instaladas na imagem base local do runner.

### 10.2 Filas Canônicas e Dependência Estrita
* Filas padronizadas: `test` (Quality Gate), `build`/`release` e `deploy`.
* A fila `test` é obrigatória; todas as filas posteriores devem declarar expressamente `needs: [test]`.
* Builds de release e empacotamento ocorrem exclusivamente na esteira; é proibido compilar releases pesadas na máquina local.

### 10.3 Zero Binários no Histórico Git (Regra `GOST-24`)
É expressamente proibido versionar artefatos compilados, arquivos ELF, pacotes ou tarballs (`.exe`, `.dll`, `.so`, `.tar.gz`, `.zip`, `.deb`, `.rpm`, `.apk`) no repositório Git.

### 10.4 Arquitetura de Distribuição Desacoplada e Retenção de Artefatos
* **Modelo Produtor–Curador Desacoplado:** A publicação de softwares opera sob divisão estrita de responsabilidades:
  - **Produtor (Repositório do Software):** Compila o código na esteira, calcula os hashes de integridade criptográfica (SHA-256), envia os artefatos diretamente para o storage de distribuição e notifica a autoridade curadora de catálogo via evento estruturado.
  - **Curador (Repositório Central de Catálogo):** Ouve os eventos, executa auditoria estrita de rede e integridade de checksum dos arquivos novos e do histórico ativo, gerencia a retenção no storage e publica o catálogo consolidado.
* **Política Mandatória de Retenção e Ciclo de Vida:** Todo catálogo ou esteira de distribuição **DEVE** adotar política formal de retenção e rotação de artefatos por canal de entrega (`stable`, `dev`, etc.), purgando compilações antigas excedentes para impedir a proliferação desgovernada de ativos e garantir previsibilidade de armazenamento. As cotas numéricas e particularidades operacionais são fixadas nos perfis operacionais.
* **Tolerância Zero a Links Quebrados:** Se qualquer artefato declarado (novo ou histórico ativo) falhar na verificação de rede ou apresentar divergência de checksum, a publicação do catálogo **DEVE** ser abortada preventivamente e um chamado/issue automático **DEVE** ser emitido para o repositório produtor.

---

## 11. Internacionalização (Intl)

### 11.1 Paridade Matemática de Chaves nas 10 Línguas
Projetos internacionalizados devem manter paridade estrita de chaves nos 10 idiomas canônicos: `en`, `pt`, `es`, `fr`, `de`, `it`, `ru`, `zh`, `ja` e `ar`.

### 11.2 Suporte RTL e Propriedades Lógicas
O idioma árabe (`ar`) requer ativação do atributo `dir="rtl"`. Interfaces devem empregar propriedades lógicas em CSS (`margin-inline`, `padding-inline`), banindo propriedades físicas horizontais estáticas.

---

## 12. Arquitetura de Domínio, DDD e SOLID (Regra `GOST-27`)

### 12.1 Pureza de Domínio (Zero I/O no Core)
O diretório e módulos de domínio/core (`src/domain/`, `lib/core.sh`, `internal/domain/`, etc.) **NÃO DEVEM** conter operações ou importações diretas de I/O (rede, filesystem, banco de dados, variáveis de ambiente ou chamadas a processos de sistema).

### 12.2 Inversão de Dependência e In-Memory Fakes
* O Core comunica-se com o mundo externo exclusivamente através de abstrações (Portas).
* Mocks dinâmicos pesados são rejeitados em favor de adaptadores em memória leves (*In-Memory Fakes*) baseados em tipos nativos da linguagem.
* Fakes e adaptadores reais compartilham a mesma suíte de testes de contrato (LSP).

### 12.3 Validação na Borda e Primitivos Confiáveis
A entrada de dados é validada e sanitizada nas bordas do sistema (*Parse, Don't Validate*). O Core opera com tipos confiáveis e enums nativos da linguagem para modelar estados finitos.

---

## 13. Aderência a Padrões de Sistema e Operação (XDG, 12-Factor & OCI)

### 13.1 Diretórios do Sistema (XDG Base Directory)
Aplicações devem respeitar `$XDG_CONFIG_HOME`, `$XDG_DATA_HOME`, `$XDG_STATE_HOME`, `$XDG_CACHE_HOME` e `$XDG_RUNTIME_DIR`. É terminantemente proibido criar arquivos e pastas ocultas soltas na home do usuário (`~/.app`). Devem oferecer suporte a `NO_COLOR`.

### 13.2 Contêineres OCI e Compose
* Manifestos de orquestração devem adotar o nome canônico `compose.yaml`.
* Dockerfiles devem implementar build multi-stage, execução sob usuário não-root, healthchecks e banimento de tags `:latest`. É proibido o uso de Watchtower.

---

## 14. Perfis de Implementação e Política de Waivers

### 14.1 Herança em Cascata de Perfis
Perfis especializados (`profile.md`) especializam e complementam a norma central em ordem hierárquica de precedência através da diretiva `extends`.

### 14.2 Mecanismo de Waivers Auditados
Exceções temporárias a regras técnicas devem ser formalizadas com auditoria rastreável:
* **Arquivo `custom_rules.json`:** Declara regras ignoradas exigindo `rule_id`, `reason` ($\ge 20$ caracteres) e ID do ticket associado.
* **Diretiva Inline:** `gost-ignore: <RULE_ID> (ticket #<id> - <motivo>)`.
* Agentes autônomos de IA são terminantemente proibidos de criar ou editar diretivas de waiver.

---

## 15. Governança de Fronteira e Quarentena de Branches (`inbox/*`)

### 15.1 Topologia de Remotes Git
O remote `origin` na forja primária é o único destino autorizado para operações de push. Espelhos remotos públicos operam exclusivamente como mirrors de consulta e devem ser configurados com `pushurl = no_push`.

### 15.2 Quarentena de Contribuições Externas
Contribuições externas e Pull Requests vindos de espelhos são recebidos sob isolamento no namespace `refs/heads/inbox/*`, sujeitos a esteiras não-confiáveis offline antes de qualquer integração.

---

## 16. Taxonomia e Contratos dos Escopos de Avaliação de Regras

As regras e verificações declaradas em toda a hierarquia normativa (`rules.toml`) operam sob escopos canônicos de avaliação que delimitam estritamente o momento do ciclo de vida, o alvo inspecionado e o contexto dos dados injetados:

1. **Escopo `commit-msg` (Interceptação Atômica de Commit):**
   - **Gatilho de Execução:** Hook de interceptação `commit-msg` ou comando do Maestro (`cccp commit`).
   - **Alvo:** Mensagem e corpo do commit em preparação (`.git/COMMIT_EDITMSG`).
   - **Responsabilidade:** Validação de convenções sintáticas de Conventional Commits, referências obrigatórias a tickets, declaração explícita de fase DD-TDD e completude do laudo estruturado de fechamento na Fase 9.
2. **Escopo `project` (Auditoria Estática de Repositório):**
   - **Gatilho de Execução:** Auditoria local (`cccp check`) e esteiras de integração contínua (CI).
   - **Alvo:** Sistema de arquivos da árvore de trabalho do projeto.
   - **Responsabilidade:** Validação da anatomia mínima da raiz, metadados (`VERSION`, `CHANGELOG.md`, `.cccprc`), manifestos de contêineres OCI, contratos OpenAPI e higiene documental permanente.
3. **Escopo `pre-push` (Interceptação de Envio Remoto):**
   - **Gatilho de Execução:** Hook `pre-push` ou comando do Maestro (`cccp push`).
   - **Alvo:** Branches locais contra referências remotas.
   - **Responsabilidade:** Prevenção de commits diretos na branch principal, confirmação de fechamento de tickets associados e checagens rápidas de integridade pré-publicação.
4. **Escopo `git-history` (Integridade Retroativa):**
   - **Gatilho de Execução:** Auditoria retroativa e checagem de integridade profunda.
   - **Alvo:** Grafo e cadeia de commits da branch contra a base canônica.
   - **Responsabilidade:** Garantia de que a sequência de fases DD-TDD e laudos históricos não foram alterados ou corrompidos.
5. **Escopo `post-build` (Sanidade e Conformidade de Artefatos Compilados):**
   - **Gatilho de Execução:** Pós-compilação em auditorias locais (`cccp check --scope post-build`), rotinas de build e esteiras de integração contínua (CI/CD).
   - **Alvo:** Artefatos executáveis ou empacotados gerados em `target/debug`, `target/release` ou `dist/`.
   - **Responsabilidade:** Verificação funcional dinâmica (*smoke tests*), prevenção de falhas de segmentação (`SIGSEGV`) ou *panics* imediatos de inicialização, integridade de ligação dinâmica (*dynamic linking* / bibliotecas compartilhadas ausentes) e conformidade das interfaces mínimas (`--help`, `--version` idêntico à versão canônica e subcomandos de ciclo de vida).

---

<!-- ===================================================================== -->
<!-- CAMADA 2: https://gosplan.2lp.in/1.0.0/ -->
<!-- ===================================================================== -->

---
standard: "gosplan"
tier: "strategic"
version: "1.0.0"
locale: "pt-br"
authority: "Standards Committee"
published_at: "2026-11-08"
inherits_from:
  standard: "gost"
  tier: "fundamental"
  version: "3.0.0"
  base_url: "https://gost.2lp.in/3.0.0/"
---

# GOSPLAN — Norma Estratégica Plurianual

## 1. Escopo e Propósito
O **GOSPLAN** define as diretrizes estratégicas plurianuais de desenvolvimento de software, a filosofia metodológica orientada a testes e documentação e os protocolos canônicos de ciclo de vida para todo o ecossistema.
Subordina-se estritamente à Metanorma Fundamental GOST v3.0.0.

---

## 2. A Regra de Ouro do GOSPLAN: Registro Prévio Universal de Tickets
1. **Antecedência Obrigatória:** Toda e qualquer alteração de código ou documentação é estritamente precedida por um ticket aberto na forja canônica.
2. **Totalidade Prévia nos Planos:** Em qualquer plano que envolva mais de uma etapa ou entrega, **o primeiro passo executório mandatório é criar e registrar na forja a totalidade dos tickets que compõem o plano completo**, antes de codificar ou alterar a primeira linha do primeiro ticket.
3. **Proibição de Tickets Ad-hoc Tardios:** É vedado avançar planos operacionais gerando tickets "sob demanda" durante o percurso. Todos os tickets devem existir como entidades públicas rastreáveis desde o momento zero da aprovação do plano.

---

## 3. O Ciclo Metodológico GOST DD-TDD e a Tipologia dos Três Ciclos

O desenvolvimento de software no ecossistema diferencia o rito metodológico conforme a natureza da alteração, assegurando o máximo rigor de testes onde há lógica e agilidade onde há documentação ou manutenção:

### 3.1 Ciclo Completo (*Full*): `feat`, `fix`, `perf`
Aplica-se a qualquer alteração de código de produção, novas capacidades funcionais ou correções de bugs, exigindo o percurso estrito das 9 fases ordinais:
1. **Fase 1 (Ticket):** Abertura prévia do ticket na forja documentando contexto e requisitos.
2. **Fase 2 (Branch):** Criação de branch dedicada isolada vinculada ao ID do ticket (`<tipo>/<id>-<descricao>`).
3. **Fase 3 (Doc/Spec):** Atualização da documentação viva e permanente do repositório (`docs/` e `README.md`) refletindo o novo estado ou capacidade pretendida, antes de qualquer código de produção. É estritamente vedada a criação de arquivos incrementais efêmeros de especificação por ticket no sistema de arquivos do repositório (ex.: `docs/specs/<id>-...md`). A especificação técnica contextual do ticket é registrada no corpo do commit da Fase 3 (`ref #<id> - ph3`) e como comentário oficial no ticket na forja.
4. **Fase 4 (Red Test):** Criação de teste automatizado comprovadamente falho que ateste a ausência da funcionalidade.
5. **Fase 5 (Green Implement):** Implementação do código estritamente necessário para fazer os testes passarem.
6. **Fase 6 (Refactor):** Melhoria da legibilidade, performance e desacoplamento com testes 100% verdes.
7. **Fase 7 (Mutation Test):** Execução de testes de mutação para aferir que os testes quebram se a lógica for corrompida.
8. **Fase 8 (Fast Local Hygiene):** Execução de formatadores determinísticos (`fmt --check`) e linters estáticos (`-D warnings`).
9. **Fase 9 (Finish & Close):** Commit de encerramento contendo o Laudo Textual obrigatório estruturado (`[Resumo]:`, `[Solução e Alterações]:`, `[Decisões Recusadas]:` detalhando trade-offs e alternativas descartadas, `[Validação]:`) com teto de $\ge 15$ linhas e tag `(closes #<id> - ph9)`, merge na branch canônica e fechamento da issue na forja com a postagem integral do Laudo.

### 3.2 Ciclo Curto (*Short*): `docs`, `style`
Aplica-se exclusivamente a alterações documentais, traduções, ajustes tipográficos e padronização visual de código. Por não envolver lógica de negócio, é isento de testes Red/Green e de mutação:
1. **Fase 1 (Ticket):** Abertura prévia do ticket na forja.
2. **Fase 2 (Branch):** Criação de branch dedicada (`docs/<id>-...` ou `style/<id>-...`).
3. **Fase 3 (Doc/Style):** Alteração direta dos arquivos de documentação ou formatação visual.
4. **Fase 8 (Fast Local Hygiene):** Validação de links, formatação determinística e conformidade documental.
5. **Fase 9 (Finish & Close):** Commit de encerramento com Laudo Textual sucinto (`closes #<id> - ph9`) e merge.

### 3.3 Ciclo Administrativo / Manutenção (*Chore*): `chore`, `ci`, `build`
Aplica-se a tarefas de manutenção de repositório, bumps de dependências, atualização de pipelines de CI/CD, configurações de build ou setup inicial de projetos:
1. **Fase de Abertura:** Abertura do ticket na forja e checkout da branch vinculada (`chore/<id>-...`, `ci/<id>-...`, `build/<id>-...`).
2. **Fase de Execução:** Alteração dos manifestos, dependências ou scripts (commits com convenção padrão `chore: ... (ref #<id>)` sem obrigatoriedade de numeração de fases intermediárias `ph3..ph7`).
3. **Fase de Conclusão:** Encerramento direto via `(closes #<id>)` ou `(closes #<id> - chore)` com Laudo de Encerramento objetivo e merge na branch canônica.

### 3.4 Vinculação aos Escopos de Regras
Os portões de qualidade das regras da governança interceptam os ciclos conforme seu escopo canônico:
- **`commit-msg`:** Intercepta cada commit em edição, validando a sintaxe, a referência obrigatória ao ticket ativo e as regras específicas do ciclo selecionado (`full`, `short` ou `chore`).
- **`project`:** Audita o sistema de arquivos e integridade permanente da árvore de trabalho (higiene de living docs, contêineres, manifestos e metadados).
- **`pre-push`:** Intercepta a operação de push, garantindo a integridade dos tickets e branches antes da publicação remota.


---

## 4. O Protocolo do Dia Zero (Gênese / Ticket #1)

A gênese de qualquer novo projeto no ecossistema obedece ao seguinte rito canônico:
1. **Passo 1:** Criação do repositório remoto vazio na forja.
2. **Passo 2:** Abertura imediata do Ticket #1 (`chore(init): constituir anatomia inicial e genesis do repositorio`).
3. **Passo 3:** Clone local e checkout da branch `chore/1-...`.
4. **Passo 4:** Criação da anatomia essencial (`README.md` $\le 50$ linhas, `CHANGELOG.md`, `VERSION`, `LICENSE`, `.gitignore`, `.gost/config`).
5. **Passo 5:** Fechamento via Fase 9 com Laudo Textual e merge na branch principal (`main`).

---

## 5. Diretriz Estratégica de Conteinerização Universal de Serviços

1. **Obrigatoriedade Universal de Containers:** Todo e qualquer serviço, microsserviço, daemon, API ou backend do ecossistema DEVE ser executado e distribuído em containers compatíveis com OCI (Open Container Initiative), garantindo portabilidade hermética e independência absoluta do sistema operacional hospedeiro.
2. **Proibição de Daemons Nativos no Host:** É expressamente proibida a execução ou instalação de serviços de backend diretamente no sistema operacional do host (bare-metal/systemd nativo sem isolamento).
3. **Segregação de Exposição de Rede:** Apenas o serviço que atua como ponto único de contato externo (Gossnab) deve possuir porta de ingresso exposta publicamente. Serviços de orquestração interna e entrega contínua (Kombinat) devem operar estritamente confinados na rede privada interna, com comunicação estritamente unidirecional para a infraestrutura de distribuição.

---

## 6. Governança Estratégica de Contratos de API (OpenAPI 3.1 Type-First)

Para mitigar quebras silenciosas e garantir resiliência e fail-fast em sistemas distribuídos:

1. **Padrão Provedor (`docs/openapi.yaml`):**
   - Todo serviço que proveja endpoints HTTP (APIs públicas ou privadas) DEVE formalizar seu contrato sob a especificação **OpenAPI 3.1**.
   - O schema DEVE ser derivado deterministicamente a partir da tipagem de código (*Type-First*, como `utoipa` no ecossistema Rust), mantendo documentação rica em linguagem natural nos campos `description` e `summary`.
   - O arquivo `docs/openapi.yaml` DEVE ser gerado ou validado como invariante obrigatória nas suítes de testes de integração.
2. **Padrão Consumidor (`docs/external/<provedor>.yaml`):**
   - Todo serviço ou ferramenta consumidora de APIs (internas ou forjas de terceiros como Forgejo, GitHub, SourceHut) DEVE versionar o snapshot contratual esperado em `docs/external/<provedor>.yaml`.
   - O cliente de consumo DEVE adotar deserialização estrita com *fail-fast* (ex.: `#[serde(deny_unknown_fields)]`), garantindo que campos ausentes ou corrompidos falhem ruidosamente na entrada.
3. **Propagação Automatizada em Cascata:**
   - O gestor de CD e orquestração contínua (Kombinat) DEVE incluir atualizações de `docs/openapi.yaml` a montante no ciclo de propagação automática em cascata, sincronizando `docs/external/<provedor>.yaml` nos repositórios a jusante e disparando seus testes de conformidade.

---

## 7. Diretriz de Documentação Canônica, SSOT e Topologia

1. **Topologia Canônica em `docs/`:**
   - **`docs/user/`:** Manuais de usuário, tutoriais práticos, guias de tarefas, FAQs e apresentações de recursos. Linguagem clara, acessível e sem jargões de implementação.
   - **`docs/dev/`:** Decisões arquiteturais (ADRs), contratos de integração, modelos de domínio, diagramas de dados e especificações de contribuição técnica.
   - **`SPEC.md`:** Especificação normativa viva do repositório.
2. **Princípio da Referência Unidirecional:**
   - Documentos técnicos (`docs/dev/` e `SPEC.md`) referenciam documentos de usuário (`docs/user/`) para contextualizar o comportamento visível sem reescrever a explicação.
   - Documentos voltados ao usuário (`docs/user/`) nunca devem conter acoplamento ou dependência direta com detalhes internos de `docs/dev/`.
3. **SSOT Única com Motores de SSG:**
   - Os geradores de site (como o Tachinha) e esteiras de publicação devem consumir diretamente os arquivos Markdown em suas localizações canônicas.
   - É estritamente proibido criar arquivos Markdown duplicados dentro do diretório do site (`site/`).

---

<!-- ===================================================================== -->
<!-- CAMADA 3: https://kollontai.2lp.in/2.1.0/ -->
<!-- ===================================================================== -->

---
standard: "kollontai"
tier: "tactical"
version: "2.1.0"
locale: "pt-br"
authority: "Standards Committee"
published_at: "2026-11-08"
inherits_from:
  standard: "gosplan"
  tier: "strategic"
  version: "1.0.0"
  base_url: "https://gosplan.2lp.in/1.0.0/"
---

# Perfil de Implementação GOST: Kollontai

> **Sobre este Perfil:**  
> O perfil **Kollontai** (em homenagem à revolucionária soviética, feminista e diplomata Alexandra Kollontai) define o ferramental concreto de engenharia, a implementação oficial do Maestro de Governança (**`cccp`**), a matriz de versões de compiladores, o protocolo de auto-atualização de binários e a arquitetura de distribuição para projetos aderentes ao padrão **GOST**.  
> Ele é agnóstico a domínios pessoais e identidades individuais, servindo como perfil base para coletivos, laboratórios ou perfis autorais derivados (como o perfil `lumen`).

---

## 1. O Maestro Concreto: `cccp` e `.cccprc`

No perfil Kollontai, a autoridade de governança definida na Seção 6 da norma GOST é materializada pelo binário **`cccp`** (integrado ao sistema de triagem **`divan`**).

### 1.1 Arquivo de Configuração `.cccprc`
Todo repositório sob o perfil Kollontai **DEVE** declarar na raiz (ou herdar via configuração global em `$XDG_CONFIG_HOME/cccp/config.env`) os parâmetros operacionais do Maestro:

```ini
# Idioma primário para triagem de tickets (divan), Laudo da Fase 9 e fallback de i18n
agent_var_lang = portuguese

# Diretivas de Forja Primária (Sobrescritas pelo perfil filho ou pelo repositório)
primary_forge_type = forgejo
primary_owner = <owner>
primary_repo = <slug>

# Espelhos Passivos e Ingestão de Fronteira (Separados por vírgula: codeberg,github)
mirrors = codeberg,github
mirror_owner = <mirror_owner>
```

### 1.2 Mapeamento de Comandos do Maestro (`cccp`)
É estritamente proibido invocar comandos brutos de mutação do Git ou CLIs de terceiros (`tea`, `gh`, `glab`). O ciclo de vida completo opera via `cccp`:

| Subcomando `cccp` | Fase / Escopo | Ação Determinística Executada pelo Maestro |
| :--- | :---: | :--- |
| `cccp init --slug <s> ...` | **Dia Zero** | Cria repo local (`main`), provisiona forja primária e espelhos (`pushurl = no_push`), abre o Ticket `#1` (`chore`), gera anatomia GOST + `.gost/metadata.json`, comita `(closes #1 - ph9)` com Laudo de Gênese, fecha o Ticket `#1` e faz o primeiro push. |
| `cccp ticket create` | **Fase 1** | Cria a issue na forja primária (com tipo, contexto e critérios de aceite) e sincroniza o cache local em `.git/cccp/tickets/<id>.json`. |
| `cccp divan` | **Fase 1** | Sessão assistida de triagem, quebra de épicos e refinamento de tickets. |
| `cccp start <id> [--base <b>]` | **Fase 2** | Lê os metadados do ticket `#<id>`, atualiza a branch base (`main`, `vN` ou `epic/*`) e faz checkout automático para `<tipo>/<id>-<slug>`. |
| `cccp status` | **Fases 1–9** | Inspeciona o histórico da branch ativa e exibe o ticket atual, a fase alcançada (`ph3`..`ph9`) e o próximo passo obrigatório. |
| `cccp commit "<resumo>"` | **Fases 3–8** | Deduz o `<id>` e a fase atual (`ph3`..`ph8`), valida as travas da fase (bloqueia código em `ph3`; exige falha de teste em `ph4`; exige sucesso em `ph5`; impõe ordem `test` $\to$ `refactor` em `ph7`) e formata `<tipo>: <resumo> (ref #<id> - ph<N>)`. |
| `cccp mutate` | **Fase 7** | Executa a ferramenta de teste de mutação da linguagem ativa restrita ao diff da branch (`git diff <base>..HEAD`). |
| `cccp hygiene` / `cccp fix` | **Fase 8** | Executa instantaneamente o linter estático (`-D warnings`) e o verificador/aplicador de formatação determinística das linguagens declaradas no `GOSTVER`. |
| `cccp close "<resumo>"` | **Fase 9** | Exige o corpo estruturado com o **Laudo Textual de Resolução** (`[Resumo]:`, `[Solução]:`, `[Validação]:`), valida a atualização de `CHANGELOG.md`/`site/` e grava o commit final `(closes #<id> - ph9)`. |
| `cccp merge` | **Pós-Fase 9** | Verifica o cumprimento das fases e do Fast Hygiene, integra a branch na base (`main`, `vN` ou `epic/*`), posta o Laudo do commit `ph9` como comentário oficial no ticket encerrando-o na forja e remove a branch local. |
| `cccp epic start|finish <slug>` | **Épicos** | Cria ou finaliza branches persistentes de integração (`epic/<slug>`) para reescritas multi-ticket. |
| `cccp push [--multidev]` | **Publicação** | Bloqueia push de branches efêmeras de ticket (salvo `--multidev`) e envia branches persistentes (`main`, `vN`, `epic/*`) para o `origin`. |
| `cccp release <patch|minor|major>` | **Release** | Executa auditoria pré-tag em container (`act`), consolida o `CHANGELOG.md`, atualiza o `VERSION`, assina a tag Git e publica. |
| `cccp gost sync | check` | **Governança** | Resolve a cascata de perfis (`extends`), propaga o `.gost/metadata.json` e executa as auditorias determinísticas do `rules.json`. |
| `cccp inbox list | review | accept` | **Fronteira** | Gerencia contribuições externas isoladas no namespace de quarentena `refs/heads/inbox/*`. |

---

## 2. Topologia de Forjas, Espelhos e Fronteira

### 2.1 Autoridade da Forja Primária e Espelhamento Passivo
1. **Unidirecionalidade Local (`Zero Dual Push`):** O ambiente local mantém o remote `origin` apontando exclusivamente para a forja primária definida no perfil final. A replicação de código para espelhos externos (Codeberg, GitHub) é responsabilidade exclusiva do *Push Mirror* automático da forja primária (`sync_on_commit: true`).
2. **Sinalização de Espelhos e Paridade de Metadados:**
   - O comando `cccp gost sync` mantém `description` (extraído de `summary`), `website` e `topics` (extraídos de `tags` no `.gost/metadata.json`) idênticos entre a forja primária e os espelhos.
   - Nos espelhos secundários, a descrição recebe automaticamente o prefixo canônico:
     `[MIRROR / BACKUP] {summary} — Repositório canônico: {primary_forge_url}`
3. **Configuração Obrigatória de Remotes para Ingestão de PRs:**
   Para permitir que o controlador de fronteira baixe Pull Requests externos sem risco de push acidental para os espelhos, o `.git/config` **DEVE** ser mantido pelo `cccp` no seguinte formato:
   ```ini
   [remote "origin"]
       url = <primary_ssh_url>
       fetch = +refs/heads/*:refs/remotes/origin/*

   [remote "codeberg"]
       url = https://codeberg.org/<mirror_owner>/<slug>.git
       fetch = +refs/pull/*/head:refs/remotes/codeberg/pr/*
       pushurl = no_push

   [remote "github"]
       url = https://github.com/<mirror_owner>/<slug>.git
       fetch = +refs/pull/*/head:refs/remotes/github/pr/*
       pushurl = no_push
   ```

---

## 3. Matriz de Toolchains Homologadas (*Toolchain Pinning*)

As regras arquiteturais (DDD/Hexagonal), estruturas de diretórios, templates de `Dockerfile` e comandos de cada linguagem residem nos módulos `specs/languages/<lang>.md` da norma GOST. O perfil Kollontai fixa as versões mínimas homologadas para execução local e nas imagens da esteira:

| Stack (`GOST_LANGUAGES`) | Versão Canônica / Runtime | Runner de Testes & Cobertura (≥90%) | Mutação (Fase 7) | Linter (`-D warnings`) & Formatador |
| :--- | :--- | :--- | :--- | :--- |
| **`rust`** | `rustc >= 1.99.0` (`edition = "2024"`, target estático `musl`) | `cargo test` + `cargo-llvm-cov` | `cargo-mutants` (`--in-diff`) | `clippy` + `rustfmt` + `cargo-audit` |
| **`php`** | `PHP >= 8.5.0` (compatibilidade reversa `>= 8.2`, `strict_types=1`) | `pest --coverage --min=90` | `infection` (`--git-diff-lines`) | `phpstan --level=max` (target 8.2) + `pint` + `composer audit` |
| **`shell`** | POSIX `/bin/sh` estrito (ou `Bash >= 5.2` justificado) | `shellspec --kcov` | Revisão de borda (Boundary/Subshells) | `shellcheck` + `shfmt -d -i 2 -ci -bn` |
| **`javascript`** | `Node.js >= 24.16.0 LTS` (ESM nativo) + `TypeScript >= 5.8` (`strict`) | `vitest` (v8 coverage) + `playwright` (E2E) | `stryker` | `biome check` + `npm audit` |
| **`r`** | `R >= 4.4.1` + `renv` (lockfile imutável `renv.lock`) | `testthat` (3e) + `covr` | Revisão de invariantes (`NA`/`NaN`/`Inf`) | `lintr` + `styler` + `roxygen2` |
| **`go`** | `Go >= 1.22+` (cgo desabilitado para binário estático puro) | `go test -race` + `go tool cover` | `gremlins` | `golangci-lint` + `gofmt` + `govulncheck` |
| **`python`** | `Python >= 3.12+` (tipagem estrita mypy, Poetry/uv) | `pytest --cov` | `mutatest` | `ruff check/format` + `mypy --strict` + `pip-audit` |

### 3.1 Framework Canônico de Interface Gráfica: Dioxus 0.8
Para aplicações gráficas multiplataforma em Rust (Web WASM, Desktop via Tao/Wry e Mobile Android/iOS), o perfil Kollontai adota o **Dioxus 0.8** (`dioxus = "0.8"`):
* **Ergonomia Mobile Obrigatória:** Respeito a `env(safe-area-inset-top)` / `env(safe-area-inset-bottom)`, alvos de toque mínimos de `48px` com `touch-action: manipulation` e navegação adaptativa (barra inferior em telas `< 640px` e barra lateral compacta em `640px–1024px`).

---

## 4. Protocolo de Auto-Atualização (XDG) e Manifesto de Distribuição

Todas as ferramentas de linha de comando (CLI) e aplicações desktop distribuídas sob o perfil Kollontai **DEVEM** implementar o protocolo de auto-atualização conectado ao catálogo de distribuição central (a **Banca**) e interoperável com a **Jornaleira**.

### 4.1 Caminhos XDG e Formato de Configuração (`config.env`)
Para manter conformidade com a regra anti-bloat da norma GOST (Seção 13.2), a configuração e o cache de estado residem estritamente em caminhos XDG:
1. **Configuração do Usuário:** `${XDG_CONFIG_HOME:-~/.config}/<slug>/config.env`  
   Formato plano canônico `CHAVE="VALOR"` (compatível com POSIX Shell, Rust, Go e C):
   ```sh
   UPDATE_CHANNEL="stable"           # Canal rastreado: "stable" ou "nightly"
   UPDATE_PINNED="false"             # Se "true", bloqueia atualizações e silencia alertas
   UPDATE_PIN_VERSION=""             # Versão fixada quando UPDATE_PINNED="true"
   UPDATE_AUTO_CHECK="true"          # Verificação periódica não-bloqueante em modo autônomo
   UPDATE_CHECK_INTERVAL_HOURS="12"  # Frequência padrão de consulta (12 horas)
   UPDATE_AUTO_UPDATE="false"        # Substituição atômica sem confirmação (opt-in estrito)
   UPDATE_MANAGED_BY_JORNALEIRA=""   # Override: "" (auto-detectar), "true" ou "false"
   ```
   *Precedência de Resolução:* Flags CLI (`--channel`, `--pin`) > Variáveis de ambiente (`<SLUG_UPPER>_UPDATE_*` / `UPDATE_*`) > `${XDG_CONFIG_HOME}/<slug>/config.env` > Padrões normativos Kollontai.

2. **Estado e Cache de Runtime:** `${XDG_STATE_HOME:-~/.local/state}/<slug>/update_check.json`  
   Registro estruturado compartilhado entre o aplicativo e a Jornaleira:
   ```json
   {
     "last_check_timestamp": 1729411200,
     "current_version": "0.1.0",
     "latest_remote_version": "0.1.2",
     "channel": "stable",
     "update_available": true,
     "last_check_status": "success",
     "managed_by": "jornaleira"
   }
   ```

### 4.2 A Jornaleira como Gestora Primária e Comportamento em Runtime
1. **Hegemonia da Jornaleira:** A Jornaleira atua como a gestora primária do ecossistema.  
   * **Detecção:** Se `UPDATE_MANAGED_BY_JORNALEIRA="true"` ou se `command -v jornaleira >/dev/null 2>&1` retornar sucesso (sem override explícito `"false"`):
     * O **auto-check nativo dos aplicativos no startup permanece sumariamente desativado**.
     * Nenhuma requisição de rede ou consulta DNS é disparada durante a invocação normal de comandos do aplicativo.
     * A consulta remota periódica e a substituição de binários passam a ser orquestradas exclusivamente pelos timers e rotinas centrais da Jornaleira.
2. **Modo Autônomo (Sem Jornaleira):**  
   * Se a Jornaleira não estiver presente (ou `UPDATE_MANAGED_BY_JORNALEIRA="false"`), o utilitário assume a responsabilidade de verificar atualizações de forma não-bloqueante no startup caso o intervalo de 12 horas tenha expirado, com timeout estrito de $\le 1\text{s}$ descartando falhas silenciosamente.
3. **Alerta Não-Intrusivo em `stderr`:**  
   Se o arquivo `${XDG_STATE_HOME}/<slug>/update_check.json` indicar `update_available: true`, emite alerta exclusivamente em `stderr` (preservando pipelines em `stdout`):
   * *Quando sob gestão da Jornaleira:*
     ```text
     💡 [<slug>] Nova versão disponível: vX.Y.Z (atual: vA.B.C). Execute 'jornaleira update <slug>' para atualizar.
     ```
   * *Quando em modo autônomo:*
     ```text
     💡 [<slug>] Nova versão disponível: vX.Y.Z (atual: vA.B.C). Execute '<slug> update' para atualizar.
     ```

### 4.3 Contrato de Subcomandos CLI e Persistência de Canais
Todo binário distribuído **DEVE** responder aos subcomandos:
* `<slug> check-update [--channel <stable|nightly>] [--stable] [--nightly]`  
  Realiza consulta remota síncrona não-bloqueante na Banca, atualiza o arquivo `${XDG_STATE_HOME}/<slug>/update_check.json` e informa o status.
* `<slug> update [--channel <stable|nightly>] [--stable] [--nightly] [--pin <ver>] [--force] [--dry-run]`  
  Efetua a substituição atômica pelo binário mais recente da Banca respeitando o canal solicitado.
* `<slug> config show | set-channel <c> | pin [<ver>] | unpin | set-auto-check <bool> | set-auto-update <bool>`  
  Exibe ou altera de forma persistente as opções em `${XDG_CONFIG_HOME}/<slug>/config.env`.

#### **Garantia de Persistência do Canal Dev / Nightly:**
* A preferência declarada em `UPDATE_CHANNEL` (em `config.env`) define a assinatura duradoura de atualizações do usuário (`stable` ou `dev`/`nightly`).
* O comando pontual `<slug> update --stable` (ou `--channel stable`) instala a compilação estável requisitada naquele momento, **mas NUNCA altera o valor de `UPDATE_CHANNEL`** no arquivo `config.env`.
* A reconfiguração persistente do canal exige comando explícito (`<slug> config set-channel stable` ou flag `--save-channel`).
* Enquanto `UPDATE_CHANNEL="dev"` (ou `"nightly"`), o verificador continuará consultando e alertando sobre novas compilações do canal `dev` assim que publicadas na Banca, mesmo que o executável local em execução seja momentaneamente uma versão estável.

### 4.4 Topologia de Armazenamento e Distribuição Desacoplada
A esteira de publicação opera sob o modelo **Produtor–Curador Desacoplado**:
* **Produtor (Repositório do Software):** Compila o código na esteira, calcula os hashes de integridade (SHA-256), envia os artefatos diretamente para o storage de distribuição, atualiza a cópia de alias e despacha um evento de notificação via API (`repository_dispatch`).
* **Curador (Repositório Central da Banca):** Ouve os eventos via esteira (`repository_dispatch`), executa auditoria estrita de existência e integridade dos arquivos novos e do histórico ativo, gerencia a retenção no storage conforme o perfil operacional (Lilya), atualiza o catálogo (`catalog.json`) e realiza o deploy.
* **Topologia do Storage (Bucket Único Particionado):**
  Todos os artefatos são mantidos sob o prefixo `banca-artifacts/{app}/{channel}/`:
  - `{app}-{versao}-{target}.{ext}` e respectivo `.sha256`
  - Cópias físicas de alias sobrescritas: `{app}-latest-{target}.{ext}` (em `stable`) ou `{app}-dev-latest-{target}.{ext}` (em `dev`), juntamente com seus arquivos `.sha256`.

### 4.5 Nomenclatura de Canais e Aliases Canônicos
A publicação e o consumo de artefatos de distribuição respeitam os canais canônicos:
1. **Canal Estável (`stable`):**
   - Rótulo na interface: *Versão Recomendada*.
   - Sufixo / padrão de versão: SemVer 2.0 (`x.y.z`).
   - Alias canônico: `*-latest.*` (ex.: `{app}-latest-{target}.{ext}`).
   - Possui **exclusividade absoluta** sobre o alias primário `latest` e versões desprovidas de sufixo.
2. **Canal de Desenvolvimento (`dev`):**
   - Rótulo na interface: *Versão Experimental*.
   - Sufixo / padrão de versão: `x.y.z-dev.{n}+{YYYYMMDD}.{commit}`.
   - Alias canônico: `*-dev-latest.*` (ex.: `{app}-dev-latest-{target}.{ext}`).
3. **Consumo de Versões Canônicas Recomendado:**
   - Em manifestos de build (`Cargo.toml`, etc.), a fixação em versões canônicas exatas (`=`) é formalmente **recomendada** (*SHOULD* / *DEVERIA*) para garantir reprodutibilidade hermética em cascata.
4. **Tolerância Zero a Links Quebrados:**
   - Se qualquer artefato declarado (novo ou histórico ativo) falhar na verificação de rede ou apresentar divergência de checksum, a publicação do catálogo é abortada imediatamente.

---

## 5. Esteira de CI/CD Hermética (`.forgejo/workflows/ci.yml`)

Em conformidade com a Seção 10 da norma GOST, todo repositório Kollontai possui um único arquivo `.forgejo/workflows/ci.yml` executando em containers locais pré-provisionados (sem downloads externos em runtime), dividido nas filas canônicas:

```yaml
name: "Gosplan Hermetic Pipeline"

on:
  push:
    branches: [main, "v*", "release/v*", "epic/*"]
  pull_request:

jobs:
  test:
    name: "Quality Gate (Lint, Tests, Coverage >= 90%, Audit)"
    runs-on: local-hermetic
    steps:
      - name: "Checkout Code"
        uses: actions/checkout@v4
      - name: "Execute GOST Compliance & Test Suite"
        run: |
          cccp gost check
          cccp test --coverage-min 90

  release:
    name: "Build Static Artifacts & Publish to Banca"
    runs-on: local-hermetic
    needs: [test]
    if: github.ref == 'refs/heads/main'
    steps:
      - name: "Checkout Code"
        uses: actions/checkout@v4
      - name: "Compile & Dispatch to R2 / Banca"
        run: cccp ci publish-artifacts

  deploy:
    name: "Deploy Documentation Site & Containers"
    runs-on: local-hermetic
    needs: [test]
    if: github.ref == 'refs/heads/main'
    steps:
      - name: "Checkout Code"
        uses: actions/checkout@v4
      - name: "Deploy Static Showcase & Orphan Pages"
        run: cccp ci deploy-site
```

* **Isolamento da Quarentena (`inbox/*`):** Para branches `refs/heads/inbox/*`, apenas o job `test` é executado em modo desprivilegiado (sem acesso a secrets de deploy ou chaves da Banca).

### 5.1 Contrato de Notificação via API (`repository_dispatch`)
O repositório produtor do software notifica a esteira central da Banca via chamada HTTP:
`POST /api/v1/repos/{owner}/banca/dispatches` com `event_type: "release-published"`.

#### Schema do Payload (`client_payload`):
```json
{
  "event_type": "release-published",
  "client_payload": {
    "app": "slug",
    "channel": "stable",
    "version": "x.y.z",
    "commit": "hash",
    "repo_slug": "owner/slug",
    "run_id": "1842",
    "assets": [
      {
        "filename": "app-0.1.0-linux-x86_64.tar.gz",
        "alias": "app-latest-linux-x86_64.tar.gz",
        "type": "archive",
        "format": "tar.gz",
        "target": "linux-x86_64",
        "sha256": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
      }
    ]
  }
}
```

### 5.2 Idempotência na Publicação e Pre-Delete de Aliases Mutáveis
Para forjas de pacotes que rejeitam sobrescrita direta de arquivos com o mesmo nome sob uma versão (como os Generic Packages do Forgejo, gerando erros `HTTP 409 Conflict`), a esteira de CI/CD **DEVE** garantir idempotência:
* Antes de executar o envio (`PUT`) para aliases mutáveis (`latest-dev`, `${BASE_VERSION}-dev`, `latest`), a esteira deve executar uma limpeza prévia com deleção idempotente (`curl -s -X DELETE ... || true`).
* Essa prática previne falhas espúrias de publicação e assegura que a forja atualize o ponteiro do alias sem interrupções.

---

## 6. Vitrine Web, Internacionalização e Gerador de Banners

### 6.1 Roteador Estático Cliente (`site/index.html`) para `site_type: doc`
A página raiz atua como roteador leve para os 10 subdiretórios de idioma (`en`, `pt`, `es`, `fr`, `de`, `it`, `ru`, `zh`, `ja`, `ar`), respeitando `localStorage.getItem('doc_lang')`, `navigator.language` e o fallback primário do `.cccprc`:

```html
<script>
  (function() {
    var supported = ['en', 'pt', 'es', 'fr', 'de', 'it', 'ru', 'zh', 'ja', 'ar'];
    var saved = localStorage.getItem('doc_lang');
    if (saved && supported.indexOf(saved) !== -1) {
      window.location.replace(saved + '/');
      return;
    }
    var nav = (navigator.language || 'pt').toLowerCase().substring(0, 2);
    window.location.replace((supported.indexOf(nav) !== -1 ? nav : 'pt') + '/');
  })();
</script>
```

### 6.2 Geração Automatizada do Banner Minimalista (`cccp site banner`)
O banner `site/assets/og_preview.png` (`1200 × 630 px`, Safe Zone 4:3 de `720px–840px` × `500px`) é gerado deterministicamente a partir do `.gost/metadata.json`:
* Renderiza o fundo com `theme.bg_color` e detalhes de contraste com `theme.accent_color`.
* Posiciona o `icon` oficial e imprime exclusivamente a chamada curta **`{title} — {tagline}`** ($\le 65$ caracteres), sem textos secundários.

### 6.3 Template Estrutural do Rodapé Canônico em Duas Linhas
O perfil Kollontai estabelece o contrato de renderização do rodapé nas 10 línguas oficiais, parametrizando as variáveis de identidade (`{author_affirmation_i18n}`, `{author_name}`, `{author_url}`, `{country_i18n}`) que são preenchidas pelo perfil final:

```html
<footer class="gost-footer">
  <p>{author_affirmation_i18n}</p>
  <p>{title} v{version_full} • <a href="{author_url}" target="_blank" rel="noopener">{author_name}</a> • {country_i18n} • <a href="LICENSE">{license_spdx}</a></p>
</footer>
```

---

## 7. Catálogo Local de Descoberta (`~/src/catalog.json`)

Todo repositório inicializado ou sincronizado pelo `cccp` registra seus metadados extraídos do `.gost/metadata.json` no índice central da estação de trabalho (`~/src/catalog.json`):

```json
{
  "slug": "<slug>",
  "title": "<title>",
  "tagline": "<tagline>",
  "summary": "<summary>",
  "tags": ["<tag1>", "<tag2>", "<tag3>"],
  "license": "<spdx>",
  "site_type": "doc",
  "primary_forge": "<primary_forge_url>",
  "website": "<public_site_url>"
}
```

---

## 8. Padrões Táticos de Conteinerização Obrigatória de Serviços

Em cumprimento à diretriz estratégica do GOSPLAN, todo serviço de backend, microsserviço ou daemon distribuído no perfil Kollontai DEVE atender aos seguintes padrões de conteinerização:

### 8.1 Anatomia de Imagem (`Dockerfile` / `Containerfile`)
1. **Multi-Stage Hermético:** O estágio de build compila o software hermeticamente; o estágio final copia apenas o binário compilado e artefatos estritamente necessários para uma imagem base minimalista (`alpine` ou `scratch`).
2. **Execução Desprivilegiada Mandatória:** O container DEVE executar sob usuário não-root explicitamente (`user: "1000:1000"`). É proibida a execução contínua como `root`.
3. **Imutabilidade e Efemeridade:** O sistema de arquivos do container é considerado volátil, delegando persistência a volumes nomeados ou bind-mounts explicitados no compose.

### 8.2 Orquestração Declarativa Local (`compose.yaml`)
1. **Extensão Canônica:** A declaração de orquestração utiliza estritamente o nome `compose.yaml` (sendo expressamente banido o uso da extensão obsoleta `.yml`).
2. **Política de Reinício e Healthcheck:** Todo serviço em produção declara `restart: unless-stopped` e uma probe ativa de integridade (`healthcheck` com `interval`, `timeout` e `retries`).
3. **Isolamento de Rede:** Serviços internos operam em redes privadas OCI (`networks: default: internal: true` ou isoladas). Portas só devem ser expostas se o serviço for explicitamente um ponto de ingresso público.

---

## 9. Padrão Tático de Sanidade Pós-Compilação (*Smoke Tests*)

Todo artefato executável gerado no perfil Kollontai está sujeito a validação no escopo canônico `post-build`:

1. **Execução Dinâmica Rápida:** O binário gerado é executado com argumentos de inspeção inofensivos em tempo limite estrito ($\le 3$ segundos).
2. **Prevenção de Falhas Críticas:** O teste detecta e bloqueia falhas de segmentação (`SIGSEGV` / sinal 11), *panics* imediatos de runtime ou ausência de dependências dinâmicas compartilhadas (*shared libraries* faltantes no ambiente).
3. **Isolamento e Limpeza:** A execução não deve deixar processos zumbis ou arquivos temporários não tratados no host.

---

<!-- ===================================================================== -->
<!-- CAMADA 4: https://lilya.2lp.in/profile.md -->
<!-- ===================================================================== -->

---
standard: "lilya"
tier: "operational"
version: "2.1.0"
locale: "pt-br"
authority: "Standards Committee"
published_at: "2026-11-08"
inherits_from:
  standard: "kollontai"
  tier: "tactical"
  version: "2.1.0"
  base_url: "https://kollontai.2lp.in/2.1.0/"
---

# Perfil de Implementação GOST: Lilya

> **Sobre este Perfil:**  
> O perfil **Lilya** (anteriormente denominado *Lumen*) é a especialização autoral, pessoal e de infraestrutura do padrão **[GOST](https://gost.2lp.in)**.  
> Ele estende diretamente o perfil base **[Kollontai](https://kollontai.2lp.in)** (`extends: https://kollontai.2lp.in/profile.md`), herdando a autoridade do Maestro **`cccp`**, a matriz de toolchains homologadas e os protocolos de distribuição, aplicando precedência estrita (*deep override*) para a forja canônica em `gambi.2lp.in`, publicação na borda (`*.2lp.in`), catálogo de binários (`banca.2lp.in`), afirmação identitária transfeminista no rodapé e entrega contínua nos servidores `obizero`, `obione` e `obitwo`.

---

## 1. Identidade, Forja Canônica (`gambi.2lp.in`) e Espelhos

### 1.1 Parâmetros Canônicos do Maestro (`.cccprc`)
No perfil Lilya, o Maestro `cccp` opera com os seguintes parâmetros canônicos de infraestrutura:

```ini
agent_var_lang = portuguese

# Forja Primária Canônica (Forgejo em gambi.2lp.in)
primary_forge_type = forgejo
primary_forge_domain = gambi.2lp.in
primary_owner = lumen
primary_repo = <slug>
primary_ssh_url = git@gambi.2lp.in:lumen/<slug>.git

# Espelhos Passivos e Ingestão de Fronteira
mirrors = codeberg,github
mirror_owner = lumenpink
```

### 1.2 Topologia de Repositórios e Push Mirrors Duplos
1. **Forja Primária Canônica:** Hospedada em `https://gambi.2lp.in/lumen/<slug>` (acesso SSH via porta 22 padrão em `git@gambi.2lp.in:lumen/<slug>.git`).
2. **Push Mirrors Automatizados (`sync_on_commit: true`):** A instância `gambi.2lp.in` replica imediatamente todo push recebido na branch `main` (e tags/branches persistentes) para dois espelhos externos de vitrine e preservação:
   - **Espelho FOSS:** Codeberg (`https://codeberg.org/lumenpink/<slug>`)
   - **Espelho Comunitário:** GitHub (`https://github.com/lumenpink/<slug>`)
3. **Remotes Locais e Ingestão de PRs:**
   - O desenvolvedor e os agentes de IA executam `cccp push` exclusivamente para o `origin` (`git@gambi.2lp.in:lumen/<slug>.git`).
   - Os remotes `codeberg` e `github` são mantidos pelo `cccp` com `pushurl = no_push` e refspec `+refs/pull/*/head:refs/remotes/<remote>/pr/*` para ingestão segura na quarentena `inbox/*`.

### 1.3 Allowlist Estrita de Rede na Esteira (`ci.yml`)
Na auditoria da regra `GOST-10-CI-HERMETIC-SINGLE-FILE`, a esteira `.forgejo/workflows/ci.yml` nos runners do ecossistema Lilya **SÓ PODE** referenciar recursos e espelhos internos sob os domínios:
* **`gambi.2lp.in`** (Actions espelhadas internamente ou relativas sem prefixo de servidor).
* **`crates.2lp.in`** (Registry interno e espelho de pacotes).
* **`banca.2lp.in`** (Despacho de artefatos compilados na fila `release`).
* Chamadas a outros domínios de rede pública quebram o `cccp check --strict` imediatamente.

---

## 2. Rodapé Canônico em Duas Linhas (Afirmação Identitária & Autonomia)

O perfil Lilya preenche o template estrutural de rodapé herdado do perfil Kollontai com a afirmação identitária transfeminista sul-americana nas 10 línguas oficiais da norma GOST.

### 2.1 Estrutura HTML Canônica
Todo aplicativo com interface gráfica, TUI/CLI com rodapé formatado, vitrine pública ou documentação estática (`./site`) **DEVE** renderizar as duas linhas rigorosamente nesta estrutura:

```html
<footer class="footer">
  <p>Orgulhosamente produzido por uma <a href="https://travesti.2lp.in/pt/" target="_blank" rel="noopener">travesti latina</a> 🏳️‍⚧️ :)</p>
  <p>{title} v{version_full} • por <a href="https://lumen.pink" target="_blank" rel="noopener">Lumen Pink</a> • Brasil 🇧🇷 • <a href="LICENSE">{license_spdx}</a></p>
</footer>
```

### 2.2 Regras Invioláveis de Renderização do Rodapé
1. **Intraduzibilidade Política do Termo `travesti`:** O substantivo *travesti* permanece rigorosamente sem tradução nos 10 idiomas oficiais, preservando sua singularidade histórica e geopolítica latino-americana. O adjetivo concorda gramaticalmente com o idioma ativo e aponta para `https://travesti.2lp.in/<lang>/`.
2. **Versão Short vs. Versão Long:**
   - Cabeçalhos, títulos de janela e badges no topo da página exibem estritamente a **Versão Short** (`v{MAJOR}.{MINOR}.{PATCH}`, truncada antes do `+`).
   - A Linha 2 do rodapé exibe obrigatoriamente a **Versão Long** (string integral do arquivo `VERSION`, incluindo metadados de build `+YYYYMMDD`, ex: `v0.1.0+20261026`).

### 2.3 Matriz Oficial de Tradução do Rodapé (10 Línguas)

| ISO | Linha 1 (Afirmação Identitária Transfeminista) | Linha 2 (Metadados de Autonomia e Licença) |
|:---:|---|---|
| **`pt`** | Orgulhosamente produzido por uma [travesti latina](https://travesti.2lp.in/pt/) 🏳️‍⚧️ :) | {title} v{ver} • por [Lumen Pink](https://lumen.pink) • Brasil 🇧🇷 • [{lic}](LICENSE) |
| **`en`** | Proudly produced by a [Latin American travesti](https://travesti.2lp.in/en/) 🏳️‍⚧️ :) | {title} v{ver} • by [Lumen Pink](https://lumen.pink) • Brazil 🇧🇷 • [{lic}](LICENSE) |
| **`es`** | Orgullosamente producido por una [travesti latinoamericana](https://travesti.2lp.in/es/) 🏳️‍⚧️ :) | {title} v{ver} • por [Lumen Pink](https://lumen.pink) • Brasil 🇧🇷 • [{lic}](LICENSE) |
| **`fr`** | Fièrement produit par une [travesti latino-américaine](https://travesti.2lp.in/fr/) 🏳️‍⚧️ :) | {title} v{ver} • par [Lumen Pink](https://lumen.pink) • Brésil 🇧🇷 • [{lic}](LICENSE) |
| **`de`** | Mit Stolz entwickelt von einer [lateinamerikanischen Travesti](https://travesti.2lp.in/de/) 🏳️‍⚧️ :) | {title} v{ver} • von [Lumen Pink](https://lumen.pink) • Brasilien 🇧🇷 • [{lic}](LICENSE) |
| **`it`** | Orgogliosamente prodotto da una [travesti latinoamericana](https://travesti.2lp.in/it/) 🏳️‍⚧️ :) | {title} v{ver} • da [Lumen Pink](https://lumen.pink) • Brasile 🇧🇷 • [{lic}](LICENSE) |
| **`ru`** | С гордостью создано [латиноамериканской travesti](https://travesti.2lp.in/ru/) 🏳️‍⚧️ :) | {title} v{ver} • от [Lumen Pink](https://lumen.pink) • Бразилия 🇧🇷 • [{lic}](LICENSE) |
| **`zh`** | 由一位[拉美 travesti](https://travesti.2lp.in/zh/) 自豪制作 🏳️‍⚧️ :) | {title} v{ver} • 来自 [Lumen Pink](https://lumen.pink) • 巴西 🇧🇷 • [{lic}](LICENSE) |
| **`ja`** | [ラテンアメリカの travesti](https://travesti.2lp.in/ja/) によって誇りを持って制作されました 🏳️‍⚧️ :) | {title} v{ver} • 制作: [Lumen Pink](https://lumen.pink) • ブラジル 🇧🇷 • [{lic}](LICENSE) |
| **`ar`** | صُنع بكل فخر بواسطة [travesti أمريكية لاتينية](https://travesti.2lp.in/ar/) 🏳️‍⚧️ :) | {title} v{ver} • بواسطة [Lumen Pink](https://lumen.pink) • البرازيل 🇧🇷 • [{lic}](LICENSE) |

---

## 3. Publicação Web na Borda (`*.2lp.in`) e Empacotamento Dioxus

### 3.1 Hospedagem Estática via Cloudflare Workers (`wrangler.jsonc`)
Toda vitrine pública (`./site`) é publicada sob o domínio `<slug>.2lp.in` através de `wrangler.jsonc` na raiz:

```jsonc
{
  "$schema": "node_modules/wrangler/config-schema.json",
  "name": "<slug>",
  "compatibility_date": "2026-01-01",
  "routes": [
    {
      "pattern": "<slug>.2lp.in",
      "custom_domain": true
    }
  ],
  "assets": {
    "directory": "./site"
  }
}
```

* **Redundância Estática:** A fila `deploy` da esteira publica simultaneamente o snapshot em branch órfã stateless `pages` para o Codeberg Pages (`https://lumenpink.codeberg.page/<slug>/`).

### 3.2 Identificador de Bundle Canônico (`Dioxus.toml`)
Para aplicações gráficas e mobile construídas com Dioxus 0.8, o namespace reverso e o publisher são fixados como:

```toml
[application]
name = "<slug>"
default_platform = "web"

[bundle]
identifier = "in.2lp.<slug>"
publisher = "Lumen Pink"
icon = ["site/assets/icon.png"]
```

---

## 4. Distribuição e Catálogo da Banca (`banca.2lp.in` e Cloudflare R2)

A autoridade central de distribuição de executáveis e pacotes definida na Seção 4 do perfil Kollontai opera sob o modelo **Produtor–Curador Desacoplado** servido via Cloudflare R2 e publicado na borda:

* **Topologia Física de Storage (Cloudflare R2):**
  Todos os artefatos são mantidos sob o prefixo:
  ```text
  banca-artifacts/{app}/{channel}/
  ```
  - Artefatos versionados e hashes: `{app}-{versao}-{target}.{ext}` e `.sha256`.
  - Cópias físicas de aliases sobrescritas: `{app}-latest-{target}.{ext}` (em `stable`) ou `{app}-dev-latest-{target}.{ext}` (em `dev`), juntamente com seus arquivos `.sha256`.
* **Cotas Operacionais e Política de Retenção Estrita:**
  - **Canal `stable` (Versão Recomendada):** Retém estritamente as **5 versões mais recentes** (`x.y.z`), com alias canônico `*-latest.*`.
  - **Canal `dev` (Versão Experimental):** Retém estritamente as **10 versões mais recentes** (`x.y.z-dev.{n}+{YYYYMMDD}.{commit}`), com alias canônico `*-dev-latest.*`.
  - Artefatos e hashes `.sha256` excedentes que ultrapassarem essas cotas são permanentemente purgados do bucket R2 pela esteira da Banca.
* **Transição para o Gossnab e Banimento de Manifestos Estáticos:**
  - **Banimento de `catalog.json` e `kombinat.toml` em Repositórios:** É expressamente banido manter arquivos estáticos de manifesto como `catalog.json` ou `kombinat.toml` dentro de repositórios de código.
  - **SSOT Centralizado Vivo (`gossnab`):** O serviço vivo **`gossnab`** (`https://gossnab.2lp.in/api/v1`) é a Fonte Única da Verdade para metadados de release, canais, inventário de artefatos e versionamento do ecossistema.
  - Durante o período de transição, a Banca gera e sincroniza views de compatibilidade retroativa em `https://banca.2lp.in/downloads/<slug>/index.json` consumindo a API viva do Gossnab.
* **Comando Universal de Instalação e Vitrine:**
  - Vitrine: `https://banca.2lp.in/downloads/#<slug>` (design brutalista/cyberpunk em Vanilla HTML/CSS/JS com cópia rápida de comando `curl` e tabela de hashes SHA-256).
  ```sh
  curl -sSf https://banca.2lp.in/downloads/<slug>/install.sh | sh
  ```
* **Protocolo de Auto-Update XDG:** Os binários CLI consultam diretamente a API viva do Gossnab (`https://gossnab.2lp.in/api/v1/apps/<slug>/update`) para o protocolo de auto-atualização (`<slug> check-update` / `<slug> update`), com fallback para `index.json`.
* **Tolerância Zero a Links Quebrados:** Se qualquer artefato declarado (novo ou histórico ativo nas cotas 5/10) falhar na verificação de rede ou divergência de checksum SHA-256, o deploy na Cloudflare é abortado imediatamente e um ticket automático é aberto na forja do projeto de origem (`POST /api/v1/repos/{repo_slug}/issues`).

---

## 5. Segregação Arquitetural: Gambi (CI) vs. Kombinat (CD)

Para eliminar atritos operacionais e garantir desacoplamento total entre integração contínua e entrega contínua, o perfil Lilya estabelece fronteiras estritas de responsabilidade:

### 5.1 O Papel da Gambi: Integração Contínua (CI)
* **Escopo:** Compilação, execução de testes unitários e de integração, verificação de linters/formatação e empacotamento.
* **Publicação Efêmera de Staging:** A esteira de CI em `gambi.2lp.in` publica artefatos compilados exclusivamente no Generic Package Registry da própria forja via `PUT /api/packages/{owner}/generic/{package_name}/{version}/{filename}`.
* **Isolamento de Produção:** A CI da Gambi **NÃO** efetua conexões SSH em servidores de produção nem realiza uploads diretos para storages externos (Cloudflare R2 / Banca / Pratilera).

### 5.2 O Papel do Kombinat: Entrega Contínua (CD) e Propagação em Cascata
* **Ingestão e Despacho de Binários:** O Kombinat consome os artefatos efêmeros de staging da Gambi, valida assinaturas e hashes SHA-256, e realiza a publicação externa definitiva na Banca (Cloudflare R2) e Pratilera.
* **Purga Imediata de Staging:** Imediatamente após a confirmação do despacho com sucesso, o Kombinat remove os artefatos efêmeros do registro da Gambi emitindo `DELETE /api/packages/{owner}/generic/{package_name}/{version}/{filename}`, mantendo a forja limpa e sem acúmulo de storage.
* **Propagação Automatizada em Cascata:** Ao detectar uma nova versão canônica de desenvolvimento publicada a montante (upstream), o Kombinat orquestra a propagação dos bumps exatos nos manifestos de compilação dos módulos a jusante (downstream). A fixação em versões canônicas exatas é recomendada, sem impor qualquer trava mandatória contra o uso prático de aliases.
* **Entrega Contínua de Servidores (CD):** O Kombinat assume a responsabilidade de acionar as atualizações dos serviços conteinerizados via SSH nos servidores (`obizero`, `obione`, `obitwo`), disparando `docker compose pull && docker compose up -d` e restart de containers.

### 5.3 Webhooks Multi-Forja e Loop de Reconciliação no Gossnab
* **Ingestão Multi-Forja:** Repositórios registrados no Gossnab possuem webhooks ativos apontando para o Gossnab (suportando Forgejo/Gambi, GitHub e SourceHut).
* **Loop de Reconciliação Anti-Drift:** Para mitigar falhas ou perdas de entrega de webhooks sem retentativa nativa, o Gossnab executa uma rotina periódica de conciliação a cada $N$ minutos, utilizando cabeçalhos condicionais HTTP (`ETag` e `If-Modified-Since`) para evitar requisições redundantes e rate limits.

---

## 6. Entrega Contínua de Servidores (CD em `obizero`, `obione` e `obitwo`)

Para serviços conteinerizados autohospedados, o perfil Lilya opera com **Continuous Deployment (CD) Determinístico** orquestrado pelo **Kombinat CD** (eliminando etapas de SSH e deploy direto da esteira de CI), com banimento total do Watchtower.

### 5.1 Padrão de Orquestração e Healthcheck Obrigatório
Todos os serviços residem em `~/stacks/<slug>/compose.yaml` (estritamente com extensão `.yaml`, sendo banida a extensão `.yml`) e declaram probe ativa de saúde:

```yaml
services:
  app:
    image: lumenpink/<slug>:${APP_VERSION:-latest}
    user: "1000:1000"
    restart: unless-stopped
    healthcheck:
      test: ["CMD-SHELL", "curl -fsS http://127.0.0.1:<port>/healthz || exit 1"]
      interval: 30s
      timeout: 3s
      retries: 3
      start_period: 10s
```

### 5.2 Topologia de Deploy por Nó de Infraestrutura

#### 1. Nó Central de Execução (`obizero` — Deploy Local via Socket)
* O `forgejo-runner` executa no host `obizero` com o socket Docker montado.
* A imagem recém-compilada na fila `build` já reside no daemon local (`lumenpink/<slug>:<version>`).
* O deploy ocorre sem tráfego externo redundante:
  ```sh
  docker compose -f /home/lumen/stacks/<slug>/compose.yaml up -d --no-build <slug>
  ```
* **Verificação Síncrona de Saúde e Rollback Automático (15s):**
  A esteira monitora o status do container por até 15 segundos. Caso o estado não atinja `healthy` (ou `running` estável), a esteira captura `docker logs --tail 50 <slug>`, reverte imediatamente para a tag estável anterior, executa `docker image prune -f` e falha o pipeline com código não-zero.

#### 2. Nós de Borda Remotos (`obione` e `obitwo` — Deploy via Mesh VPN Tailscale)
* Para serviços distribuídos nos nós de borda (acessíveis via rede privada WireGuard/Tailscale ou expostos via Cloudflare Tunnel `cloudflared` em `app.<slug>.2lp.in`):
* A esteira publica a imagem e aciona a atualização remotamente via SSH sobre a malha Tailscale:
  ```sh
  ssh -o StrictHostKeyChecking=accept-new lumen@<no-tailscale> \
    "docker compose -f ~/stacks/<slug>/compose.yaml pull && \
     docker compose -f ~/stacks/<slug>/compose.yaml up -d && \
     docker image prune -f"
  ```

### 6.3 Mandato de Conteinerização Universal em Stacks e Topologia de Rede
1. **Banimento de Daemons no Host:** É estritamente proibido rodar serviços, APIs ou daemons de backend diretamente instalados no sistema operacional hospedeiro dos servidores (`obizero`, `obione`, `obitwo`). Todo serviço roda isolado via `~/stacks/<slug>/compose.yaml`.
2. **Topologia de Segurança e Ingress:**
   - **Ponto Único de Ingress Público:** Apenas o Gossnab possui porta pública exposta (via Cloudflare Tunnel / Gateway reverso) para recepção de webhooks das forjas externas e atendimento à Banca.
   - **Isolamento do Kombinat:** O Kombinat opera estritamente confinado na rede privada interna, sem portas abertas para a internet pública, interagindo como cliente ativo unidirecional para tarefas de orquestração, build, assinatura, publicação externa e deploy via SSH.

---

## 7. Sobreposição de Regras de Conformidade (`lilya_rules.json`)

Quando o `cccp sync` compila as regras para um repositório sob o perfil Lilya, as seguintes parametrizações específicas sobrepõem (*deep merge*) o `rules.json` base da GOST:

```json
{
  "extends": "GOST-2.0.0",
  "profile": "lilya",
  "rules_override": [
    {
      "id": "GOST-09-SITE-SECURITY-HEADERS",
      "require_footer_links": [
        "https://travesti.2lp.in/",
        "https://lumen.pink"
      ]
    },
    {
      "id": "GOST-10-CI-HERMETIC-SINGLE-FILE",
      "allowed_uses_domains": ["gambi.2lp.in"],
      "allowed_run_domains": ["gambi.2lp.in", "crates.2lp.in", "banca.2lp.in"]
    },
    {
      "id": "GOST-25-REMOTES-AND-INBOX-QUARANTINE",
      "required_origin_pattern": "^git@gambi\\.2lp\\.in:lumen/[a-z0-9-]+\\.git$",
      "required_mirrors": {
        "codeberg": "https://codeberg.org/lumenpink/<slug>.git",
        "github": "https://github.com/lumenpink/<slug>.git"
      }
    }
  ]
}
```

---

## 7. Conformidade Operacional de Interfaces CLI (*Post-Build*)

Todo utilitário CLI ou ferramenta de linha de comando voltada a desenvolvedores e usuárias finais, quando submetido à validação no escopo canônico `post-build`, DEVE:
1. **Flag de Ajuda Informativa (`--help`):** Responder com código 0 e texto estruturado de instruções mínimas de operação.
2. **Fidelidade Canônica de Versão (`--version`):** Responder com código 0 e texto contendo a versão exata do manifesto `VERSION` ou `Cargo.toml`.
3. **Ciclo de Vida e Atualização:** Quando aplicável ao canal de distribuição, manter subcomando canônico de verificação ou atualização (`update` / `self-update`) integrado aos canais da Banca.

---

<!-- ===================================================================== -->
<!-- CAMADA LOCAL AUTÔNOMA: CUSTOM.md (Precedência Máxima do Repositório) -->
<!-- ===================================================================== -->

# Local GOST Directives & Overrides

## Language & Documentation Standards
- **Mandatory Language:** All documentation (`README.md`, docstrings, module documentation, code comments), metadata (`Cargo.toml`), commit messages, tickets, and forge issues **MUST** be written in English.
- **Terminology Guidelines:** Do NOT use colloquial or ideologically charged terms such as *"sovereign"*, *"soberano"*, or equivalents. Use standard, professional engineering terminology (e.g., *"high-performance"*, *"deterministic"*, *"robust"*, *"resilient"*).

