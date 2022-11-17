#!/bin/bash
set -e
cd src
dasm test-sprite.asm -o../bin/test-sprite.prg
dasm test-rasterbar.asm -o../bin/test-rasterbar.prg
dasm test-sprite-duplication.asm -o../bin/test-sprite-duplication.prg
dasm test-charset.asm -o../bin/test-charset.prg
dasm test-load-charset.asm -o../bin/test-load-charset.prg
dasm test-controller.asm -o../bin/test-controller.prg
dasm test-autostart.asm -o../bin/test-autostart.prg
dasm test-dummy.asm -o../bin/test-dummy.prg
dasm test-load-program.asm -o../bin/test-load-program.prg
dasm test-standard-text.asm -o../bin/test-standard-text.prg
cd ..

cargo run
cd ..

x64 -8 c64-assembly-collection/demo-disk1.D64

