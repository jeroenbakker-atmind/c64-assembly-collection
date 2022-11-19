    processor 6502

    include "defines.asm"
    include "basic_header.asm"

start:
	ldx #$00
print_charset:
	txa
	sta $0400,x
	inx
	cpx #$0
	bne print_charset

load_charset_file:
	/* A = Logical ID, X = Device, Y = Ignore header */
	lda #$01
	ldx #$08
	ldy #$00
	jsr KERNEL_SET_FILE_PARAMETERS

	/* A = Length of file name, X+Y contains address where name is stored. */
	lda #$06
	ldx #<font_file_name
	ldy #>font_file_name
	jsr KERNEL_SET_FILE_NAME

	/* A = LOAD (not verify) as header is ignored, X and Y contains the address
	 * where the file will be located in RAM. */
	lda #$00
	ldx #<font_address
	ldy #>font_address
	jsr KERNEL_LOAD
	bcs error

	/* text-mem at 0400, font-mem at 2000 */
	lda #$18
	sta VIDEO_MEMORY_BLOCK

end:
	rts

error:
	sta BORDER_COLOR
	rts

font_file_name:
	.byte "FONT,S"
font_address = $2000