    processor 6502
    org 2048

    include "defines.asm"
    include "basic_header.asm"

start:

test_up:
	ldx #$30
	lda #CONTROLLER_UP
	bit CONTROLLER_2
	bne result_up
	ldx #$31
result_up:
	stx $0400

test_down:
	ldx #$30
	lda #CONTROLLER_DOWN
	bit CONTROLLER_2
	bne result_down
	ldx #$31
result_down:
	stx $0401

test_left:
	ldx #$30
	lda #CONTROLLER_LEFT
	bit CONTROLLER_2
	bne result_left
	ldx #$31
result_left:
	stx $0402

test_right:
	ldx #$30
	lda #CONTROLLER_RIGHT
	bit CONTROLLER_2
	bne result_right
	ldx #$31
result_right:
	stx $0403

test_button:
	ldx #$30
	lda #CONTROLLER_BUTTON
	bit CONTROLLER_2
	bne result_button
	ldx #$31
result_button:
	stx $0404

	jmp start
