    processor 6502

    include "defines.asm"
    include "basic_header.asm"

start:
	/* text-mem at 0400, font-mem at 2000 */
	lda #$18
	sta VIDEO_MEMORY_BLOCK

	ldx #$00
print_charset:
	txa
	sta $0400,x
	inx
	cpx #$0
	bne print_charset

end:
	rts

	org    $1ffe
	incbin "ahoy_art_deco.64c"
