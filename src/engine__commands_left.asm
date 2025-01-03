
; Copy the number of commands that CURRENT_PTR is pointing at to
; commands_left.
commands_left__init:
    ldy #0
    lda (CURRENT_PTR),y
    sta commands_left
    iny
    lda (CURRENT_PTR),y
    sta commands_left+1
    rts

commands_left__decrease:
    clc
    lda commands_left
    sbc #0
    sta commands_left 
    lda commands_left+1
    sbc #0
    sta commands_left+1
    rts

; Set zero flag if commands left is zero
commands_left__is_zero:
    lda #0
    cmp commands_left
    bne commands_left__is_zero__exit
    cmp commands_left+1

commands_left__is_zero__exit:
    rts


    byte $DE, $AD
; Number of commands left to process in the current frame.
; When this is 0 the frame is finished processing.
commands_left:
    byte $00, $00

    byte $DE, $AD
