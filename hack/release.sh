#!/usr/bin/env bash
set -euo pipefail

version=${1}

sed -i "s/## Unreleased/## Unreleased\n\n## ${version}/" CHANGELOG.md
sed -i "s/version =.* # hack\/release.sh$/version = \"${version}\" # hack\/release.sh/" Cargo.toml
