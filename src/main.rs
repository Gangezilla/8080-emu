use std::rc::Rc; // RC - single threaded reference counted pointer
use std::cell::RefCell; // a mutable memory location
use std::path::Path;
use std::fs::File;
use std::io::Read;

mod linear_memory;
mod register;
mod cpu;
mod asm;

struct Invaders {
    cpu: cpu::Cpu,
    mem: Rc<RefCell<linear_memory::LinearMemory>>
}

impl Invaders {
    fn boot(rom: impl AsRef<Path>) -> Self {
        let mem = Rc::new(RefCell::new(linear_memory::LinearMemory::new()));
        let mut file = File::open(rom).unwrap();
        let mut buf = Vec::new();
        file.read_to_end(&mut buf).unwrap();
        mem.borrow_mut().data[0x00..buf.len()].clone_from_slice(&buf[..]);

        Self {
            cpu: cpu::Cpu::boot(mem.clone()),
            mem,
        }
    }

    fn next(&mut self) -> u32 {
        let opcode = self.mem.borrow().get(self.cpu.reg.pc);
        // get the opcode here to determine if the arcade machine should handle it
        // else, we let the cpu deal with it.
        match opcode {
            // TODO
            _ => ()
        }
        self.cpu.step();

        1 // TODO: SOMETHING THAT WORKS
    }

    fn step(&mut self) {
        let mut cycle = 0;
        cycle += self.next();
        // check if we should be dealing with interrupts
        // draw stuff on screen
        // handle keypresses
        
    }
}

fn main() {
    // init CPU with invaders rom
    let mut game = Invaders::boot("./static/invaders");
    loop {
        // run if game is running.
        game.step();
    }
}
