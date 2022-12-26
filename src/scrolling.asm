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
screen_row_offset = $A0

location_map = $90
location_encoded_tile = $91
location_decoded_tile = $92
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
    bcs mark_update_screen_dec_x
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
    lda #1
    sbc location_decoded_tile
    lda #3
    and location_decoded_tile
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
    sta screen_row_offset+1

    lda #0
    sta screen_row_offset
    lda grid
    sta current_tile_row
    lda grid+1
    sta current_tile_row+1
    lda grid+2
    sta current_tile_row+2
    lda grid+3
    sta current_tile_row+3
    jsr update_screen_row

    lda #40
    sta screen_row_offset
    lda grid+4
    sta current_tile_row
    lda grid+5
    sta current_tile_row+1
    lda grid+6
    sta current_tile_row+2
    lda grid+7
    sta current_tile_row+3
    jsr update_screen_row

    lda #80
    sta screen_row_offset
    lda grid+8
    sta current_tile_row
    lda grid+9
    sta current_tile_row+1
    lda grid+10
    sta current_tile_row+2
    lda grid+11
    sta current_tile_row+3
    jsr update_screen_row

    lda #120
    sta screen_row_offset+0
    lda grid+12
    sta current_tile_row
    lda grid+13
    sta current_tile_row+1
    lda grid+14
    sta current_tile_row+2
    lda grid+15
    sta current_tile_row+3
    jsr update_screen_row

    rts

update_screen_row:
    ldy #39
    ldx #$03
update_screen_row_1:
    lda current_tile_row,x
    sta (screen_row_offset),y

    dex
    txa
    and #$03
    tax

    dey
    bpl update_screen_row_1

    rts



current_tile_row:
    .byte 0,0,0,0

grid:
    .byte 112, 64, 64, 110
    .byte 66, 32, 32, 66
    .byte 66, 32, 32, 66
    .byte 109, 64, 64, 125


