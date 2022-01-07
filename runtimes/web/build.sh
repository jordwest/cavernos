#/bin/bash

set -euo pipefail

npm install
npm run-script build
cp src/index.html dist/index.html
