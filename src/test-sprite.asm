  processor 6502
  org 2048

  include "defines.asm"
  include "basic_header.asm"


start:
    ldx #COLOR_GREEN
    stx BORDER_COLOR
    ldx #COLOR_LIGHT_GREEN
    stx BACKGROUND_COLOR

    lda #1
    sta SPRITE_ENABLE
    lda #(sprite1 / 64)
    sta SPRITE_0_NUM

    lda #COLOR_BLACK
    sta SPRITE_0_COLOR

    ldx #0
    ldy #0

update_pos:
    stx SPRITE_0_X
    stx SPRITE_0_Y
    dex

wait:
    dey
    bne wait

    jmp update_pos

    rts

scratch:
    byte #0

    align 64
sprite1:
    include "sprite-data-1.asm"