/*
 * This program when started loads in another program with the name DUMMY and executes it.
 * The loaded program will be located in RAM indicated by its header.
 */

    processor 6502

    include "defines.asm"
    include "basic_header.asm"

start:

load_dummy_file:
	/* A = Logical ID, X = Device, Y = Use header */
	lda #$01
	ldx #$08
	ldy #$01
	jsr KERNEL_SET_FILE_PARAMETERS

	/* A = Length of file name, X+Y contains address where name is stored. */
	lda #$05
	ldx #<dummy_file_name
	ldy #>dummy_file_name
	jsr KERNEL_SET_FILE_NAME

	/* A = LOAD (not verify), X and Y are unused as file header is used. */
	lda #$00
	jsr KERNEL_LOAD
	bcs error

end:
	/* Call just loaded program. Its header placed it at $c000. */
	jsr $c000
	rts

error:
	sta BORDER_COLOR
	rts

dummy_file_name:
	.byte "DUMMY"