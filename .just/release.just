# define and push a new release
[group: 'release']
release:
  #!/usr/bin/env bash
  set -euo pipefail
  git pull --tags >/dev/null
  last_tag=$(git describe --tags --abbrev=0)
  echo "current latest release: ${last_tag}"
  read -p "new version: v" new_tag
  if [ -z "${new_tag}" ]; then
    echo "bad tag"
    exit 1
  fi
  echo "releasing ${new_tag}"
  sed -i "s|^version =.*$|version = \"${new_tag}\"|" Cargo.toml
  sed -i "s|^version =.*$|version = \"${new_tag}\"|" macros/Cargo.toml
  sed -i "s|^version:.*$|version: \"${new_tag}\"|" helm/Chart.yaml
  sed -i "s|^appVersion:.*$|appVersion: \"${new_tag}\"|" helm/Chart.yaml
  cargo update
  sed -i "s|^	\"version\":.*$|	\"version\": \"${new_tag}\",|" web/package.json
  (cd web && npm i)
  sed -i "s|^  version:.*$|  version: \"${new_tag}\"|" openapi.yml
  git add macros/Cargo.toml Cargo.toml Cargo.lock web/package.json web/package-lock.json openapi.yml
  git commit -sm "release v${new_tag}"
  git tag "v${new_tag}"
  git push --tags
  git push
