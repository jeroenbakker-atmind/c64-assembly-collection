# C64 assembly collection

This repository contains several experiences performed in 2022 to get back into C64 (6502/VIC-II) assembly programming. The experiments are selected based on knowledge missing during preparation of a new JRPG game being developed. When the time is right more information about the game will be shared. Of course all development will happen in the open using a free and open source license.

All content in this repository is not written for shareability. It is used for personal learning. Of course your welcome to check it out.

## Tools & build system

The main tools are developed in rust and require a Linux/Mac system. Windows could also work when adding a mk file.

Tools that are provided can generate PETSCII art from png files. The PETSCII art is formatted as 6502 assembly.

Other tool can generate the disk images to be loaded directly into an emulator. We chose not to use .prg files directly as the project requires more storage than a single .prg file can handle.

## Experiments

### sprite.prg

Assembly example to show a sprite on the screen.

### rasterbar.prg

Try to use interrupts to display a horizontal line on the screen by changing the background color of the VIC-II chip.

### sprite-duplication.prg

Using raster interrupts to duplicate sprites. This technique allows more sprites to be drawn, but only 8 in the vicinity of the current raster-line.

### charset.prg

Change the current character set used by the VIC-II. The used character set is stored inside the charset.

### load-charset.prg

Change the current character set. The used character set is read from a file on disk.

### controller.prg

Try to read the state of the a joystick. This will drive the IO handling of the targeted game.

### autostart.prg

Example how to autostart a program after loading.

### load-program.prg

Load and execute another application. Will be used to (un)load logic that isn't needed. The loaded sub-program is "dummy".

### standard-text.prg

Use standard text mode to display an image.

### standard-bitmap.prg

Use standard bitmap mode to display an image.