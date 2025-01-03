#!/bin/bash
set -e

export DASM_BIN=/Users/jeroen/bin/dasm
export X64_BIN=/Applications/vice-arm64-gtk3-3.8/x64sc.app/Contents/MacOS/x64sc
export DISK_IMAGE_PATH=/Users/jeroen/workspace/c64-assembly-collection/development.D64


# Compile resources.
: '
RUST_BACKTRACE=1 cargo run --release --bin convert -- -i=resources/test.png -o=src/temp-test-image.asm --format=standard-text --output-encoding=asm --output-variable-prefix=image
RUST_BACKTRACE=1 cargo run --release --bin convert -- -i=resources/test.png -o=src/temp-test-image-bitmap.asm --format=standard-bitmap --output-encoding=asm --output-variable-prefix=image
RUST_BACKTRACE=1 cargo run --release --bin convert -- -i=resources/test.png -o=src/temp-test-image-custom.asm --format=standard-text-custom-charset --output-encoding=asm --output-variable-prefix=image
'

cd src
: '
${DASM_BIN} sprite.asm -o../bin/sprite.prg
${DASM_BIN} rasterbar.asm -o../bin/rasterbar.prg
${DASM_BIN} sprite-duplication.asm -o../bin/sprite-duplication.prg
${DASM_BIN} charset.asm -o../bin/charset.prg
${DASM_BIN} load-charset.asm -o../bin/load-charset.prg
${DASM_BIN} controller.asm -o../bin/controller.prg
${DASM_BIN} autostart.asm -o../bin/autostart.prg
${DASM_BIN} dummy.asm -o../bin/dummy.prg
${DASM_BIN} load-program.asm -o../bin/load-program.prg

${DASM_BIN} standard-text.asm -o../bin/standard-text.prg
${DASM_BIN} standard-bitmap.asm -o../bin/standard-bitmap.prg
${DASM_BIN} standard-text-custom.asm -o../bin/standard-text-cs.prg

${DASM_BIN} stripes.asm -o../bin/stripes.prg
${DASM_BIN} scrolling.asm -o../bin/scrolling.prg
'
${DASM_BIN} engine.asm -o../bin/engine.prg
cd ..

cargo run --release --bin builder

${X64_BIN} ${DISK_IMAGE_PATH}