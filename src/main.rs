use std::rc::Rc; // RC - single threaded reference counted pointer
use std::cell::RefCell; // a mutable memory location
use std::path::Path;
use std::fs::File;
use std::io::Read;

mod linear_memory;
mod register;
mod cpu;

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

    fn step(&mut self) {
        println!("stepping")
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
