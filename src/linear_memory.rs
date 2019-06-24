  pub struct LinearMemory {
    // linear because it gets treated as one continuous address space
    pub data: Vec<u8>,
  }

  impl LinearMemory {
    pub fn get(&self, a: u16) -> u8 {
      self.data[usize::from(a)]
    }

    pub fn set(&mut self, a: u16, v: u8) {
      self.data[usize::from(a)] = v
    }

    pub fn get_word(&mut self, a: u16) -> u16 {
      println!("{:#015b}", u16::from(self.get(a)));
      println!("{:#015b}", (u16::from(self.get(a + 1)) << 8));
      println!("{:#015b}", u16::from(self.get(a)) | (u16::from(self.get(a + 1)) << 8));
      u16::from(self.get(a)) | (u16::from(self.get(a + 1)) << 8)
      // makes a u16 out of the variable
      // then does a logical or with the next byte bit shifted by 8.
      // remember, logical or compares the bits on both numbers, and puts a 1 in the result if either have a 1
      // eg: a =  0b0000011010100
      // b =      0b1100000000000
      // result = 0b1100011010100
      // why bitwise or?
      // why do we bitshift it by 8 bits (1 byte) to the left?
      // bitshift to the left is like a multiply, to the right is like divide.
      // 
      
    }

    pub fn set_word() {

    }

    pub fn new() -> Self {
      Self {data: vec![0; 65536]}
    }
  }

// usize is a pointer sized unsigned integer type.
// the size is how many bytes it takes to ref a particular location in memory.