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
}


fn main() {
    let cpu = Cpu::new();
    println!("Cpu started at {}", cpu.pc);
}
