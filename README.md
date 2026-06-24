
# chip8

A CHIP-8 emulator written in Rust.

## Dependencies

- Rust
- SDL2

On Fedora:
```
sudo dnf install SDL2-devel
```

## Building

```
cargo build --release
```

## Running

ROMs are included in the `games/` folder.

```
cargo run -- games/<game_name>
```

## Controls

CHIP-8 uses a 16-key hex keypad mapped to:

```
CHIP-8   Keyboard
1 2 3 C  1 2 3 4
4 5 6 D  Q W E R
7 8 9 E  A S D F
A 0 B F  Z X C V
```

## Status

- All 35 opcodes implemented
- Display rendering via SDL2
- Keyboard input
- Delay and sound timers
- Audio not yet implemented
