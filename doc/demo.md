# Goal
## Look
Main look is black and white using dithering
result is 2x2 pixels (resolution = 160x100)
on top of that painterly lines are drawn in purple color using sprites.

## Act 1: Title

lines draw the outline of a rect.
rect starts to rotate 2d
rect starts to rotate 3d
flash => title.
the words of the title are added one by one, where the letters. Cube is fixed in position
fade from pure white to its regular color.
Some purple lines are added to highlight some features


rotating object (skull?) resizes none fluent motion.
painterly lines highlights some features
 - Adds glasses
 - Party head
 - Beard
 - Tooth
 - Hair
 - Eye patch


## Required commands

- set sprite data
- set charset
- update text (rle kind of compression)
- wait
- randomize sprite coords (using xor of an input value on x and y positions of sprite?)

## Needed tools

- extract painterly into sprite data
- create charset and text from input image
