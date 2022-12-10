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
    lda image_charset,x
    sta $2000,x
    lda image_charset+$0400,x
    sta $2400,x
    lda image_colors,x
    sta $d800,x

    lda image_chars+$0100,x
    sta $0500,x
    lda image_charset+$0100,x
    sta $2100,x
    lda image_charset+$0500,x
    sta $2500,x
    lda image_colors+$0100,x
    sta $d900,x

    lda image_chars+$0200,x
    sta $0600,x
    lda image_charset+$0200,x
    sta $2200,x
    lda image_charset+$0600,x
    sta $2600,x
    lda image_colors+$0200,x
    sta $da00,x

    lda image_chars+$0300,x
    sta $0700,x
    lda image_charset+$0300,x
    sta $2300,x
    lda image_charset+$0700,x
    sta $2700,x
    lda image_colors+$0300,x
    sta $db00,x
skip:

    inx
    bne copy_text

	lda #$18
	sta VIDEO_MEMORY_BLOCK

stall:
    jmp stall

    rts

    include "temp-test-image-custom.asm"