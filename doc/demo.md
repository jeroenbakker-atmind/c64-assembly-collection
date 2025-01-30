# Goal

Main look is black and white using dithering
result is 2x2 pixels (resolution = 160x100)
on top of that painterly lines are drawn in purple color using sprites.

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
