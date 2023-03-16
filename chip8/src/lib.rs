mod font;

pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;

pub struct VirtualMachine {
    memory: [u8; 4096],
    registers: [u8; 16],
    stack: [u16; 16],
    stack_pointer: u8,
    screen: [u8; SCREEN_HEIGHT * SCREEN_WIDTH],
    index_register: u16,
    program_counter: u16,
    delay_timer: u8,
    sound_timer: u8,
    pub key_state: [bool; 16],
    pub blocked_on_key_press: bool,
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
            screen: [0; SCREEN_HEIGHT * SCREEN_WIDTH],
            index_register: 0,
            program_counter: 0x200,
            delay_timer: 0,
            sound_timer: 0,
            key_state: [false; 16],
            blocked_on_key_press: false,
        }
    }

    pub fn load_rom(&mut self, rom: &[u8]) {
        let free_space = &mut self.memory[0x200..];
        for (byte, address) in rom.iter().zip(free_space) {
            *address = *byte;
        }
    }

    pub fn execute_instruction(&mut self) {
        if !self.blocked_on_key_press {
            let opcode = self.fetch_opcode();

            self.execute_opcode(opcode);
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

                self.registers[register_x] = self.registers[register_x].wrapping_add(value);
                self.program_counter += 2;
            }
            0x8000 => match opcode & 0x000F {
                0x0 => {
                    // 8XY0
                    self.registers[Self::get_register_x(opcode)] =
                        self.registers[Self::get_register_y(opcode)];

                    self.program_counter += 2;
                }
                0x1 => {
                    // 8XY1
                    let vx = self.registers[Self::get_register_x(opcode)];
                    self.registers[Self::get_register_x(opcode)] =
                        vx | self.registers[Self::get_register_y(opcode)];

                    self.program_counter += 2;
                }
                0x2 => {
                    // 8XY2
                    let vx = self.registers[Self::get_register_x(opcode)];
                    self.registers[Self::get_register_x(opcode)] =
                        vx & self.registers[Self::get_register_y(opcode)];

                    self.program_counter += 2;
                }
                0x3 => {
                    // 8XY3
                    let vx = self.registers[Self::get_register_x(opcode)];
                    self.registers[Self::get_register_x(opcode)] =
                        vx ^ self.registers[Self::get_register_y(opcode)];

                    self.program_counter += 2;
                }
                0x4 => {
                    // 8XY4
                    let vx = self.registers[Self::get_register_x(opcode)];
                    let vy = self.registers[Self::get_register_y(opcode)];
                    let (sum, did_overflow) = vx.overflowing_add(vy);

                    if did_overflow {
                        self.registers[0xF] = 1;
                    } else {
                        self.registers[0xF] = 0;
                    }

                    self.registers[Self::get_register_x(opcode)] = sum;

                    self.program_counter += 2;
                }
                0x5 => {
                    // 8XY5

                    let vx = self.registers[Self::get_register_x(opcode)];
                    let vy = self.registers[Self::get_register_y(opcode)];

                    if vx > vy {
                        self.registers[0xF] = 1;
                    } else {
                        self.registers[0xF] = 0;
                    }

                    self.registers[Self::get_register_x(opcode)] = vx.wrapping_sub(vy);

                    self.program_counter += 2;
                }
                0x6 => {
                    // 8XY6

                    let vx = self.registers[Self::get_register_x(opcode)];
                    self.registers[Self::get_register_x(opcode)] = vx.wrapping_shr(1);
                    self.registers[0xF] = vx & 1;

                    self.program_counter += 2;
                }
                0x7 => {
                    // 8XY7
                    let vx = self.registers[Self::get_register_x(opcode)];
                    let vy = self.registers[Self::get_register_y(opcode)];

                    if vy > vx {
                        self.registers[0xF] = 1;
                    } else {
                        self.registers[0xF] = 0;
                    }

                    self.registers[Self::get_register_x(opcode)] = vy.wrapping_sub(vx);

                    self.program_counter += 2;
                }
                0xE => {
                    // 8XYE

                    let vx = self.registers[Self::get_register_x(opcode)];
                    self.registers[Self::get_register_x(opcode)] = vx.wrapping_shl(1);
                    self.registers[0xF] = (vx >> 7) & 1;

                    self.program_counter += 2;
                }
                _ => panic!("Unknown opcode: {:X}", opcode),
            },
            0x9000 => {
                // 9XY0
                let register_x = Self::get_register_x(opcode);
                let register_y = Self::get_register_y(opcode);

                if self.registers[register_x] != self.registers[register_y] {
                    self.program_counter += 4
                } else {
                    self.program_counter += 2;
                }
            }
            0xA000 => {
                // ANNN
                let address = opcode & 0x0FFF;
                self.index_register = address;

                self.program_counter += 2;
            }
            0xB000 => {
                // BNNN
                let address = opcode & 0x0FFF;
                let v0 = self.registers[0] as u16;
                self.index_register = address + v0;

                self.program_counter += 2;
            }
            0xC000 => {
                // CXNN
                let register_x = Self::get_register_x(opcode);
                let random_byte = fastrand::u8(..);

                self.registers[register_x] = random_byte & ((opcode & 0x00FF) as u8);

                self.program_counter += 2;
            }
            0xD000 => {
                // DXYN
                let vx = self.registers[Self::get_register_x(opcode)] as usize;
                let vy = self.registers[Self::get_register_y(opcode)] as usize;
                let height = (opcode & 0x000F) as usize;

                self.registers[0xF] = 0;

                let wrap = vx >= SCREEN_WIDTH || vy >= SCREEN_HEIGHT;

                for (y, mut row) in (vy..(vy + height)).enumerate() {
                    let byte = self.memory[self.index_register as usize + y];

                    if wrap {
                        row %= SCREEN_HEIGHT;
                    } else if row >= SCREEN_HEIGHT {
                        break;
                    }

                    for (x, mut col) in (vx..(vx + 8)).enumerate() {
                        if wrap {
                            col %= SCREEN_WIDTH
                        } else if col >= SCREEN_WIDTH {
                            break;
                        }
                        let pixel = self.get_pixel_mut(row, col);
                        let sprite_value = byte & (0x80 >> x);

                        let old_pixel = *pixel;

                        if sprite_value != 0 {
                            *pixel ^= 1;

                            if old_pixel == 1 && *pixel == 0 {
                                self.registers[0xF] = 1;
                            }
                        }
                    }
                }

                self.program_counter += 2;
            }
            0xE000 => match opcode & 0x00FF {
                0x009E => {
                    // EX9E
                    let register_x = Self::get_register_x(opcode);
                    let vx = self.registers[register_x];

                    if self.key_state[vx as usize] {
                        self.program_counter += 4;
                    } else {
                        self.program_counter += 2;
                    }
                }
                0x00A1 => {
                    // EXA1
                    let register_x = Self::get_register_x(opcode);
                    let vx = self.registers[register_x];

                    if !self.key_state[vx as usize] {
                        self.program_counter += 4;
                    } else {
                        self.program_counter += 2;
                    }
                }
                _ => panic!("Unknown opcode: {:X}", opcode),
            },
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
                    self.clear_key_state();
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

                    assert!(i + 2 <= self.memory.len());

                    self.memory[i] = val / 100;
                    self.memory[i + 1] = (val / 10) % 10;
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

    fn get_pixel_mut(&mut self, row: usize, col: usize) -> &mut u8 {
        &mut self.screen[row * SCREEN_WIDTH + col]
    }

    pub fn screen_rows(&self) -> impl Iterator<Item = &[u8]> {
        self.screen.chunks_exact(SCREEN_WIDTH)
    }

    fn clear_key_state(&mut self) {
        self.key_state.fill(false);
    }

    pub fn decrement_timers(&mut self) {
        self.sound_timer = self.sound_timer.saturating_sub(1);
        self.delay_timer = self.delay_timer.saturating_sub(1);
    }
}
