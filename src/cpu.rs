use std::cell::RefCell;
use std::rc::Rc; // RC - single threaded reference counted pointer // a mutable memory location

use crate::linear_memory::LinearMemory;
use crate::register::Register;

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

  fn get_current_opcode(&mut self) -> u8 {
    let byte = self.mem.borrow().get(self.reg.pc);
    self.reg.pc += 1;
    byte
  }

  pub fn next(&mut self) -> u32 {
    let opcode = self.get_current_opcode();
    println!("{}", opcode);
    1 // TODO return something for real.
  }

  pub fn step(&mut self) -> u32 {
    // check some stuff in here.
    let cycles = self.next();
    self.step_cycles += cycles;
    cycles

  }
}