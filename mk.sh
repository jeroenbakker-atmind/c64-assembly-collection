#!/bin/bash
set -e
cd src
dasm test-sprite.asm -o../bin/test-sprite.prg
dasm test-rasterbar.asm -o../bin/test-rasterbar.prg
dasm test-sprite-duplication.asm -o../bin/test-sprite-duplication.prg
dasm test-charset.asm -o../bin/test-charset.prg

cd ../..
x64 c64-assembly-collection/bin/test-charset.prg

