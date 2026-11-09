# Especificação Técnica: Graus Epistêmicos de Rima e Detector de Rimas Clichê

**Ticket:** #20  
**Branch:** `feat/20-graus-epistemicos-de-rima-e-detector-de-cliches`  
**Escopo:** `joaju-lirica` (`src/morphology/`: `classifier.rs`, `cliches.rs`, `pos_mask.rs`, `dafsa.rs`, `lookback.rs`)  
**Status:** Em Desenvolvimento (Fase 3: Spec)

---

## 1. Visão Geral e Princípios de Projeto

Este ciclo aperfeiçoa o classificador morfológico de rimas com duas inovações fundamentais:

1. **Conservadorismo e Graus Epistêmicos de Certeza:**
   - **`Rica` e `Pobre`:** Reservadas exclusivamente para **100% de certeza** morfológica e sintática.
   - **`ProvavelRica` e `ProvavelPobre`:** Diagnósticos com rastreabilidade explicativa para termos com homografia secundária atestada em dicionário (ex.: *ouro*, *hora*, *palma*, *violas* frente a verbos arcaicos ou hipotéticos) e para pares de particípios passados (*amada* x *desejada*).
   - **`Rara`:** Diferenciada de forma inequívoca da `Preciosa`. Enquanto a `Preciosa` reside em ênclises e mesóclises assimétricas (*estrela* x *vê-la*), a `Rara` decorre de terminações com escassez fonológica atestada no léxico (ex.: terminações em *-urbe*, *-austro*, *-ésbio*, *-íx* ou proparoxítonas eruditas).

2. **Detecção Estilística de Rimas Clichê:**
   - A análise de rimas introduz uma flag ortogonal `eh_cliche: bool` e `motivo_cliche: Option<String>`.
   - Identifica pares e feixes saturados pela história literária (ex.: *amor* + *dor*, *mar* + *amar*, *coração* + *paixão* + *ilusão* + *solidão*, *vento* + *pensamento*, *flor* + *amor*, *céu* + *véu*, *vida* + *querida*).

---

## 2. Tipagem e Álgebra de Estados

### 2.1. Extensão de `QualidadeRima`

```rust
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum QualidadeRima {
    // 100% de Certeza Categórica
    Pobre,
    Rica,
    Preciosa,
    Rara,

    // Diagnósticos Epistêmicos com Hipótese Prevalente
    ProvavelPobre {
        justificativa: String,
    },
    ProvavelRica {
        justificativa: String,
    },

    // Indeterminação Estrita
    Indeterminada {
        motivo: IndeterminadoMotivo,
    },
}
```

### 2.2. Estrutura do Par Avaliado (`ParRimaAvaliado`)

```rust
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParRimaAvaliado {
    pub versos: (usize, usize),
    pub tokens: (String, String),
    pub classificacao: QualidadeRima,
    pub auditoria: JustificativaAuditoria,
    pub eh_cliche: bool,
    pub motivo_cliche: Option<String>,
}
```

---

## 3. Catálogo Canônico de Rimas Clichê

O módulo `src/morphology/cliches.rs` conterá a tabela estática com as duplas e feixes semântico-fonéticos clássicos:
1. *amor* x *dor*, *flor*, *clamor*, *ardor*, *fervor*
2. *mar* x *amar*, *navegar*, *luar*, *olhar*, *cantar*
3. *coração* x *paixão*, *ilusão*, *solidão*, *canção*, *razão*, *emoção*
4. *céu* x *véu*, *troféu*, *léu*
5. *vento* x *pensamento*, *sentimento*, *tormento*, *momento*
6. *vida* x *querida*, *perdida*, *despedida*, *ferida*, *lida*
7. *luz* x *seduz*, *conduz*, *cruz*
8. *noite* x *açoite*
9. *alma* x *palma*, *calma*
10. *tristeza* x *beleza*, *natureza*
11. *paz* x *capaz*, *traz*
12. *sonho* x *risonho*, *medonho*
13. *chama* x *ama*, *drama*, *cama*
14. *mundo* x *profundo*, *segundo*
15. *esperança* x *lembrança*, *criança*
16. *dia* x *alegria*, *fria*
17. *canto* x *pranto*, *encanto*
18. *destino* x *menino*, *desatino*

A verificação é bidirecional e simétrica: `verificar_cliche(w1, w2)` é equivalente a `verificar_cliche(w2, w1)`.

---

## 4. Critérios de Aceite

1. `QualidadeRima` atualizada com suporte completo a serde, preservando retrocompatibilidade com testes existentes.
2. `eh_cliche` reportado como `true` para todas as rimas do catálogo canônico de clichês (ex.: *amor* x *dor*, *mar* x *amar*).
3. `ProvavelRica` e `ProvavelPobre` emitidas adequadamente no corpus, mantendo `Rica` e `Pobre` puras apenas para 100% de certeza.
4. Suíte do Gabarito Canônico e testes de mutações mantidos em 100% verdes.
5. Conformidade Gosplan estrita.
