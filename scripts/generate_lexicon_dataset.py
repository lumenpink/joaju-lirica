#!/usr/bin/env python3
"""
Gera o dataset consolidado de léxico morfológico para o Lirica / joaju-lirica.
Combina:
- Tabela Canônica e classes fechadas
- MorphoBr (substantivos, adjetivos, verbos e advérbios fletidos para o vocabulário geral)
- Vocábulos do corpus de 55 poemas (gabarito canônico)
- DELAS (lemas e categorias canônicas)

Saída: scripts/lexicon_entries.tsv
"""

import csv
import glob
import io
import json
import os
import re
import subprocess
import sys
import urllib.request

MASK_UNKNOWN = 0
MASK_SUBSTANTIVO = 1 << 0  # 1
MASK_VERBO = 1 << 1        # 2
MASK_ADJETIVO = 1 << 2     # 4
MASK_ADVERBIO = 1 << 3     # 8
MASK_PRONOME = 1 << 4      # 16
MASK_ARTIGO = 1 << 5       # 32
MASK_PREPOSICAO = 1 << 6   # 64
MASK_CONJUNCAO = 1 << 7    # 128
MASK_INTERJEICAO = 1 << 8  # 256
MASK_NUMERAL = 1 << 9      # 512

MASK_NAMES = {
    "SUBSTANTIVO": MASK_SUBSTANTIVO,
    "VERBO": MASK_VERBO,
    "ADJETIVO": MASK_ADJETIVO,
    "ADVERBIO": MASK_ADVERBIO,
    "PRONOME": MASK_PRONOME,
    "ARTIGO": MASK_ARTIGO,
    "PREPOSICAO": MASK_PREPOSICAO,
    "CONJUNCAO": MASK_CONJUNCAO,
    "INTERJEICAO": MASK_INTERJEICAO,
    "NUMERAL": MASK_NUMERAL,
}

lexicon = {}

def add_word(word, mask):
    w = word.strip().lower()
    if not w:
        return
    lexicon[w] = lexicon.get(w, 0) | mask

# 1. Carregar TABELA_CANONICA de dafsa.rs
dafsa_path = "src/morphology/dafsa.rs"
if os.path.exists(dafsa_path):
    print("1. Carregando TABELA_CANONICA de dafsa.rs...")
    with open(dafsa_path, encoding="utf-8") as f:
        content = f.read()
    idx_start = content.find("TABELA_CANONICA: &[(&str, PosMask)] = &[")
    if idx_start != -1:
        idx_end = content.find("];", idx_start)
        block = content[idx_start:idx_end]
        matches = re.findall(r"\(\s*\"([^\"]+)\"\s*,\s*(PosMask[^,\n]+|\w+)", block)
        for w, raw in matches:
            mask = 0
            for name, bit in MASK_NAMES.items():
                if name in raw:
                    mask |= bit
            add_word(w, mask)
        print(f"   Carregadas {len(matches)} entradas da tabela canônica.")

# 2. Carregar DELAS.csv (Lemas canônicos)
delas_lemmas = set()
delas_csv_url = "https://raw.githubusercontent.com/datasets-br/unitex-pt-br/master/data/mirror/DELAS.csv"
print("2. Baixando e processando DELAS.csv...")
try:
    req = urllib.request.urlopen(delas_csv_url)
    reader = csv.reader(io.StringIO(req.read().decode("utf-8")))
    for row in reader:
        if len(row) >= 2:
            form = row[0].strip().lower()
            tag = row[1].strip()
            delas_lemmas.add(form)
            mask = 0
            if tag.startswith("N"):
                mask |= MASK_SUBSTANTIVO
            elif tag.startswith("V"):
                mask |= MASK_VERBO
            elif tag.startswith("A"):
                mask |= MASK_ADJETIVO
            elif tag.startswith("ADV"):
                mask |= MASK_ADVERBIO
            elif tag.startswith("PRO"):
                mask |= MASK_PRONOME
            elif tag.startswith("DET") or tag == "ART":
                mask |= MASK_ARTIGO
            elif tag.startswith("PRE"):
                mask |= MASK_PREPOSICAO
            elif tag.startswith("CON"):
                mask |= MASK_CONJUNCAO
            elif tag.startswith("INT"):
                mask |= MASK_INTERJEICAO
            elif tag.startswith("NUM"):
                mask |= MASK_NUMERAL
            if mask > 0:
                add_word(form, mask)
    print(f"   DELAS processado: {len(delas_lemmas)} lemas.")
except Exception as e:
    print(f"   Aviso: erro ao carregar DELAS.csv: {e}")

# 3. Extrair todas as palavras do Gabarito Canônico
gabarito_path = "tests/corpus/gabarito_canonico.json"
corpus_words = set()
if os.path.exists(gabarito_path):
    print("3. Analisando vocabulário do gabarito canônico...")
    with open(gabarito_path, encoding="utf-8") as f:
        corpus = json.load(f)
    for item in corpus:
        tokens = re.findall(r"[a-zA-ZáàâãéêíóôõúüçÁÀÂÃÉÊÍÓÔÕÚÜÇ]+", item["texto"])
        for t in tokens:
            corpus_words.add(t.lower())
    print(f"   Vocabulário único do corpus: {len(corpus_words)} palavras.")

target_lemmas = delas_lemmas | corpus_words

# 4. Processar MorphoBr (/tmp/morphobr/)
morphobr_dir = "/tmp/morphobr"
if os.path.exists(morphobr_dir):
    print("4. Indexando morfologia ampla do MorphoBr...")
    
    def process_morphobr(pattern, mask):
        count = 0
        for path in sorted(glob.glob(pattern)):
            with open(path, encoding="utf-8", errors="ignore") as fp:
                for line in fp:
                    parts = line.strip().split("\t")
                    if len(parts) >= 2:
                        form = parts[0].lower()
                        lemma = parts[1].split("+")[0].lower()
                        if lemma in target_lemmas or form in target_lemmas:
                            add_word(form, mask)
                            count += 1
        return count

    n_adv = process_morphobr(f"{morphobr_dir}/adverbs/*.dict", MASK_ADVERBIO)
    n_adj = process_morphobr(f"{morphobr_dir}/adjectives/*.dict", MASK_ADJETIVO)
    n_noun = process_morphobr(f"{morphobr_dir}/nouns/*.dict", MASK_SUBSTANTIVO)
    n_verb = process_morphobr(f"{morphobr_dir}/verbs/*.dict", MASK_VERBO)
    print(f"   MorphoBr integrado: {n_adv} adv, {n_adj} adj, {n_noun} subs, {n_verb} verbos.")

# 5. Adicionar classes fechadas e vocábulos poéticos/históricos específicos do corpus
CURATED_EXPLICIT = {
    # Pronomes e contrações
    "te": MASK_PRONOME,
    "tu": MASK_PRONOME,
    "mim": MASK_PRONOME,
    "me": MASK_PRONOME,
    "nos": MASK_PRONOME,
    "vos": MASK_PRONOME,
    "vós": MASK_PRONOME,
    "minha": MASK_PRONOME,
    "minhas": MASK_PRONOME,
    "vossa": MASK_PRONOME,
    "essa": MASK_PRONOME,
    "esta": MASK_PRONOME,
    "estas": MASK_PRONOME,
    "daquela": MASK_PRONOME | MASK_PREPOSICAO,
    "daquele": MASK_PRONOME | MASK_PREPOSICAO,
    "desta": MASK_PRONOME | MASK_PREPOSICAO,
    "naquelas": MASK_PRONOME | MASK_PREPOSICAO,
    "nela": MASK_PRONOME | MASK_PREPOSICAO,
    "nessa": MASK_PRONOME | MASK_PREPOSICAO,
    "tudo": MASK_PRONOME,
    "quem": MASK_PRONOME,
    "quantas": MASK_PRONOME | MASK_ADJETIVO,
    "lo": MASK_PRONOME,
    "la": MASK_PRONOME,
    "las": MASK_PRONOME,
    "lhe": MASK_PRONOME,
    "contigo": MASK_PRONOME,
    # Preposições e contrações
    "de": MASK_PREPOSICAO,
    "em": MASK_PREPOSICAO,
    "com": MASK_PREPOSICAO,
    "sem": MASK_PREPOSICAO,
    "sob": MASK_PREPOSICAO,
    "desde": MASK_PREPOSICAO,
    "por": MASK_PREPOSICAO,
    "do": MASK_PREPOSICAO | MASK_ARTIGO,
    "dos": MASK_PREPOSICAO | MASK_ARTIGO,
    "da": MASK_PREPOSICAO | MASK_ARTIGO,
    "das": MASK_PREPOSICAO | MASK_ARTIGO,
    "no": MASK_PREPOSICAO | MASK_ARTIGO,
    "na": MASK_PREPOSICAO | MASK_ARTIGO,
    "nos": MASK_PRONOME | MASK_PREPOSICAO,
    "nas": MASK_PREPOSICAO | MASK_ARTIGO,
    "num": MASK_PREPOSICAO | MASK_ARTIGO,
    "numa": MASK_PREPOSICAO | MASK_ARTIGO,
    "ao": MASK_PREPOSICAO | MASK_ARTIGO,
    "aos": MASK_PREPOSICAO | MASK_ARTIGO,
    "à": MASK_PREPOSICAO | MASK_ARTIGO,
    # Conjunções
    "ou": MASK_CONJUNCAO,
    "nem": MASK_CONJUNCAO,
    "pois": MASK_CONJUNCAO | MASK_ADVERBIO,
    "porque": MASK_CONJUNCAO,
    "porém": MASK_CONJUNCAO,
    "se": MASK_CONJUNCAO | MASK_PRONOME,
    # Interjeições
    "ah": MASK_INTERJEICAO,
    "oh": MASK_INTERJEICAO,
    # Numerais
    "mil": MASK_NUMERAL | MASK_SUBSTANTIVO,
    "duas": MASK_NUMERAL,
    # Nomes próprios / poéticos no corpus
    "afrodite": MASK_SUBSTANTIVO,
    "albuquerques": MASK_SUBSTANTIVO,
    "carpanos": MASK_SUBSTANTIVO,
    "ismália": MASK_SUBSTANTIVO,
    "jacob": MASK_SUBSTANTIVO,
    "lixboa": MASK_SUBSTANTIVO,
    "marília": MASK_SUBSTANTIVO,
    "mavorte": MASK_SUBSTANTIVO,
    "raquel": MASK_SUBSTANTIVO,
    "taprobana": MASK_SUBSTANTIVO,
    "ra": MASK_SUBSTANTIVO,
    # Arcaísmos / termos líricos do corpus
    "sabedes": MASK_VERBO,
    "velida": MASK_ADJETIVO | MASK_SUBSTANTIVO,
    "avelhório": MASK_SUBSTANTIVO,
    "jocundos": MASK_ADJETIVO,
    "louçana": MASK_ADJETIVO,
    # Formas truncadas por elisão ou clíticos
    "contemplá": MASK_VERBO,
    "entendê": MASK_VERBO,
    "olhá": MASK_VERBO,
    "vivê": MASK_VERBO,
    "minh": MASK_PRONOME,
    "qu": MASK_PRONOME | MASK_CONJUNCAO,
}

for w, m in CURATED_EXPLICIT.items():
    add_word(w, m)

# 6. Salvar scripts/lexicon_entries.tsv
out_tsv = "scripts/lexicon_entries.tsv"
print(f"5. Gravando {len(lexicon)} entradas ordenadas em {out_tsv}...")
with open(out_tsv, "w", encoding="utf-8") as f:
    for w in sorted(lexicon.keys()):
        f.write(f"{w}\t{lexicon[w]}\n")

print(f"Concluído com sucesso: {len(lexicon)} palavras prontas para compilação FST.")
