    processor 6502

    include "defines.asm"
    include "basic_header.asm"

; Pointer to the next frame or command in the frame.
CURRENT_PTR = $FE
CURRENT_PTR_LOW = $FE
CURRENT_PTR_HIGH = $FF

start:

engine__init:
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
    jsr commands_left__init
    lda #2
    jsr current_ptr__advance

frame_commands__process__next
    ; Check if there are commands left to process
    jsr commands_left__is_zero
    bne frame_command__process

frame_commands__process__end:
    ; no commands left, frame_commands_process is completed.
    ; TODO: wait for v-blank
    ; update border color, charset address and video memory address
    jsr set_border_color__vblank
    
    rts

frame_command__process:
    ; TODO: add switch statement
    jsr set_border_color__process

    jsr commands_left__decrease
    jmp frame_commands__process__next

    include "engine__current_ptr.asm"
    include "engine__commands_left.asm"
    include "engine__set_border_color.asm"

engine_data:
   byte $01, $00   ; One frame

engine_data__frame_1:
    byte $04, $00   ; 1 command
engine_data__frame_1__command_1:
    byte $03, $03   ; Set border color to cyan.
engine_data__frame_1__command_2:
    byte $03, $02   ; Set border color to red.
engine_data__frame_1__command_3:
    byte $03, $01   ; Set border color to white.
engine_data__frame_1__command_4:
    byte $03, $00   ; Set border color to black.

engine_data_end:

    byte $DE, $AD