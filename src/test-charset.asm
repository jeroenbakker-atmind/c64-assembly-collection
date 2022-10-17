    processor 6502
    org 2048

    include "defines.asm"
    include "basic_header.asm"

start:
	/* text-mem at 0400, font-mem at 2000 */
	lda #$18
	sta VIDEO_MEMORY_BLOCK

end:
	rts

	org    $1ffe
	incbin "ahoy_art_deco.64c"
