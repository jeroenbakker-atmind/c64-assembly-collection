    processor 6502


    include "defines.asm"
    include "basic_header.asm"

start:
    lda #$0b
    sta BORDER_COLOR


    /* Enable bitmap mode (enable BMM flag) */
    lda #$3b
    sta VIDEO_CONTROL_REG_1

    /* bitmap on $a000, colors at $8000. */
    lda #$01
    sta VIDEO_MEMORY_LOCATION
    lda #$08
    sta VIDEO_MEMORY_BLOCK

    ldx #$00
copy_a:
    lda image_colors_1,x
    sta $8000,x
    lda image_colors_1+$0100,x
    sta $8100,x
    lda image_colors_1+$0200,x
    sta $8200,x
    lda image_colors_1+$0300,x
    sta $8300,x

    lda image_colors_2,x
    sta $8400,x
    lda image_colors_2+$0100,x
    sta $8500,x
    lda image_colors_2+$0200,x
    sta $8600,x
    lda image_colors_2+$0300,x
    sta $8700,x
    inx
    bne copy_a

copy_b:
    lda image_bitmap,x
    sta $a000,x
    lda image_bitmap+$0100,x
    sta $a100,x
    lda image_bitmap+$0200,x
    sta $a200,x
    lda image_bitmap+$0300,x
    sta $a300,x
    lda image_bitmap+$0400,x
    sta $a400,x
    lda image_bitmap+$0500,x
    sta $a500,x
    lda image_bitmap+$0600,x
    sta $a600,x
    lda image_bitmap+$0700,x
    sta $a700,x
    lda image_bitmap+$0800,x
    sta $a800,x
    lda image_bitmap+$0900,x
    sta $a900,x
    lda image_bitmap+$0a00,x
    sta $aa00,x
    lda image_bitmap+$0b00,x
    sta $ab00,x
    lda image_bitmap+$0c00,x
    sta $ac00,x
    lda image_bitmap+$0d00,x
    sta $ad00,x
    inx
    bne copy_b

copy_c:
    lda image_bitmap+$0e00,x
    sta $ae00,x
    lda image_bitmap+$0f00,x
    sta $af00,x
    lda image_bitmap+$1000,x
    sta $b000,x
    lda image_bitmap+$1100,x
    sta $b100,x
    lda image_bitmap+$1200,x
    sta $b200,x
    lda image_bitmap+$1300,x
    sta $b300,x
    lda image_bitmap+$1400,x
    sta $b400,x
    lda image_bitmap+$1500,x
    sta $b500,x
    lda image_bitmap+$1600,x
    sta $b600,x
    lda image_bitmap+$1700,x
    sta $b700,x
    lda image_bitmap+$1800,x
    sta $b800,x
    lda image_bitmap+$1900,x
    sta $b900,x
    lda image_bitmap+$1a00,x
    sta $ba00,x
    lda image_bitmap+$1b00,x
    sta $bb00,x
    lda image_bitmap+$1c00,x
    sta $bc00,x
    lda image_bitmap+$1d00,x
    sta $bd00,x
    lda image_bitmap+$1e00,x
    sta $be00,x
    lda image_bitmap+$1f00,x
    sta $bf00,x
    inx
    bne copy_c

loop:
wait_for_next_frame_1:
     bit $d011
     bpl wait_for_next_frame_1
     lda $d012
f1:  cmp $d012
     bmi f1

    lda #$08
    sta VIDEO_MEMORY_BLOCK

wait_for_next_frame_2:
     bit $d011
     bpl wait_for_next_frame_2
     lda $d012
f2:  cmp $d012
     bmi f2

    lda #$18
    sta VIDEO_MEMORY_BLOCK

    jmp loop


    include "temp-test-image-bitmap-256.asm"