    processor 6502

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

; IRQ to draw black background + border
irq:
    lda #COLOR_BLACK
    sta BACKGROUND_COLOR
    sta BORDER_COLOR

    LDA #<irq2            ; set interrupt vectors to the second interrupt service routine at Irq2
    STA $0314
    LDA #>irq2
    STA $0315

    LDA #200
    STA $D012            ; next interrupt will occur at line no. 200

    ASL $D019            ; acknowledge the interrupt by clearing the VIC's interrupt flag

    JMP $EA81            ; jump into shorter ROM routine to only restore registers from the stack etc

; IRQ to draw grey background + border
irq2:
    lda #COLOR_GREY
    sta BACKGROUND_COLOR
    sta BORDER_COLOR

    LDA #<irq             ; set interrupt vectors back to the first interrupt service routine at Irq
    STA $0314
    LDA #>irq
    STA $0315

    LDA #202
    STA $D012            ; next interrupt will occur at line no. 202

    ASL $D019            ; acknowledge the interrupt by clearing the VIC's interrupt flag

    JMP $EA81            ; jump into shorter ROM routine to only restore registers from the stack etc
