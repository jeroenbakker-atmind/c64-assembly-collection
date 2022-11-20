    processor 6502


    include "defines.asm"
    include "basic_header.asm"

start:
    /* Screen memory (color) start at 15360 ($3c00)
     * Bitmap memory start at 16384  ($4000)
    lda #$f1
    sta VIDEO_MEMORY_BLOCK
    */

    /* Enable bitmap mode (enable BMM flag)
    lda #32
    ora VIDEO_CONTROL_REG_1*/
    lda #$3b
    sta VIDEO_CONTROL_REG_1
    lda #$18
    sta VIDEO_MEMORY_BLOCK

    ldx #$00
copy:
    lda image_colors,x
    sta $0400,x
    lda image_colors+$0100,x
    sta $0500,x
    lda image_colors+$0200,x
    sta $0600,x
    lda image_colors+$0300,x
    sta $0700,x

    lda image_bitmap,x
    sta $2000,x
    lda image_bitmap+$0100,x
    sta $2100,x
    lda image_bitmap+$0200,x
    sta $2200,x
    lda image_bitmap+$0300,x
    sta $2300,x
    lda image_bitmap+$0400,x
    sta $2400,x
    lda image_bitmap+$0500,x
    sta $2500,x
    lda image_bitmap+$0600,x
    sta $2600,x
    lda image_bitmap+$0700,x
    sta $2700,x
    lda image_bitmap+$0800,x
    sta $2800,x

    inx
    bne copy

exit:
    jmp exit
    rts

    include "temp-test-image-bitmap.asm"