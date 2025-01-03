; Advance current_ptr with the content of the accumulator
; Usage:
;   LDA #2
;   JSR current_ptr__advance
current_ptr__advance:
    clc
    adc CURRENT_PTR_LOW
    sta CURRENT_PTR_LOW
    lda #0
    adc CURRENT_PTR_HIGH
    sta CURRENT_PTR_HIGH
    rts
