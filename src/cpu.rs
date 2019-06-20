use std::rc::Rc; // RC - single threaded reference counted pointer
use std::cell::RefCell; // a mutable memory location

use linear_memory;
mod register;

pub struct Cpu {
  pub reg: Register,
  pub mem: Rc<RefCell<linear_memory::LinearMemory>>,
}

impl Cpu {
  pub fn boot(mem: Rc<RefCell<linear_memory::LinearMemory>>) -> Self {
    Self {
      reg: register::Register::boot(),
      mem,
    }
  }
}