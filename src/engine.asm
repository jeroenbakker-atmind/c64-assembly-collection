    processor 6502

    include "defines.asm"
    include "basic_header.asm"

CURRENT_PTR = $FE
CURRENT_PTR_LOW = $FE
CURRENT_PTR_HIGH = $FF

start:
    lda #COLOR_BLACK
    sta BORDER_COLOR

init_engine:
    lda #<engine_data
    sta CURRENT_PTR_LOW
    lda #>engine_data
    sta CURRENT_PTR_HIGH

    ; Advance the pointer with 2 bytes. The number of frames is needed
    ; when reading directly from disk.
    lda #2
    jsr current_ptr__advance

    jsr frame_commands__process
    rts

    
; Current pointer points to the beginning of a frame.
frame_commands__process:
; Copy the number of commands to process to commands left.
    ldy #0
    lda (CURRENT_PTR),y
    sta commands_left
    iny
    lda (CURRENT_PTR),y
    sta commands_left+1

    lda #2
    jsr current_ptr__advance

frame_commands__process__next
    ; Check if there are commands left to process
    lda #0
    cmp commands_left
    bne frame_command__process
    cmp commands_left+1
    bne frame_command__process

frame_commands__process__end:
    ; no commands left, frame_commands_process is completed.
    rts

frame_command__process:
    ; TODO: process command
    ; TODO: decrease commands_left
    rts
    jmp frame_commands__process__next

    include "engine__current_ptr.asm"

engine_data:
   byte $01, $00   ; One frame

engine_data_frame_1:
    byte $01, $00   ; 1 command
engine_data_command_1_1:
    byte $03, $00   ; Set border color to black.

engine_data_end:
    byte $DE, $AD

; Pointer to the next frame or command in the frame.
; Number of commands left to process in the current frame.
; When this is 0 the frame is finished processing.
commands_left:
    byte $00, $00


    byte $DE, $AD