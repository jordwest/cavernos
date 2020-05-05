#/bin/bash

set -euo pipefail

(cd web && yarn devbuild)
cp web/dist/main.js www/
cp web/dist/main.js.map www/

