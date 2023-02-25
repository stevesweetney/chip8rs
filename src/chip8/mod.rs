mod font;

struct VirtualMachine {
    memory: [u8; 4096],
    registers: [u8; 16],
    stack: [u16; 16],
    stack_pointer: u8,
    screen: [u8; 64 * 32],
    index_register: u16,
    program_counter: u16,
    delay_timer: u8,
    sound_timer: u8,
}

impl VirtualMachine {
    fn new() -> VirtualMachine {
        VirtualMachine {
            memory: [0; 4096],
            registers: [0; 16],
            stack: [0; 16],
            stack_pointer: 0,
            screen: [0; 64 * 32],
            index_register: 0,
            program_counter: 0,
            delay_timer: 0,
            sound_timer: 0,
        }
    }
}
