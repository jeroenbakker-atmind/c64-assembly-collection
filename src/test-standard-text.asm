    processor 6502


    include "defines.asm"
    include "basic_header.asm"
start:


start:
    lda image_background
    sta BACKGROUND_COLOR
    sta BORDER_COLOR


    ldx #$00
copy_text:
    lda image_chars,x
    sta $0400,x
    lda image_colors,x
    sta $d800,x

    lda image_chars+$0100,x
    sta $0500,x
    lda image_colors+$0100,x
    sta $d900,x

    cpx #168
    bcs skip


    lda image_chars+$0200,x
    sta $0600,x
    lda image_colors+$0200,x
    sta $da00,x

skip:

    inx
    bne copy_text
    rts

    include "temp-test-image.asm"