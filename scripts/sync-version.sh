#!/usr/bin/env bash
# scripts/sync-version.sh — Sincroniza a versão canônica do CCCP com o Cargo.toml
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"

TARGET_VERSION="${1:-}"

if [[ -z "${TARGET_VERSION}" ]]; then
    if [[ -f "${REPO_ROOT}/VERSION" ]]; then
        RAW_VERSION="$(cat "${REPO_ROOT}/VERSION" | tr -d '[:space:]')"
        # Remove sufixos -dev e build metadata para obter a versão canônica do pacote
        TARGET_VERSION="${RAW_VERSION%%-*}"
        TARGET_VERSION="${TARGET_VERSION%%+*}"
    else
        echo "Erro: Arquivo VERSION não encontrado e nenhuma versão foi especificada." >&2
        exit 1
    fi
fi

echo "Sincronizando Cargo.toml para a versão canônica do CCCP: ${TARGET_VERSION}"

# Atualiza a chave version = "..." na seção [package] de forma portável
sed -i.bak -E '0,/^version = ".*"/s/^version = ".*"/version = "'"${TARGET_VERSION}"'"/' "${REPO_ROOT}/Cargo.toml"
rm -f "${REPO_ROOT}/Cargo.toml.bak"

# Atualiza Cargo.lock
(cd "${REPO_ROOT}" && cargo check --quiet)

echo "Cargo.toml sincronizado com sucesso para ${TARGET_VERSION}."
