# joaju-lirica

Motor determinístico de escansão poética e análise rítmica para versos em língua portuguesa, construído em Rust.

## 🏛️ Gabarito Canônico (*Ground Truth*)

O projeto rejeita métricas de aproximação probabilística ou estatística para formas fechadas da poesia lusófona. Toda a validação da suíte é confrontada verso a verso contra o **Gabarito Canônico** (`tests/corpus/gabarito_canonico.json`), um corpus histórico de 51 poemas auditados (do Trovadorismo ao Modernismo):

- **Verificação Exata por Verso**: Cada verso possui o número exato de sílabas métricas canônicas (*Contagem de Castilho* até a última tônica).
- **Tratamento Fonológico de Clíticos**: Contrações átonas indivisíveis (`ao`, `aos`, `pro`, `pros`, `pra`, `pras`) e fusão de cadeias de clíticos contíguos (`Transforma-se o amador` $\to$ `se_o_a`).
- **Sinérese Pós-Tônica Canônica**: Encontros vocálicos após acento gráfico (`etéreo`, `mágoa`, `glória`, `níveo`) tratados como ditongos crescentes por padrão métrico.
- **Formas Poéticas Canônicas**: Diagnóstico e inferência de Sonetos Petrarquianos e Shakespearianos, Oitavas Camonianas, Décimas Espinelas, Haicais e Redondilhas.
- **Rimas Ricas e Pobres**: Classificação morfológica e esquemas rimáticos estritos (ABBA, CDCD, etc.).

## 📋 Conformidade GOST

Este projeto adota a norma **[GOST](https://codeberg.org/lumenpink/gost)** (Gosplan Standard for Technical Software).
- **Versão GOST:** `1.5.0`
- **Linguagens declaradas:** `rust,shell`
- **Licença:** `Apache-2.0`

## 🚀 Como Iniciar

```bash
# Clonar o repositório
git clone https://codeberg.org/lumenpink/joaju-lirica.git
cd joaju-lirica

# Executar a suíte de testes com o Gabarito Canônico
cargo test --test gabarito_canonico_tests

# Verificar conformidade estrita Gosplan
cccp check --strict
```

## 📄 Licença

Distribuído sob a licença [Apache-2.0](./LICENSE).
