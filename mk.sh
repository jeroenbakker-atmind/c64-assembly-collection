#!/bin/bash
set -e

# Compile resources.
RUST_BACKTRACE=1 cargo run --release --bin convert -- -i=resources/test.png -o=src/temp-test-image.asm -f=standard-text --output-encoding=asm --output-variable-prefix=image
RUST_BACKTRACE=1 cargo run --release --bin convert -- -i=resources/test.png -o=src/temp-test-image-bitmap.asm -f=standard-bitmap --output-encoding=asm --output-variable-prefix=image

cd src
dasm sprite.asm -o../bin/sprite.prg
dasm rasterbar.asm -o../bin/rasterbar.prg
dasm sprite-duplication.asm -o../bin/sprite-duplication.prg
dasm charset.asm -o../bin/charset.prg
dasm load-charset.asm -o../bin/load-charset.prg
dasm controller.asm -o../bin/controller.prg
dasm autostart.asm -o../bin/autostart.prg
dasm dummy.asm -o../bin/dummy.prg
dasm load-program.asm -o../bin/load-program.prg

dasm standard-text.asm -o../bin/standard-text.prg
dasm standard-bitmap.asm -o../bin/standard-bitmap.prg
cd ..

cargo run
cd ..

x64 -8 c64-assembly-collection/demo-disk1.D64

