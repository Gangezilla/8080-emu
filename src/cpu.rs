use std::rc::Rc; // RC - single threaded reference counted pointer
use std::cell::RefCell; // a mutable memory location

use crate::linear_memory::LinearMemory;
use crate::register::Register;

pub struct Cpu {
  pub reg: Register,
  pub mem: Rc<RefCell<LinearMemory>>,
}

impl Cpu {
  pub fn boot(mem: Rc<RefCell<LinearMemory>>) -> Self {
    Self {
      reg: Register::boot(),
      mem,
    }
  }
}