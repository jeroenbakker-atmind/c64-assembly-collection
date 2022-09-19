#!/bin/bash
set -e
cd src
dasm test-sprite.asm -o../bin/test-sprite.prg
dasm test-rasterbar.asm -o../bin/test-rasterbar.prg

cd ../..
x64 c64-assembly-collection/bin/test-rasterbar.prg

