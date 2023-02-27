mod font;

pub struct VirtualMachine {
    memory: [u8; 4096],
    registers: [u8; 16],
    stack: [u16; 16],
    stack_pointer: u8,
    screen: [u8; 64 * 32],
    index_register: u16,
    program_counter: u16,
    delay_timer: u8,
    sound_timer: u8,
    pub key_state: [bool; 16],
    blocked_on_key_press: bool,
}

impl VirtualMachine {
    pub fn new() -> VirtualMachine {
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
            key_state: [false; 16],
            blocked_on_key_press: false,
        }
    }

    fn load_rom(&mut self, rom: &[u8]) {
        for (i, &byte) in rom.iter().enumerate() {
            self.memory[i + 0x200] = byte;
        }
    }

    pub fn run_cyle(&mut self) {
        if !self.blocked_on_key_press {
            let opcode = self.fetch_opcode();
            self.execute_opcode(opcode);
            
            self.sound_timer = self.sound_timer.saturating_sub(1);
            self.delay_timer = self.delay_timer.saturating_sub(1);
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

                    self.program_counter += 2;
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
                let register_idx = Self::get_register_x(opcode);
                let value = (opcode & 0x00FF) as u8;

                if self.registers[register_idx] == value {
                    self.program_counter += 4;
                } else {
                    self.program_counter += 2;
                }
            }
            0x4000 => {
                // 4XNN
                let register_x = Self::get_register_x(opcode);
                let value = (opcode & 0x00FF) as u8;

                if self.registers[register_x] != value {
                    self.program_counter += 4;
                } else {
                    self.program_counter += 2;
                }
            }
            0x5000 => {
                // 5XY0
                let register_x = Self::get_register_x(opcode);
                let register_y = Self::get_register_y(opcode);

                if self.registers[register_x] == self.registers[register_y] {
                    self.program_counter += 4;
                } else {
                    self.program_counter += 2;
                }
            }
            0x6000 => {
                // 6XNN
                let register_x = Self::get_register_x(opcode);
                let value = (opcode & 0x00FF) as u8;

                self.registers[register_x] = value;
                self.program_counter += 2;
            }
            0x7000 => {
                // 7XNN
                let register_x = Self::get_register_x(opcode);
                let value = (opcode & 0x00FF) as u8;

                self.registers[register_x] = self.registers[register_x].saturating_add(value);
                self.program_counter += 2;
            }
            0xF000 => match opcode & 0x000F {
                0x0007 => {
                    // FX07
                    let register_x = Self::get_register_x(opcode);
                    self.registers[register_x] = self.delay_timer;

                    self.program_counter += 2;
                }
                0x000A => {
                    // FX0A
                    self.blocked_on_key_press = true;
                }
                0x0005 => match opcode & 0x00F0 {
                    0x0010 => {
                        // FX15
                        let register_x = Self::get_register_x(opcode);
                        self.delay_timer = self.registers[register_x];

                        self.program_counter += 2;
                    }
                    0x0050 => {
                        // FX55
                        let register_x = Self::get_register_x(opcode);
                        let i = self.index_register as usize;
                        for (idx, mem) in self.memory[i..=(i + register_x)].iter_mut().enumerate() {
                            *mem = self.registers[idx];
                        }

                        self.program_counter += 2;
                    }
                    0x0060 => {
                        // FX65
                        let register_x = Self::get_register_x(opcode);
                        let i = self.index_register as usize;
                        for (idx, mem) in self.memory[i..=(i + register_x)].iter().enumerate() {
                            self.registers[idx] = *mem;
                        }

                        self.program_counter += 2;
                    }
                    _ => panic!("Unknown opcode: {:X}", opcode),
                },
                0x0008 => {
                    // FX18
                    let register_x = Self::get_register_x(opcode);
                    self.sound_timer = self.registers[register_x];

                    self.program_counter += 2;
                }
                0x000E => {
                    // FX1E
                    let register_x = Self::get_register_x(opcode);
                    self.index_register += self.registers[register_x] as u16;

                    self.program_counter += 2;
                }
                0x0009 => {
                    // FX29
                    let register_x = Self::get_register_x(opcode);
                    self.index_register =
                        Self::get_sprite_address(self.registers[register_x]) as u16;

                    self.program_counter += 2;
                }
                0x0003 => {
                    // FX33
                    let register_x = Self::get_register_x(opcode);
                    let val = self.registers[register_x];
                    let i = self.index_register as usize;

                    self.memory[i] = val / 100;
                    self.memory[i] = (val / 10) % 10;
                    self.memory[i + 2] = val % 10;

                    self.program_counter += 2;
                }
                _ => panic!("Unknown opcode: {:X}", opcode),
            },
            _ => panic!("Unknown opcode: {:X}", opcode),
        }
    }

    fn get_sprite_address(sprite_id: u8) -> u8 {
        assert!(sprite_id <= 0x0F);

        sprite_id * 5
    }

    pub fn complete_fx0a(&mut self, key_value: u8) {
        let opcode = self.fetch_opcode();
        let register_x = Self::get_register_x(opcode);

        self.registers[register_x] = key_value;

        self.blocked_on_key_press = false;
        self.program_counter += 2;
    }

    fn get_register_x(opcode: u16) -> usize {
        ((opcode & 0x0F00) >> 8) as usize
    }

    fn get_register_y(opcode: u16) -> usize {
        ((opcode & 0x0F0) >> 4) as usize
    }

}
