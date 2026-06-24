use rand::random;
use sdl2;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

const MAX_MEMORY : usize = 1 << 12; // 4096 bytes
const WIDTH : usize = 64;
const HEIGHT : usize = 32;
const SCALE : u32 = 15;
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
            },
            // Return from subroutine
            (0, 0, 0xE, 0xE) => {
                let ret_addr = self.pop();
                self.pc = ret_addr;
            },
            // Jump to NNN
            (1, _, _, _) => {
                self.pc = op & 0xFFF;
            },
            // Call Subroutine
            (2, _, _, _) => {
                self.push(self.pc);
                self.pc = op & 0xFFF;
            },
            // skip if VX(<x> register) == NN
            (3, _, _, _) => {
                let x = digit2 as usize;
                let nn = (op & 0xFF) as u8;
                if self.v[x] == nn {
                    self.pc += 2;
                }
            },
            // skip if VX != NN
            (4, _, _, _) => {
                let x = digit2 as usize;
                let nn = (op & 0xFF) as u8;
                if self.v[x] != nn {
                    self.pc += 2;
                }
            },
            // skip if VX == VY. x is digit2, y is digit3
            (5, _, _, _) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                if self.v[x] == self.v[y] {
                    self.pc += 2;
                }
            },
            // set VX == NN
            (6, _, _, _) => {
                let x = digit2 as usize;
                self.v[x] = (op & 0xFF) as u8;
            },
            // set VX += NN
            (7, _, _, _) => {
                let x = digit2 as usize;
                self.v[x] = self.v[x].wrapping_add((op & 0xFF) as u8);
            },
            // set VX = VY;
            (8, _, _, 0) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                self.v[x] = self.v[y];
            },
            // bitwise OR
            (8, _, _, 1) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                self.v[x] |= self.v[y];
            },
            // bitwise AND
            (8, _, _, 2) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                self.v[x] &= self.v[y];
            },
            // bitwise XOR
            (8, _, _, 3) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                self.v[x] ^= self.v[y];
            },
            // set v[x] += v[y];
            (8, _, _, 4) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                let (new_vx, carry) = self.v[x].overflowing_add(self.v[y]);
                let new_vf = if carry { 1 } else { 0 };
                self.v[x] = new_vx;
                self.v[0xF] = new_vf;
            },
            // set v[x] -= v[y]
            (8, _, _, 5) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                let (new_vx, borrow) = self.v[x].overflowing_sub(self.v[y]);
                let new_vf = if borrow { 0 } else { 1 };
                self.v[x] = new_vx;
                self.v[0xF] = new_vf;
            },
            // single right shift;
            (8, _, _, 6) => {
                let x = digit2 as usize;
                self.v[0xF] = self.v[x] & 1;
                self.v[x] >>= 1;
            },
            // set v[x] = v[y] - v[x]
            (8, _, _, 7) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                let (diff, borrow) = self.v[y].overflowing_sub(self.v[x]);
                self.v[0xF] = if borrow { 0 } else { 1 };
                self.v[x] = diff;
            },
            // single left shift
            (8, _, _, 0xE) => {
                let x = digit2 as usize;
                self.v[0xF] = (self.v[x] >> 7) & 1;
                self.v[x] <<= 1;
            },
            // skip if VX != VY
            (9, _, _, 0) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                if self.v[x] != self.v[y] {
                    self.pc += 2;
                }
            },
            // set IR = NNN
            (0xA, _, _, _) => {
                self.i = op & 0xFFF;
            },
            // jump to v[0] + NNN
            (0xB, _, _, _) => {
                self.pc = (self.v[0] as u16) + (op & 0xFFF);
            },
            // set v[x] = rand() & NN
            (0xC, _, _, _) => {
                let x = digit2 as usize;
                let r : u8 = random();
                self.v[x] = r & (op as u8 & 0xFF);
            },
            // draw sprite
            (0xD, _, _, _) => {
                let x_coord = self.v[digit2 as usize] as u16;
                let y_coord = self.v[digit3 as usize] as u16;
                let num_rows = digit4;
                let mut flipped = false;

                for y_line in 0..num_rows {
                    let addr = self.i + y_line as u16;
                    let pixels = self.memory[addr as usize];

                    for x_line in 0..8 {
                        if (pixels & (0b1000_0000 >> x_line)) != 0 {
                            let x = (x_coord + x_line) as usize % WIDTH;
                            let y = (y_coord + y_line) as usize % HEIGHT;
                            let idx = x + WIDTH * y;
                            flipped |= self.display[idx];
                            self.display[idx] ^= true;
                        }
                    }
                }

                if flipped {
                    self.v[0xF] = 1;
                } else {
                    self.v[0xF] = 0;
                }
            },
            // skip if key in v[x] is pressed
            (0xE, _, 9, 0xE) => {
                let x = digit2 as usize;
                if self.keys[self.v[x] as usize] {
                    self.pc += 2;
                }
            },
            // skip if key in v[x] isn't pressed
            (0xE, _, 0xA, 1) => {
                let x = digit2 as usize;
                if !self.keys[self.v[x] as usize] {
                    self.pc += 2;
                }
            },
            // set v[x] to dt
            (0xF, _, 0, 7) => {
                let x = digit2 as usize;
                self.v[x] = self.dt;
            },
            // wait for any key press
            (0xF, _, 0, 0xA) => {
                let x = digit2 as usize;
                let mut pressed = false;
                for i in 0..self.keys.len() {
                    if self.keys[i] {
                        self.v[x] = i as u8;
                        pressed = true;
                        break;
                    }
                }
                if !pressed {
                    self.pc -= 2;
                }
            },
            // set dt to v[x]
            (0xF, _, 1, 5) => {
                let x = digit2 as usize;
                self.dt = self.v[x];
            },
            // set st to v[x]
            (0xF, _, 1, 8) => {
                let x = digit2 as usize;
                self.st = self.v[x];
            },
            // set IR += v[x]
            (0xF, _, 1, 0xE) => {
                let x = digit2 as usize;
                self.i = self.i.wrapping_add(self.v[x] as u16);
            },
            // set IR to font addr
            (0xF, _, 2, 9) => {
                let x = digit2 as usize;
                self.i = (self.v[x] as u16) * 5;
            },
            // set IR to BCD(binary coded decimal) of v[x]
            (0xF, _, 3, 3) => {
                let x = digit2 as usize;
                let mut dec_num = self.v[x];
                for i in 0..=2 {
                    self.memory[self.i as usize + 2 - i] = dec_num % 10;
                    dec_num = dec_num / 10;
                }
            },
            // store from v[0] to v[x] into IR
            (0xF, _, 5, 5) => {
                let x = digit2 as usize;
                let i = self.i as usize;
                for idx in 0..=x {
                    self.memory[i + idx] = self.v[idx];
                }
            },
            // load IR into v[0] to v[x]
            (0xF, _, 6, 5) => {
                let x = digit2 as usize;
                let i = self.i as usize;
                for idx in 0..=x {
                    self.v[idx] = self.memory[i+idx];
                }
            },
            (_, _, _, _) => unimplemented!("Unimplemented opcode: {}", op),
        }
    }
}

fn map_key(keycode: Keycode) -> Option<usize> {
    match keycode {
        Keycode::Num1 => Some(0x1),
        Keycode::Num2 => Some(0x2),
        Keycode::Num3 => Some(0x3),
        Keycode::Num4 => Some(0xC),
        Keycode::Q    => Some(0x4),
        Keycode::W    => Some(0x5),
        Keycode::E    => Some(0x6),
        Keycode::R    => Some(0xD),
        Keycode::A    => Some(0x7),
        Keycode::S    => Some(0x8),
        Keycode::D    => Some(0x9),
        Keycode::F    => Some(0xE),
        Keycode::Z    => Some(0xA),
        Keycode::X    => Some(0x0),
        Keycode::C    => Some(0xB),
        Keycode::V    => Some(0xF),
        _             => None,
    }
}

fn main() {
    let rom_path = std::env::args().nth(1).expect("please provide a rom path");
    let rom = std::fs::read(rom_path).expect("failed to read rom");

    let mut cpu = Cpu::new();
    cpu.load_rom(&rom);

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("CHIP8", WIDTH as u32 * SCALE, HEIGHT as u32 * SCALE)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    'gameloop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'gameloop,
                    Event::KeyDown { keycode: Some(kc), .. } => {
                        if let Some(i) = map_key(kc) {
                            cpu.keys[i] = true;
                        }
                    },
                    Event::KeyUp { keycode: Some(kc), .. } => {
                        if let Some(i) = map_key(kc) {
                            cpu.keys[i] = false;
                        }
                    },
                    _ => {}
            }
        }
        cpu.tick();
        cpu.tick_timers();
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();
        canvas.set_draw_color(Color::WHITE);

        for (i, &pixel) in cpu.display.iter().enumerate() {
            if pixel {
                let x = (i % WIDTH) as i32;
                let y = (i / WIDTH) as i32;
                canvas.fill_rect(Rect::new(
                            x * SCALE as i32,
                            y * SCALE as i32,
                            SCALE,
                            SCALE,
                            )).unwrap();
            }
        }

        canvas.present();
        std::thread::sleep(std::time::Duration::from_micros(2000));
    }
}
