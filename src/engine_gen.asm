; --- Application: ENGINE ---
; NOTE: This file is generated, do not modify

  processor 6502

VIC20_BORDER_COLOR = $D020
CURRENT_PTR = $FE

  org $0800

; --- Module begin: MAIN ---
  byte $00, $0C, $08     ; New basic line
  ; 10 SYS 2062
  byte $0A, $00, $9E, $20, $32, $30, $36, $32
  byte $00, $00, $00     ; End basic program

main_entry_point:
  jsr engine__init
  jsr engine__frame__process
  rts
; --- Module end: MAIN ---

; --- Module begin: ENGINE ---

; --- Function begin: ENGINE__INIT ---

; Initialize the engine.
; 
;  - assumes engine data is stored at 'engine-data'
;  - sets the current pointer to the first frame
engine__init:
  lda #<engine_data
  sta CURRENT_PTR
  lda #>engine_data
  sta CURRENT_PTR+1
  lda #$02               ; Advance the pointer with 2 bytes.
                         ; Number of frames is only needed when reading directly from disk.
  jsr engine__current_ptr__advance
  rts
; --- Function end: ENGINE__INIT ---

; --- Function begin: ENGINE__FRAME__PROCESS ---

engine__frame__process:
  jsr engine__commands_left__init
  lda #$02
  jsr engine__current_ptr__advance

engine__frame_commands__next:
  jsr engine__commands_left__is_zero
  bne engine__frame_command__process

  ; All commands in frame have been processed.
  ; TODO: Wait for vblank
  ; Update the VIC20 state.
  ; TODO: Use shadow VIC20 registers
engine__frame__exit:
  jsr set_border_color__vblank
  rts

engine__frame_command__process:
  jsr set_border_color__process
  jsr engine__commands_left__decrease
  jmp engine__frame_commands__next
; --- Function end: ENGINE__FRAME__PROCESS ---
; --- Module end: ENGINE ---

; --- Module begin: ENGINE__CURRENT_PTR ---

; --- Function begin: ENGINE__CURRENT_PTR__ADVANCE ---

; Advance current pointer with accumulator
; 
; Advance the pointer stored at CURRENT_PTR with the value stored in the accumulator.
; The accumulator is number of bytes to advance.
engine__current_ptr__advance:
  clc
  adc CURRENT_PTR
  sta CURRENT_PTR
  lda #$00
  adc CURRENT_PTR+1
  sta CURRENT_PTR+1
  rts
; --- Function end: ENGINE__CURRENT_PTR__ADVANCE ---
; --- Module end: ENGINE__CURRENT_PTR ---

; --- Module begin: ENGINE__COMMANDS_LEFT ---

  ; Number of commands left to process in the current frame.
  ; When 0 the frame is finished processing
engine__commands_left:
  byte $00, $00

; --- Function begin: ENGINE__COMMANDS_LEFT__INIT ---

engine__commands_left__init:
  ldy #$00
  lda (CURRENT_PTR),y
  sta engine__commands_left
  iny
  lda (CURRENT_PTR),y
  sta engine__commands_left+1
  rts
; --- Function end: ENGINE__COMMANDS_LEFT__INIT ---

; --- Function begin: ENGINE__COMMANDS_LEFT__DECREASE ---

engine__commands_left__decrease:
  clc
  lda engine__commands_left
  sbc #$00
  sta engine__commands_left
  lda engine__commands_left+1
  sbc #$00
  sta engine__commands_left+1
  rts
; --- Function end: ENGINE__COMMANDS_LEFT__DECREASE ---

; --- Function begin: ENGINE__COMMANDS_LEFT__IS_ZERO ---

engine__commands_left__is_zero:
  lda #$00
  cmp engine__commands_left
  bne engine__commands_left__is_zero__exit
  cmp engine__commands_left+1

engine__commands_left__is_zero__exit:
  rts
; --- Function end: ENGINE__COMMANDS_LEFT__IS_ZERO ---
; --- Module end: ENGINE__COMMANDS_LEFT ---

; --- Module begin: SET_BORDER_COLOR ---

  ; Border color to set at the next vblank
set_border_color__data:
  byte $00

; --- Function begin: SET_BORDER_COLOR__PROCESS ---

set_border_color__process:
  ldy #$01
  lda (CURRENT_PTR),y
  sta set_border_color__data
  lda #$02
  jsr engine__current_ptr__advance
  rts
; --- Function end: SET_BORDER_COLOR__PROCESS ---

; --- Function begin: SET_BORDER_COLOR__VBLANK ---

set_border_color__vblank:
  lda set_border_color__data
  sta VIC20_BORDER_COLOR
  rts
; --- Function end: SET_BORDER_COLOR__VBLANK ---
; --- Module end: SET_BORDER_COLOR ---

; --- Module begin: ENGINE_DATA ---

engine_data:
  byte $01, $00, $01, $00, $03, $00
; --- Module end: ENGINE_DATA ---