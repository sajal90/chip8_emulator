const MAX_MEMORY : usize = 1 << 12; // 4096 bytes
const WIDTH : usize = 64;
const HEIGHT : usize = 32;

/* Chip8 contains 16 general purpose registers, each of 8 bits, a index register
 * and a program counter register of 16 bits.
 * It has a memory of 4096 bytes.
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
        };
        cpu.memory[0..80].copy_from_slice(&FONT);
        cpu
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

const FONT: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

fn main() {
    let mut cpu = Cpu::new();

    let rom: &[u8] = &[0x00, 0xE0]; // fake rom for test
    cpu.load_rom(rom);

    let opcode = cpu.fetch();
    println!("Cpu started at {}", cpu.pc);
}
