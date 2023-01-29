use self::{
    memory::Request,
    registers::{DReg, Reg},
};

mod memory;
mod registers;

pub struct GameboyColor {
    memory: memory::Memory,
    registers: registers::Registers,
}

pub type Cycles = usize;

impl GameboyColor {
    pub fn new() -> Self {
        Self {
            memory: memory::Memory::new(),
            registers: registers::Registers::new(),
        }
    }

    pub fn set_cartridge(&mut self, cartridge: &[u8]) -> Result<(), ()> {
        self.memory.set_cartridge(cartridge);
        self.registers.pc = 0x0100;
        Ok(())
    }

    pub fn step(&mut self) -> Cycles {
        self.handle_interrupts();
        let opcode = self.memory.next_program_byte(&mut self.registers.pc);

        println!("PC: 0x{:04X}, Op: 0x{opcode:02X}", self.registers.pc - 1);

        // TODO HANDLE FLAG CHANGES

        match opcode {
            0x00 => 4,

            0x01 => {
                self.registers[DReg::BC] =
                    u16::from_le_bytes(self.memory[Request::<2>(self.registers.pc)]);
                self.registers.pc += 2;
                12
            }

            // ld (bc), a
            0x02 => {
                self.memory[self.registers[DReg::BC]] = self.registers[Reg::A];
                8
            }
            // inc bc
            0x03 => {
                self.registers[DReg::BC] += 1;
                8
            }
            // inc register
            0x04 | 0x0C | 0x14 | 0x1C | 0x24 | 0x2C | 0x34 | 0x3C => {
                let reg = Reg::from(opcode >> 3 & 7);
                self.registers[reg] = self.registers.add(reg, 1, None);
                4
            }
            // dec register
            0x05 | 0x0D => {
                // TODO SET FLAGS
                let reg = Reg::from(opcode >> 3 & 7);
                self.registers[reg] = self.registers.sub(reg, 1, None);
                4
            }
            0x06 => {
                self.registers[Reg::B] = self.memory[self.registers.pc];
                self.registers.pc += 1;
                8
            }
            0x07 => {
                // TODO SET FLAGS
                let a = &mut self.registers[Reg::A];
                *a = u8::rotate_left(*a, 1); // copy to carry flag too
                4
            }
            0x08 => {
                self.registers.sp =
                    u16::from_be_bytes(self.memory[Request::<2>(self.registers.pc)]);
                self.registers.pc += 2;
                20
            }
            0x09 => {
                // TODO SET FLAGS
                self.registers[DReg::HL] += self.registers[DReg::BC];
                8
            }
            0x0A => {
                self.registers[Reg::A] = self.memory[self.registers[DReg::BC]];
                8
            }
            0x0B => {
                self.registers[Reg::B] -= 1;
                8
            }
            0x10 => {
                todo!("STOP opcode: low power standby");
            }
            0x18 => {
                let d = i8::from_ne_bytes([self.memory[self.registers.pc]])+1;
                if d.is_negative() {
                    self.registers.pc -= d as u16;
                } else {
                    self.registers.pc += d as u16;
                }
                12
            }
            0x20 | 0x28 => {
                let mut flag = self.registers.zero_flag();

                if opcode == 0x20 {
                    flag = !flag;
                }

                match flag {
                    true => {
                        let d = i8::from_le_bytes([self.memory[self.registers.pc]]) + 1;
                        if d.is_negative() {
                            self.registers.pc -= d as u16;
                        } else {
                            self.registers.pc += d as u16;
                        }
                        12
                    }
                    false => {
                        self.registers.pc += 1;
                        8
                    }
                }
            }
            0x40..=0x45
            | 0x47..=0x4D
            | 0x4F..=0x55
            | 0x57..=0x5D
            | 0x5F..=0x65
            | 0x67..=0x6D
            | 0x6F..=0x75
            | 0x77..=0x7D
            | 0x7F => {
                self.registers[Reg::from(opcode >> 3 & 7)] = self.registers[Reg::from(opcode & 7)];
                8
            }
            0x46 | 0x4E | 0x56 | 0x5E | 0x66 | 0x6E | 0x76 | 0x7E => {
                self.registers[Reg::from(opcode >> 3 & 7)] = self.memory[self.registers[DReg::HL]];
                8
            }
            0x80..=0x85 | 0x87 => {
                self.registers[Reg::A] += self.registers[Reg::from(opcode & 7)];
                4
            }
            0xA8..=0xAD | 0xAF => {
                self.registers[Reg::A] ^= self.registers[Reg::from(opcode & 7)];
                if self.registers[Reg::A] == 0 {
                    self.registers[Reg::F] |= Reg::ZERO_FLAG;
                }
                4
            }
            0xC3 => {
                self.registers.pc =
                    u16::from_le_bytes(self.memory[Request::<2>(self.registers.pc)]);
                16
            }
            0xE6 => {
                self.registers[Reg::A] +=
                    self.registers
                        .add(Reg::A, self.memory[self.registers.pc], None);
                self.registers.pc += 1;
                8
            }
            0xF0 => {
                self.registers[Reg::A] =
                    self.memory[self.memory[self.registers.pc] as u16 + 0xFF00];
                self.registers.pc += 1;
                12
            }
            0xFE => {
                let (_, overflow) =
                    self.registers[Reg::A].overflowing_sub(self.memory[self.registers.pc]);
                if overflow {
                    self.registers[Reg::F] |= Reg::CARRY_FLAG;
                }
                self.registers.pc += 1;
                8
            }
            _ => panic!("Unknown opcode 0x{opcode:X}!"),
        }
    }

    #[must_use = "The bitmap must be used!"]
    pub fn render(&self) -> &[u8] {
        &[]
    }

    pub fn handle_interrupts(&mut self) {}
}
