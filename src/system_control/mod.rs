#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 8usize],
    #[doc = "0x08 - Auxiliary Control Register,"]
    pub actlr: ACTLR,
    _reserved1: [u8; 3316usize],
    #[doc = "0xd00 - CPUID Base Register"]
    pub cpuid: CPUID,
    #[doc = "0xd04 - Interrupt Control and State Register"]
    pub icsr: ICSR,
    #[doc = "0xd08 - Vector Table Offset Register"]
    pub vtor: VTOR,
    #[doc = "0xd0c - Application Interrupt and Reset Control Register"]
    pub aircr: AIRCR,
    #[doc = "0xd10 - System Control Register"]
    pub scr: SCR,
    #[doc = "0xd14 - Configuration and Control Register"]
    pub ccr: CCR,
    #[doc = "0xd18 - System Handler Priority Register 1"]
    pub shpr1: SHPR1,
    #[doc = "0xd1c - System Handler Priority Register 2"]
    pub shpr2: SHPR2,
    #[doc = "0xd20 - System Handler Priority Register 3"]
    pub shpr3: SHPR3,
    #[doc = "0xd24 - System Handler Control and State Register"]
    pub shcsr: SHCSR,
    #[doc = "0xd28 - Configurable Fault Status Register"]
    pub cfsr: CFSR,
    #[doc = "0xd2c - HardFault Status register"]
    pub hfsr: HFSR,
    #[doc = "0xd30 - Debug Fault Status Register"]
    pub dfsr: DFSR,
    #[doc = "0xd34 - MemManage Fault Address Register"]
    pub mmfar: MMFAR,
    #[doc = "0xd38 - BusFault Address Register"]
    pub bfar: BFAR,
    _reserved2: [u8; 4usize],
    #[doc = "0xd40 - Processor Feature Register 0"]
    pub id_pfr0: ID_PFR0,
    #[doc = "0xd44 - Processor Feature Register 1"]
    pub id_pfr1: ID_PFR1,
    #[doc = "0xd48 - Debug Feature Register"]
    pub id_dfr0: ID_DFR0,
    #[doc = "0xd4c - Auxiliary Feature Register"]
    pub id_afr0: ID_AFR0,
    #[doc = "0xd50 - Memory Model Feature Register 0"]
    pub id_mmfr0: ID_MMFR0,
    #[doc = "0xd54 - Memory Model Feature Register 1"]
    pub id_mmfr1: ID_MMFR1,
    #[doc = "0xd58 - Memory Model Feature Register 2"]
    pub id_mmfr2: ID_MMFR2,
    #[doc = "0xd5c - Memory Model Feature Register 3"]
    pub id_mmfr3: ID_MMFR3,
    #[doc = "0xd60 - Instruction Set Attributes Register 0"]
    pub id_isar0: ID_ISAR0,
    #[doc = "0xd64 - Instruction Set Attributes Register 1"]
    pub id_isar1: ID_ISAR1,
    #[doc = "0xd68 - Instruction Set Attributes Register 2"]
    pub id_isar2: ID_ISAR2,
    #[doc = "0xd6c - Instruction Set Attributes Register 3"]
    pub id_isar3: ID_ISAR3,
    #[doc = "0xd70 - Instruction Set Attributes Register 4"]
    pub id_isar4: ID_ISAR4,
    _reserved3: [u8; 4usize],
    #[doc = "0xd78 - Cache Level ID register"]
    pub clidr: CLIDR,
    #[doc = "0xd7c - Cache Type register"]
    pub ctr: CTR,
    #[doc = "0xd80 - Cache Size ID Register"]
    pub ccsidr: CCSIDR,
    #[doc = "0xd84 - Cache Size Selection Register"]
    pub csselr: CSSELR,
    #[doc = "0xd88 - Coprocessor Access Control Register"]
    pub cpacr: CPACR,
    _reserved4: [u8; 372usize],
    #[doc = "0xf00 - Instruction cache invalidate all to Point of Unification (PoU)"]
    pub stir: STIR,
    _reserved5: [u8; 76usize],
    #[doc = "0xf50 - Instruction cache invalidate all to Point of Unification (PoU)"]
    pub iciallu: ICIALLU,
    _reserved6: [u8; 4usize],
    #[doc = "0xf58 - Instruction cache invalidate by address to PoU"]
    pub icimvau: ICIMVAU,
    #[doc = "0xf5c - Data cache invalidate by address to Point of Coherency (PoC)"]
    pub dcimvac: DCIMVAC,
    #[doc = "0xf60 - Data cache invalidate by set/way"]
    pub dcisw: DCISW,
    #[doc = "0xf64 - Data cache by address to PoU"]
    pub dccmvau: DCCMVAU,
    #[doc = "0xf68 - Data cache clean by address to PoC"]
    pub dccmvac: DCCMVAC,
    #[doc = "0xf6c - Data cache clean by set/way"]
    pub dccsw: DCCSW,
    #[doc = "0xf70 - Data cache clean and invalidate by address to PoC"]
    pub dccimvac: DCCIMVAC,
    #[doc = "0xf74 - Data cache clean and invalidate by set/way"]
    pub dccisw: DCCISW,
    _reserved7: [u8; 24usize],
    #[doc = "0xf90 - Instruction Tightly-Coupled Memory Control Register"]
    pub cm7_itcmcr: CM7_ITCMCR,
    #[doc = "0xf94 - Data Tightly-Coupled Memory Control Register"]
    pub cm7_dtcmcr: CM7_DTCMCR,
    #[doc = "0xf98 - AHBP Control Register"]
    pub cm7_ahbpcr: CM7_AHBPCR,
    #[doc = "0xf9c - L1 Cache Control Register"]
    pub cm7_cacr: CM7_CACR,
    #[doc = "0xfa0 - AHB Slave Control Register"]
    pub cm7_ahbscr: CM7_AHBSCR,
    _reserved8: [u8; 4usize],
    #[doc = "0xfa8 - Auxiliary Bus Fault Status Register"]
    pub cm7_abfsr: CM7_ABFSR,
}
#[doc = "Auxiliary Control Register,"]
pub struct ACTLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Auxiliary Control Register,"]
pub mod actlr;
#[doc = "CPUID Base Register"]
pub struct CPUID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CPUID Base Register"]
pub mod cpuid;
#[doc = "Interrupt Control and State Register"]
pub struct ICSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Control and State Register"]
pub mod icsr;
#[doc = "Vector Table Offset Register"]
pub struct VTOR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Vector Table Offset Register"]
pub mod vtor;
#[doc = "Application Interrupt and Reset Control Register"]
pub struct AIRCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Application Interrupt and Reset Control Register"]
pub mod aircr;
#[doc = "System Control Register"]
pub struct SCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System Control Register"]
pub mod scr;
#[doc = "Configuration and Control Register"]
pub struct CCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration and Control Register"]
pub mod ccr;
#[doc = "System Handler Priority Register 1"]
pub struct SHPR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System Handler Priority Register 1"]
pub mod shpr1;
#[doc = "System Handler Priority Register 2"]
pub struct SHPR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System Handler Priority Register 2"]
pub mod shpr2;
#[doc = "System Handler Priority Register 3"]
pub struct SHPR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System Handler Priority Register 3"]
pub mod shpr3;
#[doc = "System Handler Control and State Register"]
pub struct SHCSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System Handler Control and State Register"]
pub mod shcsr;
#[doc = "Configurable Fault Status Register"]
pub struct CFSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configurable Fault Status Register"]
pub mod cfsr;
#[doc = "HardFault Status register"]
pub struct HFSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HardFault Status register"]
pub mod hfsr;
#[doc = "Debug Fault Status Register"]
pub struct DFSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Debug Fault Status Register"]
pub mod dfsr;
#[doc = "MemManage Fault Address Register"]
pub struct MMFAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MemManage Fault Address Register"]
pub mod mmfar;
#[doc = "BusFault Address Register"]
pub struct BFAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "BusFault Address Register"]
pub mod bfar;
#[doc = "Processor Feature Register 0"]
pub struct ID_PFR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Processor Feature Register 0"]
pub mod id_pfr0;
#[doc = "Processor Feature Register 1"]
pub struct ID_PFR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Processor Feature Register 1"]
pub mod id_pfr1;
#[doc = "Debug Feature Register"]
pub struct ID_DFR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Debug Feature Register"]
pub mod id_dfr0;
#[doc = "Auxiliary Feature Register"]
pub struct ID_AFR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Auxiliary Feature Register"]
pub mod id_afr0;
#[doc = "Memory Model Feature Register 0"]
pub struct ID_MMFR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Memory Model Feature Register 0"]
pub mod id_mmfr0;
#[doc = "Memory Model Feature Register 1"]
pub struct ID_MMFR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Memory Model Feature Register 1"]
pub mod id_mmfr1;
#[doc = "Memory Model Feature Register 2"]
pub struct ID_MMFR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Memory Model Feature Register 2"]
pub mod id_mmfr2;
#[doc = "Memory Model Feature Register 3"]
pub struct ID_MMFR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Memory Model Feature Register 3"]
pub mod id_mmfr3;
#[doc = "Instruction Set Attributes Register 0"]
pub struct ID_ISAR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Instruction Set Attributes Register 0"]
pub mod id_isar0;
#[doc = "Instruction Set Attributes Register 1"]
pub struct ID_ISAR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Instruction Set Attributes Register 1"]
pub mod id_isar1;
#[doc = "Instruction Set Attributes Register 2"]
pub struct ID_ISAR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Instruction Set Attributes Register 2"]
pub mod id_isar2;
#[doc = "Instruction Set Attributes Register 3"]
pub struct ID_ISAR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Instruction Set Attributes Register 3"]
pub mod id_isar3;
#[doc = "Instruction Set Attributes Register 4"]
pub struct ID_ISAR4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Instruction Set Attributes Register 4"]
pub mod id_isar4;
#[doc = "Cache Level ID register"]
pub struct CLIDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Cache Level ID register"]
pub mod clidr;
#[doc = "Cache Type register"]
pub struct CTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Cache Type register"]
pub mod ctr;
#[doc = "Cache Size ID Register"]
pub struct CCSIDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Cache Size ID Register"]
pub mod ccsidr;
#[doc = "Cache Size Selection Register"]
pub struct CSSELR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Cache Size Selection Register"]
pub mod csselr;
#[doc = "Coprocessor Access Control Register"]
pub struct CPACR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Coprocessor Access Control Register"]
pub mod cpacr;
#[doc = "Instruction cache invalidate all to Point of Unification (PoU)"]
pub struct STIR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Instruction cache invalidate all to Point of Unification (PoU)"]
pub mod stir;
#[doc = "Instruction cache invalidate all to Point of Unification (PoU)"]
pub struct ICIALLU {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Instruction cache invalidate all to Point of Unification (PoU)"]
pub mod iciallu;
#[doc = "Instruction cache invalidate by address to PoU"]
pub struct ICIMVAU {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Instruction cache invalidate by address to PoU"]
pub mod icimvau;
#[doc = "Data cache invalidate by address to Point of Coherency (PoC)"]
pub struct DCIMVAC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data cache invalidate by address to Point of Coherency (PoC)"]
pub mod dcimvac;
#[doc = "Data cache invalidate by set/way"]
pub struct DCISW {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data cache invalidate by set/way"]
pub mod dcisw;
#[doc = "Data cache by address to PoU"]
pub struct DCCMVAU {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data cache by address to PoU"]
pub mod dccmvau;
#[doc = "Data cache clean by address to PoC"]
pub struct DCCMVAC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data cache clean by address to PoC"]
pub mod dccmvac;
#[doc = "Data cache clean by set/way"]
pub struct DCCSW {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data cache clean by set/way"]
pub mod dccsw;
#[doc = "Data cache clean and invalidate by address to PoC"]
pub struct DCCIMVAC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data cache clean and invalidate by address to PoC"]
pub mod dccimvac;
#[doc = "Data cache clean and invalidate by set/way"]
pub struct DCCISW {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data cache clean and invalidate by set/way"]
pub mod dccisw;
#[doc = "Instruction Tightly-Coupled Memory Control Register"]
pub struct CM7_ITCMCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Instruction Tightly-Coupled Memory Control Register"]
pub mod cm7_itcmcr;
#[doc = "Data Tightly-Coupled Memory Control Register"]
pub struct CM7_DTCMCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data Tightly-Coupled Memory Control Register"]
pub mod cm7_dtcmcr;
#[doc = "AHBP Control Register"]
pub struct CM7_AHBPCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AHBP Control Register"]
pub mod cm7_ahbpcr;
#[doc = "L1 Cache Control Register"]
pub struct CM7_CACR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "L1 Cache Control Register"]
pub mod cm7_cacr;
#[doc = "AHB Slave Control Register"]
pub struct CM7_AHBSCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AHB Slave Control Register"]
pub mod cm7_ahbscr;
#[doc = "Auxiliary Bus Fault Status Register"]
pub struct CM7_ABFSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Auxiliary Bus Fault Status Register"]
pub mod cm7_abfsr;
