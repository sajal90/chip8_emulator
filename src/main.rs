const MAX_MEMORY : usize = 1 << 12; // 4096 bytes
const WIDTH : usize = 64;
const HEIGHT : usize = 32;
const REG_COUNT : usize = 16;
const STACK_SIZE : usize = 16;
const NUM_KEYS: usize = 16;

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


/* Chip8 contains 16 general purpose registers, each of 8 bits, a index register
 * and a program counter register of 16 bits.
 * It has a memory of 4096 bytes, A small stack, 16 different keys, and two
 * timers; a dt (delay timer) and a st(sound timer).
 */
struct Cpu {
    v: [u8; REG_COUNT],
    i: u16,
    pc: u16,
    memory: [u8; MAX_MEMORY],
    display: [bool; WIDTH * HEIGHT],
    sp: u16,
    stack: [u16; STACK_SIZE],
    keys: [bool; NUM_KEYS],
    dt: u8,
    st: u8,
}

impl Cpu {
    fn new() -> Self {
        let mut cpu = Cpu {
            v: [0; 16],
            i: 0,
            pc: 0x200, // Programs starts here
            memory: [0; MAX_MEMORY],
            display: [false; WIDTH * HEIGHT],
            sp: 0,
            stack: [0; STACK_SIZE],
            keys: [false; NUM_KEYS],
            dt: 0,
            st: 0,
        };
        cpu.memory[0..80].copy_from_slice(&FONT);
        cpu
    }

    fn load_rom(&mut self, data: &[u8]) {
        for (i, byte) in data.iter().enumerate() {
            self.memory[0x200 + i] = *byte;
        }
    }

    fn push(&mut self, val: u16) {
        self.stack[self.sp as usize] = val;
        self.sp += 1;
    }

    fn pop(&mut self) -> u16 {
        self.sp -= 1;
        self.stack[self.sp as usize]
    }

    fn tick(&mut self) {
        let op = self.fetch();
        self.execute(op);
    }


    pub fn tick_timers(&mut self) {
        if self.dt > 0 {
            self.dt -= 1;
        }
        if self.st > 0 {
            if self.st == 1 {
                // TODO: Implement audio
            }
            self.st -= 1;
        }
    }

    fn fetch(&mut self) -> u16 {
        let fir = self.memory[self.pc as usize] as u16;
        let sec = self.memory[self.pc as usize + 1] as u16;
        self.pc += 2;
        (fir << 8) | sec
    }

    fn execute(&mut self, op: u16) {
        let digit1 = (op & 0xF000) >> 12;
        let digit2 = (op & 0x0F00) >> 8;
        let digit3 = (op & 0x00F0) >> 4;
        let digit4 = op & 0x000F;

        match (digit1, digit2, digit3, digit4) {
            // NOP
            (0, 0, 0, 0) => return,
            // clear screen
            (0, 0, 0xE, 0) => {
                self.display = [false; WIDTH * HEIGHT];
            }
            (_, _, _, _) => unimplemented!("Unimplemented opcode: {}", op),
        }
    }
}

fn main() {
    let mut cpu = Cpu::new();

    let rom: &[u8] = &[0x00, 0xE0]; // fake rom for test
    cpu.load_rom(rom);

    cpu.tick();
    println!("Cpu started at {}", cpu.pc);
}
