use std::cell::RefCell;
use std::rc::Rc; 
// RC - single threaded reference counted pointer // a mutable memory location

use crate::linear_memory::LinearMemory;
use crate::register::Register;
use crate::asm;

pub struct Cpu {
  pub reg: Register,
  pub mem: Rc<RefCell<LinearMemory>>,

  step_cycles: u32,
}

impl Cpu {
  pub fn boot(mem: Rc<RefCell<LinearMemory>>) -> Self {
    Self {
      reg: Register::boot(),
      mem,
      step_cycles: 0,
      // cycle is a cpu thing, does 4 steps. fetch instruction, decode, execute, store results.
    }
  }

  fn get_current_instruction(&mut self) -> u8 {
    let instruction = self.mem.borrow().get(self.reg.pc);
    self.reg.pc += 1;
    instruction
  }

  // Integrated Management Module
  // i think its just a generic term for the components of the processor?
  fn imm_dw(&mut self) -> u16 {
    let v = self.mem.borrow_mut().get_word(self.reg.pc);
    self.reg.pc += 2;
    v
  }

  pub fn next(&mut self) -> u32 {
    let opcode = self.get_current_instruction();
    // println!("{} PC={:04x} SP={:04x} A={:02x} F={:02x} B={:02x} C={:02x} D={:02x} E={:02x} H={:02x} L={:02x}",
    //   asm::convert_opcode(opcode),
    //   self.reg.pc,
    //   self.reg.sp,
    //   self.reg.a,
    //   self.reg.f,
    //   self.reg.b,
    //   self.reg.c,
    //   self.reg.d,
    //   self.reg.e,
    //   self.reg.h,
    //   self.reg.l,
    // );

    match opcode {
      // ACI Add Immediate to Accumulator with Carry
      // ADC Add Register or Memory to Accumulator with Carry
      // ADD Add Register or Memory to Accumulator
      // ADI Add Immediate to Accumulator
      // ANA Logical And Register or Memory with Accumulator
      // ANI Logical And Immediate with Acumulator
      // CALL Subroutines Instructions
      // CALL CALL Unconditional
      // CC Call On Carry
      // CM Call On Minus
      // CNC Call On No Carry
      // CNZ Call On No Zero
      // CP Call On Positive
      // CPE Call On Parity Even
      // CPO Call On Parity Odd
      // CZ Call On Zero
      // CMA Compliment Accumulator
      // CMC Compliment Carry
      // CMP Compare Memory or Register With Accumulator
      // CPI Compare Immediate With Accumulator
      // DAA Decimal Adjust Accumulator
      // DAD Double Add
      // DCR Decrement Register or Memory
      // DI Disable Interrupt
      // EI Enable Interrupt
      // HLT HALT
      // INPUT/OUTPUT Instructions
      // INR Increment Register or Memory
      // JUMP INSTRUCTIONS
      0xC3 => {
        let a = self.imm_dw();
      }
      // JC Jump If Carry
      // JM Jump If Minus
      // JMP Jump Unconditional
      // JNC Jump If No Carry
      // JNZ Jump If No Zero
      // JP Jump If Positive
      // JPE Jump on Parity Even
      // JPO Jump on Parity Odd
      // JZ Jump on Zero
      // LDA Load Accumulator Direct
      // LDAX Load Accumulator
      // LHLD Load HAnd L Direct
      // LXI Load Immediate Data
      // MVI Move Immediate Register or Memory
      // MOV Instruction
      // NOP No Operation
      0x00 => {},
      // ORA Logical Or Register or Memory With Accumulator
      // ORI Logical Or Immediate with Accumulator
      // OUT Output
      // PCHL Load Program Counter
      // POP Pop Data Off Stack
      // PUSH Push Data On Stack
      // RAL Rotate Accumulator Left Through Carry
      // RAR Rotate Accumulator Right Through Carry
      // RETURN FROM SUBROUTINE INSTRUCTIONS
      // RC Return If Carry
      // RET Return Unconditional
      // RM Return On Minus
      // RNC Return If No Carry
      // RNZ Return If No Zero
      // RP Return If Positive
      // RPE Return If Parity Even
      // RPO Return If Parity Odd
      // RZ Return If Zero
      // RLC Rotate Accumulator Left
      // RRC Rotate Accumulator Right
      // RST Restart
      // SBB Subtract Memory or Register from Accumulator
      // SBI Subtract Immediate from Accumulator With Borrow
      // SHLD Store H and L Direct
      // SPHL Load SP From H and L
      // STA Store Accumulator Direct
      // STAX Store Accumulator
      // STC Set Carry
      // SUB Subtract Register or Memory From Accumulator
      // SUI Subtract Immediate From Accumulator
      // XCHG Exchange Registers
      // XRA Logical Exclusive-Or Register or Memory With Accumulator
      // XRI Logical Exclusive-Or Immediate With Accumulator
      // XTHL Exchange Stack
      _ => {
        println!("{} PC={:04x} SP={:04x} A={:02x} F={:02x} B={:02x} C={:02x} D={:02x} E={:02x} H={:02x} L={:02x}",
          asm::convert_opcode(opcode),
          self.reg.pc,
          self.reg.sp,
          self.reg.a,
          self.reg.f,
          self.reg.b,
          self.reg.c,
          self.reg.d,
          self.reg.e,
          self.reg.h,
          self.reg.l,
        );
        panic!();
      }
    }
    1 // TODO return something for real.

  }

  pub fn step(&mut self) -> u32 {
    // check some stuff in here.
    let cycles = self.next();
    self.step_cycles += cycles;
    cycles

  }
}