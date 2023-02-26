mod font;
mod util;

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
        let mut memory = [0; 4096];

        for (i, &byte) in font::FONTSET.iter().enumerate() {
            memory[i] = byte;
        }
        VirtualMachine {
            memory: [0; 4096],
            registers: [0; 16],
            stack: [0; 16],
            stack_pointer: 0,
            screen: [0; 64 * 32],
            index_register: 0,
            program_counter: 0x200,
            delay_timer: 0,
            sound_timer: 0,
        }
    }

    fn load_rom(&mut self, rom: &[u8]) {
        for (i, &byte) in rom.iter().enumerate() {
            self.memory[i + 0x200] = byte;
        }
    }

    fn fetch_opcode(&self) -> u16 {
        let pc: usize = self.program_counter.into();

        (self.memory[pc] as u16) << 8 | self.memory[pc + 1] as u16
    }

    fn execute_opcode(&mut self, opcode: u16) {
        match opcode & 0xF000 {
            0x0000 => match opcode & 0x000F {
                0x0000 => {
                    // 00E0: Clear display
                    self.screen.iter_mut().for_each(|x| *x = 0);
                    self.program_counter += 2;
                }
                0x000E => {
                    // 00EE
                    self.program_counter = self.stack[self.stack_pointer as usize - 1];
                    self.stack_pointer -= 1;
                }
                _ => panic!("Unknown opcode: {:X}", opcode),
            },
            0x1000 => {
                // 1NNN
                let address = opcode & 0x0FFF;
                self.program_counter = address;
            }
            0x2000 => {
                // 2NNN
                self.stack[self.stack_pointer as usize] = self.program_counter;
                self.stack_pointer += 1;

                let address = opcode & 0x0FFF;
                self.program_counter = address;
            }
            0x3000 => {
                // 3XNN
                let register_idx = ((opcode & 0x0F00) >> 8) as usize;
                let value = (opcode & 0x00FF) as u8;

                if self.registers[register_idx] == value {
                    self.program_counter += 4;
                } else {
                    self.program_counter += 2;
                }
            }
            0x4000 => {
                // 4XNN
                let register_x = ((opcode & 0x0F00) >> 8) as usize;
                let value = (opcode & 0x00FF) as u8;

                if self.registers[register_x] != value {
                    self.program_counter += 4;
                } else {
                    self.program_counter += 2;
                }
            }
            0x5000 => {
                // 5XY0
                let register_x = ((opcode & 0x0F00) >> 8) as usize;
                let register_y = ((opcode & 0x00F0) >> 4) as usize;

                if self.registers[register_x] == self.registers[register_y] {
                    self.program_counter += 4;
                } else {
                    self.program_counter += 2;
                }
            }
            0x6000 => {
                // 6XNN
                let register_x = ((opcode & 0x0F00) >> 8) as usize;
                let value = (opcode & 0x00FF) as u8;

                self.registers[register_x] = value;
                self.program_counter += 2;
            }
            0x7000 => {
                // 7XNN
                let register_x = ((opcode & 0x0F00) >> 8) as usize;
                let value = (opcode & 0x00FF) as u8;

                self.registers[register_x] = self.registers[register_x].saturating_add(value);
                self.program_counter += 2;
            }
            _ => panic!("Unknown opcode: {:X}", opcode),
        }
    }
}
