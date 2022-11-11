  org $0800

basic:
  .byte $00
basic_line1:
  ; pointer to next line.
  .byte $0c, $08
  ; 10 SYS 2062
  .byte $0a, $00, $9e, $20
  .byte $32, $30, $36, $32
  .byte $00
basic_line2:
  ; pointer to next line (end basic program)
  .byte $00, $00


