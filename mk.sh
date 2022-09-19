#!/bin/bash
set -e
cd src
dasm test-sprite.asm -o../bin/test-sprite.prg

cd ../..
x64 c64-assembly-collection/bin/test-sprite.prg

