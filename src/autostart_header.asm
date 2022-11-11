       org $02ed

;There are 89 free bytes @ $2A7 - $300
;point STOP vector here
;Repair stop check vector (it's altered
;furtheron in the code). Only high byte
;was altered
       lda #$f6
       sta $0329
;Hat trick: call load code @ get-next-
;byte-loop start
       jsr $f4f3
       jmp start

;---------------------------------------
;System vectors will be overwritten with
;0's if we leave this file empty up to
;stprog. See i'net for default values.
       org $0300
       .word $e38b,$a483,$a57c,$a71a
       .word $a7e4,$ae86
       .byte 0,0,0,0 ;Tmp storage CPU
       jmp $b248 ;Usr functon, jmp+adr
       .byte 0 ;$313 unused
       .word $ea31,$fe66,$fe47,$f34a
       .word $f291,$f20e,$f250,$f333
       .word $f157,$f1ca
;TRAP! Point STOP vector which is @ $328
;to $02ED. Default points to $F6ED
       .word $02ed
       .word $f13e,$f32f,$fe66,$f4a5
       .word $f5ed

;---------------------------------------
;Start of actual prog
;stprog = $startaddress e.g. $800
;      *= stprog

