const MAX_MEMORY : i32 = 1 << 12;
const WIDTH : i32 = 64;
const HEIGHT : i32 = 32;

/*
 * chip-8 contains 16 8-bit general purpose registers, a 16-bit program counter
 * and a 16-bit index register
 */
// TODO: refactor R_I and R_PC, as they are 16-bit, unlike other regs
enum Registers {
    V0,
    V1,
    V2,
    V3,
    V4,
    V5,
    V6,
    V7,
    V8,
    V9,
    VA,
    VB,
    VC,
    VD,
    VE,
    VF,   // also used as flag register for overflow operations
    I,
    PC,
}


fn main() {
    println!("Hello, world!");
}
