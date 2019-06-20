// linear because it gets treated as one continuous address space

pub struct LinearMemory {
  pub data: Vec<u8>,
}

impl LinearMemory {
  fn get(&self, a: u16) -> u8 {
    self.data[usize::from(a)]
  }

  fn set(&mut self, a: u16, v: u8) {
    self.data[usize::from(a)] = v
  }

  // set_word, get_word??

  pub fn new() -> Self {
    Self {data: vec![0; 65536]}
  }
}

// usize is a pointer sized unsigned integer type.
// the size is how many bytes it takes to ref a particular location in memory.