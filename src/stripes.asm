    processor 6502

    include "defines.asm"
    include "basic_header.asm"

start:
    lda #$00
    sta BORDER_COLOR
    sta BACKGROUND_COLOR

	/* text-mem at 0400, font-mem at 2000 */
	lda #$18
	sta VIDEO_MEMORY_BLOCK

    jsr generate_charset

end:
    rts

generate_charset:
    ldx #$00
gen_char_next:
    lda #$00
    sta $2000,x
    sta $2100,x
    sta $2200,x
    sta $2300,x

    inx
    bne gen_char_next

    rts
