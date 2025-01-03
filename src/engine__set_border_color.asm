set_border_color__process:
    ldy #1
    lda (CURRENT_PTR),y
    sta set_border_color__data

set_border_color__process__advance:
    lda #2
    jsr current_ptr__advance

set_border_color__process__exit:
    rts

; During v-blank we set the border color that is stored locally to reduce tearing.
set_border_color__vblank:
    lda set_border_color__data
    sta BORDER_COLOR
    rts

set_border_color__data:
    byte $00