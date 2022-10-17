  processor 6502
  org 2048

  include "defines.asm"
  include "basic_header.asm"


start:

    lda #1
    sta SPRITE_ENABLE
    lda #(sprite1 / 64)
    sta SPRITE_0_NUM

    lda #COLOR_BLACK
    sta SPRITE_0_COLOR

setup_irqs:
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

    LDA #50             ; set rasterline where interrupt shall occur
    STA $D012

    LDA #%00000001       ; enable raster interrupt signals from VIC
    STA $D01A

    CLI                  ; clear interrupt flag, allowing the CPU to respond to interrupt requests
loop:
    jmp loop

irq:
    lda #50
    sta SPRITE_0_X
    lda #50
    sta SPRITE_0_Y

    LDA #<irq2            ; set interrupt vectors to the second interrupt service routine at Irq2
    STA $0314
    LDA #>irq2
    STA $0315

    LDA #71
    STA $D012            ; next interrupt will occur at line no. 200

    ASL $D019            ; acknowledge the interrupt by clearing the VIC's interrupt flag

    JMP $EA81            ; jump into shorter ROM routine to only restore registers from the stack etc

irq2:
    lda #50
    sta SPRITE_0_X
    lda #71
    sta SPRITE_0_Y

    LDA #<irq            ; set interrupt vectors to the second interrupt service routine at Irq2
    STA $0314
    LDA #>irq
    STA $0315

    LDA #50
    STA $D012            ; next interrupt will occur at line no. 200

    ASL $D019            ; acknowledge the interrupt by clearing the VIC's interrupt flag

    JMP $EA81            ; jump into shorter ROM routine to only restore registers from the stack etc


    align 64
sprite1:
    include "sprite-data-1.asm"