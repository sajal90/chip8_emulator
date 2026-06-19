const MAX_MEMORY : usize = 1 << 12; // 4096 bytes
const WIDTH : usize = 64;
const HEIGHT : usize = 32;

/* Chip8 contains 16 general purpose registers, each of 8 bits, a index register
 * and a program counter register of 16 bits.
 * It has a memory of 4096 bytes
 */
struct Cpu {
    v: [u8; 16],
    i: u16,
    pc: u16,
    memory: [u8; MAX_MEMORY],
}

impl Cpu {
    fn new() -> Self {
        Cpu {
            v: [0; 16],
            i: 0,
            pc: 0x200, // Programs starts here
            memory: [0; MAX_MEMORY],
        }
    }

    fn load_rom(&mut self, data: &[u8]) {
        for (i, byte) in data.iter().enumerate() {
            self.memory[0x200 + i] = *byte;
        }
    }

    fn fetch(&mut self) -> u16 {
        let fir = self.memory[self.pc as usize] as u16;
        let sec = self.memory[self.pc as usize + 1] as u16;
        self.pc += 2;
        (fir << 8) | sec
    }
}


fn main() {
    let mut cpu = Cpu::new();

    let rom: &[u8] = &[0x00, 0xE0]; // fake rom for test
    cpu.load_rom(rom);

    let opcode = cpu.fetch();
    println!("Cpu started at {}", cpu.pc);
}
