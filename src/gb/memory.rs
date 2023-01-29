use std::ops::{Index, IndexMut};

pub struct Memory {
    cartridge: Vec<u8>,
    work: Box<[u8; Self::WRAM_SIZE]>,
    work_bank: u8,
    rom_bank: u8,
    video_bank: bool,
}

pub type Address = u16;

pub struct Request<const N: usize>(pub Address);

pub enum MappedMemory {
    Rom(Address),
    Video(Address),
    ExternalRam(Address),
    Work(Address),
    Oam(Address),
    Io(Address),
    Hram(Address),
    InterruptRegister,
}

impl Memory {

    const WRAM_SIZE: usize = usize::pow(2, 18); // 32kbytes
    const VRAM_SIZE: usize = usize::pow(2, 17); // 16kbytes

    pub fn new() -> Self {
        Self {
            cartridge: Vec::new(),
            work: Box::new([0; Self::WRAM_SIZE]),
            work_bank: 1,
            rom_bank: 1,
            video_bank: false,
        }
    }

    pub fn set_cartridge(&mut self, cart: &[u8]) {
        self.cartridge.clear();
        self.cartridge.extend_from_slice(cart);
    }

    pub fn map(&self, address: Address) -> MappedMemory {
        match address {
            0x0000..=0x3FFF => MappedMemory::Rom(address),
            0x4000..=0x7FFF => MappedMemory::Rom(address + (self.rom_bank as u16 - 1) * 0x4000),
            0x8000..=0x9FFF => MappedMemory::Video(address - if self.video_bank { 0x8000 } else { 0x6000 }),
            0xA000..=0xBFFF => MappedMemory::ExternalRam(address - 0xA000),
            0xC000..=0xCFFF => MappedMemory::Work(address - 0xC000),
            0xD000..=0xDFFF => {
                MappedMemory::Work(address - 0xD000 + self.work_bank as Address * 0x1000)
            }
            0xE000..=0xFDFF => panic!("Cannot map to ECHO memory!"),
            0xFE00..=0xFE9F => MappedMemory::Oam(address - 0xFE00),
            0xFEA0..=0xFEFF => panic!("Illegal memory map address {address:X}!"),
            0xFF00..=0xFF7F => MappedMemory::Io(address - 0xFF00),
            0xFF80..=0xFFFE => MappedMemory::Hram(address - 0xFF80),
            0xFFFF => MappedMemory::InterruptRegister,
        }
    }

    // pub fn read_multi(&self, addresses: impl RangeBounds<Address>) -> &[u8] {
    //     let m = MappedMemory::try_from(addresses);
    // }

    pub fn next_program_byte(&self, pc: &mut u16) -> u8 {
        let next = self[*pc];
        *pc = pc.wrapping_add(1);
        next
    }
}

impl Index<Address> for Memory {
    type Output = u8;

    fn index(&self, address: Address) -> &Self::Output {
        match self.map(address) {
            MappedMemory::Work(addr) => &self.work[addr as usize],
            MappedMemory::Rom(addr) => &self.cartridge[addr as usize],
            MappedMemory::Video(_) => todo!("vram"),
            MappedMemory::ExternalRam(addr) => &self.cartridge[addr as usize + 0xA000],
            MappedMemory::InterruptRegister => todo!("ireg"),
            MappedMemory::Oam(_) => todo!("oam"),
            MappedMemory::Io(_) => todo!("io"),
            MappedMemory::Hram(_) => todo!("hram"),
        }
    }
}

impl IndexMut<Address> for Memory {
    fn index_mut(&mut self, address: Address) -> &mut Self::Output {
        match self.map(address) {
            MappedMemory::Work(addr) => &mut self.work[addr as usize],
            MappedMemory::Rom(..) => panic!("Cannot mutate the ROM!"),
            MappedMemory::Video(addr) => todo!("mut vram"),
            MappedMemory::ExternalRam(addr) => &mut self.cartridge[addr as usize + 0xA000],
            MappedMemory::InterruptRegister => todo!("mut ireg"),
            MappedMemory::Oam(_) => todo!("mut oam"),
            MappedMemory::Io(_) => todo!("mut io"),
            MappedMemory::Hram(_) => todo!("mut hram"),
        }
    }
}

impl<const N: usize> Index<Request<N>> for Memory {
    type Output = [u8; N];

    fn index(&self, request: Request<N>) -> &Self::Output {
        match self.map(request.0) {
            MappedMemory::Work(addr) => {
                let addr = addr as usize;
                // ??
                unsafe { &*(&self.work[addr] as *const u8 as *const [u8; N]) }
            }
            MappedMemory::Rom(addr) => {
                let addr = addr as usize;
                // ??
                unsafe { &*(&self.cartridge[addr] as *const u8 as *const [u8; N]) }
            },
            MappedMemory::Video(_) => todo!("sized vram"),
            MappedMemory::ExternalRam(addr) => {
                let addr = addr as usize;
                // ??
                unsafe { &*(&self.cartridge[addr] as *const u8 as *const [u8; N]) }
            },
            MappedMemory::InterruptRegister => todo!("sized ireg"),
            MappedMemory::Oam(_) => todo!("sized oam"),
            MappedMemory::Io(_) => todo!("sized io"),
            MappedMemory::Hram(_) => todo!("sized hram"),
        }
    }
}
