    processor 6502

    include "defines.asm"
    include "autostart_header.asm"

    /* Blank the screen during loading by filling the screen with 32 (space). */
    org $0400
    repeat 1000
    .byte $20
    repend

start:
    /* Make border white. */
    lda $00
    sta BORDER_COLOR

    /* Application cannot exit to DOS anymore, just do a loop. */
loop:
    jmp loop