 /*
    - Each tile is a grid of 4 by 4 chars.
    - Each char is 8 by 8 pixels.
    - So 16 bits per tile, (1 bit for even/uneven chars) and 3 bits
      for smooth scrolling offset
    - when even/uneven char changes in column or row the screen needs
      to be repopulated.
    - interrupts might be needed for smooth updating of the screen. (draw
      chars when rasterlines have finishes drawing.)
    - the scrolling should be controlled by the joystick

    This is also a test to do most of the work inside video interrupts.
    - joystick and scrolling
    - screen update when scrolling more than a single character
    - when updating the screen the rows before the current rasterline can be updated.

    Memory:
    - tiles
    - map (not required for this example)
    - decoded tiles (16x8)
    - decoded row


    Research after this task would be to check how to do different tiles.
  */
    processor 6502

/* Zero page addressing.  */
location_map = $90
location_encoded_tile = $91
location_sub_tile_char = $93

screen_should_be_updated = $94

    include "defines.asm"
    include "basic_header.asm"

start:

    SEI                  ; set interrupt bit, make the CPU ignore interrupt requests
    LDA #%01111111       ; switch off interrupt signals from CIA-1
    STA $DC0D

    AND $D011            ; clear most significant bit of VIC's raster register
    STA $D011

    LDA $DC0D            ; acknowledge pending interrupts from CIA-1
    LDA $DD0D            ; acknowledge pending interrupts from CIA-2

    LDA #<irq            ; set interrupt vectors, pointing to interrupt service routine below
    STA $0314
    LDA #>irq
    STA $0315

    LDA #0             ; set rasterline where interrupt shall occur
    STA $D012

    LDA #%00000001       ; enable raster interrupt signals from VIC
    STA $D01A

    CLI                  ; clear interrupt flag, allowing the CPU to respond to interrupt requests
loop:
    jmp loop

irq:
smooth_hor:
    ldx location_sub_tile_char
    dex
    bmi mark_update_screen_dec_x
continue_mark_update_screen:
    txa
    and #7
    tax
    stx location_sub_tile_char
    stx VIDEO_CONTROL_REG_2

    lda #0
    cmp screen_should_be_updated
    beq skip_update_screen

    /* Unset that the screen should be updated. */
    sta screen_should_be_updated
    jsr update_screen

skip_update_screen:
    ASL $D019            ; acknowledge the interrupt by clearing the VIC's interrupt flag
    JMP $EA81            ; jump into shorter ROM routine to only restore registers from the stack etc

mark_update_screen_dec_x:
    lda location_decoded_tile
    clc
    adc #1
    and #3
    sta location_decoded_tile
    lda #1
    sta screen_should_be_updated
    jmp continue_mark_update_screen



wait:
    iny
    bne wait

    jmp smooth_hor



end:
    rts

update_screen:
    lda #$04
    sta screen_row_pointer+1
    lda #0
    sta screen_row_pointer

    jsr _update_screen_draw_complete_tile_row
    jsr _update_screen_draw_complete_tile_row
    jsr _update_screen_draw_complete_tile_row
    jsr _update_screen_draw_complete_tile_row
    jsr _update_screen_draw_complete_tile_row
    rts

_update_screen_draw_complete_tile_row:
    lda location_decoded_tile
    sta tile_id_row_char_offset
    lda #<grid
    sta tiles_char_pointer
    lda #>grid
    sta tiles_char_pointer+1
    jsr update_screen_row_b

    lda location_decoded_tile
    sta tile_id_row_char_offset
    lda #<(grid+4)
    sta tiles_char_pointer
    lda #>(grid+4)
    sta tiles_char_pointer+1
    jsr update_screen_row_b

    lda location_decoded_tile
    sta tile_id_row_char_offset
    lda #<(grid+8)
    sta tiles_char_pointer
    lda #>(grid+8)
    sta tiles_char_pointer+1
    jsr update_screen_row_b

    lda location_decoded_tile
    sta tile_id_row_char_offset
    lda #<(grid+12)
    sta tiles_char_pointer
    lda #>(grid+12)
    sta tiles_char_pointer+1
    jsr update_screen_row_b
    rts


screen_row_pointer = $A0
tile_ids_row_pointer = $A2
tiles_char_pointer = $A4
tile_id_row_char_offset = $A6
_screen_row_pointer_end_low = $A8

/**
 * Routine to update a single row on the screen.
 *
 * Input:
 *  - `tile_ids_row_pointer` pointer to the buffer containing tile_ids.
 *    Pointer should point to the first tile_id for the first
 *    character on the screen.
 *  - `tile_id_row_char_offset` number of chars to skip for the first
 *    drawn tile. Must be 0<=x<=3.
 *  - `tiles_char_pointer` points to the list of chars that defines
 *    the chars to for tile_id 0 for this row. This pointer should
 *    be modified to use the characters for a different row.
 *    Values can be `grid`, `grid+4`, `grid+8` or `grid+12`.
 *  - `screen_row_pointer`.
 *
 * Output:
 *  - Draws 40 chars to the screen starting from `screen_row_pointer`.
 *  - `tile_id_row_char_offset` will be modified and should be re-initialized.
 *
 * Future changes:
 *  - Include copying of color codes.
 */
update_screen_row_b:
    lda screen_row_pointer
    clc
    adc #39
    sta _screen_row_pointer_end_low

_update_screen_row_1:

    ldy tile_id_row_char_offset
    lda (tiles_char_pointer),y
    ldy #0
    sta (screen_row_pointer),y

    /* Move offset to next char. Rotate when all chars are read. */
    lda tile_id_row_char_offset
    clc
    adc #1
    and #3
    sta tile_id_row_char_offset


    /* Are we at the last char, then exit. */
    lda screen_row_pointer
    cmp _screen_row_pointer_end_low
    beq _update_screen_row_finished

    /* move screen_row_pointer to next position. */
    jsr advance_screen_row_pointer
    jmp _update_screen_row_1

_update_screen_row_finished
    jsr advance_screen_row_pointer
    rts

advance_screen_row_pointer:
    lda screen_row_pointer
    clc
    adc #01
    sta screen_row_pointer
    lda screen_row_pointer+1
    adc #0
    sta screen_row_pointer+1
    rts



current_tile_row:
    .byte 0,0,0,0

grid:
    .byte 112, 64, 64, 110
    .byte 66, 32, 32, 66
    .byte 66, 32, 32, 66
    .byte 109, 64, 64, 125


location_decoded_tile:
    .byte 0