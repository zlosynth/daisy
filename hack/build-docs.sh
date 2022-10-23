#!/usr/bin/bash

set -ex

DESTINATION=${DESTINATION:-_docs}

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

  <style>
    body {
      background-color: #f9f7ec;
      padding-top: 100px;
    }
    a {
      text-decoration: none;
      color: #2b4d28;
    }
    a:hover {
      text-decoration: underline;
      text-decoration-thickness: 4px;
    }
  </style>
</head>

<body>
  <center><h1>
    board:
    <a href="seed/daisy">Seed</a>,
    <a href="seed_1_1/daisy">Seed 1.1</a>,
    <a href="patch_sm/daisy">Patch SM</a>
  </h1></center>
</body>
</html>
EOF

cargo doc --no-deps --features seed
cp -r target/thumbv7em-none-eabihf/doc ${DESTINATION}/seed
cargo doc --no-deps --features seed_1_1
cp -r target/thumbv7em-none-eabihf/doc ${DESTINATION}/seed_1_1
cargo doc --no-deps --features patch_sm
cp -r target/thumbv7em-none-eabihf/doc ${DESTINATION}/patch_sm
