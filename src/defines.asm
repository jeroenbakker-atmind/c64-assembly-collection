/* VIC: Where to look for text, charset and bitmap graphics
 * default: 21 (0001 0101)
 *
 * When VIC is in text mode the 4 most significant bits multiplied by 1024
 * points to the start address of the screen characters in RAM.
 * Bits 1-3 multiplied by 2048 points to the start address of the character
 * set.
 *
 * When VIC is in bitmap mode the 4 most significant buts multiplied by 1024
 * point to the start address of the color buffer. Bits 3 indicated if bitmap
 * memory starts at address $0000 (unset) or $2000 (set).
 *
 * Taking this into account when the c64 starts it points to $0400 for screen
 * characters and $1000 for the charset. I haven't seen any evidence what
 * bit 1 does.
 */
VIDEO_MEMORY_BLOCK = $d018
VIDEO_CONTROL_REG_1 = $d011
VIDEO_CONTROL_REG_2 = $d016
BORDER_COLOR = $d020
BACKGROUND_COLOR = $d021

COLOR_BLACK = $00
COLOR_WHITE = $01
COLOR_RED = $02
COLOR_CYAN = $03
COLOR_PURPLE = $04
COLOR_GREEN = $05
COLOR_BLUE = $06
COLOR_YELLOW = $07
COLOR_ORANGE = $08
COLOR_BROWN = $09
COLOR_PINK = $0a
COLOR_DARK_GREY = $0b
COLOR_GREY = $0c
COLOR_LIGHT_GREEN = $0d
COLOR_LIGHT_BLUE = $0e
COLOR_LIGHT_GREY = $0f

SPRITE_0_X = $d000
SPRITE_0_Y = $d001
SPRITE_1_X = $d002
SPRITE_1_Y = $d003
SPRITE_2_X = $d004
SPRITE_2_Y = $d005
SPRITE_3_X = $d006
SPRITE_3_Y = $d007
SPRITE_4_X = $d008
SPRITE_4_Y = $d009
SPRITE_5_X = $d00a
SPRITE_5_Y = $d00b
SPRITE_6_X = $d00c
SPRITE_6_Y = $d00d
SPRITE_7_X = $d00e
SPRITE_7_Y = $d00f
SPRITE_X_MSB = $d010
SPRITE_MULTICOLOR_1 = $d025
SPRITE_MULTICOLOR_2 = $d026
SPRITE_0_COLOR = $d027
SPRITE_1_COLOR = $d028
SPRITE_2_COLOR = $d029
SPRITE_3_COLOR = $d02a
SPRITE_4_COLOR = $d02b
SPRITE_5_COLOR = $d02c
SPRITE_6_COLOR = $d02d
SPRITE_7_COLOR = $d02e
SPRITE_ENABLE = $d015
SPRITE_MULTICOLOR = $d01c
SPRITE_0_NUM = $07f8
SPRITE_1_NUM = $07f9
SPRITE_2_NUM = $07fa
SPRITE_3_NUM = $07fb
SPRITE_4_NUM = $07fc
SPRITE_5_NUM = $07fd
SPRITE_6_NUM = $07fe
SPRITE_7_NUM = $07ff

/*
 * Contollers/joystick
 * bit 1 = up
 *     2 = down
 *     3 = left
 *     4 = right
 *     5 = button
 */
CONTROLLER_1 = $dc01
CONTROLLER_2 = $dc00

CONTROLLER_UP = $01
CONTROLLER_DOWN = $02
CONTROLLER_LEFT = $04
CONTROLLER_RIGHT = $08
CONTROLLER_BUTTON = $10

/*
 * https://sta.c64.org/cbm64krnfunc.html
 * https://www.pagetable.com/c64ref/kernal/
 */
KERNEL_LOAD = $ffd5
KERNEL_SET_FILE_PARAMETERS = $ffba
KERNEL_SET_FILE_NAME = $ffbd