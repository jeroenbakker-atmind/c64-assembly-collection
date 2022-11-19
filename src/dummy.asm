    processor 6502

    include "defines.asm"

    org $c000

start:
    lda #COLOR_ORANGE
    sta BACKGROUND_COLOR
    sta BORDER_COLOR
    rts