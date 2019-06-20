#[derive(Default)]
pub struct Register {
  pub a: u8,
  pub f: u8, // indirectly accessible
  pub b: u8,
  pub c: u8,
  pub d: u8,
  pub e: u8,
  pub h: u8,
  pub l: u8,
  pub sp: u16, // stack pointer
  pub pc: u16, // program counter
}

pub enum Flag {
  S = 7, // Sign Flag
  Z = 6, // Zero Flag
  A = 4, // AC, Auxilary Cary Flag
  P = 2, // Parity Flag
  C = 0, // Carry Flag
}

impl Register {
  // lots of get and set above register functions...

  pub fn get_flag(&self, f: Flag) -> bool {
    // 
  }

  pub fn set_flag(&mut self, f: Flag, v: bool) {
    //
  }

  pub fn boot() -> Self {
    let mut reg = Self::default();
    // reg.f = 0b0000_0010; // why?
    r
  }
}