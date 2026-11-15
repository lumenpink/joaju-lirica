# Especificação Técnica: Sinalefa Terciária com Clítico Átono Intermediário

**Ticket:** #21  
**Branch:** `feat/21-sinalefa-terciaria-com-clitico-atono-intermediario`  
**Escopo:** `joaju-lirica` (`src/scansion.rs`)  
**Status:** Concluído e Validado (Fases 1 a 8)

---

## 1. Contexto e Motivação

Na escansão poética de versos em língua portuguesa, a sinalefa canônica é frequentemente binária (fusão de duas sílabas vocálicas contíguas entre palavras). Contudo, a tradição prosódica e poética luso-brasileira (cf. Manuel Said Ali, Antônio Feliciano de Castilho e Olavo Bilac) documenta com clareza a **sinalefa terciária** (ou crase e ecthlipse de três núcleos vocálicos contínuos) quando o elemento central é um **clítico átono monossilábico** (*a*, *o*, *e*, *se*, *me*, *de*, *que*) atuando como ponte prosódica entre a vogal átona final de uma palavra e a vogal átona inicial da seguinte.

### O Caso de Quebra
No verso:
> *"O horizonte da serra a alvorecer"*

A articulação natural é um **Decassílabo Martelo** com 10 sílabas poéticas e acentos em 3, 6 e 10:
- `1:O_ho` (sinalefa)
- `2:ri`
- `3:[zon]` (tônica)
- `4:te`
- `5:da`
- `6:[ser]` (tônica)
- `7:ra_a_al` (sinalefa terciária por crase múltipla de vogais átonas)
- `8:vo`
- `9:re`
- `10:[cer]` (tônica final)

No motor atual, a sílaba `ra` (de *serra*) funde-se com `a`, mas como `ra` não é um clítico átono monossilábico, o laço de fusão interrompe-se precocemente (`atual.eh_clitico_atono` era falso), deixando `al-` isolado em falso hiato e inflando o verso para 11 sílabas (*hendecassílabo* espúrio com tônicas em 3, 6 e 11).

---

## 2. Árvore de Decisão e Racional Técnico (Para Natasha)

### Decisão 1: Critério para Continuidade de Sinalefa além da Segunda Sílaba
- **Opção A (Descartada):** Permitir fusão irrestrita de qualquer cadeia vocálica até 3 ou 4 sílabas.
  - *Problema:* Criaria sinéreses forçadas e falsos positivos grotescos em palavras plenas contíguas.
- **Opção B (Descartada):** Criar exceções ad-hoc ou regex por palavra (ex.: casar *"serra a alvorecer"*).
  - *Problema:* Gambiarra frágil que não resolve outros versos análogos e deixa dívida técnica para a Natasha.
- **Opção C (Escolhida):** Permitir que o laço de fusão continue para uma terceira sílaba quando a sílaba absorvida for um **clítico átono monossilábico** (`prox.eh_clitico_atono && !prox.tonica && !atual.tonica`).
  - *Racional:* Se a sílaba do meio é um monossílabo átono áfono (artigo, preposição, conjunção), ela funciona como elemento de ligação articulatória entre a vogal precedente e a vogal seguinte. A trava existente (`bloqueia_terciaria_por_tonica: atual.texto.contains('_') && prox.tonica`) garante que a terceira sílaba jamais seja absorvida se for tônica, impedindo destruição de apoios métricos vitais.

### Decisão 2: Proteção Absoluta contra Absorção de Tônicas
- Se a terceira sílaba contiver acento tônico (ex.: *"serra a águia"* ou *"terra e alma"*), a trava `bloqueia_terciaria_por_tonica` rejeita a fusão com `á-` ou `al-` tônico, preservando o hiato obrigatório e a contagem correta.

---

## 3. Matriz de Testes e Validação

1. **Caso Canônico Positivo (Decassílabo):**
   - *"O horizonte da serra a alvorecer"* -> exatamente 10 sílabas poéticas, tônicas em [3, 6, 10], classificação Decassílabo (Martelo).
   - Sílaba resultante: `ra_a_al`.
2. **Outros Casos com Clítico Ponte:**
   - *"Deixou a terra e a esperança ao longe"* -> fusão em `e_a_es` e preservação métrica.
3. **Contraprova (Tônica Bloqueia Terciária):**
   - *"A serra a águia viu passar"* -> `ra_a` elide, mas `á` de *águia* permanece em hiato, sem fusão ternária.
4. **Regressão Zero no Gabarito Canônico:**
   - Execução integral dos testes de `gabarito_canonico_tests.rs` (50 poemas, 100% de conformidade preservada).
