#!/usr/bin/bash

set -ex

DESTINATION=${DESTINATION:-_docs}
URL=${URL:-https://zlosynth.com/daisy}

rm -rf ${DESTINATION}
mkdir -p ${DESTINATION}

cat << EOF > ${DESTINATION}/index.html
<!doctype html>

<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">

  <title>Daisy</title>
  <meta name="description" content="Root of daisy Rust crate documentations.">
  <meta name="author" content="Petr Horacek">

  <meta property="og:title" content="Daisy">
  <meta property="og:type" content="website">
  <meta property="og:url" content="https://zlosynth.com/daisy">
  <meta property="og:description" content="Root of daisy Rust crate documentations.">
</head>

<body>
  <center><h1>
    documentation:
    <a href="daisy/">Seed</a>,
    <a href="daisy_1_1/">Seed 1.1</a>,
    <a href="patch_sm/">Patch SM</a>
  </h1></center>
</body>
</html>
EOF

cargo doc --no-deps --features seed --target-dir ${DESTINATION}/seed
cargo doc --no-deps --features seed_1_1 --target-dir ${DESTINATION}/seed_1_1
cargo doc --no-deps --features patch_sm --target-dir ${DESTINATION}/patch_sm
