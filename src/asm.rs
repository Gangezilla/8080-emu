pub fn convert_opcode(opcode: u8) -> &'static str {
    match opcode {
        0x00 => "0x00: NOP",
        0x01 => "0x01: LXI B",
        // Load byte 3 into register B. Load byte 2 into register C.
        0x02 => "0x02: STAX B",
        // Store A indirect
        // The content of register A is moved to the memory location whose address is in the register pair rp.
        // Note: only register pairs rp=B (registers B and C) or rp=D (registers D and E) may be specified.
        0x03 => "0x03: INX B",
        // BC <- BC + 1
        0x04 => "0x04: INR B",
        // Increment B
        0x05 => "0x05: DCR B",
        // Decrement B
        0x06 => "0x06: MVI B.",
        // Move immediate second byte to register B
        0x07 => "0x07: RLC",
        // Rotate left.
        // Rotating can be used to square a binary number.
        // The content of the accumulator is rotated left one position.
        // The low order bit and the CY flag are both set to the value shifted out of the high order bit position.
        // Only the CY flag is affected.
        0x08 => "0x08: -",
        0x09 => "0x09: DAD B",
        // HL = HL + BC
        0x0A => "0x0A: LDAX B",
        // Load B indirect
        0x0B => "0x0b: DCX B",
        0x0C => "0x0C: INR C",
        // C <- C + 1;
        0x0D => "0x0D: DCR C",
        // Decrement C.
        0x0E => "0x0E: MVI C",
        // Move byte 2 into register C.
        // MVI stands for Move Immediate.
        0x0F => "0x0F: RRC",
        // The content of the accumulator is rotated right one position.
        // The high order bit and the CY flag are both set to the value shifted out of the low order bit position. Only the CY flag is affected.



        0x10 => "0x10: -",
        0x11 => "0x11: LXI D",
        // Move byte 3 into register D. Move byte 2 into register E.
        // LXI stands for Load Immediate Register
        0x12 => "0x12: STAX D",
        // A <- (DE)
        0x13 => "0x13: INX D",
        // Increment register DE
        0x14 => "0x14: INR D",
        // Increment Register D
        0x15 => "0x15: DCR D",
        // Decrement D
        0x16 => "0x16: MVI D",
        // Move byte 2 into D
        0x17 => "UNIMPLEMENTED!!!",
        0x18 => "0x18: -",
        0x19 => "0x19: DAD D",
        // Add D & E to H & L
        0x1A => "0x1A: LDAX D",
        // Load A indirect
        // The content of the memory location, whose address is in the register pair rp, is moved to register A.
        // Note: only register pairs rp=B (registers B and C·) or rp=D (registers D and E) may be specified.
        0x1B => "0x1B: DCX D",
        // DE = DE - 1
        0x1C => "0x1c: INR E",
        // E <- E + 1
        0x1D => "0x1d: DCR E",
        // E <- E - 1
        0x1E => "0x1e: MVI E",
        // move byte 2 into E
        0x1F => "0x1f: RAR",
        // 	A = A >> 1; bit 7 = prev bit 7; CY = prev bit 0
        // Rotate A right thru carry



        0x20 => "0x20: -",
        0x21 => "0x21: LXI H",
        // Load immediate register
        // LXI rp, data 16
        // Byte 3 of the instruction is moved into the high order register of the register pair rp.
        // byte 2 of the instruction is moved into the low order register (rl) of the restier pair rp.
        0x22 => "0x22: SHLD adr",
        // Store H & L direct
        // The content of register L is moved to the memory location whose address is specified in byte 2 and byte 3.
        // The content of register H is moved to the next memory location.
        // ((byte 3) (byte 2)) <- (L)
        // ((byte 3)(byte 2) + 1) <- (H)
        0x23 => "0x23: INX H",
        // Increment H & L registers
        0x24 => "0x24: INR H",
        // Increment H register.
        0x25 => "0x25: DCR H",
        // H <- H - 1
        0x26 => "0x26: MVI H",
        // Move byte into register H.
        0x27 => "0x27: DAA",
        // Decimal Adjust A(ccumulator)
        // The eight-bit number in the accumulator is adjusted to form two four-bit Binary-Coded-Decimal digits by the following process:
        // If the value of the least significant 4 bits of the accumulator is greater than 9 or if the AC flag is set, 6 is added to the accumulator.
        // If the value of the most significant 4 bits of the accumulator is now greater than 9, or if the CY flag is set, 6 is added to the most significant 4 bits of the accumulator.
        0x28 => "0x28: -",
        0x29 => "0x29: DAD H",
        // Adds HI to HL.
        0x2A => "0x2a: LHLD addr",
        // Load H & L direct
        // The content of the memory location, whose address is specified in byte 2 and byte 3 of the instruction, is moved to register L.
        //  The content of the memory location at the succeeding address is moved to register H.
        0x2B => "0x2b: DCX H",
        // Decrement H & L
        0x2C => "0x2c: INR L",
        // Increment Register L
        0x2E => "0x2e: MVI L",
        // Move byte 2 into register L
        0x2F => "0x2f: CMA",
        // Compliment A
        // The contents of the accumulator are complemented- (zero bits become 1, one bits become 0).
        // A <- !A



        0x30 => "0x30: -",
        0x31 => "0x31: LXI SP",
        // move byte 3 into high order location of register SP. move byte 2 into low order location of register SP.
        0x32 => "0x32: STA",
        // Store A direct.
        // The content of the accumulator is moved to the next two bytes.
        0x34 => "0x34: INR M",
        // Increment Register M (HL)
        0x35 => "0x35: DCR M",
        // Decrements Register M.
        // The content of the memory location whose address is contained in the H and L registers is decremented by one.
        // Sets the flags 	Z, S, P, AC
        0x36 => "0x36: MVI M",
        // Move second byte, into register HL.
        0x37 => "0x37: STC",
        // Set carry
        // Sets carry flag to 1
        0x38 => "0x38: -",
        0x39 => "0x39: DAD SP",
        // 	HL = HL + SP
        0x3A => "0x3a: LDA adr",
        // Load Accumulator Direct
        // The content of the memory location, whose address is specified in byte 2 and byte 3 of the instruction, is moved to register A.
        0x3B => "UNIMPLEMENTED!!!",
        0x3C => "0x3c: INR A",
        // Increment Register A
        0x3D => "0x3d: DCR A",
        // Decrement Register A
        0x3E => "0x3e: MVI A",
        // Move immediate register
        0x3F => "0x3f: CMC",
        // CY=!CY



        0x40 => "0x40: MOV B,B",
        0x41 => "0x41: MOV B,C",
        // Move C into B.
        0x42 => "0x42: MOV B,D",
        // B <- D
        0x43 => "0x43: MOV B,E",
        0x44 => "0x44: MOV B,H",
        0x45 => "0x45: MOV B,L",
        0x46 => "0x46: MOV B,M",
        // Move the contents of HL to B
        0x47 => "0x47: MOV B,A",
        // Move contents of A to B.
        0x48 => "0x48: MOV C,B",
        // Move B into C.
        0x49 => "0x49: MOV C,C",
        // C <- C
        0x4A => "0x4a: MOV C,D",
        // C <- D
        0x4B => "0x4b: MOV C,E",
        0x4C => "0x4c: MOV C,H",
        0x4D => "0x4d: MOV C,L",
        0x4E => "0x4e: MOV C,M",
        // Move HL to C
        0x4F => "0x4f: MOV C,A",
        // Move A to C.



        0x50 => "0x50: MOV D,B",
        0x51 => "0x51: MOV D,C",
        0x54 => "0x54: MOV D,H",
        // Move H into D
        0x56 => "0x56: MOV D,M",
        // Move register HL into register D.
        0x57 => "0x57: MOV D,A",
        // Move D into A.
        0x59 => "0x59: MOV E,C",
        0x5B => "0x5b: MOV E,E",
        0x5E => "0x5e: MOV E,M",
        // Move register HL to register E.
        0x5F => "0x5f: MOV E,A",
        // Move contents of A to E.



        0x60 => "0x60: MOV H,B",
        // Move B to H
        0x61 => "0x61: MOV H,C",
        // Move contents of register C to register H
        0x62 => "0x62: MOV H,D",
        0x63 => "0x63: MOV H,E",
        0x64 => "0x64: MOV H,H",
        // Move H to H (??)
        0x65 => "0x65: MOV H,L",
        // H <- L
        0x66 => "0x66: MOV H,M",
        // Move memory (HL) to register H
        // H <- (HL)
        0x67 => "0x67: MOV H,A",
        // Move register A to register H
        0x68 => "0x68: MOV L,B",
        // Move register B to register A.
        0x69 => "0x69: MOV L,C",
        // Move value of C into register L.
        0x6C => "0x6c: MOV L,H",
        // L <- H
        0x6D => "0x6d: MOV L,L",
        0x6E => "0x6e: MOV L,M",
        0x6F => "0x6f: MOV L,A",
        // Move memory (L) to register A
        // L <- A



        0x70 => "0x70: MOV M,B",
        // Move register B to register M (HL);
        0x71 => "0x71: MOV M,C",
        // Move register C into register M.
        0x72 => "0x72: MOV M,D",
        // Move register D into register M(HL);
        0x73 => "0x73: MOV M,E",
        // Move register E to register M (HL)
        0x74 => "0x74: MOV M,H",
        // (HL) <- H
        0x76 => "0x76: HLT",
        // HALT (??)
        // The processor is stopped.
        0x77 => "0x77: MOV M,A",
        // Move register A to register HL.
        0x78 => "0x78: MOV A,B",
        // Move contents of register B to register A.
        0x79 => "0x79: MOV A,C",
        // Move register C to A.
        0x7A => "0x7a: MOV A,D",
        // Move contents of register D to register A
        0x7B => "0x7b: MOV A,E",
        // Move contents of E to A
        0x7C => "0x7c: MOV A,H",
        // Move register H into register A.
        0x7D => "0x7d: MOV A,L",
        // Move contents of L to A
        0x7E => "0x7e: MOV A,M",
        0x7F => "0x7f: MOV A,A",



        0x80 => "0x80: ADD B",
        // Add value of B to the accumulator to A
        0x81 => "0x81: ADD C",
        // A <- A + C
        0x82 => "0x82: ADD D",
        // A <- A + D
        0x83 => "0x83: ADD E",
        // Add E to A.
        0x84 => "0x84: ADD H",
        // A <- A + H;
        0x85 => "0x85: ADD L",
        // Adds register L to register A.
        0x86 => "0x86: ADD M",
        // Add M (HL) to register A.
        0x88 => "0x88: ADC B",
        // A <- A + B + CY
        0x8A => "0x8a: ADC D",
        // Add D, and CY to A.
        0x8B => "0x8b: ADC E",
        // A <- A + E + CY
        0x8E => "0x8e: ADC M",
        // 	A <- A + (HL) + CY



        0x90 => "0x90: SUB B",
        // A <- A + B
        0x94 => "0x94: SUB H",
        // A <- A + H
        0x97 => "0x97: SUB A",
        // Subtract value of A from A (i guess sets A to 0)
        0x98 => "0x98: SBB B",
        // 	A <- A - B - CY
        0x99 => "0x99: SBB C",
        // A <- A - C - CY
        0x9A => "0x9a: SBB D",
        // A <- A - D - CY
        0x9B => "0x9b: SBB E",
        // 	A <- A - E - CY
        0x9D => "0x9d: SBB L",
        // A <- A - L - CY
        0x9E => "0x9e: SBB M",
        // 	A <- A - (HL) - CY



        0xA0 => "0xa0: ANA B",
        // Does an and of A & B on A
        0xA3 => "0xa3: ANA E",
        // A <- A * E
        0xA6 => "0xa6: ANA M",
        // 	A <- A & (HL)
        0xA7 => "0xa7: ANA A",
        // AND register
        // The content of register r is logically anded with the content of the accumulator.
        // The result is placed in the accumulator. The CY flag is cleared.
        0xA8 => "0xa8: XRA B",
        // 	A <- A ^ B
        // Exclusive OR
        0xAA => "0xaa: XRA D",
        // A <- A ^ D
        0xAF => "0xaf: XRA A",
        // Exclusive OR Register
        // The content of register r is exclusive-or'd with the content of the accumulator. The result is placed in the accumulator. The CY and AC flags are cleared.



        0xB0 => "0xB0: ORA B",
        // A <- A | B
        // Or register with A.
        // Does an OR with register B on register A.
        0xB3 => "0xb3: ORA E",
        // A <- A | E
        0xB4 => "0xb4: ORA H",
        // A <- A | H
        // Or register H with register A.
        0xB6 => "0xb6: ORA M",
        // Does an or with whats in register M (HL) versus accumulator
        0xB8 => "0xb8: CMP B",
        // Compare register B to A.
        // Contents of register B are substracted from A.
        // couldnt tell you why its called compare
        0xBB => "0xbb: CMP E",
        // A - E
        0xBC => "0xbc: CMP H",
        // A - H
        0xBE => "0xbe: CMP M",
        // A - (HL)
        // Subtract HL from A.



        0xC0 => "0xc0: RNZ",
        // If NZ (non-zero), RET (return).
        0xC1 => "0xc1: POP B",
        // Pop register pair B & C off stack
        0xC2 => "0xc2: JNZ adr",
        // Jump on no zero.
        // not sure, but i think its jump if a check is non zero.
        0xC3 => "0xc3: JMP",
        // JMP, uses next two bytes to determine where to jump to.
        0xC4 => "0xc4: CNZ",
        // Call address if non zero
        0xC5 => "0xc5: PUSH B",
        // Push register Pair B & C on stack
        0xC6 => "0xc6: ADI",
        // Add immediate to A.
        // The content of the second byte of the instruction is added to the content of the accumulator.
        // The result is placed in the accumulator.
        0xC8 => "0xc8: RZ",
        // 	If Z flag is set, RET
        0xC9 => "0xc9: RET",
        // Return
        // The content of the memory location whose address is specified in register SP is moved to the low-order eight bits of register PC.
        // The content of the memory location whose address is one more than the content of register SP is moved to the high-order eight bits of register PC. The content of register SP is incremented by 2.
        0xCA => "0xca: JZ",
        // Jump on zero
        // not sure. i imagine it checks to see if something is 0, then jumps...
        0xCC => "0xcc: CZ",
        // If Z, call adr
        0xCD => "0xcd: CALL adr",
        // The high-order eight bits of the next instruction address are moved to the memory location whose address is one less than the content of register SP.
        // The low-order eight bits of the next instruction address are moved to the memory location whose address is two less than the content of register SP.
        // The content of register SP is decremented by 2. Control is transferred to the instruction whose address is specified in byte 3 and byte 2 of the current instruction.



        0xD0 => "0xd0: RNC",
        // 	if NCY, RET
        // return on no carry. if the carry flag hasn't been set, return.
        0xD1 => "0xd1: POP D",
        // Pop register pair D & E off stack
        0xD2 => "0xd2: JNC",
        // Jump if no carry
        // not sure, but i imagine it's jump if the no carry flag hasn't been set.
        0xD3 => "0xd3: OUT",
        // Output
        // The content of register A is placed on the eight bit bi-directional data bus for transmission to the specified port.
        0xD4 => "0xd4: CNC adr",
        // If NCY (no carry) call address.
        0xD5 => "0xd5: PUSH D",
        // Push register Pair D & E on stack
        0xD6 => "0xd6: SUI",
        // Subtract immediate from A
        // Subtracts the byte value from A.
        0xD8 => "0xd8: RC",
        // if carry flag is set, return.
        0xDA => "0xda: JC adr",
        // Conditional Jump
        // If the specified condition is true, control is transferred to the instruction whose address is specified in byte 3 and byte 2 of the current instruction; other- wise, control continues sequentially.
        0xDB => "0xdb: IN",
        // IN port
        // The data placed on the eight bit bi-directional data bus by the specified port is moved to register A.
        0xDE => "0xde: SPI",
        // Subtract immediate from A with borrow
        // The contents of the second byte of the instruction and the contents of the CY flag are both subtracted from the accumulator.
        // The result is placed in the accumulator.



        0xE0 => "0xeo: RPO",
        // 	if PO, RET
        0xE1 => "0xe1: POP H",
        // Pop register pair H & L off stack
        0xE2 => "0xe2: JPO adr",
        // Jump on parity odd
        0xE3 => "0xe3: XTHL",
        // Exchange top of stack, H & L.
        // The content of the L register is exchanged with the content of the memory location whose address is specified by the content of register SP.
        // The content of the H register is exchanged with the content of the memory location whose address is one more than the content of register SP.
        0xE5 => "0xe5: PUSH H",
        // Push register Pair H & L on stack
        0xE6 => "0xe6: ANI",
        // And immediate with A.
        // The content of the second byte of the instruction is logically anded with the contents of the accumulator.
        // The result is placed in the accumulator. The CY and AC flags are cleared.
        0xE9 => "0xe9: PCHL",
        // H & L to program counter.
        // The content of register H is moved to the high-order eight bits of register PC.
        // The content of register l is moved to the low-order eight bits of register PC.
        0xEB => "0xeb: XCHG",
        // Exchange D & E, H& L Registers
        0xEC => "0xec: CPE adr",
        0xEE => "0xee: XRI",
        // Exclusive Or immediate with A



        0xF0 => "0xf0: RP",
        // if P, RET
        0xF1 => "0xf1: POP PSW",
        // Pop A and Flags off stack
        0xF5 => "0xf5: PUSH PSW",
        // PSW means "processor state word", adds 1 to accumulator.
        // Push A and Flags on stack
        0xF6 => "0xf6: ORI",
        // does an or of A and the byte data, loads that into the accumulator.
        0xF8 => "0xf8: RM",
        // if M, return
        0xFA => "0xfa: JM adr",
        //	if M, PC <- adr
        // Jump on Minus
        // ok, cool. you can see this in 4-11
        // minus is if S = 1
        // S is a flag that stands for Sign
        0xFB => "0xfb: EI",
        // Enable interrupts
        // The interrupt system is enabled following the execu- tion of the next instruction.
        0xFC => "0xfc: CM adr",
        // if M, CALL adr
        0xFE => "0xfe: CPI",
        // Compare immediate
        // The content of the second byte of the instruction is subtracted from the accumulator.
        // The condition flags are set by the result of the subtraction.
        // The Z flag is set to 1 if (A) = (byte 2). The CY flag is set to 1 if (A) <(byte 2).
        0xFF => "0xff: RST 7",
        // CALL $38
        _ => "Unimplemented instruction",
    }
}
