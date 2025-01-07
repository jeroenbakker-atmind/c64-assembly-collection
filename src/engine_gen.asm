; --- Application: ENGINE ---
; NOTE: This file is generated, do not modify

  processor 6502

CURRENT_PTR = $00FE

  org $0800

; --- Module begin: MAIN ---
  byte $00, $0C, $08                                        ; New basic line
  byte $0A, $00, $9E, $20, $32, $30, $36, $32               ; 10 SYS 2062
  byte $00, $00, $00                                        ; End basic program

main_entry_point:
  jsr engine__init
  rts
; --- Module end: MAIN ---

; --- Module begin: ENGINE ---

;  --- Function begin: ENGINE__INIT ---
engine__init:
  lda #<engine_data
  sta CURRENT_PTR
  lda #>engine_data
  sta CURRENT_PTR+1
  rts
; --- Function end: ENGINE__INIT ---
; --- Module end: ENGINE ---

; --- Module begin: ENGINE_DATA ---

engine_data:
  byte $01, $00                                             ; Data contains one frame

engine_data__frame_1:
  byte $01, $00                                             ; Frame contains one command

engine_data__frame_1__command_1:
  byte $03, $00                                             ; Set border color to black
; --- Module end: ENGINE_DATA ---