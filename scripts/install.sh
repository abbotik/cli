#!/usr/bin/env bash
set -euo pipefail

REPO="${ABBOTIK_CLI_REPO:-abbotik/cli}"
INSTALL_DIR="${ABBOTIK_CLI_INSTALL_DIR:-${HOME}/.local/bin}"
VERSION="${ABBOTIK_CLI_VERSION:-}"

if [[ -z "${VERSION}" ]]; then
  VERSION="$(curl -fsSL "https://api.github.com/repos/${REPO}/releases/latest" | sed -n 's/.*"tag_name": *"\([^"]*\)".*/\1/p' | head -n 1)"
fi

if [[ -z "${VERSION}" ]]; then
  echo "Could not determine the latest release version for ${REPO}." >&2
  exit 1
fi

os="$(uname -s)"
arch="$(uname -m)"

case "${os}" in
  Darwin)
    case "${arch}" in
      arm64) target="aarch64-apple-darwin" ;;
      x86_64) target="x86_64-apple-darwin" ;;
      *) echo "Unsupported macOS architecture: ${arch}" >&2; exit 1 ;;
    esac
    ;;
  Linux)
    case "${arch}" in
      aarch64|arm64) target="aarch64-unknown-linux-gnu" ;;
      x86_64) target="x86_64-unknown-linux-gnu" ;;
      *) echo "Unsupported Linux architecture: ${arch}" >&2; exit 1 ;;
    esac
    ;;
  *)
    echo "Unsupported operating system: ${os}" >&2
    exit 1
    ;;
esac

asset="abbotik-cli-${VERSION}-${target}.tar.gz"
base_url="https://github.com/${REPO}/releases/download/${VERSION}/${asset}"
sha_url="${base_url}.sha256"

workdir="$(mktemp -d)"
trap 'rm -rf "${workdir}"' EXIT

archive_path="${workdir}/${asset}"
sha_path="${workdir}/${asset}.sha256"

curl -fsSL -o "${archive_path}" "${base_url}"
curl -fsSL -o "${sha_path}" "${sha_url}"

expected_sha="$(awk '{print $1}' "${sha_path}")"
if command -v sha256sum >/dev/null 2>&1; then
  actual_sha="$(sha256sum "${archive_path}" | awk '{print $1}')"
else
  actual_sha="$(shasum -a 256 "${archive_path}" | awk '{print $1}')"
fi

if [[ "${expected_sha}" != "${actual_sha}" ]]; then
  echo "Checksum mismatch for ${asset}" >&2
  echo "expected: ${expected_sha}" >&2
  echo "actual:   ${actual_sha}" >&2
  exit 1
fi

mkdir -p "${INSTALL_DIR}"
tar -xzf "${archive_path}" -C "${INSTALL_DIR}"
chmod +x "${INSTALL_DIR}/abbot"

echo "Installed abbot to ${INSTALL_DIR}/abbot"
