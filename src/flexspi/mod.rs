#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Module Control Register 0"]
    pub mcr0: MCR0,
    #[doc = "0x04 - Module Control Register 1"]
    pub mcr1: MCR1,
    #[doc = "0x08 - Module Control Register 2"]
    pub mcr2: MCR2,
    #[doc = "0x0c - AHB Bus Control Register"]
    pub ahbcr: AHBCR,
    #[doc = "0x10 - Interrupt Enable Register"]
    pub inten: INTEN,
    #[doc = "0x14 - Interrupt Register"]
    pub intr: INTR,
    #[doc = "0x18 - LUT Key Register"]
    pub lutkey: LUTKEY,
    #[doc = "0x1c - LUT Control Register"]
    pub lutcr: LUTCR,
    #[doc = "0x20 - AHB RX Buffer 0 Control Register 0"]
    pub ahbrxbuf0cr0: AHBRXBUF0CR0,
    #[doc = "0x24 - AHB RX Buffer 1 Control Register 0"]
    pub ahbrxbuf1cr0: AHBRXBUF1CR0,
    #[doc = "0x28 - AHB RX Buffer 2 Control Register 0"]
    pub ahbrxbuf2cr0: AHBRXBUF2CR0,
    #[doc = "0x2c - AHB RX Buffer 3 Control Register 0"]
    pub ahbrxbuf3cr0: AHBRXBUF3CR0,
    _reserved0: [u8; 48usize],
    #[doc = "0x60 - Flash A1 Control Register 0"]
    pub flsha1cr0: FLSHA1CR0,
    #[doc = "0x64 - Flash A2 Control Register 0"]
    pub flsha2cr0: FLSHA2CR0,
    #[doc = "0x68 - Flash B1 Control Register 0"]
    pub flshb1cr0: FLSHB1CR0,
    #[doc = "0x6c - Flash B2 Control Register 0"]
    pub flshb2cr0: FLSHB2CR0,
    #[doc = "0x70 - Flash A1 Control Register 1"]
    pub flshcr1a1: FLSHCR1,
    #[doc = "0x74 - Flash A1 Control Register 1"]
    pub flshcr1a2: FLSHCR1,
    #[doc = "0x78 - Flash A1 Control Register 1"]
    pub flshcr1b1: FLSHCR1,
    #[doc = "0x7c - Flash A1 Control Register 1"]
    pub flshcr1b2: FLSHCR1,
    #[doc = "0x80 - Flash A1 Control Register 2"]
    pub flshcr2a1: FLSHCR2,
    #[doc = "0x84 - Flash A1 Control Register 2"]
    pub flshcr2a2: FLSHCR2,
    #[doc = "0x88 - Flash A1 Control Register 2"]
    pub flshcr2b1: FLSHCR2,
    #[doc = "0x8c - Flash A1 Control Register 2"]
    pub flshcr2b2: FLSHCR2,
    _reserved1: [u8; 4usize],
    #[doc = "0x94 - Flash Control Register 4"]
    pub flshcr4: FLSHCR4,
    _reserved2: [u8; 8usize],
    #[doc = "0xa0 - IP Control Register 0"]
    pub ipcr0: IPCR0,
    #[doc = "0xa4 - IP Control Register 1"]
    pub ipcr1: IPCR1,
    _reserved3: [u8; 8usize],
    #[doc = "0xb0 - IP Command Register"]
    pub ipcmd: IPCMD,
    _reserved4: [u8; 4usize],
    #[doc = "0xb8 - IP RX FIFO Control Register"]
    pub iprxfcr: IPRXFCR,
    #[doc = "0xbc - IP TX FIFO Control Register"]
    pub iptxfcr: IPTXFCR,
    #[doc = "0xc0 - DLL Control Register 0"]
    pub dllcra: DLLCR,
    #[doc = "0xc4 - DLL Control Register 0"]
    pub dllcrb: DLLCR,
    _reserved5: [u8; 24usize],
    #[doc = "0xe0 - Status Register 0"]
    pub sts0: STS0,
    #[doc = "0xe4 - Status Register 1"]
    pub sts1: STS1,
    #[doc = "0xe8 - Status Register 2"]
    pub sts2: STS2,
    #[doc = "0xec - AHB Suspend Status Register"]
    pub ahbspndsts: AHBSPNDSTS,
    #[doc = "0xf0 - IP RX FIFO Status Register"]
    pub iprxfsts: IPRXFSTS,
    #[doc = "0xf4 - IP TX FIFO Status Register"]
    pub iptxfsts: IPTXFSTS,
    _reserved6: [u8; 8usize],
    #[doc = "0x100 - IP RX FIFO Data Register 0"]
    pub rfdr: [RFDR; 32],
    #[doc = "0x180 - IP TX FIFO Data Register 0"]
    pub tfdr: [TFDR; 32],
    #[doc = "0x200 - LUT 0"]
    pub lut: [LUT; 64],
}
#[doc = "Module Control Register 0"]
pub struct MCR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Module Control Register 0"]
pub mod mcr0;
#[doc = "Module Control Register 1"]
pub struct MCR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Module Control Register 1"]
pub mod mcr1;
#[doc = "Module Control Register 2"]
pub struct MCR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Module Control Register 2"]
pub mod mcr2;
#[doc = "AHB Bus Control Register"]
pub struct AHBCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AHB Bus Control Register"]
pub mod ahbcr;
#[doc = "Interrupt Enable Register"]
pub struct INTEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable Register"]
pub mod inten;
#[doc = "Interrupt Register"]
pub struct INTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Register"]
pub mod intr;
#[doc = "LUT Key Register"]
pub struct LUTKEY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LUT Key Register"]
pub mod lutkey;
#[doc = "LUT Control Register"]
pub struct LUTCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LUT Control Register"]
pub mod lutcr;
#[doc = "AHB RX Buffer 0 Control Register 0"]
pub struct AHBRXBUF0CR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AHB RX Buffer 0 Control Register 0"]
pub mod ahbrxbuf0cr0;
#[doc = "AHB RX Buffer 1 Control Register 0"]
pub struct AHBRXBUF1CR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AHB RX Buffer 1 Control Register 0"]
pub mod ahbrxbuf1cr0;
#[doc = "AHB RX Buffer 2 Control Register 0"]
pub struct AHBRXBUF2CR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AHB RX Buffer 2 Control Register 0"]
pub mod ahbrxbuf2cr0;
#[doc = "AHB RX Buffer 3 Control Register 0"]
pub struct AHBRXBUF3CR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AHB RX Buffer 3 Control Register 0"]
pub mod ahbrxbuf3cr0;
#[doc = "Flash A1 Control Register 0"]
pub struct FLSHA1CR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash A1 Control Register 0"]
pub mod flsha1cr0;
#[doc = "Flash A2 Control Register 0"]
pub struct FLSHA2CR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash A2 Control Register 0"]
pub mod flsha2cr0;
#[doc = "Flash B1 Control Register 0"]
pub struct FLSHB1CR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash B1 Control Register 0"]
pub mod flshb1cr0;
#[doc = "Flash B2 Control Register 0"]
pub struct FLSHB2CR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash B2 Control Register 0"]
pub mod flshb2cr0;
#[doc = "Flash A1 Control Register 1"]
pub struct FLSHCR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash A1 Control Register 1"]
pub mod flshcr1;
#[doc = "Flash A1 Control Register 2"]
pub struct FLSHCR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash A1 Control Register 2"]
pub mod flshcr2;
#[doc = "Flash Control Register 4"]
pub struct FLSHCR4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Control Register 4"]
pub mod flshcr4;
#[doc = "IP Control Register 0"]
pub struct IPCR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IP Control Register 0"]
pub mod ipcr0;
#[doc = "IP Control Register 1"]
pub struct IPCR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IP Control Register 1"]
pub mod ipcr1;
#[doc = "IP Command Register"]
pub struct IPCMD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IP Command Register"]
pub mod ipcmd;
#[doc = "IP RX FIFO Control Register"]
pub struct IPRXFCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IP RX FIFO Control Register"]
pub mod iprxfcr;
#[doc = "IP TX FIFO Control Register"]
pub struct IPTXFCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IP TX FIFO Control Register"]
pub mod iptxfcr;
#[doc = "DLL Control Register 0"]
pub struct DLLCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DLL Control Register 0"]
pub mod dllcr;
#[doc = "Status Register 0"]
pub struct STS0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status Register 0"]
pub mod sts0;
#[doc = "Status Register 1"]
pub struct STS1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status Register 1"]
pub mod sts1;
#[doc = "Status Register 2"]
pub struct STS2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status Register 2"]
pub mod sts2;
#[doc = "AHB Suspend Status Register"]
pub struct AHBSPNDSTS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AHB Suspend Status Register"]
pub mod ahbspndsts;
#[doc = "IP RX FIFO Status Register"]
pub struct IPRXFSTS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IP RX FIFO Status Register"]
pub mod iprxfsts;
#[doc = "IP TX FIFO Status Register"]
pub struct IPTXFSTS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IP TX FIFO Status Register"]
pub mod iptxfsts;
#[doc = "IP RX FIFO Data Register 0"]
pub struct RFDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IP RX FIFO Data Register 0"]
pub mod rfdr;
#[doc = "IP TX FIFO Data Register 0"]
pub struct TFDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IP TX FIFO Data Register 0"]
pub mod tfdr;
#[doc = "LUT 0"]
pub struct LUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LUT 0"]
pub mod lut;
