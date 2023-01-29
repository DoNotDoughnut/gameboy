use std::ops::{Index, IndexMut};

use num_traits::{Zero, ops::overflowing::*, };

#[derive(Default)]
pub struct Registers {
    registers: [u8; 8],
    pub sp: u16,
    pub pc: u16,
}

pub trait RegisterType: Into<usize> + Copy {
    type Size: Zero + Copy + OverflowingAdd + OverflowingSub;

}

#[derive(Debug, Clone, Copy)]
pub enum Reg {
    B = 0,
    C = 1,
    D = 2,
    E = 3,
    H = 4,
    L = 5,
    /// Flags
    F = 6,
    /// Accumulator
    A = 7,
}

#[derive(Debug, Clone, Copy)]
pub enum DReg {
    BC = 0,
    DE = 2,
    HL = 4,
    /// Accumulator & Flags
    AF = 6,
}

// pub struct Register<const N: usize>([u8; N]);

impl Registers {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add<R: RegisterType>(&mut self, reg: R, value: R::Size, options: Option<()>) -> R::Size {
        let (new, overflow) = self[reg].overflowing_add(&value);
        let flags = &mut self[Reg::F];
        *flags &= Reg::SUBTRACTION_FLAG;
        if overflow {
            *flags |= Reg::CARRY_FLAG;
        }
        if new.is_zero() {
            *flags |= Reg::ZERO_FLAG;
        }
        new
    }

    pub fn sub<R: RegisterType>(&mut self, reg: R, value: R::Size, options: Option<()>) -> R::Size {
        let (new, overflow) = self[reg].overflowing_sub(&value);
        let flags = &mut self[Reg::F];
        *flags |= Reg::SUBTRACTION_FLAG;
        if overflow {
            *flags |= Reg::CARRY_FLAG;
        }
        if new.is_zero() {
            *flags |= Reg::ZERO_FLAG;
        }
        new
    }

    pub fn zero_flag(&self) -> bool {
        self[Reg::F] >> 7 != 0
    }

    pub fn carry_flag(&self) -> bool {
        self[Reg::F] >> 5 & 1 != 0
    }

}

impl Reg {
    /// Bit position of the carry flag
    pub const CARRY_FLAG: u8 = 1 << 4;
    pub const HALF_CARRY_FLAG: u8 = 1 << 5;
    pub const SUBTRACTION_FLAG: u8 = 1 << 6;
    pub const ZERO_FLAG: u8 = 1 << 7;
}

impl From<u8> for Reg {
    fn from(value: u8) -> Self {
        match value {
            0 => Reg::B,
            1 => Reg::C,
            2 => Reg::D,
            3 => Reg::E,
            4 => Reg::H,
            5 => Reg::L,
            6 => Reg::F,
            7 => Reg::A,
            _ => unreachable!("Reg value too large"),
        }
    }
}

impl RegisterType for Reg {
    type Size = u8;

}

impl RegisterType for DReg {
    type Size = u16;
}

impl<R: RegisterType> Index<R> for Registers {
    type Output = R::Size;

    fn index(&self, register: R) -> &Self::Output {
        unsafe { std::mem::transmute(&self.registers[register.into()]) }
    }
}

impl<R: RegisterType> IndexMut<R> for Registers {
    fn index_mut(&mut self, register: R) -> &mut Self::Output {
        unsafe { std::mem::transmute(&mut self.registers[register.into()]) }
    }
}


impl Into<usize> for Reg {
    fn into(self) -> usize {
        match self {
            Reg::B => 0,
            Reg::C => 1,
            Reg::D => 2,
            Reg::E => 3,
            Reg::H => 4,
            Reg::L => 5,
            Reg::F => 6,
            Reg::A => 7,
        }
    }
}

impl Into<usize> for DReg {
    fn into(self) -> usize {
        match self {
            DReg::BC => 0,
            DReg::DE => 2,
            DReg::HL => 4,
            DReg::AF => 6,
        }
    }
}