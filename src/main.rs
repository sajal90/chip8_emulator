use sdl2;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
mod cpu;
use cpu::Cpu;

const WIDTH : usize = 64;
const HEIGHT : usize = 32;
const SCALE : u32 = 15;

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
