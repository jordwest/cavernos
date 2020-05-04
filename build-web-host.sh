#/bin/bash

set -euo pipefail

(cd web && yarn build)
cp web/dist/main.js www/
cp web/dist/main.js.map www/

