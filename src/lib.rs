#![doc = "Peripheral access API for MIMXRT1052 microcontrollers (generated using svd2rust v0.13.1)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.13.1/svd2rust/#peripheral-api"]
#![deny(missing_docs)]
// #![deny(warnings)]
#![allow(intra_doc_link_resolution_failure)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![no_std]
extern crate bare_metal;
extern crate cortex_m;
#[cfg(feature = "rt")]
extern crate cortex_m_rt;
extern crate vcell;
use core::marker::PhantomData;
use core::ops::Deref;
#[macro_use]
extern crate cfg_if;

#[doc = r" Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 4;
#[cfg(feature = "rt")]
extern "C" {
    fn DMA0_DMA16();
    fn DMA1_DMA17();
    fn DMA2_DMA18();
    fn DMA3_DMA19();
    fn DMA4_DMA20();
    fn DMA5_DMA21();
    fn DMA6_DMA22();
    fn DMA7_DMA23();
    fn DMA8_DMA24();
    fn DMA9_DMA25();
    fn DMA10_DMA26();
    fn DMA11_DMA27();
    fn DMA12_DMA28();
    fn DMA13_DMA29();
    fn DMA14_DMA30();
    fn DMA15_DMA31();
    fn DMA_ERROR();
    fn CTI0_ERROR();
    fn CTI1_ERROR();
    fn CORE();
    fn LPUART1();
    fn LPUART2();
    fn LPUART3();
    fn LPUART4();
    fn LPUART5();
    fn LPUART6();
    fn LPUART7();
    fn LPUART8();
    fn LPI2C1();
    fn LPI2C2();
    fn LPI2C3();
    fn LPI2C4();
    fn LPSPI1();
    fn LPSPI2();
    fn LPSPI3();
    fn LPSPI4();
    fn CAN1();
    fn CAN2();
    fn FLEXRAM();
    fn KPP();
    fn TSC_DIG();
    fn GPR_IRQ();
    fn LCDIF();
    fn CSI();
    fn PXP();
    fn WDOG2();
    fn SNVS_HP_WRAPPER();
    fn SNVS_HP_WRAPPER_TZ();
    fn SNVS_LP_WRAPPER();
    fn CSU();
    fn DCP();
    fn DCP_VMI();
    fn RESERVED68();
    fn TRNG();
    fn SJC();
    fn BEE();
    fn SAI1();
    fn SAI2();
    fn SAI3_RX();
    fn SAI3_TX();
    fn SPDIF();
    fn ANATOP_EVENT0();
    fn ANATOP_EVENT1();
    fn ANATOP_TAMP_LOW_HIGH();
    fn ANATOP_TEMP_PANIC();
    fn USB_PHY1();
    fn USB_PHY2();
    fn ADC1();
    fn ADC2();
    fn DCDC();
    fn RESERVED86();
    fn RESERVED87();
    fn GPIO1_INT0();
    fn GPIO1_INT1();
    fn GPIO1_INT2();
    fn GPIO1_INT3();
    fn GPIO1_INT4();
    fn GPIO1_INT5();
    fn GPIO1_INT6();
    fn GPIO1_INT7();
    fn GPIO1_COMBINED_0_15();
    fn GPIO1_COMBINED_16_31();
    fn GPIO2_COMBINED_0_15();
    fn GPIO2_COMBINED_16_31();
    fn GPIO3_COMBINED_0_15();
    fn GPIO3_COMBINED_16_31();
    fn GPIO4_COMBINED_0_15();
    fn GPIO4_COMBINED_16_31();
    fn GPIO5_COMBINED_0_15();
    fn GPIO5_COMBINED_16_31();
    fn FLEXIO1();
    fn FLEXIO2();
    fn WDOG1();
    fn RTWDOG();
    fn EWM();
    fn CCM_1();
    fn CCM_2();
    fn GPC();
    fn SRC();
    fn RESERVED115();
    fn GPT1();
    fn GPT2();
    fn PWM1_0();
    fn PWM1_1();
    fn PWM1_2();
    fn PWM1_3();
    fn PWM1_FAULT();
    fn RESERVED123();
    fn FLEXSPI();
    fn SEMC();
    fn USDHC1();
    fn USDHC2();
    fn USB_OTG2();
    fn USB_OTG1();
    fn ENET();
    fn ENET_1588_TIMER();
    fn XBAR1_IRQ_0_1();
    fn XBAR1_IRQ_2_3();
    fn ADC_ETC_IRQ0();
    fn ADC_ETC_IRQ1();
    fn ADC_ETC_IRQ2();
    fn ADC_ETC_ERROR_IRQ();
    fn PIT();
    fn ACMP1();
    fn ACMP2();
    fn ACMP3();
    fn ACMP4();
    fn RESERVED143();
    fn RESERVED144();
    fn ENC1();
    fn ENC2();
    fn ENC3();
    fn ENC4();
    fn TMR1();
    fn TMR2();
    fn TMR3();
    fn TMR4();
    fn PWM2_0();
    fn PWM2_1();
    fn PWM2_2();
    fn PWM2_3();
    fn PWM2_FAULT();
    fn PWM3_0();
    fn PWM3_1();
    fn PWM3_2();
    fn PWM3_3();
    fn PWM3_FAULT();
    fn PWM4_0();
    fn PWM4_1();
    fn PWM4_2();
    fn PWM4_3();
    fn PWM4_FAULT();
    fn RESERVED168();
    fn RESERVED169();
    fn RESERVED170();
    fn RESERVED171();
    fn RESERVED172();
    fn RESERVED173();
    fn SJC_ARM_DEBUG();
    fn NMI_WAKEUP();
}
#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 160] = [
    Vector {
        _handler: DMA0_DMA16,
    },
    Vector {
        _handler: DMA1_DMA17,
    },
    Vector {
        _handler: DMA2_DMA18,
    },
    Vector {
        _handler: DMA3_DMA19,
    },
    Vector {
        _handler: DMA4_DMA20,
    },
    Vector {
        _handler: DMA5_DMA21,
    },
    Vector {
        _handler: DMA6_DMA22,
    },
    Vector {
        _handler: DMA7_DMA23,
    },
    Vector {
        _handler: DMA8_DMA24,
    },
    Vector {
        _handler: DMA9_DMA25,
    },
    Vector {
        _handler: DMA10_DMA26,
    },
    Vector {
        _handler: DMA11_DMA27,
    },
    Vector {
        _handler: DMA12_DMA28,
    },
    Vector {
        _handler: DMA13_DMA29,
    },
    Vector {
        _handler: DMA14_DMA30,
    },
    Vector {
        _handler: DMA15_DMA31,
    },
    Vector {
        _handler: DMA_ERROR,
    },
    Vector {
        _handler: CTI0_ERROR,
    },
    Vector {
        _handler: CTI1_ERROR,
    },
    Vector { _handler: CORE },
    Vector { _handler: LPUART1 },
    Vector { _handler: LPUART2 },
    Vector { _handler: LPUART3 },
    Vector { _handler: LPUART4 },
    Vector { _handler: LPUART5 },
    Vector { _handler: LPUART6 },
    Vector { _handler: LPUART7 },
    Vector { _handler: LPUART8 },
    Vector { _handler: LPI2C1 },
    Vector { _handler: LPI2C2 },
    Vector { _handler: LPI2C3 },
    Vector { _handler: LPI2C4 },
    Vector { _handler: LPSPI1 },
    Vector { _handler: LPSPI2 },
    Vector { _handler: LPSPI3 },
    Vector { _handler: LPSPI4 },
    Vector { _handler: CAN1 },
    Vector { _handler: CAN2 },
    Vector { _handler: FLEXRAM },
    Vector { _handler: KPP },
    Vector { _handler: TSC_DIG },
    Vector { _handler: GPR_IRQ },
    Vector { _handler: LCDIF },
    Vector { _handler: CSI },
    Vector { _handler: PXP },
    Vector { _handler: WDOG2 },
    Vector {
        _handler: SNVS_HP_WRAPPER,
    },
    Vector {
        _handler: SNVS_HP_WRAPPER_TZ,
    },
    Vector {
        _handler: SNVS_LP_WRAPPER,
    },
    Vector { _handler: CSU },
    Vector { _handler: DCP },
    Vector { _handler: DCP_VMI },
    Vector {
        _handler: RESERVED68,
    },
    Vector { _handler: TRNG },
    Vector { _handler: SJC },
    Vector { _handler: BEE },
    Vector { _handler: SAI1 },
    Vector { _handler: SAI2 },
    Vector { _handler: SAI3_RX },
    Vector { _handler: SAI3_TX },
    Vector { _handler: SPDIF },
    Vector {
        _handler: ANATOP_EVENT0,
    },
    Vector {
        _handler: ANATOP_EVENT1,
    },
    Vector {
        _handler: ANATOP_TAMP_LOW_HIGH,
    },
    Vector {
        _handler: ANATOP_TEMP_PANIC,
    },
    Vector { _handler: USB_PHY1 },
    Vector { _handler: USB_PHY2 },
    Vector { _handler: ADC1 },
    Vector { _handler: ADC2 },
    Vector { _handler: DCDC },
    Vector {
        _handler: RESERVED86,
    },
    Vector {
        _handler: RESERVED87,
    },
    Vector {
        _handler: GPIO1_INT0,
    },
    Vector {
        _handler: GPIO1_INT1,
    },
    Vector {
        _handler: GPIO1_INT2,
    },
    Vector {
        _handler: GPIO1_INT3,
    },
    Vector {
        _handler: GPIO1_INT4,
    },
    Vector {
        _handler: GPIO1_INT5,
    },
    Vector {
        _handler: GPIO1_INT6,
    },
    Vector {
        _handler: GPIO1_INT7,
    },
    Vector {
        _handler: GPIO1_COMBINED_0_15,
    },
    Vector {
        _handler: GPIO1_COMBINED_16_31,
    },
    Vector {
        _handler: GPIO2_COMBINED_0_15,
    },
    Vector {
        _handler: GPIO2_COMBINED_16_31,
    },
    Vector {
        _handler: GPIO3_COMBINED_0_15,
    },
    Vector {
        _handler: GPIO3_COMBINED_16_31,
    },
    Vector {
        _handler: GPIO4_COMBINED_0_15,
    },
    Vector {
        _handler: GPIO4_COMBINED_16_31,
    },
    Vector {
        _handler: GPIO5_COMBINED_0_15,
    },
    Vector {
        _handler: GPIO5_COMBINED_16_31,
    },
    Vector { _handler: FLEXIO1 },
    Vector { _handler: FLEXIO2 },
    Vector { _handler: WDOG1 },
    Vector { _handler: RTWDOG },
    Vector { _handler: EWM },
    Vector { _handler: CCM_1 },
    Vector { _handler: CCM_2 },
    Vector { _handler: GPC },
    Vector { _handler: SRC },
    Vector {
        _handler: RESERVED115,
    },
    Vector { _handler: GPT1 },
    Vector { _handler: GPT2 },
    Vector { _handler: PWM1_0 },
    Vector { _handler: PWM1_1 },
    Vector { _handler: PWM1_2 },
    Vector { _handler: PWM1_3 },
    Vector {
        _handler: PWM1_FAULT,
    },
    Vector {
        _handler: RESERVED123,
    },
    Vector { _handler: FLEXSPI },
    Vector { _handler: SEMC },
    Vector { _handler: USDHC1 },
    Vector { _handler: USDHC2 },
    Vector { _handler: USB_OTG2 },
    Vector { _handler: USB_OTG1 },
    Vector { _handler: ENET },
    Vector {
        _handler: ENET_1588_TIMER,
    },
    Vector {
        _handler: XBAR1_IRQ_0_1,
    },
    Vector {
        _handler: XBAR1_IRQ_2_3,
    },
    Vector {
        _handler: ADC_ETC_IRQ0,
    },
    Vector {
        _handler: ADC_ETC_IRQ1,
    },
    Vector {
        _handler: ADC_ETC_IRQ2,
    },
    Vector {
        _handler: ADC_ETC_ERROR_IRQ,
    },
    Vector { _handler: PIT },
    Vector { _handler: ACMP1 },
    Vector { _handler: ACMP2 },
    Vector { _handler: ACMP3 },
    Vector { _handler: ACMP4 },
    Vector {
        _handler: RESERVED143,
    },
    Vector {
        _handler: RESERVED144,
    },
    Vector { _handler: ENC1 },
    Vector { _handler: ENC2 },
    Vector { _handler: ENC3 },
    Vector { _handler: ENC4 },
    Vector { _handler: TMR1 },
    Vector { _handler: TMR2 },
    Vector { _handler: TMR3 },
    Vector { _handler: TMR4 },
    Vector { _handler: PWM2_0 },
    Vector { _handler: PWM2_1 },
    Vector { _handler: PWM2_2 },
    Vector { _handler: PWM2_3 },
    Vector {
        _handler: PWM2_FAULT,
    },
    Vector { _handler: PWM3_0 },
    Vector { _handler: PWM3_1 },
    Vector { _handler: PWM3_2 },
    Vector { _handler: PWM3_3 },
    Vector {
        _handler: PWM3_FAULT,
    },
    Vector { _handler: PWM4_0 },
    Vector { _handler: PWM4_1 },
    Vector { _handler: PWM4_2 },
    Vector { _handler: PWM4_3 },
    Vector {
        _handler: PWM4_FAULT,
    },
    Vector {
        _handler: RESERVED168,
    },
    Vector {
        _handler: RESERVED169,
    },
    Vector {
        _handler: RESERVED170,
    },
    Vector {
        _handler: RESERVED171,
    },
    Vector {
        _handler: RESERVED172,
    },
    Vector {
        _handler: RESERVED173,
    },
    Vector {
        _handler: SJC_ARM_DEBUG,
    },
    Vector {
        _handler: NMI_WAKEUP,
    },
];
#[doc = r" Macro to override a device specific interrupt handler"]
#[doc = r""]
#[doc = r" # Syntax"]
#[doc = r""]
#[doc = r" ``` ignore"]
#[doc = r" interrupt!("]
#[doc = r"     // Name of the interrupt"]
#[doc = r"     $Name:ident,"]
#[doc = r""]
#[doc = r"     // Path to the interrupt handler (a function)"]
#[doc = r"     $handler:path,"]
#[doc = r""]
#[doc = r"     // Optional, state preserved across invocations of the handler"]
#[doc = r"     state: $State:ty = $initial_state:expr,"]
#[doc = r" );"]
#[doc = r" ```"]
#[doc = r""]
#[doc = r" Where `$Name` must match the name of one of the variants of the `Interrupt`"]
#[doc = r" enum."]
#[doc = r""]
#[doc = r" The handler must have signature `fn()` is no state was associated to it;"]
#[doc = r" otherwise its signature must be `fn(&mut $State)`."]
#[cfg(feature = "rt")]
#[macro_export]
macro_rules! interrupt {
    ($Name:ident, $handler:path,state: $State:ty = $initial_state:expr) => {
        #[allow(unsafe_code)]
        #[deny(private_no_mangle_fns)]
        #[no_mangle]
        pub unsafe extern "C" fn $Name() {
            static mut STATE: $State = $initial_state;
            let _ = $crate::Interrupt::$Name;
            let f: fn(&mut $State) = $handler;
            f(&mut STATE)
        }
    };
    ($Name:ident, $handler:path) => {
        #[allow(unsafe_code)]
        #[deny(private_no_mangle_fns)]
        #[no_mangle]
        pub unsafe extern "C" fn $Name() {
            let _ = $crate::Interrupt::$Name;
            let f: fn() = $handler;
            f()
        }
    };
}
#[doc = r" Enumeration of all the interrupts"]
pub enum Interrupt {
    #[doc = "0 - DMA0_DMA16"]
    DMA0_DMA16,
    #[doc = "1 - DMA1_DMA17"]
    DMA1_DMA17,
    #[doc = "2 - DMA2_DMA18"]
    DMA2_DMA18,
    #[doc = "3 - DMA3_DMA19"]
    DMA3_DMA19,
    #[doc = "4 - DMA4_DMA20"]
    DMA4_DMA20,
    #[doc = "5 - DMA5_DMA21"]
    DMA5_DMA21,
    #[doc = "6 - DMA6_DMA22"]
    DMA6_DMA22,
    #[doc = "7 - DMA7_DMA23"]
    DMA7_DMA23,
    #[doc = "8 - DMA8_DMA24"]
    DMA8_DMA24,
    #[doc = "9 - DMA9_DMA25"]
    DMA9_DMA25,
    #[doc = "10 - DMA10_DMA26"]
    DMA10_DMA26,
    #[doc = "11 - DMA11_DMA27"]
    DMA11_DMA27,
    #[doc = "12 - DMA12_DMA28"]
    DMA12_DMA28,
    #[doc = "13 - DMA13_DMA29"]
    DMA13_DMA29,
    #[doc = "14 - DMA14_DMA30"]
    DMA14_DMA30,
    #[doc = "15 - DMA15_DMA31"]
    DMA15_DMA31,
    #[doc = "16 - DMA_ERROR"]
    DMA_ERROR,
    #[doc = "17 - CTI0_ERROR"]
    CTI0_ERROR,
    #[doc = "18 - CTI1_ERROR"]
    CTI1_ERROR,
    #[doc = "19 - CORE"]
    CORE,
    #[doc = "20 - LPUART1"]
    LPUART1,
    #[doc = "21 - LPUART2"]
    LPUART2,
    #[doc = "22 - LPUART3"]
    LPUART3,
    #[doc = "23 - LPUART4"]
    LPUART4,
    #[doc = "24 - LPUART5"]
    LPUART5,
    #[doc = "25 - LPUART6"]
    LPUART6,
    #[doc = "26 - LPUART7"]
    LPUART7,
    #[doc = "27 - LPUART8"]
    LPUART8,
    #[doc = "28 - LPI2C1"]
    LPI2C1,
    #[doc = "29 - LPI2C2"]
    LPI2C2,
    #[doc = "30 - LPI2C3"]
    LPI2C3,
    #[doc = "31 - LPI2C4"]
    LPI2C4,
    #[doc = "32 - LPSPI1"]
    LPSPI1,
    #[doc = "33 - LPSPI2"]
    LPSPI2,
    #[doc = "34 - LPSPI3"]
    LPSPI3,
    #[doc = "35 - LPSPI4"]
    LPSPI4,
    #[doc = "36 - CAN1"]
    CAN1,
    #[doc = "37 - CAN2"]
    CAN2,
    #[doc = "38 - FLEXRAM"]
    FLEXRAM,
    #[doc = "39 - KPP"]
    KPP,
    #[doc = "40 - TSC_DIG"]
    TSC_DIG,
    #[doc = "41 - GPR_IRQ"]
    GPR_IRQ,
    #[doc = "42 - LCDIF"]
    LCDIF,
    #[doc = "43 - CSI"]
    CSI,
    #[doc = "44 - PXP"]
    PXP,
    #[doc = "45 - WDOG2"]
    WDOG2,
    #[doc = "46 - SNVS_HP_WRAPPER"]
    SNVS_HP_WRAPPER,
    #[doc = "47 - SNVS_HP_WRAPPER_TZ"]
    SNVS_HP_WRAPPER_TZ,
    #[doc = "48 - SNVS_LP_WRAPPER"]
    SNVS_LP_WRAPPER,
    #[doc = "49 - CSU"]
    CSU,
    #[doc = "50 - DCP"]
    DCP,
    #[doc = "51 - DCP_VMI"]
    DCP_VMI,
    #[doc = "52 - Reserved68"]
    RESERVED68,
    #[doc = "53 - TRNG"]
    TRNG,
    #[doc = "54 - SJC"]
    SJC,
    #[doc = "55 - BEE"]
    BEE,
    #[doc = "56 - SAI1"]
    SAI1,
    #[doc = "57 - SAI2"]
    SAI2,
    #[doc = "58 - SAI3_RX"]
    SAI3_RX,
    #[doc = "59 - SAI3_TX"]
    SAI3_TX,
    #[doc = "60 - SPDIF"]
    SPDIF,
    #[doc = "61 - ANATOP_EVENT0"]
    ANATOP_EVENT0,
    #[doc = "62 - ANATOP_EVENT1"]
    ANATOP_EVENT1,
    #[doc = "63 - ANATOP_TAMP_LOW_HIGH"]
    ANATOP_TAMP_LOW_HIGH,
    #[doc = "64 - ANATOP_TEMP_PANIC"]
    ANATOP_TEMP_PANIC,
    #[doc = "65 - USB_PHY1"]
    USB_PHY1,
    #[doc = "66 - USB_PHY2"]
    USB_PHY2,
    #[doc = "67 - ADC1"]
    ADC1,
    #[doc = "68 - ADC2"]
    ADC2,
    #[doc = "69 - DCDC"]
    DCDC,
    #[doc = "70 - Reserved86"]
    RESERVED86,
    #[doc = "71 - Reserved87"]
    RESERVED87,
    #[doc = "72 - GPIO1_INT0"]
    GPIO1_INT0,
    #[doc = "73 - GPIO1_INT1"]
    GPIO1_INT1,
    #[doc = "74 - GPIO1_INT2"]
    GPIO1_INT2,
    #[doc = "75 - GPIO1_INT3"]
    GPIO1_INT3,
    #[doc = "76 - GPIO1_INT4"]
    GPIO1_INT4,
    #[doc = "77 - GPIO1_INT5"]
    GPIO1_INT5,
    #[doc = "78 - GPIO1_INT6"]
    GPIO1_INT6,
    #[doc = "79 - GPIO1_INT7"]
    GPIO1_INT7,
    #[doc = "80 - GPIO1_Combined_0_15"]
    GPIO1_COMBINED_0_15,
    #[doc = "81 - GPIO1_Combined_16_31"]
    GPIO1_COMBINED_16_31,
    #[doc = "82 - GPIO2_Combined_0_15"]
    GPIO2_COMBINED_0_15,
    #[doc = "83 - GPIO2_Combined_16_31"]
    GPIO2_COMBINED_16_31,
    #[doc = "84 - GPIO3_Combined_0_15"]
    GPIO3_COMBINED_0_15,
    #[doc = "85 - GPIO3_Combined_16_31"]
    GPIO3_COMBINED_16_31,
    #[doc = "86 - GPIO4_Combined_0_15"]
    GPIO4_COMBINED_0_15,
    #[doc = "87 - GPIO4_Combined_16_31"]
    GPIO4_COMBINED_16_31,
    #[doc = "88 - GPIO5_Combined_0_15"]
    GPIO5_COMBINED_0_15,
    #[doc = "89 - GPIO5_Combined_16_31"]
    GPIO5_COMBINED_16_31,
    #[doc = "90 - FLEXIO1"]
    FLEXIO1,
    #[doc = "91 - FLEXIO2"]
    FLEXIO2,
    #[doc = "92 - WDOG1"]
    WDOG1,
    #[doc = "93 - RTWDOG"]
    RTWDOG,
    #[doc = "94 - EWM"]
    EWM,
    #[doc = "95 - CCM_1"]
    CCM_1,
    #[doc = "96 - CCM_2"]
    CCM_2,
    #[doc = "97 - GPC"]
    GPC,
    #[doc = "98 - SRC"]
    SRC,
    #[doc = "99 - Reserved115"]
    RESERVED115,
    #[doc = "100 - GPT1"]
    GPT1,
    #[doc = "101 - GPT2"]
    GPT2,
    #[doc = "102 - PWM1_0"]
    PWM1_0,
    #[doc = "103 - PWM1_1"]
    PWM1_1,
    #[doc = "104 - PWM1_2"]
    PWM1_2,
    #[doc = "105 - PWM1_3"]
    PWM1_3,
    #[doc = "106 - PWM1_FAULT"]
    PWM1_FAULT,
    #[doc = "107 - Reserved123"]
    RESERVED123,
    #[doc = "108 - FLEXSPI"]
    FLEXSPI,
    #[doc = "109 - SEMC"]
    SEMC,
    #[doc = "110 - USDHC1"]
    USDHC1,
    #[doc = "111 - USDHC2"]
    USDHC2,
    #[doc = "112 - USB_OTG2"]
    USB_OTG2,
    #[doc = "113 - USB_OTG1"]
    USB_OTG1,
    #[doc = "114 - ENET"]
    ENET,
    #[doc = "115 - ENET_1588_Timer"]
    ENET_1588_TIMER,
    #[doc = "116 - XBAR1_IRQ_0_1"]
    XBAR1_IRQ_0_1,
    #[doc = "117 - XBAR1_IRQ_2_3"]
    XBAR1_IRQ_2_3,
    #[doc = "118 - ADC_ETC_IRQ0"]
    ADC_ETC_IRQ0,
    #[doc = "119 - ADC_ETC_IRQ1"]
    ADC_ETC_IRQ1,
    #[doc = "120 - ADC_ETC_IRQ2"]
    ADC_ETC_IRQ2,
    #[doc = "121 - ADC_ETC_ERROR_IRQ"]
    ADC_ETC_ERROR_IRQ,
    #[doc = "122 - PIT"]
    PIT,
    #[doc = "123 - ACMP1"]
    ACMP1,
    #[doc = "124 - ACMP2"]
    ACMP2,
    #[doc = "125 - ACMP3"]
    ACMP3,
    #[doc = "126 - ACMP4"]
    ACMP4,
    #[doc = "127 - Reserved143"]
    RESERVED143,
    #[doc = "128 - Reserved144"]
    RESERVED144,
    #[doc = "129 - ENC1"]
    ENC1,
    #[doc = "130 - ENC2"]
    ENC2,
    #[doc = "131 - ENC3"]
    ENC3,
    #[doc = "132 - ENC4"]
    ENC4,
    #[doc = "133 - TMR1"]
    TMR1,
    #[doc = "134 - TMR2"]
    TMR2,
    #[doc = "135 - TMR3"]
    TMR3,
    #[doc = "136 - TMR4"]
    TMR4,
    #[doc = "137 - PWM2_0"]
    PWM2_0,
    #[doc = "138 - PWM2_1"]
    PWM2_1,
    #[doc = "139 - PWM2_2"]
    PWM2_2,
    #[doc = "140 - PWM2_3"]
    PWM2_3,
    #[doc = "141 - PWM2_FAULT"]
    PWM2_FAULT,
    #[doc = "142 - PWM3_0"]
    PWM3_0,
    #[doc = "143 - PWM3_1"]
    PWM3_1,
    #[doc = "144 - PWM3_2"]
    PWM3_2,
    #[doc = "145 - PWM3_3"]
    PWM3_3,
    #[doc = "146 - PWM3_FAULT"]
    PWM3_FAULT,
    #[doc = "147 - PWM4_0"]
    PWM4_0,
    #[doc = "148 - PWM4_1"]
    PWM4_1,
    #[doc = "149 - PWM4_2"]
    PWM4_2,
    #[doc = "150 - PWM4_3"]
    PWM4_3,
    #[doc = "151 - PWM4_FAULT"]
    PWM4_FAULT,
    #[doc = "152 - Reserved168"]
    RESERVED168,
    #[doc = "153 - Reserved169"]
    RESERVED169,
    #[doc = "154 - Reserved170"]
    RESERVED170,
    #[doc = "155 - Reserved171"]
    RESERVED171,
    #[doc = "156 - Reserved172"]
    RESERVED172,
    #[doc = "157 - Reserved173"]
    RESERVED173,
    #[doc = "158 - SJC_ARM_DEBUG"]
    SJC_ARM_DEBUG,
    #[doc = "159 - NMI_WAKEUP"]
    NMI_WAKEUP,
}
unsafe impl ::bare_metal::Nr for Interrupt {
    #[inline]
    fn nr(&self) -> u8 {
        match *self {
            Interrupt::DMA0_DMA16 => 0,
            Interrupt::DMA1_DMA17 => 1,
            Interrupt::DMA2_DMA18 => 2,
            Interrupt::DMA3_DMA19 => 3,
            Interrupt::DMA4_DMA20 => 4,
            Interrupt::DMA5_DMA21 => 5,
            Interrupt::DMA6_DMA22 => 6,
            Interrupt::DMA7_DMA23 => 7,
            Interrupt::DMA8_DMA24 => 8,
            Interrupt::DMA9_DMA25 => 9,
            Interrupt::DMA10_DMA26 => 10,
            Interrupt::DMA11_DMA27 => 11,
            Interrupt::DMA12_DMA28 => 12,
            Interrupt::DMA13_DMA29 => 13,
            Interrupt::DMA14_DMA30 => 14,
            Interrupt::DMA15_DMA31 => 15,
            Interrupt::DMA_ERROR => 16,
            Interrupt::CTI0_ERROR => 17,
            Interrupt::CTI1_ERROR => 18,
            Interrupt::CORE => 19,
            Interrupt::LPUART1 => 20,
            Interrupt::LPUART2 => 21,
            Interrupt::LPUART3 => 22,
            Interrupt::LPUART4 => 23,
            Interrupt::LPUART5 => 24,
            Interrupt::LPUART6 => 25,
            Interrupt::LPUART7 => 26,
            Interrupt::LPUART8 => 27,
            Interrupt::LPI2C1 => 28,
            Interrupt::LPI2C2 => 29,
            Interrupt::LPI2C3 => 30,
            Interrupt::LPI2C4 => 31,
            Interrupt::LPSPI1 => 32,
            Interrupt::LPSPI2 => 33,
            Interrupt::LPSPI3 => 34,
            Interrupt::LPSPI4 => 35,
            Interrupt::CAN1 => 36,
            Interrupt::CAN2 => 37,
            Interrupt::FLEXRAM => 38,
            Interrupt::KPP => 39,
            Interrupt::TSC_DIG => 40,
            Interrupt::GPR_IRQ => 41,
            Interrupt::LCDIF => 42,
            Interrupt::CSI => 43,
            Interrupt::PXP => 44,
            Interrupt::WDOG2 => 45,
            Interrupt::SNVS_HP_WRAPPER => 46,
            Interrupt::SNVS_HP_WRAPPER_TZ => 47,
            Interrupt::SNVS_LP_WRAPPER => 48,
            Interrupt::CSU => 49,
            Interrupt::DCP => 50,
            Interrupt::DCP_VMI => 51,
            Interrupt::RESERVED68 => 52,
            Interrupt::TRNG => 53,
            Interrupt::SJC => 54,
            Interrupt::BEE => 55,
            Interrupt::SAI1 => 56,
            Interrupt::SAI2 => 57,
            Interrupt::SAI3_RX => 58,
            Interrupt::SAI3_TX => 59,
            Interrupt::SPDIF => 60,
            Interrupt::ANATOP_EVENT0 => 61,
            Interrupt::ANATOP_EVENT1 => 62,
            Interrupt::ANATOP_TAMP_LOW_HIGH => 63,
            Interrupt::ANATOP_TEMP_PANIC => 64,
            Interrupt::USB_PHY1 => 65,
            Interrupt::USB_PHY2 => 66,
            Interrupt::ADC1 => 67,
            Interrupt::ADC2 => 68,
            Interrupt::DCDC => 69,
            Interrupt::RESERVED86 => 70,
            Interrupt::RESERVED87 => 71,
            Interrupt::GPIO1_INT0 => 72,
            Interrupt::GPIO1_INT1 => 73,
            Interrupt::GPIO1_INT2 => 74,
            Interrupt::GPIO1_INT3 => 75,
            Interrupt::GPIO1_INT4 => 76,
            Interrupt::GPIO1_INT5 => 77,
            Interrupt::GPIO1_INT6 => 78,
            Interrupt::GPIO1_INT7 => 79,
            Interrupt::GPIO1_COMBINED_0_15 => 80,
            Interrupt::GPIO1_COMBINED_16_31 => 81,
            Interrupt::GPIO2_COMBINED_0_15 => 82,
            Interrupt::GPIO2_COMBINED_16_31 => 83,
            Interrupt::GPIO3_COMBINED_0_15 => 84,
            Interrupt::GPIO3_COMBINED_16_31 => 85,
            Interrupt::GPIO4_COMBINED_0_15 => 86,
            Interrupt::GPIO4_COMBINED_16_31 => 87,
            Interrupt::GPIO5_COMBINED_0_15 => 88,
            Interrupt::GPIO5_COMBINED_16_31 => 89,
            Interrupt::FLEXIO1 => 90,
            Interrupt::FLEXIO2 => 91,
            Interrupt::WDOG1 => 92,
            Interrupt::RTWDOG => 93,
            Interrupt::EWM => 94,
            Interrupt::CCM_1 => 95,
            Interrupt::CCM_2 => 96,
            Interrupt::GPC => 97,
            Interrupt::SRC => 98,
            Interrupt::RESERVED115 => 99,
            Interrupt::GPT1 => 100,
            Interrupt::GPT2 => 101,
            Interrupt::PWM1_0 => 102,
            Interrupt::PWM1_1 => 103,
            Interrupt::PWM1_2 => 104,
            Interrupt::PWM1_3 => 105,
            Interrupt::PWM1_FAULT => 106,
            Interrupt::RESERVED123 => 107,
            Interrupt::FLEXSPI => 108,
            Interrupt::SEMC => 109,
            Interrupt::USDHC1 => 110,
            Interrupt::USDHC2 => 111,
            Interrupt::USB_OTG2 => 112,
            Interrupt::USB_OTG1 => 113,
            Interrupt::ENET => 114,
            Interrupt::ENET_1588_TIMER => 115,
            Interrupt::XBAR1_IRQ_0_1 => 116,
            Interrupt::XBAR1_IRQ_2_3 => 117,
            Interrupt::ADC_ETC_IRQ0 => 118,
            Interrupt::ADC_ETC_IRQ1 => 119,
            Interrupt::ADC_ETC_IRQ2 => 120,
            Interrupt::ADC_ETC_ERROR_IRQ => 121,
            Interrupt::PIT => 122,
            Interrupt::ACMP1 => 123,
            Interrupt::ACMP2 => 124,
            Interrupt::ACMP3 => 125,
            Interrupt::ACMP4 => 126,
            Interrupt::RESERVED143 => 127,
            Interrupt::RESERVED144 => 128,
            Interrupt::ENC1 => 129,
            Interrupt::ENC2 => 130,
            Interrupt::ENC3 => 131,
            Interrupt::ENC4 => 132,
            Interrupt::TMR1 => 133,
            Interrupt::TMR2 => 134,
            Interrupt::TMR3 => 135,
            Interrupt::TMR4 => 136,
            Interrupt::PWM2_0 => 137,
            Interrupt::PWM2_1 => 138,
            Interrupt::PWM2_2 => 139,
            Interrupt::PWM2_3 => 140,
            Interrupt::PWM2_FAULT => 141,
            Interrupt::PWM3_0 => 142,
            Interrupt::PWM3_1 => 143,
            Interrupt::PWM3_2 => 144,
            Interrupt::PWM3_3 => 145,
            Interrupt::PWM3_FAULT => 146,
            Interrupt::PWM4_0 => 147,
            Interrupt::PWM4_1 => 148,
            Interrupt::PWM4_2 => 149,
            Interrupt::PWM4_3 => 150,
            Interrupt::PWM4_FAULT => 151,
            Interrupt::RESERVED168 => 152,
            Interrupt::RESERVED169 => 153,
            Interrupt::RESERVED170 => 154,
            Interrupt::RESERVED171 => 155,
            Interrupt::RESERVED172 => 156,
            Interrupt::RESERVED173 => 157,
            Interrupt::SJC_ARM_DEBUG => 158,
            Interrupt::NMI_WAKEUP => 159,
        }
    }
}
#[doc(hidden)]
pub mod interrupt;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, FPU, ITM, MPU, NVIC, SCB, SYST, TPIU};

cfg_if! {
    if #[cfg(any(feature = "aipstz1", feature = "everything"))] {
        #[doc = "AIPSTZ Control Registers"]
        pub struct AIPSTZ1 {
            _marker: PhantomData<*const ()>,
        }
        unsafe impl Send for AIPSTZ1 {}
        impl AIPSTZ1 {
            #[doc = r" Returns a pointer to the register block"]
            pub fn ptr() -> *const aipstz1::RegisterBlock {
                1074249728 as *const _
            }
        }
        impl Deref for AIPSTZ1 {
            type Target = aipstz1::RegisterBlock;
            fn deref(&self) -> &aipstz1::RegisterBlock {
                unsafe { &*AIPSTZ1::ptr() }
            }
        }
        #[doc = "AIPSTZ Control Registers"]
        pub mod aipstz1;
    }
}

cfg_if! {
    if #[cfg(any(feature = "ccm_analog", feature = "everything"))] {
        #[doc = "CCM_ANALOG"]
        pub struct CCM_ANALOG {
            _marker: PhantomData<*const ()>,
        }
        unsafe impl Send for CCM_ANALOG {}
        impl CCM_ANALOG {
            #[doc = r" Returns a pointer to the register block"]
            pub fn ptr() -> *const ccm_analog::RegisterBlock {
                1074626560 as *const _
            }
        }
        impl Deref for CCM_ANALOG {
            type Target = ccm_analog::RegisterBlock;
            fn deref(&self) -> &ccm_analog::RegisterBlock {
                unsafe { &*CCM_ANALOG::ptr() }
            }
        }
        #[doc = "CCM_ANALOG"]
        pub mod ccm_analog;
    }
}

cfg_if! {
    if #[cfg(any(feature = "ccm", feature = "everything"))] {
        #[doc = "CCM"]
        pub struct CCM {
            _marker: PhantomData<*const ()>,
        }
        unsafe impl Send for CCM {}
        impl CCM {
            #[doc = r" Returns a pointer to the register block"]
            pub fn ptr() -> *const ccm::RegisterBlock {
                1074774016 as *const _
            }
        }
        impl Deref for CCM {
            type Target = ccm::RegisterBlock;
            fn deref(&self) -> &ccm::RegisterBlock {
                unsafe { &*CCM::ptr() }
            }
        }
        #[doc = "CCM"]
        pub mod ccm;
    }
}

cfg_if! {
    if #[cfg(any(feature = "gpio1", feature = "everything"))] {
        #[doc = "GPIO"]
        pub struct GPIO1 {
            _marker: PhantomData<*const ()>,
        }
        unsafe impl Send for GPIO1 {}
        impl GPIO1 {
            #[doc = r" Returns a pointer to the register block"]
            pub fn ptr() -> *const gpio1::RegisterBlock {
                1075544064 as *const _
            }
        }
        impl Deref for GPIO1 {
            type Target = gpio1::RegisterBlock;
            fn deref(&self) -> &gpio1::RegisterBlock {
                unsafe { &*GPIO1::ptr() }
            }
        }
        #[doc = "GPIO"]
        pub mod gpio1;
    }
}

cfg_if! {
    if #[cfg(any(feature = "iomuxc", feature = "everything"))] {
        #[doc = "IOMUXC"]
        pub struct IOMUXC {
            _marker: PhantomData<*const ()>,
        }
        unsafe impl Send for IOMUXC {}
        impl IOMUXC {
            #[doc = r" Returns a pointer to the register block"]
            pub fn ptr() -> *const iomuxc::RegisterBlock {
                1075806208 as *const _
            }
        }
        impl Deref for IOMUXC {
            type Target = iomuxc::RegisterBlock;
            fn deref(&self) -> &iomuxc::RegisterBlock {
                unsafe { &*IOMUXC::ptr() }
            }
        }
        #[doc = "IOMUXC"]
        pub mod iomuxc;
    }
}

cfg_if! {
    if #[cfg(any(feature = "dcdc", feature = "everything"))] {
        #[doc = "DCDC"]
        pub struct DCDC {
            _marker: PhantomData<*const ()>,
        }
        unsafe impl Send for DCDC {}
        impl DCDC {
            #[doc = r" Returns a pointer to the register block"]
            pub fn ptr() -> *const dcdc::RegisterBlock {
                1074266112 as *const _
            }
        }
        impl Deref for DCDC {
            type Target = dcdc::RegisterBlock;
            fn deref(&self) -> &dcdc::RegisterBlock {
                unsafe { &*DCDC::ptr() }
            }
        }
        #[doc = "DCDC"]
        pub mod dcdc;
    }
}

cfg_if! {
    if #[cfg(any(feature = "enet", feature = "everything"))] {
        #[doc = "Ethernet MAC-NET Core"]
        pub struct ENET {
            _marker: PhantomData<*const ()>,
        }
        unsafe impl Send for ENET {}
        impl ENET {
            #[doc = r" Returns a pointer to the register block"]
            pub fn ptr() -> *const enet::RegisterBlock {
                1076723712 as *const _
            }
        }
        impl Deref for ENET {
            type Target = enet::RegisterBlock;
            fn deref(&self) -> &enet::RegisterBlock {
                unsafe { &*ENET::ptr() }
            }
        }
        #[doc = "Ethernet MAC-NET Core"]
        pub mod enet;
    }
}

cfg_if! {
    if #[cfg(any(feature = "wdog", feature = "everything"))] {
        #[doc = "WDOG"]
        pub struct WDOG1 {
            _marker: PhantomData<*const ()>,
        }
        unsafe impl Send for WDOG1 {}
        impl WDOG1 {
            #[doc = r" Returns a pointer to the register block"]
            pub fn ptr() -> *const wdog1::RegisterBlock {
                1074495488 as *const _
            }
        }
        impl Deref for WDOG1 {
            type Target = wdog1::RegisterBlock;
            fn deref(&self) -> &wdog1::RegisterBlock {
                unsafe { &*WDOG1::ptr() }
            }
        }
        #[doc = "WDOG"]
        pub mod wdog1;
        #[doc = "WDOG"]
        pub struct WDOG2 {
            _marker: PhantomData<*const ()>,
        }
        unsafe impl Send for WDOG2 {}
        impl WDOG2 {
            #[doc = r" Returns a pointer to the register block"]
            pub fn ptr() -> *const wdog1::RegisterBlock {
                1074593792 as *const _
            }
        }
        impl Deref for WDOG2 {
            type Target = wdog1::RegisterBlock;
            fn deref(&self) -> &wdog1::RegisterBlock {
                unsafe { &*WDOG2::ptr() }
            }
        }
        #[doc = "WDOG"]
        pub struct RTWDOG {
            _marker: PhantomData<*const ()>,
        }
        unsafe impl Send for RTWDOG {}
        impl RTWDOG {
            #[doc = r" Returns a pointer to the register block"]
            pub fn ptr() -> *const rtwdog::RegisterBlock {
                1074511872 as *const _
            }
        }
        impl Deref for RTWDOG {
            type Target = rtwdog::RegisterBlock;
            fn deref(&self) -> &rtwdog::RegisterBlock {
                unsafe { &*RTWDOG::ptr() }
            }
        }
        #[doc = "WDOG"]
        pub mod rtwdog;
    }
}

cfg_if! {
    if #[cfg(any(feature = "xtalosc24m", feature = "everything"))] {
        #[doc = "XTALOSC24M"]
        pub struct XTALOSC24M {
            _marker: PhantomData<*const ()>,
        }
        unsafe impl Send for XTALOSC24M {}
        impl XTALOSC24M {
            #[doc = r" Returns a pointer to the register block"]
            pub fn ptr() -> *const xtalosc24m::RegisterBlock {
                1074626560 as *const _
            }
        }
        impl Deref for XTALOSC24M {
            type Target = xtalosc24m::RegisterBlock;
            fn deref(&self) -> &xtalosc24m::RegisterBlock {
                unsafe { &*XTALOSC24M::ptr() }
            }
        }
        #[doc = "XTALOSC24M"]
        pub mod xtalosc24m;
    }
}

cfg_if! {
    if #[cfg(any(feature = "iomuxc_gpr", feature = "everything"))] {
        #[doc = "IOMUXC_GPR"]
        pub struct IOMUXC_GPR {
            _marker: PhantomData<*const ()>,
        }
        unsafe impl Send for IOMUXC_GPR {}
        impl IOMUXC_GPR {
            #[doc = r" Returns a pointer to the register block"]
            pub fn ptr() -> *const iomuxc_gpr::RegisterBlock {
                1074446336 as *const _
            }
        }
        impl Deref for IOMUXC_GPR {
            type Target = iomuxc_gpr::RegisterBlock;
            fn deref(&self) -> &iomuxc_gpr::RegisterBlock {
                unsafe { &*IOMUXC_GPR::ptr() }
            }
        }
        #[doc = "IOMUXC_GPR"]
        pub mod iomuxc_gpr;
    }
}

cfg_if! {
    if #[cfg(any(feature = "notimplemented", feature = "everything"))] {
        #[doc = "AIPSTZ Control Registers"]
        pub struct AIPSTZ2 {
            _marker: PhantomData<*const ()>,
        }
        unsafe impl Send for AIPSTZ2 {}
        impl AIPSTZ2 {
            #[doc = r" Returns a pointer to the register block"]
            pub fn ptr() -> *const aipstz1::RegisterBlock {
                1075298304 as *const _
            }
        }
        impl Deref for AIPSTZ2 {
            type Target = aipstz1::RegisterBlock;
            fn deref(&self) -> &aipstz1::RegisterBlock {
                unsafe { &*AIPSTZ2::ptr() }
            }
        }
        #[doc = "AIPSTZ Control Registers"]
        pub struct AIPSTZ3 {
            _marker: PhantomData<*const ()>,
        }
        unsafe impl Send for AIPSTZ3 {}
        impl AIPSTZ3 {
            #[doc = r" Returns a pointer to the register block"]
            pub fn ptr() -> *const aipstz1::RegisterBlock {
                1076346880 as *const _
            }
        }
        impl Deref for AIPSTZ3 {
            type Target = aipstz1::RegisterBlock;
            fn deref(&self) -> &aipstz1::RegisterBlock {
                unsafe { &*AIPSTZ3::ptr() }
            }
        }
        #[doc = "AIPSTZ Control Registers"]
        pub struct AIPSTZ4 {
            _marker: PhantomData<*const ()>,
        }
        unsafe impl Send for AIPSTZ4 {}
        impl AIPSTZ4 {
            #[doc = r" Returns a pointer to the register block"]
            pub fn ptr() -> *const aipstz1::RegisterBlock {
                1077395456 as *const _
            }
        }
        impl Deref for AIPSTZ4 {
            type Target = aipstz1::RegisterBlock;
            fn deref(&self) -> &aipstz1::RegisterBlock {
                unsafe { &*AIPSTZ4::ptr() }
            }
        }
        #[doc = "PIT"]
        pub struct PIT {
            _marker: PhantomData<*const ()>,
        }
        unsafe impl Send for PIT {}
        impl PIT {
            #[doc = r" Returns a pointer to the register block"]
            pub fn ptr() -> *const pit::RegisterBlock {
                1074282496 as *const _
            }
        }
        impl Deref for PIT {
            type Target = pit::RegisterBlock;
            fn deref(&self) -> &pit::RegisterBlock {
                unsafe { &*PIT::ptr() }
            }
        }
        #[doc = "PIT"]
        pub mod pit;
        #[doc = "High-Speed Comparator (CMP), Voltage Reference (VREF) Digital-to-Analog Converter (DAC), and Analog Mux (ANMUX)"]
        pub struct CMP1 {
            _marker: PhantomData<*const ()>,
        }
        unsafe impl Send for CMP1 {}
        impl CMP1 {
            #[doc = r" Returns a pointer to the register block"]
            pub fn ptr() -> *const cmp1::RegisterBlock {
                1074348032 as *const _
            }
        }
        impl Deref for CMP1 {
            type Target = cmp1::RegisterBlock;
            fn deref(&self) -> &cmp1::RegisterBlock {
                unsafe { &*CMP1::ptr() }
            }
        }
        #[doc = "High-Speed Comparator (CMP), Voltage Reference (VREF) Digital-to-Analog Converter (DAC), and Analog Mux (ANMUX)"]
        pub mod cmp1;
        #[doc = "High-Speed Comparator (CMP), Voltage Reference (VREF) Digital-to-Analog Converter (DAC), and Analog Mux (ANMUX)"]
        pub struct CMP2 {
            _marker: PhantomData<*const ()>,
        }
        unsafe impl Send for CMP2 {}
        impl CMP2 {
            #[doc = r" Returns a pointer to the register block"]
            pub fn ptr() -> *const cmp1::RegisterBlock {
                1074348040 as *const _
            }
        }
        impl Deref for CMP2 {
            type Target = cmp1::RegisterBlock;
            fn deref(&self) -> &cmp1::RegisterBlock {
                unsafe { &*CMP2::ptr() }
            }
        }
        #[doc = "High-Speed Comparator (CMP), Voltage Reference (VREF) Digital-to-Analog Converter (DAC), and Analog Mux (ANMUX)"]
        pub struct CMP3 {
            _marker: PhantomData<*const ()>,
        }
        unsafe impl Send for CMP3 {}
        impl CMP3 {
            #[doc = r" Returns a pointer to the register block"]
            pub fn ptr() -> *const cmp1::RegisterBlock {
                1074348048 as *const _
            }
        }
        impl Deref for CMP3 {
            type Target = cmp1::RegisterBlock;
            fn deref(&self) -> &cmp1::RegisterBlock {
                unsafe { &*CMP3::ptr() }
            }
        }
        #[doc = "High-Speed Comparator (CMP), Voltage Reference (VREF) Digital-to-Analog Converter (DAC), and Analog Mux (ANMUX)"]
        pub struct CMP4 {
            _marker: PhantomData<*const ()>,
        }
        unsafe impl Send for CMP4 {}
        impl CMP4 {
            #[doc = r" Returns a pointer to the register block"]
            pub fn ptr() -> *const cmp1::RegisterBlock {
                1074348056 as *const _
            }
        }
        impl Deref for CMP4 {
            type Target = cmp1::RegisterBlock;
            fn deref(&self) -> &cmp1::RegisterBlock {
                unsafe { &*CMP4::ptr() }
            }
        }
        #[doc = "IOMUXC"]
        pub struct IOMUXC_SNVS_GPR {
            _marker: PhantomData<*const ()>,
        }
        unsafe impl Send for IOMUXC_SNVS_GPR {}
        impl IOMUXC_SNVS_GPR {
            #[doc = r" Returns a pointer to the register block"]
            pub fn ptr() -> *const iomuxc_snvs_gpr::RegisterBlock {
                1074413568 as *const _
            }
        }
        impl Deref for IOMUXC_SNVS_GPR {
            type Target = iomuxc_snvs_gpr::RegisterBlock;
            fn deref(&self) -> &iomuxc_snvs_gpr::RegisterBlock {
                unsafe { &*IOMUXC_SNVS_GPR::ptr() }
            }
        }
        #[doc = "IOMUXC"]
        pub mod iomuxc_snvs_gpr;
        #[doc = "IOMUXC_SNVS"]
        pub struct IOMUXC_SNVS {
            _marker: PhantomData<*const ()>,
        }
        unsafe impl Send for IOMUXC_SNVS {}
        impl IOMUXC_SNVS {
            #[doc = r" Returns a pointer to the register block"]
            pub fn ptr() -> *const iomuxc_snvs::RegisterBlock {
                1074429952 as *const _
            }
        }
        impl Deref for IOMUXC_SNVS {
            type Target = iomuxc_snvs::RegisterBlock;
            fn deref(&self) -> &iomuxc_snvs::RegisterBlock {
                unsafe { &*IOMUXC_SNVS::ptr() }
            }
        }
        #[doc = "IOMUXC_SNVS"]
        pub mod iomuxc_snvs;
        #[doc = "FLEXRAM"]
        pub struct FLEXRAM {
            _marker: PhantomData<*const ()>,
        }
        unsafe impl Send for FLEXRAM {}
        impl FLEXRAM {
            #[doc = r" Returns a pointer to the register block"]
            pub fn ptr() -> *const flexram::RegisterBlock {
                1074462720 as *const _
            }
        }
        impl Deref for FLEXRAM {
            type Target = flexram::RegisterBlock;
            fn deref(&self) -> &flexram::RegisterBlock {
                unsafe { &*FLEXRAM::ptr() }
            }
        }
        #[doc = "FLEXRAM"]
        pub mod flexram;
        #[doc = "EWM"]
        pub struct EWM {
            _marker: PhantomData<*const ()>,
        }
        unsafe impl Send for EWM {}
        impl EWM {
            #[doc = r" Returns a pointer to the register block"]
            pub fn ptr() -> *const ewm::RegisterBlock {
                1074479104 as *const _
            }
        }
        impl Deref for EWM {
            type Target = ewm::RegisterBlock;
            fn deref(&self) -> &ewm::RegisterBlock {
                unsafe { &*EWM::ptr() }
            }
        }
        #[doc = "EWM"]
        pub mod ewm;
        #[doc = "Analog-to-Digital Converter"]
        pub struct ADC1 {
            _marker: PhantomData<*const ()>,
        }
        unsafe impl Send for ADC1 {}
        impl ADC1 {
            #[doc = r" Returns a pointer to the register block"]
            pub fn ptr() -> *const adc1::RegisterBlock {
                1074544640 as *const _
            }
        }
        impl Deref for ADC1 {
            type Target = adc1::RegisterBlock;
            fn deref(&self) -> &adc1::RegisterBlock {
                unsafe { &*ADC1::ptr() }
            }
        }
        #[doc = "Analog-to-Digital Converter"]
        pub mod adc1;
        #[doc = "Analog-to-Digital Converter"]
        pub struct ADC2 {
            _marker: PhantomData<*const ()>,
        }
        unsafe impl Send for ADC2 {}
        impl ADC2 {
            #[doc = r" Returns a pointer to the register block"]
            pub fn ptr() -> *const adc1::RegisterBlock {
                1074561024 as *const _
            }
        }
        impl Deref for ADC2 {
            type Target = adc1::RegisterBlock;
            fn deref(&self) -> &adc1::RegisterBlock {
                unsafe { &*ADC2::ptr() }
            }
        }
        #[doc = "TRNG"]
        pub struct TRNG {
            _marker: PhantomData<*const ()>,
        }
        unsafe impl Send for TRNG {}
        impl TRNG {
            #[doc = r" Returns a pointer to the register block"]
            pub fn ptr() -> *const trng::RegisterBlock {
                1074577408 as *const _
            }
        }
        impl Deref for TRNG {
            type Target = trng::RegisterBlock;
            fn deref(&self) -> &trng::RegisterBlock {
                unsafe { &*TRNG::ptr() }
            }
        }
        #[doc = "TRNG"]
        pub mod trng;
        #[doc = "SNVS"]
        pub struct SNVS {
            _marker: PhantomData<*const ()>,
        }
        unsafe impl Send for SNVS {}
        impl SNVS {
            #[doc = r" Returns a pointer to the register block"]
            pub fn ptr() -> *const snvs::RegisterBlock {
                1074610176 as *const _
            }
        }
        impl Deref for SNVS {
            type Target = snvs::RegisterBlock;
            fn deref(&self) -> &snvs::RegisterBlock {
                unsafe { &*SNVS::ptr() }
            }
        }
        #[doc = "SNVS"]
        pub mod snvs;
        #[doc = "PMU"]
        pub struct PMU {
            _marker: PhantomData<*const ()>,
        }
        unsafe impl Send for PMU {}
        impl PMU {
            #[doc = r" Returns a pointer to the register block"]
            pub fn ptr() -> *const pmu::RegisterBlock {
                1074626560 as *const _
            }
        }
        impl Deref for PMU {
            type Target = pmu::RegisterBlock;
            fn deref(&self) -> &pmu::RegisterBlock {
                unsafe { &*PMU::ptr() }
            }
        }
        #[doc = "PMU"]
        pub mod pmu;
        #[doc = "Temperature Monitor"]
        pub struct TEMPMON {
            _marker: PhantomData<*const ()>,
        }
        unsafe impl Send for TEMPMON {}
        impl TEMPMON {
            #[doc = r" Returns a pointer to the register block"]
            pub fn ptr() -> *const tempmon::RegisterBlock {
                1074626560 as *const _
            }
        }
        impl Deref for TEMPMON {
            type Target = tempmon::RegisterBlock;
            fn deref(&self) -> &tempmon::RegisterBlock {
                unsafe { &*TEMPMON::ptr() }
            }
        }
        #[doc = "Temperature Monitor"]
        pub mod tempmon;
        #[doc = "USB Analog"]
        pub struct USB_ANALOG {
            _marker: PhantomData<*const ()>,
        }
        unsafe impl Send for USB_ANALOG {}
        impl USB_ANALOG {
            #[doc = r" Returns a pointer to the register block"]
            pub fn ptr() -> *const usb_analog::RegisterBlock {
                1074626560 as *const _
            }
        }
        impl Deref for USB_ANALOG {
            type Target = usb_analog::RegisterBlock;
            fn deref(&self) -> &usb_analog::RegisterBlock {
                unsafe { &*USB_ANALOG::ptr() }
            }
        }
        #[doc = "USB Analog"]
        pub mod usb_analog;
        #[doc = "USBPHY Register Reference Index"]
        pub struct USBPHY1 {
            _marker: PhantomData<*const ()>,
        }
        unsafe impl Send for USBPHY1 {}
        impl USBPHY1 {
            #[doc = r" Returns a pointer to the register block"]
            pub fn ptr() -> *const usbphy1::RegisterBlock {
                1074630656 as *const _
            }
        }
        impl Deref for USBPHY1 {
            type Target = usbphy1::RegisterBlock;
            fn deref(&self) -> &usbphy1::RegisterBlock {
                unsafe { &*USBPHY1::ptr() }
            }
        }
        #[doc = "USBPHY Register Reference Index"]
        pub mod usbphy1;
        #[doc = "USBPHY Register Reference Index"]
        pub struct USBPHY2 {
            _marker: PhantomData<*const ()>,
        }
        unsafe impl Send for USBPHY2 {}
        impl USBPHY2 {
            #[doc = r" Returns a pointer to the register block"]
            pub fn ptr() -> *const usbphy1::RegisterBlock {
                1074634752 as *const _
            }
        }
        impl Deref for USBPHY2 {
            type Target = usbphy1::RegisterBlock;
            fn deref(&self) -> &usbphy1::RegisterBlock {
                unsafe { &*USBPHY2::ptr() }
            }
        }
        #[doc = "CSU registers"]
        pub struct CSU {
            _marker: PhantomData<*const ()>,
        }
        unsafe impl Send for CSU {}
        impl CSU {
            #[doc = r" Returns a pointer to the register block"]
            pub fn ptr() -> *const csu::RegisterBlock {
                1074642944 as *const _
            }
        }
        impl Deref for CSU {
            type Target = csu::RegisterBlock;
            fn deref(&self) -> &csu::RegisterBlock {
                unsafe { &*CSU::ptr() }
            }
        }
        #[doc = "CSU registers"]
        pub mod csu;
        #[doc = "Touch Screen Controller"]
        pub struct TSC {
            _marker: PhantomData<*const ()>,
        }
        unsafe impl Send for TSC {}
        impl TSC {
            #[doc = r" Returns a pointer to the register block"]
            pub fn ptr() -> *const tsc::RegisterBlock {
                1074659328 as *const _
            }
        }
        impl Deref for TSC {
            type Target = tsc::RegisterBlock;
            fn deref(&self) -> &tsc::RegisterBlock {
                unsafe { &*TSC::ptr() }
            }
        }
        #[doc = "Touch Screen Controller"]
        pub mod tsc;
        #[doc = "DMA"]
        pub struct DMA0 {
            _marker: PhantomData<*const ()>,
        }
        unsafe impl Send for DMA0 {}
        impl DMA0 {
            #[doc = r" Returns a pointer to the register block"]
            pub fn ptr() -> *const dma0::RegisterBlock {
                1074692096 as *const _
            }
        }
        impl Deref for DMA0 {
            type Target = dma0::RegisterBlock;
            fn deref(&self) -> &dma0::RegisterBlock {
                unsafe { &*DMA0::ptr() }
            }
        }
        #[doc = "DMA"]
        pub mod dma0;
        #[doc = "DMA_CH_MUX"]
        pub struct DMAMUX {
            _marker: PhantomData<*const ()>,
        }
        unsafe impl Send for DMAMUX {}
        impl DMAMUX {
            #[doc = r" Returns a pointer to the register block"]
            pub fn ptr() -> *const dmamux::RegisterBlock {
                1074708480 as *const _
            }
        }
        impl Deref for DMAMUX {
            type Target = dmamux::RegisterBlock;
            fn deref(&self) -> &dmamux::RegisterBlock {
                unsafe { &*DMAMUX::ptr() }
            }
        }
        #[doc = "DMA_CH_MUX"]
        pub mod dmamux;
        #[doc = "GPC"]
        pub struct GPC {
            _marker: PhantomData<*const ()>,
        }
        unsafe impl Send for GPC {}
        impl GPC {
            #[doc = r" Returns a pointer to the register block"]
            pub fn ptr() -> *const gpc::RegisterBlock {
                1074741248 as *const _
            }
        }
        impl Deref for GPC {
            type Target = gpc::RegisterBlock;
            fn deref(&self) -> &gpc::RegisterBlock {
                unsafe { &*GPC::ptr() }
            }
        }
        #[doc = "GPC"]
        pub mod gpc;
        #[doc = "PGC"]
        pub struct PGC {
            _marker: PhantomData<*const ()>,
        }
        unsafe impl Send for PGC {}
        impl PGC {
            #[doc = r" Returns a pointer to the register block"]
            pub fn ptr() -> *const pgc::RegisterBlock {
                1074741248 as *const _
            }
        }
        impl Deref for PGC {
            type Target = pgc::RegisterBlock;
            fn deref(&self) -> &pgc::RegisterBlock {
                unsafe { &*PGC::ptr() }
            }
        }
        #[doc = "PGC"]
        pub mod pgc;
        #[doc = "SRC"]
        pub struct SRC {
            _marker: PhantomData<*const ()>,
        }
        unsafe impl Send for SRC {}
        impl SRC {
            #[doc = r" Returns a pointer to the register block"]
            pub fn ptr() -> *const src::RegisterBlock {
                1074757632 as *const _
            }
        }
        impl Deref for SRC {
            type Target = src::RegisterBlock;
            fn deref(&self) -> &src::RegisterBlock {
                unsafe { &*SRC::ptr() }
            }
        }
        #[doc = "SRC"]
        pub mod src;
        #[doc = "ROMC"]
        pub struct ROMC {
            _marker: PhantomData<*const ()>,
        }
        unsafe impl Send for ROMC {}
        impl ROMC {
            #[doc = r" Returns a pointer to the register block"]
            pub fn ptr() -> *const romc::RegisterBlock {
                1075314688 as *const _
            }
        }
        impl Deref for ROMC {
            type Target = romc::RegisterBlock;
            fn deref(&self) -> &romc::RegisterBlock {
                unsafe { &*ROMC::ptr() }
            }
        }
        #[doc = "ROMC"]
        pub mod romc;
        #[doc = "LPUART"]
        pub struct LPUART1 {
            _marker: PhantomData<*const ()>,
        }
        unsafe impl Send for LPUART1 {}
        impl LPUART1 {
            #[doc = r" Returns a pointer to the register block"]
            pub fn ptr() -> *const lpuart1::RegisterBlock {
                1075331072 as *const _
            }
        }
        impl Deref for LPUART1 {
            type Target = lpuart1::RegisterBlock;
            fn deref(&self) -> &lpuart1::RegisterBlock {
                unsafe { &*LPUART1::ptr() }
            }
        }
        #[doc = "LPUART"]
        pub mod lpuart1;
        #[doc = "LPUART"]
        pub struct LPUART2 {
            _marker: PhantomData<*const ()>,
        }
        unsafe impl Send for LPUART2 {}
        impl LPUART2 {
            #[doc = r" Returns a pointer to the register block"]
            pub fn ptr() -> *const lpuart1::RegisterBlock {
                1075347456 as *const _
            }
        }
        impl Deref for LPUART2 {
            type Target = lpuart1::RegisterBlock;
            fn deref(&self) -> &lpuart1::RegisterBlock {
                unsafe { &*LPUART2::ptr() }
            }
        }
        #[doc = "LPUART"]
        pub struct LPUART3 {
            _marker: PhantomData<*const ()>,
        }
        unsafe impl Send for LPUART3 {}
        impl LPUART3 {
            #[doc = r" Returns a pointer to the register block"]
            pub fn ptr() -> *const lpuart1::RegisterBlock {
                1075363840 as *const _
            }
        }
        impl Deref for LPUART3 {
            type Target = lpuart1::RegisterBlock;
            fn deref(&self) -> &lpuart1::RegisterBlock {
                unsafe { &*LPUART3::ptr() }
            }
        }
        #[doc = "LPUART"]
        pub struct LPUART4 {
            _marker: PhantomData<*const ()>,
        }
        unsafe impl Send for LPUART4 {}
        impl LPUART4 {
            #[doc = r" Returns a pointer to the register block"]
            pub fn ptr() -> *const lpuart1::RegisterBlock {
                1075380224 as *const _
            }
        }
        impl Deref for LPUART4 {
            type Target = lpuart1::RegisterBlock;
            fn deref(&self) -> &lpuart1::RegisterBlock {
                unsafe { &*LPUART4::ptr() }
            }
        }
        #[doc = "LPUART"]
        pub struct LPUART5 {
            _marker: PhantomData<*const ()>,
        }
        unsafe impl Send for LPUART5 {}
        impl LPUART5 {
            #[doc = r" Returns a pointer to the register block"]
            pub fn ptr() -> *const lpuart1::RegisterBlock {
                1075396608 as *const _
            }
        }
        impl Deref for LPUART5 {
            type Target = lpuart1::RegisterBlock;
            fn deref(&self) -> &lpuart1::RegisterBlock {
                unsafe { &*LPUART5::ptr() }
            }
        }
        #[doc = "LPUART"]
        pub struct LPUART6 {
            _marker: PhantomData<*const ()>,
        }
        unsafe impl Send for LPUART6 {}
        impl LPUART6 {
            #[doc = r" Returns a pointer to the register block"]
            pub fn ptr() -> *const lpuart1::RegisterBlock {
                1075412992 as *const _
            }
        }
        impl Deref for LPUART6 {
            type Target = lpuart1::RegisterBlock;
            fn deref(&self) -> &lpuart1::RegisterBlock {
                unsafe { &*LPUART6::ptr() }
            }
        }
        #[doc = "LPUART"]
        pub struct LPUART7 {
            _marker: PhantomData<*const ()>,
        }
        unsafe impl Send for LPUART7 {}
        impl LPUART7 {
            #[doc = r" Returns a pointer to the register block"]
            pub fn ptr() -> *const lpuart1::RegisterBlock {
                1075429376 as *const _
            }
        }
        impl Deref for LPUART7 {
            type Target = lpuart1::RegisterBlock;
            fn deref(&self) -> &lpuart1::RegisterBlock {
                unsafe { &*LPUART7::ptr() }
            }
        }
        #[doc = "LPUART"]
        pub struct LPUART8 {
            _marker: PhantomData<*const ()>,
        }
        unsafe impl Send for LPUART8 {}
        impl LPUART8 {
            #[doc = r" Returns a pointer to the register block"]
            pub fn ptr() -> *const lpuart1::RegisterBlock {
                1075445760 as *const _
            }
        }
        impl Deref for LPUART8 {
            type Target = lpuart1::RegisterBlock;
            fn deref(&self) -> &lpuart1::RegisterBlock {
                unsafe { &*LPUART8::ptr() }
            }
        }
        #[doc = "FLEXIO"]
        pub struct FLEXIO1 {
            _marker: PhantomData<*const ()>,
        }
        unsafe impl Send for FLEXIO1 {}
        impl FLEXIO1 {
            #[doc = r" Returns a pointer to the register block"]
            pub fn ptr() -> *const flexio1::RegisterBlock {
                1075494912 as *const _
            }
        }
        impl Deref for FLEXIO1 {
            type Target = flexio1::RegisterBlock;
            fn deref(&self) -> &flexio1::RegisterBlock {
                unsafe { &*FLEXIO1::ptr() }
            }
        }
        #[doc = "FLEXIO"]
        pub mod flexio1;
        #[doc = "FLEXIO"]
        pub struct FLEXIO2 {
            _marker: PhantomData<*const ()>,
        }
        unsafe impl Send for FLEXIO2 {}
        impl FLEXIO2 {
            #[doc = r" Returns a pointer to the register block"]
            pub fn ptr() -> *const flexio1::RegisterBlock {
                1075511296 as *const _
            }
        }
        impl Deref for FLEXIO2 {
            type Target = flexio1::RegisterBlock;
            fn deref(&self) -> &flexio1::RegisterBlock {
                unsafe { &*FLEXIO2::ptr() }
            }
        }
        #[doc = "GPIO"]
        pub struct GPIO5 {
            _marker: PhantomData<*const ()>,
        }
        unsafe impl Send for GPIO5 {}
        impl GPIO5 {
            #[doc = r" Returns a pointer to the register block"]
            pub fn ptr() -> *const gpio1::RegisterBlock {
                1074528256 as *const _
            }
        }
        impl Deref for GPIO5 {
            type Target = gpio1::RegisterBlock;
            fn deref(&self) -> &gpio1::RegisterBlock {
                unsafe { &*GPIO5::ptr() }
            }
        }
        #[doc = "GPIO"]
        pub struct GPIO2 {
            _marker: PhantomData<*const ()>,
        }
        unsafe impl Send for GPIO2 {}
        impl GPIO2 {
            #[doc = r" Returns a pointer to the register block"]
            pub fn ptr() -> *const gpio1::RegisterBlock {
                1075560448 as *const _
            }
        }
        impl Deref for GPIO2 {
            type Target = gpio1::RegisterBlock;
            fn deref(&self) -> &gpio1::RegisterBlock {
                unsafe { &*GPIO2::ptr() }
            }
        }
        #[doc = "GPIO"]
        pub struct GPIO3 {
            _marker: PhantomData<*const ()>,
        }
        unsafe impl Send for GPIO3 {}
        impl GPIO3 {
            #[doc = r" Returns a pointer to the register block"]
            pub fn ptr() -> *const gpio1::RegisterBlock {
                1075576832 as *const _
            }
        }
        impl Deref for GPIO3 {
            type Target = gpio1::RegisterBlock;
            fn deref(&self) -> &gpio1::RegisterBlock {
                unsafe { &*GPIO3::ptr() }
            }
        }
        #[doc = "GPIO"]
        pub struct GPIO4 {
            _marker: PhantomData<*const ()>,
        }
        unsafe impl Send for GPIO4 {}
        impl GPIO4 {
            #[doc = r" Returns a pointer to the register block"]
            pub fn ptr() -> *const gpio1::RegisterBlock {
                1075593216 as *const _
            }
        }
        impl Deref for GPIO4 {
            type Target = gpio1::RegisterBlock;
            fn deref(&self) -> &gpio1::RegisterBlock {
                unsafe { &*GPIO4::ptr() }
            }
        }
        #[doc = "FLEXCAN"]
        pub struct CAN1 {
            _marker: PhantomData<*const ()>,
        }
        unsafe impl Send for CAN1 {}
        impl CAN1 {
            #[doc = r" Returns a pointer to the register block"]
            pub fn ptr() -> *const can1::RegisterBlock {
                1075642368 as *const _
            }
        }
        impl Deref for CAN1 {
            type Target = can1::RegisterBlock;
            fn deref(&self) -> &can1::RegisterBlock {
                unsafe { &*CAN1::ptr() }
            }
        }
        #[doc = "FLEXCAN"]
        pub mod can1;
        #[doc = "FLEXCAN"]
        pub struct CAN2 {
            _marker: PhantomData<*const ()>,
        }
        unsafe impl Send for CAN2 {}
        impl CAN2 {
            #[doc = r" Returns a pointer to the register block"]
            pub fn ptr() -> *const can1::RegisterBlock {
                1075658752 as *const _
            }
        }
        impl Deref for CAN2 {
            type Target = can1::RegisterBlock;
            fn deref(&self) -> &can1::RegisterBlock {
                unsafe { &*CAN2::ptr() }
            }
        }
        #[doc = "Quad Timer"]
        pub struct TMR1 {
            _marker: PhantomData<*const ()>,
        }
        unsafe impl Send for TMR1 {}
        impl TMR1 {
            #[doc = r" Returns a pointer to the register block"]
            pub fn ptr() -> *const tmr1::RegisterBlock {
                1075691520 as *const _
            }
        }
        impl Deref for TMR1 {
            type Target = tmr1::RegisterBlock;
            fn deref(&self) -> &tmr1::RegisterBlock {
                unsafe { &*TMR1::ptr() }
            }
        }
        #[doc = "Quad Timer"]
        pub mod tmr1;
        #[doc = "Quad Timer"]
        pub struct TMR2 {
            _marker: PhantomData<*const ()>,
        }
        unsafe impl Send for TMR2 {}
        impl TMR2 {
            #[doc = r" Returns a pointer to the register block"]
            pub fn ptr() -> *const tmr1::RegisterBlock {
                1075707904 as *const _
            }
        }
        impl Deref for TMR2 {
            type Target = tmr1::RegisterBlock;
            fn deref(&self) -> &tmr1::RegisterBlock {
                unsafe { &*TMR2::ptr() }
            }
        }
        #[doc = "Quad Timer"]
        pub struct TMR3 {
            _marker: PhantomData<*const ()>,
        }
        unsafe impl Send for TMR3 {}
        impl TMR3 {
            #[doc = r" Returns a pointer to the register block"]
            pub fn ptr() -> *const tmr1::RegisterBlock {
                1075724288 as *const _
            }
        }
        impl Deref for TMR3 {
            type Target = tmr1::RegisterBlock;
            fn deref(&self) -> &tmr1::RegisterBlock {
                unsafe { &*TMR3::ptr() }
            }
        }
        #[doc = "Quad Timer"]
        pub struct TMR4 {
            _marker: PhantomData<*const ()>,
        }
        unsafe impl Send for TMR4 {}
        impl TMR4 {
            #[doc = r" Returns a pointer to the register block"]
            pub fn ptr() -> *const tmr1::RegisterBlock {
                1075740672 as *const _
            }
        }
        impl Deref for TMR4 {
            type Target = tmr1::RegisterBlock;
            fn deref(&self) -> &tmr1::RegisterBlock {
                unsafe { &*TMR4::ptr() }
            }
        }
        #[doc = "GPT"]
        pub struct GPT1 {
            _marker: PhantomData<*const ()>,
        }
        unsafe impl Send for GPT1 {}
        impl GPT1 {
            #[doc = r" Returns a pointer to the register block"]
            pub fn ptr() -> *const gpt1::RegisterBlock {
                1075757056 as *const _
            }
        }
        impl Deref for GPT1 {
            type Target = gpt1::RegisterBlock;
            fn deref(&self) -> &gpt1::RegisterBlock {
                unsafe { &*GPT1::ptr() }
            }
        }
        #[doc = "GPT"]
        pub mod gpt1;
        #[doc = "GPT"]
        pub struct GPT2 {
            _marker: PhantomData<*const ()>,
        }
        unsafe impl Send for GPT2 {}
        impl GPT2 {
            #[doc = r" Returns a pointer to the register block"]
            pub fn ptr() -> *const gpt1::RegisterBlock {
                1075773440 as *const _
            }
        }
        impl Deref for GPT2 {
            type Target = gpt1::RegisterBlock;
            fn deref(&self) -> &gpt1::RegisterBlock {
                unsafe { &*GPT2::ptr() }
            }
        }
        #[doc = "no description available"]
        pub struct OCOTP {
            _marker: PhantomData<*const ()>,
        }
        unsafe impl Send for OCOTP {}
        impl OCOTP {
            #[doc = r" Returns a pointer to the register block"]
            pub fn ptr() -> *const ocotp::RegisterBlock {
                1075789824 as *const _
            }
        }
        impl Deref for OCOTP {
            type Target = ocotp::RegisterBlock;
            fn deref(&self) -> &ocotp::RegisterBlock {
                unsafe { &*OCOTP::ptr() }
            }
        }
        #[doc = "no description available"]
        pub mod ocotp;
        #[doc = "KPP Registers"]
        pub struct KPP {
            _marker: PhantomData<*const ()>,
        }
        unsafe impl Send for KPP {}
        impl KPP {
            #[doc = r" Returns a pointer to the register block"]
            pub fn ptr() -> *const kpp::RegisterBlock {
                1075822592 as *const _
            }
        }
        impl Deref for KPP {
            type Target = kpp::RegisterBlock;
            fn deref(&self) -> &kpp::RegisterBlock {
                unsafe { &*KPP::ptr() }
            }
        }
        #[doc = "KPP Registers"]
        pub mod kpp;
        #[doc = "FlexSPI"]
        pub struct FLEXSPI {
            _marker: PhantomData<*const ()>,
        }
        unsafe impl Send for FLEXSPI {}
        impl FLEXSPI {
            #[doc = r" Returns a pointer to the register block"]
            pub fn ptr() -> *const flexspi::RegisterBlock {
                1076527104 as *const _
            }
        }
        impl Deref for FLEXSPI {
            type Target = flexspi::RegisterBlock;
            fn deref(&self) -> &flexspi::RegisterBlock {
                unsafe { &*FLEXSPI::ptr() }
            }
        }
        #[doc = "FlexSPI"]
        pub mod flexspi;
        #[doc = "PXP v2.0 Register Reference Index"]
        pub struct PXP {
            _marker: PhantomData<*const ()>,
        }
        unsafe impl Send for PXP {}
        impl PXP {
            #[doc = r" Returns a pointer to the register block"]
            pub fn ptr() -> *const pxp::RegisterBlock {
                1076576256 as *const _
            }
        }
        impl Deref for PXP {
            type Target = pxp::RegisterBlock;
            fn deref(&self) -> &pxp::RegisterBlock {
                unsafe { &*PXP::ptr() }
            }
        }
        #[doc = "PXP v2.0 Register Reference Index"]
        pub mod pxp;
        #[doc = "LCDIF Register Reference Index"]
        pub struct LCDIF {
            _marker: PhantomData<*const ()>,
        }
        unsafe impl Send for LCDIF {}
        impl LCDIF {
            #[doc = r" Returns a pointer to the register block"]
            pub fn ptr() -> *const lcdif::RegisterBlock {
                1076592640 as *const _
            }
        }
        impl Deref for LCDIF {
            type Target = lcdif::RegisterBlock;
            fn deref(&self) -> &lcdif::RegisterBlock {
                unsafe { &*LCDIF::ptr() }
            }
        }
        #[doc = "LCDIF Register Reference Index"]
        pub mod lcdif;
        #[doc = "CSI"]
        pub struct CSI {
            _marker: PhantomData<*const ()>,
        }
        unsafe impl Send for CSI {}
        impl CSI {
            #[doc = r" Returns a pointer to the register block"]
            pub fn ptr() -> *const csi::RegisterBlock {
                1076609024 as *const _
            }
        }
        impl Deref for CSI {
            type Target = csi::RegisterBlock;
            fn deref(&self) -> &csi::RegisterBlock {
                unsafe { &*CSI::ptr() }
            }
        }
        #[doc = "CSI"]
        pub mod csi;
        #[doc = "uSDHC"]
        pub struct USDHC1 {
            _marker: PhantomData<*const ()>,
        }
        unsafe impl Send for USDHC1 {}
        impl USDHC1 {
            #[doc = r" Returns a pointer to the register block"]
            pub fn ptr() -> *const usdhc1::RegisterBlock {
                1076625408 as *const _
            }
        }
        impl Deref for USDHC1 {
            type Target = usdhc1::RegisterBlock;
            fn deref(&self) -> &usdhc1::RegisterBlock {
                unsafe { &*USDHC1::ptr() }
            }
        }
        #[doc = "uSDHC"]
        pub mod usdhc1;
        #[doc = "uSDHC"]
        pub struct USDHC2 {
            _marker: PhantomData<*const ()>,
        }
        unsafe impl Send for USDHC2 {}
        impl USDHC2 {
            #[doc = r" Returns a pointer to the register block"]
            pub fn ptr() -> *const usdhc1::RegisterBlock {
                1076641792 as *const _
            }
        }
        impl Deref for USDHC2 {
            type Target = usdhc1::RegisterBlock;
            fn deref(&self) -> &usdhc1::RegisterBlock {
                unsafe { &*USDHC2::ptr() }
            }
        }
        #[doc = "USB"]
        pub struct USB1 {
            _marker: PhantomData<*const ()>,
        }
        unsafe impl Send for USB1 {}
        impl USB1 {
            #[doc = r" Returns a pointer to the register block"]
            pub fn ptr() -> *const usb1::RegisterBlock {
                1076756480 as *const _
            }
        }
        impl Deref for USB1 {
            type Target = usb1::RegisterBlock;
            fn deref(&self) -> &usb1::RegisterBlock {
                unsafe { &*USB1::ptr() }
            }
        }
        #[doc = "USB"]
        pub mod usb1;
        #[doc = "USB"]
        pub struct USB2 {
            _marker: PhantomData<*const ()>,
        }
        unsafe impl Send for USB2 {}
        impl USB2 {
            #[doc = r" Returns a pointer to the register block"]
            pub fn ptr() -> *const usb1::RegisterBlock {
                1076756992 as *const _
            }
        }
        impl Deref for USB2 {
            type Target = usb1::RegisterBlock;
            fn deref(&self) -> &usb1::RegisterBlock {
                unsafe { &*USB2::ptr() }
            }
        }
        #[doc = "USB"]
        pub struct USBNC1 {
            _marker: PhantomData<*const ()>,
        }
        unsafe impl Send for USBNC1 {}
        impl USBNC1 {
            #[doc = r" Returns a pointer to the register block"]
            pub fn ptr() -> *const usbnc1::RegisterBlock {
                1076756480 as *const _
            }
        }
        impl Deref for USBNC1 {
            type Target = usbnc1::RegisterBlock;
            fn deref(&self) -> &usbnc1::RegisterBlock {
                unsafe { &*USBNC1::ptr() }
            }
        }
        #[doc = "USB"]
        pub mod usbnc1;
        #[doc = "USB"]
        pub struct USBNC2 {
            _marker: PhantomData<*const ()>,
        }
        unsafe impl Send for USBNC2 {}
        impl USBNC2 {
            #[doc = r" Returns a pointer to the register block"]
            pub fn ptr() -> *const usbnc1::RegisterBlock {
                1076756484 as *const _
            }
        }
        impl Deref for USBNC2 {
            type Target = usbnc1::RegisterBlock;
            fn deref(&self) -> &usbnc1::RegisterBlock {
                unsafe { &*USBNC2::ptr() }
            }
        }
        #[doc = "SEMC"]
        pub struct SEMC {
            _marker: PhantomData<*const ()>,
        }
        unsafe impl Send for SEMC {}
        impl SEMC {
            #[doc = r" Returns a pointer to the register block"]
            pub fn ptr() -> *const semc::RegisterBlock {
                1076822016 as *const _
            }
        }
        impl Deref for SEMC {
            type Target = semc::RegisterBlock;
            fn deref(&self) -> &semc::RegisterBlock {
                unsafe { &*SEMC::ptr() }
            }
        }
        #[doc = "SEMC"]
        pub mod semc;
        #[doc = "DCP register reference index"]
        pub struct DCP {
            _marker: PhantomData<*const ()>,
        }
        unsafe impl Send for DCP {}
        impl DCP {
            #[doc = r" Returns a pointer to the register block"]
            pub fn ptr() -> *const dcp::RegisterBlock {
                1076871168 as *const _
            }
        }
        impl Deref for DCP {
            type Target = dcp::RegisterBlock;
            fn deref(&self) -> &dcp::RegisterBlock {
                unsafe { &*DCP::ptr() }
            }
        }
        #[doc = "DCP register reference index"]
        pub mod dcp;
        #[doc = "SPDIF"]
        pub struct SPDIF {
            _marker: PhantomData<*const ()>,
        }
        unsafe impl Send for SPDIF {}
        impl SPDIF {
            #[doc = r" Returns a pointer to the register block"]
            pub fn ptr() -> *const spdif::RegisterBlock {
                1077411840 as *const _
            }
        }
        impl Deref for SPDIF {
            type Target = spdif::RegisterBlock;
            fn deref(&self) -> &spdif::RegisterBlock {
                unsafe { &*SPDIF::ptr() }
            }
        }
        #[doc = "SPDIF"]
        pub mod spdif;
        #[doc = "I2S"]
        pub struct SAI1 {
            _marker: PhantomData<*const ()>,
        }
        unsafe impl Send for SAI1 {}
        impl SAI1 {
            #[doc = r" Returns a pointer to the register block"]
            pub fn ptr() -> *const sai1::RegisterBlock {
                1077428224 as *const _
            }
        }
        impl Deref for SAI1 {
            type Target = sai1::RegisterBlock;
            fn deref(&self) -> &sai1::RegisterBlock {
                unsafe { &*SAI1::ptr() }
            }
        }
        #[doc = "I2S"]
        pub mod sai1;
        #[doc = "I2S"]
        pub struct SAI2 {
            _marker: PhantomData<*const ()>,
        }
        unsafe impl Send for SAI2 {}
        impl SAI2 {
            #[doc = r" Returns a pointer to the register block"]
            pub fn ptr() -> *const sai1::RegisterBlock {
                1077444608 as *const _
            }
        }
        impl Deref for SAI2 {
            type Target = sai1::RegisterBlock;
            fn deref(&self) -> &sai1::RegisterBlock {
                unsafe { &*SAI2::ptr() }
            }
        }
        #[doc = "I2S"]
        pub struct SAI3 {
            _marker: PhantomData<*const ()>,
        }
        unsafe impl Send for SAI3 {}
        impl SAI3 {
            #[doc = r" Returns a pointer to the register block"]
            pub fn ptr() -> *const sai1::RegisterBlock {
                1077460992 as *const _
            }
        }
        impl Deref for SAI3 {
            type Target = sai1::RegisterBlock;
            fn deref(&self) -> &sai1::RegisterBlock {
                unsafe { &*SAI3::ptr() }
            }
        }
        #[doc = "LPSPI"]
        pub struct LPSPI1 {
            _marker: PhantomData<*const ()>,
        }
        unsafe impl Send for LPSPI1 {}
        impl LPSPI1 {
            #[doc = r" Returns a pointer to the register block"]
            pub fn ptr() -> *const lpspi1::RegisterBlock {
                1077493760 as *const _
            }
        }
        impl Deref for LPSPI1 {
            type Target = lpspi1::RegisterBlock;
            fn deref(&self) -> &lpspi1::RegisterBlock {
                unsafe { &*LPSPI1::ptr() }
            }
        }
        #[doc = "LPSPI"]
        pub mod lpspi1;
        #[doc = "LPSPI"]
        pub struct LPSPI2 {
            _marker: PhantomData<*const ()>,
        }
        unsafe impl Send for LPSPI2 {}
        impl LPSPI2 {
            #[doc = r" Returns a pointer to the register block"]
            pub fn ptr() -> *const lpspi1::RegisterBlock {
                1077510144 as *const _
            }
        }
        impl Deref for LPSPI2 {
            type Target = lpspi1::RegisterBlock;
            fn deref(&self) -> &lpspi1::RegisterBlock {
                unsafe { &*LPSPI2::ptr() }
            }
        }
        #[doc = "LPSPI"]
        pub struct LPSPI3 {
            _marker: PhantomData<*const ()>,
        }
        unsafe impl Send for LPSPI3 {}
        impl LPSPI3 {
            #[doc = r" Returns a pointer to the register block"]
            pub fn ptr() -> *const lpspi1::RegisterBlock {
                1077526528 as *const _
            }
        }
        impl Deref for LPSPI3 {
            type Target = lpspi1::RegisterBlock;
            fn deref(&self) -> &lpspi1::RegisterBlock {
                unsafe { &*LPSPI3::ptr() }
            }
        }
        #[doc = "LPSPI"]
        pub struct LPSPI4 {
            _marker: PhantomData<*const ()>,
        }
        unsafe impl Send for LPSPI4 {}
        impl LPSPI4 {
            #[doc = r" Returns a pointer to the register block"]
            pub fn ptr() -> *const lpspi1::RegisterBlock {
                1077542912 as *const _
            }
        }
        impl Deref for LPSPI4 {
            type Target = lpspi1::RegisterBlock;
            fn deref(&self) -> &lpspi1::RegisterBlock {
                unsafe { &*LPSPI4::ptr() }
            }
        }
        #[doc = "ADC_ETC"]
        pub struct ADC_ETC {
            _marker: PhantomData<*const ()>,
        }
        unsafe impl Send for ADC_ETC {}
        impl ADC_ETC {
            #[doc = r" Returns a pointer to the register block"]
            pub fn ptr() -> *const adc_etc::RegisterBlock {
                1077608448 as *const _
            }
        }
        impl Deref for ADC_ETC {
            type Target = adc_etc::RegisterBlock;
            fn deref(&self) -> &adc_etc::RegisterBlock {
                unsafe { &*ADC_ETC::ptr() }
            }
        }
        #[doc = "ADC_ETC"]
        pub mod adc_etc;
        #[doc = "AND/OR/INVERT module"]
        pub struct AOI1 {
            _marker: PhantomData<*const ()>,
        }
        unsafe impl Send for AOI1 {}
        impl AOI1 {
            #[doc = r" Returns a pointer to the register block"]
            pub fn ptr() -> *const aoi1::RegisterBlock {
                1077624832 as *const _
            }
        }
        impl Deref for AOI1 {
            type Target = aoi1::RegisterBlock;
            fn deref(&self) -> &aoi1::RegisterBlock {
                unsafe { &*AOI1::ptr() }
            }
        }
        #[doc = "AND/OR/INVERT module"]
        pub mod aoi1;
        #[doc = "AND/OR/INVERT module"]
        pub struct AOI2 {
            _marker: PhantomData<*const ()>,
        }
        unsafe impl Send for AOI2 {}
        impl AOI2 {
            #[doc = r" Returns a pointer to the register block"]
            pub fn ptr() -> *const aoi1::RegisterBlock {
                1077641216 as *const _
            }
        }
        impl Deref for AOI2 {
            type Target = aoi1::RegisterBlock;
            fn deref(&self) -> &aoi1::RegisterBlock {
                unsafe { &*AOI2::ptr() }
            }
        }
        #[doc = "Crossbar Switch"]
        pub struct XBARA1 {
            _marker: PhantomData<*const ()>,
        }
        unsafe impl Send for XBARA1 {}
        impl XBARA1 {
            #[doc = r" Returns a pointer to the register block"]
            pub fn ptr() -> *const xbara1::RegisterBlock {
                1077657600 as *const _
            }
        }
        impl Deref for XBARA1 {
            type Target = xbara1::RegisterBlock;
            fn deref(&self) -> &xbara1::RegisterBlock {
                unsafe { &*XBARA1::ptr() }
            }
        }
        #[doc = "Crossbar Switch"]
        pub mod xbara1;
        #[doc = "Crossbar Switch"]
        pub struct XBARB2 {
            _marker: PhantomData<*const ()>,
        }
        unsafe impl Send for XBARB2 {}
        impl XBARB2 {
            #[doc = r" Returns a pointer to the register block"]
            pub fn ptr() -> *const xbarb2::RegisterBlock {
                1077673984 as *const _
            }
        }
        impl Deref for XBARB2 {
            type Target = xbarb2::RegisterBlock;
            fn deref(&self) -> &xbarb2::RegisterBlock {
                unsafe { &*XBARB2::ptr() }
            }
        }
        #[doc = "Crossbar Switch"]
        pub mod xbarb2;
        #[doc = "Crossbar Switch"]
        pub struct XBARB3 {
            _marker: PhantomData<*const ()>,
        }
        unsafe impl Send for XBARB3 {}
        impl XBARB3 {
            #[doc = r" Returns a pointer to the register block"]
            pub fn ptr() -> *const xbarb2::RegisterBlock {
                1077690368 as *const _
            }
        }
        impl Deref for XBARB3 {
            type Target = xbarb2::RegisterBlock;
            fn deref(&self) -> &xbarb2::RegisterBlock {
                unsafe { &*XBARB3::ptr() }
            }
        }
        #[doc = "Quadrature Decoder"]
        pub struct ENC1 {
            _marker: PhantomData<*const ()>,
        }
        unsafe impl Send for ENC1 {}
        impl ENC1 {
            #[doc = r" Returns a pointer to the register block"]
            pub fn ptr() -> *const enc1::RegisterBlock {
                1077706752 as *const _
            }
        }
        impl Deref for ENC1 {
            type Target = enc1::RegisterBlock;
            fn deref(&self) -> &enc1::RegisterBlock {
                unsafe { &*ENC1::ptr() }
            }
        }
        #[doc = "Quadrature Decoder"]
        pub mod enc1;
        #[doc = "Quadrature Decoder"]
        pub struct ENC2 {
            _marker: PhantomData<*const ()>,
        }
        unsafe impl Send for ENC2 {}
        impl ENC2 {
            #[doc = r" Returns a pointer to the register block"]
            pub fn ptr() -> *const enc1::RegisterBlock {
                1077723136 as *const _
            }
        }
        impl Deref for ENC2 {
            type Target = enc1::RegisterBlock;
            fn deref(&self) -> &enc1::RegisterBlock {
                unsafe { &*ENC2::ptr() }
            }
        }
        #[doc = "Quadrature Decoder"]
        pub struct ENC3 {
            _marker: PhantomData<*const ()>,
        }
        unsafe impl Send for ENC3 {}
        impl ENC3 {
            #[doc = r" Returns a pointer to the register block"]
            pub fn ptr() -> *const enc1::RegisterBlock {
                1077739520 as *const _
            }
        }
        impl Deref for ENC3 {
            type Target = enc1::RegisterBlock;
            fn deref(&self) -> &enc1::RegisterBlock {
                unsafe { &*ENC3::ptr() }
            }
        }
        #[doc = "Quadrature Decoder"]
        pub struct ENC4 {
            _marker: PhantomData<*const ()>,
        }
        unsafe impl Send for ENC4 {}
        impl ENC4 {
            #[doc = r" Returns a pointer to the register block"]
            pub fn ptr() -> *const enc1::RegisterBlock {
                1077755904 as *const _
            }
        }
        impl Deref for ENC4 {
            type Target = enc1::RegisterBlock;
            fn deref(&self) -> &enc1::RegisterBlock {
                unsafe { &*ENC4::ptr() }
            }
        }
        #[doc = "PWM"]
        pub struct PWM1 {
            _marker: PhantomData<*const ()>,
        }
        unsafe impl Send for PWM1 {}
        impl PWM1 {
            #[doc = r" Returns a pointer to the register block"]
            pub fn ptr() -> *const pwm1::RegisterBlock {
                1077788672 as *const _
            }
        }
        impl Deref for PWM1 {
            type Target = pwm1::RegisterBlock;
            fn deref(&self) -> &pwm1::RegisterBlock {
                unsafe { &*PWM1::ptr() }
            }
        }
        #[doc = "PWM"]
        pub mod pwm1;
        #[doc = "PWM"]
        pub struct PWM2 {
            _marker: PhantomData<*const ()>,
        }
        unsafe impl Send for PWM2 {}
        impl PWM2 {
            #[doc = r" Returns a pointer to the register block"]
            pub fn ptr() -> *const pwm1::RegisterBlock {
                1077805056 as *const _
            }
        }
        impl Deref for PWM2 {
            type Target = pwm1::RegisterBlock;
            fn deref(&self) -> &pwm1::RegisterBlock {
                unsafe { &*PWM2::ptr() }
            }
        }
        #[doc = "PWM"]
        pub struct PWM3 {
            _marker: PhantomData<*const ()>,
        }
        unsafe impl Send for PWM3 {}
        impl PWM3 {
            #[doc = r" Returns a pointer to the register block"]
            pub fn ptr() -> *const pwm1::RegisterBlock {
                1077821440 as *const _
            }
        }
        impl Deref for PWM3 {
            type Target = pwm1::RegisterBlock;
            fn deref(&self) -> &pwm1::RegisterBlock {
                unsafe { &*PWM3::ptr() }
            }
        }
        #[doc = "PWM"]
        pub struct PWM4 {
            _marker: PhantomData<*const ()>,
        }
        unsafe impl Send for PWM4 {}
        impl PWM4 {
            #[doc = r" Returns a pointer to the register block"]
            pub fn ptr() -> *const pwm1::RegisterBlock {
                1077837824 as *const _
            }
        }
        impl Deref for PWM4 {
            type Target = pwm1::RegisterBlock;
            fn deref(&self) -> &pwm1::RegisterBlock {
                unsafe { &*PWM4::ptr() }
            }
        }
        #[doc = "Bus Encryption Engine"]
        pub struct BEE {
            _marker: PhantomData<*const ()>,
        }
        unsafe impl Send for BEE {}
        impl BEE {
            #[doc = r" Returns a pointer to the register block"]
            pub fn ptr() -> *const bee::RegisterBlock {
                1077854208 as *const _
            }
        }
        impl Deref for BEE {
            type Target = bee::RegisterBlock;
            fn deref(&self) -> &bee::RegisterBlock {
                unsafe { &*BEE::ptr() }
            }
        }
        #[doc = "Bus Encryption Engine"]
        pub mod bee;
        #[doc = "LPI2C"]
        pub struct LPI2C1 {
            _marker: PhantomData<*const ()>,
        }
        unsafe impl Send for LPI2C1 {}
        impl LPI2C1 {
            #[doc = r" Returns a pointer to the register block"]
            pub fn ptr() -> *const lpi2c1::RegisterBlock {
                1077870592 as *const _
            }
        }
        impl Deref for LPI2C1 {
            type Target = lpi2c1::RegisterBlock;
            fn deref(&self) -> &lpi2c1::RegisterBlock {
                unsafe { &*LPI2C1::ptr() }
            }
        }
        #[doc = "LPI2C"]
        pub mod lpi2c1;
        #[doc = "LPI2C"]
        pub struct LPI2C2 {
            _marker: PhantomData<*const ()>,
        }
        unsafe impl Send for LPI2C2 {}
        impl LPI2C2 {
            #[doc = r" Returns a pointer to the register block"]
            pub fn ptr() -> *const lpi2c1::RegisterBlock {
                1077886976 as *const _
            }
        }
        impl Deref for LPI2C2 {
            type Target = lpi2c1::RegisterBlock;
            fn deref(&self) -> &lpi2c1::RegisterBlock {
                unsafe { &*LPI2C2::ptr() }
            }
        }
        #[doc = "LPI2C"]
        pub struct LPI2C3 {
            _marker: PhantomData<*const ()>,
        }
        unsafe impl Send for LPI2C3 {}
        impl LPI2C3 {
            #[doc = r" Returns a pointer to the register block"]
            pub fn ptr() -> *const lpi2c1::RegisterBlock {
                1077903360 as *const _
            }
        }
        impl Deref for LPI2C3 {
            type Target = lpi2c1::RegisterBlock;
            fn deref(&self) -> &lpi2c1::RegisterBlock {
                unsafe { &*LPI2C3::ptr() }
            }
        }
        #[doc = "LPI2C"]
        pub struct LPI2C4 {
            _marker: PhantomData<*const ()>,
        }
        unsafe impl Send for LPI2C4 {}
        impl LPI2C4 {
            #[doc = r" Returns a pointer to the register block"]
            pub fn ptr() -> *const lpi2c1::RegisterBlock {
                1077919744 as *const _
            }
        }
        impl Deref for LPI2C4 {
            type Target = lpi2c1::RegisterBlock;
            fn deref(&self) -> &lpi2c1::RegisterBlock {
                unsafe { &*LPI2C4::ptr() }
            }
        }
        #[doc = "System Control Block"]
        pub struct SYSTEMCONTROL {
            _marker: PhantomData<*const ()>,
        }
        unsafe impl Send for SYSTEMCONTROL {}
        impl SYSTEMCONTROL {
            #[doc = r" Returns a pointer to the register block"]
            pub fn ptr() -> *const system_control::RegisterBlock {
                3758153728 as *const _
            }
        }
        impl Deref for SYSTEMCONTROL {
            type Target = system_control::RegisterBlock;
            fn deref(&self) -> &system_control::RegisterBlock {
                unsafe { &*SYSTEMCONTROL::ptr() }
            }
        }
        #[doc = "System Control Block"]
        pub mod system_control;
    }
}

#[allow(private_no_mangle_statics)]
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r" All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[cfg(any(feature = "aipstz1", feature = "everything"))]
    #[doc = "AIPSTZ1"]
    pub AIPSTZ1: AIPSTZ1,
    #[cfg(any(feature = "notimplemented", feature = "everything"))]
    #[doc = "AIPSTZ2"]
    pub AIPSTZ2: AIPSTZ2,
    #[cfg(any(feature = "notimplemented", feature = "everything"))]
    #[doc = "AIPSTZ3"]
    pub AIPSTZ3: AIPSTZ3,
    #[cfg(any(feature = "notimplemented", feature = "everything"))]
    #[doc = "AIPSTZ4"]
    pub AIPSTZ4: AIPSTZ4,
    #[cfg(any(feature = "dcdc", feature = "everything"))]
    #[doc = "DCDC"]
    pub DCDC: DCDC,
    #[cfg(any(feature = "notimplemented", feature = "everything"))]
    #[doc = "PIT"]
    pub PIT: PIT,
    #[cfg(any(feature = "notimplemented", feature = "everything"))]
    #[doc = "CMP1"]
    pub CMP1: CMP1,
    #[cfg(any(feature = "notimplemented", feature = "everything"))]
    #[doc = "CMP2"]
    pub CMP2: CMP2,
    #[cfg(any(feature = "notimplemented", feature = "everything"))]
    #[doc = "CMP3"]
    pub CMP3: CMP3,
    #[cfg(any(feature = "notimplemented", feature = "everything"))]
    #[doc = "CMP4"]
    pub CMP4: CMP4,
    #[cfg(any(feature = "notimplemented", feature = "everything"))]
    #[doc = "IOMUXC_SNVS_GPR"]
    pub IOMUXC_SNVS_GPR: IOMUXC_SNVS_GPR,
    #[cfg(any(feature = "notimplemented", feature = "everything"))]
    #[doc = "IOMUXC_SNVS"]
    pub IOMUXC_SNVS: IOMUXC_SNVS,
    #[cfg(any(feature = "iomuxc_gpr", feature = "everything"))]
    #[doc = "IOMUXC_GPR"]
    pub IOMUXC_GPR: IOMUXC_GPR,
    #[cfg(any(feature = "notimplemented", feature = "everything"))]
    #[doc = "FLEXRAM"]
    pub FLEXRAM: FLEXRAM,
    #[cfg(any(feature = "notimplemented", feature = "everything"))]
    #[doc = "EWM"]
    pub EWM: EWM,
    #[cfg(any(feature = "wdog", feature = "everything"))]
    #[doc = "WDOG1"]
    pub WDOG1: WDOG1,
    #[cfg(any(feature = "wdog", feature = "everything"))]
    #[doc = "WDOG2"]
    pub WDOG2: WDOG2,
    #[cfg(any(feature = "wdog", feature = "everything"))]
    #[doc = "RTWDOG"]
    pub RTWDOG: RTWDOG,
    #[cfg(any(feature = "notimplemented", feature = "everything"))]
    #[doc = "ADC1"]
    pub ADC1: ADC1,
    #[cfg(any(feature = "notimplemented", feature = "everything"))]
    #[doc = "ADC2"]
    pub ADC2: ADC2,
    #[cfg(any(feature = "notimplemented", feature = "everything"))]
    #[doc = "TRNG"]
    pub TRNG: TRNG,
    #[cfg(any(feature = "notimplemented", feature = "everything"))]
    #[doc = "SNVS"]
    pub SNVS: SNVS,
    #[cfg(any(feature = "ccm_analog", feature = "everything"))]
    #[doc = "CCM_ANALOG"]
    pub CCM_ANALOG: CCM_ANALOG,
    #[cfg(any(feature = "notimplemented", feature = "everything"))]
    #[doc = "PMU"]
    pub PMU: PMU,
    #[cfg(any(feature = "notimplemented", feature = "everything"))]
    #[doc = "TEMPMON"]
    pub TEMPMON: TEMPMON,
    #[cfg(any(feature = "notimplemented", feature = "everything"))]
    #[doc = "USB_ANALOG"]
    pub USB_ANALOG: USB_ANALOG,
    #[cfg(any(feature = "xtalosc24m", feature = "everything"))]
    #[doc = "XTALOSC24M"]
    pub XTALOSC24M: XTALOSC24M,
    #[cfg(any(feature = "notimplemented", feature = "everything"))]
    #[doc = "USBPHY1"]
    pub USBPHY1: USBPHY1,
    #[cfg(any(feature = "notimplemented", feature = "everything"))]
    #[doc = "USBPHY2"]
    pub USBPHY2: USBPHY2,
    #[cfg(any(feature = "notimplemented", feature = "everything"))]
    #[doc = "CSU"]
    pub CSU: CSU,
    #[cfg(any(feature = "notimplemented", feature = "everything"))]
    #[doc = "TSC"]
    pub TSC: TSC,
    #[cfg(any(feature = "notimplemented", feature = "everything"))]
    #[doc = "DMA0"]
    pub DMA0: DMA0,
    #[cfg(any(feature = "notimplemented", feature = "everything"))]
    #[doc = "DMAMUX"]
    pub DMAMUX: DMAMUX,
    #[cfg(any(feature = "notimplemented", feature = "everything"))]
    #[doc = "GPC"]
    pub GPC: GPC,
    #[cfg(any(feature = "notimplemented", feature = "everything"))]
    #[doc = "PGC"]
    pub PGC: PGC,
    #[cfg(any(feature = "notimplemented", feature = "everything"))]
    #[doc = "SRC"]
    pub SRC: SRC,
    #[cfg(any(feature = "ccm", feature = "everything"))]
    #[doc = "CCM"]
    pub CCM: CCM,
    #[cfg(any(feature = "notimplemented", feature = "everything"))]
    #[doc = "ROMC"]
    pub ROMC: ROMC,
    #[cfg(any(feature = "notimplemented", feature = "everything"))]
    #[doc = "LPUART1"]
    pub LPUART1: LPUART1,
    #[cfg(any(feature = "notimplemented", feature = "everything"))]
    #[doc = "LPUART2"]
    pub LPUART2: LPUART2,
    #[cfg(any(feature = "notimplemented", feature = "everything"))]
    #[doc = "LPUART3"]
    pub LPUART3: LPUART3,
    #[cfg(any(feature = "notimplemented", feature = "everything"))]
    #[doc = "LPUART4"]
    pub LPUART4: LPUART4,
    #[cfg(any(feature = "notimplemented", feature = "everything"))]
    #[doc = "LPUART5"]
    pub LPUART5: LPUART5,
    #[cfg(any(feature = "notimplemented", feature = "everything"))]
    #[doc = "LPUART6"]
    pub LPUART6: LPUART6,
    #[cfg(any(feature = "notimplemented", feature = "everything"))]
    #[doc = "LPUART7"]
    pub LPUART7: LPUART7,
    #[cfg(any(feature = "notimplemented", feature = "everything"))]
    #[doc = "LPUART8"]
    pub LPUART8: LPUART8,
    #[cfg(any(feature = "notimplemented", feature = "everything"))]
    #[doc = "FLEXIO1"]
    pub FLEXIO1: FLEXIO1,
    #[cfg(any(feature = "notimplemented", feature = "everything"))]
    #[doc = "FLEXIO2"]
    pub FLEXIO2: FLEXIO2,
    #[cfg(any(feature = "gpio1", feature = "everything"))]
    #[doc = "GPIO1"]
    pub GPIO1: GPIO1,
    #[cfg(any(feature = "notimplemented", feature = "everything"))]
    #[doc = "GPIO5"]
    pub GPIO5: GPIO5,
    #[cfg(any(feature = "notimplemented", feature = "everything"))]
    #[doc = "GPIO2"]
    pub GPIO2: GPIO2,
    #[cfg(any(feature = "notimplemented", feature = "everything"))]
    #[doc = "GPIO3"]
    pub GPIO3: GPIO3,
    #[cfg(any(feature = "notimplemented", feature = "everything"))]
    #[doc = "GPIO4"]
    pub GPIO4: GPIO4,
    #[cfg(any(feature = "notimplemented", feature = "everything"))]
    #[doc = "CAN1"]
    pub CAN1: CAN1,
    #[cfg(any(feature = "notimplemented", feature = "everything"))]
    #[doc = "CAN2"]
    pub CAN2: CAN2,
    #[cfg(any(feature = "notimplemented", feature = "everything"))]
    #[doc = "TMR1"]
    pub TMR1: TMR1,
    #[cfg(any(feature = "notimplemented", feature = "everything"))]
    #[doc = "TMR2"]
    pub TMR2: TMR2,
    #[cfg(any(feature = "notimplemented", feature = "everything"))]
    #[doc = "TMR3"]
    pub TMR3: TMR3,
    #[cfg(any(feature = "notimplemented", feature = "everything"))]
    #[doc = "TMR4"]
    pub TMR4: TMR4,
    #[cfg(any(feature = "notimplemented", feature = "everything"))]
    #[doc = "GPT1"]
    pub GPT1: GPT1,
    #[cfg(any(feature = "notimplemented", feature = "everything"))]
    #[doc = "GPT2"]
    pub GPT2: GPT2,
    #[cfg(any(feature = "notimplemented", feature = "everything"))]
    #[doc = "OCOTP"]
    pub OCOTP: OCOTP,
    #[cfg(any(feature = "iomuxc", feature = "everything"))]
    #[doc = "IOMUXC"]
    pub IOMUXC: IOMUXC,
    #[cfg(any(feature = "notimplemented", feature = "everything"))]
    #[doc = "KPP"]
    pub KPP: KPP,
    #[cfg(any(feature = "notimplemented", feature = "everything"))]
    #[doc = "FLEXSPI"]
    pub FLEXSPI: FLEXSPI,
    #[cfg(any(feature = "notimplemented", feature = "everything"))]
    #[doc = "PXP"]
    pub PXP: PXP,
    #[cfg(any(feature = "notimplemented", feature = "everything"))]
    #[doc = "LCDIF"]
    pub LCDIF: LCDIF,
    #[cfg(any(feature = "notimplemented", feature = "everything"))]
    #[doc = "CSI"]
    pub CSI: CSI,
    #[cfg(any(feature = "notimplemented", feature = "everything"))]
    #[doc = "USDHC1"]
    pub USDHC1: USDHC1,
    #[cfg(any(feature = "notimplemented", feature = "everything"))]
    #[doc = "USDHC2"]
    pub USDHC2: USDHC2,
    #[cfg(any(feature = "enet", feature = "everything"))]
    #[doc = "ENET"]
    pub ENET: ENET,
    #[cfg(any(feature = "notimplemented", feature = "everything"))]
    #[doc = "USB1"]
    pub USB1: USB1,
    #[cfg(any(feature = "notimplemented", feature = "everything"))]
    #[doc = "USB2"]
    pub USB2: USB2,
    #[cfg(any(feature = "notimplemented", feature = "everything"))]
    #[doc = "USBNC1"]
    pub USBNC1: USBNC1,
    #[cfg(any(feature = "notimplemented", feature = "everything"))]
    #[doc = "USBNC2"]
    pub USBNC2: USBNC2,
    #[cfg(any(feature = "notimplemented", feature = "everything"))]
    #[doc = "SEMC"]
    pub SEMC: SEMC,
    #[cfg(any(feature = "notimplemented", feature = "everything"))]
    #[doc = "DCP"]
    pub DCP: DCP,
    #[cfg(any(feature = "notimplemented", feature = "everything"))]
    #[doc = "SPDIF"]
    pub SPDIF: SPDIF,
    #[cfg(any(feature = "notimplemented", feature = "everything"))]
    #[doc = "SAI1"]
    pub SAI1: SAI1,
    #[cfg(any(feature = "notimplemented", feature = "everything"))]
    #[doc = "SAI2"]
    pub SAI2: SAI2,
    #[cfg(any(feature = "notimplemented", feature = "everything"))]
    #[doc = "SAI3"]
    pub SAI3: SAI3,
    #[cfg(any(feature = "notimplemented", feature = "everything"))]
    #[doc = "LPSPI1"]
    pub LPSPI1: LPSPI1,
    #[cfg(any(feature = "notimplemented", feature = "everything"))]
    #[doc = "LPSPI2"]
    pub LPSPI2: LPSPI2,
    #[cfg(any(feature = "notimplemented", feature = "everything"))]
    #[doc = "LPSPI3"]
    pub LPSPI3: LPSPI3,
    #[cfg(any(feature = "notimplemented", feature = "everything"))]
    #[doc = "LPSPI4"]
    pub LPSPI4: LPSPI4,
    #[cfg(any(feature = "notimplemented", feature = "everything"))]
    #[doc = "ADC_ETC"]
    pub ADC_ETC: ADC_ETC,
    #[cfg(any(feature = "notimplemented", feature = "everything"))]
    #[doc = "AOI1"]
    pub AOI1: AOI1,
    #[cfg(any(feature = "notimplemented", feature = "everything"))]
    #[doc = "AOI2"]
    pub AOI2: AOI2,
    #[cfg(any(feature = "notimplemented", feature = "everything"))]
    #[doc = "XBARA1"]
    pub XBARA1: XBARA1,
    #[cfg(any(feature = "notimplemented", feature = "everything"))]
    #[doc = "XBARB2"]
    pub XBARB2: XBARB2,
    #[cfg(any(feature = "notimplemented", feature = "everything"))]
    #[doc = "XBARB3"]
    pub XBARB3: XBARB3,
    #[cfg(any(feature = "notimplemented", feature = "everything"))]
    #[doc = "ENC1"]
    pub ENC1: ENC1,
    #[cfg(any(feature = "notimplemented", feature = "everything"))]
    #[doc = "ENC2"]
    pub ENC2: ENC2,
    #[cfg(any(feature = "notimplemented", feature = "everything"))]
    #[doc = "ENC3"]
    pub ENC3: ENC3,
    #[cfg(any(feature = "notimplemented", feature = "everything"))]
    #[doc = "ENC4"]
    pub ENC4: ENC4,
    #[cfg(any(feature = "notimplemented", feature = "everything"))]
    #[doc = "PWM1"]
    pub PWM1: PWM1,
    #[cfg(any(feature = "notimplemented", feature = "everything"))]
    #[doc = "PWM2"]
    pub PWM2: PWM2,
    #[cfg(any(feature = "notimplemented", feature = "everything"))]
    #[doc = "PWM3"]
    pub PWM3: PWM3,
    #[cfg(any(feature = "notimplemented", feature = "everything"))]
    #[doc = "PWM4"]
    pub PWM4: PWM4,
    #[cfg(any(feature = "notimplemented", feature = "everything"))]
    #[doc = "BEE"]
    pub BEE: BEE,
    #[cfg(any(feature = "notimplemented", feature = "everything"))]
    #[doc = "LPI2C1"]
    pub LPI2C1: LPI2C1,
    #[cfg(any(feature = "notimplemented", feature = "everything"))]
    #[doc = "LPI2C2"]
    pub LPI2C2: LPI2C2,
    #[cfg(any(feature = "notimplemented", feature = "everything"))]
    #[doc = "LPI2C3"]
    pub LPI2C3: LPI2C3,
    #[cfg(any(feature = "notimplemented", feature = "everything"))]
    #[doc = "LPI2C4"]
    pub LPI2C4: LPI2C4,
    #[cfg(any(feature = "notimplemented", feature = "everything"))]
    #[doc = "SYSTEMCONTROL"]
    pub SYSTEMCONTROL: SYSTEMCONTROL,
}
impl Peripherals {
    #[doc = r" Returns all the peripherals *once*"]
    #[inline]
    pub fn take() -> Option<Self> {
        cortex_m::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { Peripherals::steal() })
            }
        })
    }
    #[doc = r" Unchecked version of `Peripherals::take`"]
    pub unsafe fn steal() -> Self {
        debug_assert!(!DEVICE_PERIPHERALS);
        DEVICE_PERIPHERALS = true;
        Peripherals {
            #[cfg(any(feature = "aipstz1", feature = "everything"))]
            AIPSTZ1: AIPSTZ1 {
                _marker: PhantomData,
            },
            #[cfg(any(feature = "notimplemented", feature = "everything"))]
            AIPSTZ2: AIPSTZ2 {
                _marker: PhantomData,
            },
            #[cfg(any(feature = "notimplemented", feature = "everything"))]
            AIPSTZ3: AIPSTZ3 {
                _marker: PhantomData,
            },
            #[cfg(any(feature = "notimplemented", feature = "everything"))]
            AIPSTZ4: AIPSTZ4 {
                _marker: PhantomData,
            },
            #[cfg(any(feature = "dcdc", feature = "everything"))]
            DCDC: DCDC {
                _marker: PhantomData,
            },
            #[cfg(any(feature = "notimplemented", feature = "everything"))]
            PIT: PIT {
                _marker: PhantomData,
            },
            #[cfg(any(feature = "notimplemented", feature = "everything"))]
            CMP1: CMP1 {
                _marker: PhantomData,
            },
            #[cfg(any(feature = "notimplemented", feature = "everything"))]
            CMP2: CMP2 {
                _marker: PhantomData,
            },
            #[cfg(any(feature = "notimplemented", feature = "everything"))]
            CMP3: CMP3 {
                _marker: PhantomData,
            },
            #[cfg(any(feature = "notimplemented", feature = "everything"))]
            CMP4: CMP4 {
                _marker: PhantomData,
            },
            #[cfg(any(feature = "notimplemented", feature = "everything"))]
            IOMUXC_SNVS_GPR: IOMUXC_SNVS_GPR {
                _marker: PhantomData,
            },
            #[cfg(any(feature = "notimplemented", feature = "everything"))]
            IOMUXC_SNVS: IOMUXC_SNVS {
                _marker: PhantomData,
            },
            #[cfg(any(feature = "iomuxc_gpr", feature = "everything"))]
            IOMUXC_GPR: IOMUXC_GPR {
                _marker: PhantomData,
            },
            #[cfg(any(feature = "notimplemented", feature = "everything"))]
            FLEXRAM: FLEXRAM {
                _marker: PhantomData,
            },
            #[cfg(any(feature = "notimplemented", feature = "everything"))]
            EWM: EWM {
                _marker: PhantomData,
            },
            #[cfg(any(feature = "wdog", feature = "everything"))]
            WDOG1: WDOG1 {
                _marker: PhantomData,
            },
            #[cfg(any(feature = "wdog", feature = "everything"))]
            WDOG2: WDOG2 {
                _marker: PhantomData,
            },
            #[cfg(any(feature = "wdog", feature = "everything"))]
            RTWDOG: RTWDOG {
                _marker: PhantomData,
            },
            #[cfg(any(feature = "notimplemented", feature = "everything"))]
            ADC1: ADC1 {
                _marker: PhantomData,
            },
            #[cfg(any(feature = "notimplemented", feature = "everything"))]
            ADC2: ADC2 {
                _marker: PhantomData,
            },
            #[cfg(any(feature = "notimplemented", feature = "everything"))]
            TRNG: TRNG {
                _marker: PhantomData,
            },
            #[cfg(any(feature = "notimplemented", feature = "everything"))]
            SNVS: SNVS {
                _marker: PhantomData,
            },
            #[cfg(any(feature = "ccm_analog", feature = "everything"))]
            CCM_ANALOG: CCM_ANALOG {
                _marker: PhantomData,
            },
            #[cfg(any(feature = "notimplemented", feature = "everything"))]
            PMU: PMU {
                _marker: PhantomData,
            },
            #[cfg(any(feature = "notimplemented", feature = "everything"))]
            TEMPMON: TEMPMON {
                _marker: PhantomData,
            },
            #[cfg(any(feature = "notimplemented", feature = "everything"))]
            USB_ANALOG: USB_ANALOG {
                _marker: PhantomData,
            },
            #[cfg(any(feature = "xtalosc24m", feature = "everything"))]
            XTALOSC24M: XTALOSC24M {
                _marker: PhantomData,
            },
            #[cfg(any(feature = "notimplemented", feature = "everything"))]
            USBPHY1: USBPHY1 {
                _marker: PhantomData,
            },
            #[cfg(any(feature = "notimplemented", feature = "everything"))]
            USBPHY2: USBPHY2 {
                _marker: PhantomData,
            },
            #[cfg(any(feature = "notimplemented", feature = "everything"))]
            CSU: CSU {
                _marker: PhantomData,
            },
            #[cfg(any(feature = "notimplemented", feature = "everything"))]
            TSC: TSC {
                _marker: PhantomData,
            },
            #[cfg(any(feature = "notimplemented", feature = "everything"))]
            DMA0: DMA0 {
                _marker: PhantomData,
            },
            #[cfg(any(feature = "notimplemented", feature = "everything"))]
            DMAMUX: DMAMUX {
                _marker: PhantomData,
            },
            #[cfg(any(feature = "notimplemented", feature = "everything"))]
            GPC: GPC {
                _marker: PhantomData,
            },
            #[cfg(any(feature = "notimplemented", feature = "everything"))]
            PGC: PGC {
                _marker: PhantomData,
            },
            #[cfg(any(feature = "notimplemented", feature = "everything"))]
            SRC: SRC {
                _marker: PhantomData,
            },
            #[cfg(any(feature = "ccm", feature = "everything"))]
            CCM: CCM {
                _marker: PhantomData,
            },
            #[cfg(any(feature = "notimplemented", feature = "everything"))]
            ROMC: ROMC {
                _marker: PhantomData,
            },
            #[cfg(any(feature = "notimplemented", feature = "everything"))]
            LPUART1: LPUART1 {
                _marker: PhantomData,
            },
            #[cfg(any(feature = "notimplemented", feature = "everything"))]
            LPUART2: LPUART2 {
                _marker: PhantomData,
            },
            #[cfg(any(feature = "notimplemented", feature = "everything"))]
            LPUART3: LPUART3 {
                _marker: PhantomData,
            },
            #[cfg(any(feature = "notimplemented", feature = "everything"))]
            LPUART4: LPUART4 {
                _marker: PhantomData,
            },
            #[cfg(any(feature = "notimplemented", feature = "everything"))]
            LPUART5: LPUART5 {
                _marker: PhantomData,
            },
            #[cfg(any(feature = "notimplemented", feature = "everything"))]
            LPUART6: LPUART6 {
                _marker: PhantomData,
            },
            #[cfg(any(feature = "notimplemented", feature = "everything"))]
            LPUART7: LPUART7 {
                _marker: PhantomData,
            },
            #[cfg(any(feature = "notimplemented", feature = "everything"))]
            LPUART8: LPUART8 {
                _marker: PhantomData,
            },
            #[cfg(any(feature = "notimplemented", feature = "everything"))]
            FLEXIO1: FLEXIO1 {
                _marker: PhantomData,
            },
            #[cfg(any(feature = "notimplemented", feature = "everything"))]
            FLEXIO2: FLEXIO2 {
                _marker: PhantomData,
            },
            #[cfg(any(feature = "gpio1", feature = "everything"))]
            GPIO1: GPIO1 {
                _marker: PhantomData,
            },
            #[cfg(any(feature = "notimplemented", feature = "everything"))]
            GPIO5: GPIO5 {
                _marker: PhantomData,
            },
            #[cfg(any(feature = "notimplemented", feature = "everything"))]
            GPIO2: GPIO2 {
                _marker: PhantomData,
            },
            #[cfg(any(feature = "notimplemented", feature = "everything"))]
            GPIO3: GPIO3 {
                _marker: PhantomData,
            },
            #[cfg(any(feature = "notimplemented", feature = "everything"))]
            GPIO4: GPIO4 {
                _marker: PhantomData,
            },
            #[cfg(any(feature = "notimplemented", feature = "everything"))]
            CAN1: CAN1 {
                _marker: PhantomData,
            },
            #[cfg(any(feature = "notimplemented", feature = "everything"))]
            CAN2: CAN2 {
                _marker: PhantomData,
            },
            #[cfg(any(feature = "notimplemented", feature = "everything"))]
            TMR1: TMR1 {
                _marker: PhantomData,
            },
            #[cfg(any(feature = "notimplemented", feature = "everything"))]
            TMR2: TMR2 {
                _marker: PhantomData,
            },
            #[cfg(any(feature = "notimplemented", feature = "everything"))]
            TMR3: TMR3 {
                _marker: PhantomData,
            },
            #[cfg(any(feature = "notimplemented", feature = "everything"))]
            TMR4: TMR4 {
                _marker: PhantomData,
            },
            #[cfg(any(feature = "notimplemented", feature = "everything"))]
            GPT1: GPT1 {
                _marker: PhantomData,
            },
            #[cfg(any(feature = "notimplemented", feature = "everything"))]
            GPT2: GPT2 {
                _marker: PhantomData,
            },
            #[cfg(any(feature = "notimplemented", feature = "everything"))]
            OCOTP: OCOTP {
                _marker: PhantomData,
            },
            #[cfg(any(feature = "iomuxc", feature = "everything"))]
            IOMUXC: IOMUXC {
                _marker: PhantomData,
            },
            #[cfg(any(feature = "notimplemented", feature = "everything"))]
            KPP: KPP {
                _marker: PhantomData,
            },
            #[cfg(any(feature = "notimplemented", feature = "everything"))]
            FLEXSPI: FLEXSPI {
                _marker: PhantomData,
            },
            #[cfg(any(feature = "notimplemented", feature = "everything"))]
            PXP: PXP {
                _marker: PhantomData,
            },
            #[cfg(any(feature = "notimplemented", feature = "everything"))]
            LCDIF: LCDIF {
                _marker: PhantomData,
            },
            #[cfg(any(feature = "notimplemented", feature = "everything"))]
            CSI: CSI {
                _marker: PhantomData,
            },
            #[cfg(any(feature = "notimplemented", feature = "everything"))]
            USDHC1: USDHC1 {
                _marker: PhantomData,
            },
            #[cfg(any(feature = "notimplemented", feature = "everything"))]
            USDHC2: USDHC2 {
                _marker: PhantomData,
            },
            #[cfg(any(feature = "enet", feature = "everything"))]
            ENET: ENET {
                _marker: PhantomData,
            },
            #[cfg(any(feature = "notimplemented", feature = "everything"))]
            USB1: USB1 {
                _marker: PhantomData,
            },
            #[cfg(any(feature = "notimplemented", feature = "everything"))]
            USB2: USB2 {
                _marker: PhantomData,
            },
            #[cfg(any(feature = "notimplemented", feature = "everything"))]
            USBNC1: USBNC1 {
                _marker: PhantomData,
            },
            #[cfg(any(feature = "notimplemented", feature = "everything"))]
            USBNC2: USBNC2 {
                _marker: PhantomData,
            },
            #[cfg(any(feature = "notimplemented", feature = "everything"))]
            SEMC: SEMC {
                _marker: PhantomData,
            },
            #[cfg(any(feature = "notimplemented", feature = "everything"))]
            DCP: DCP {
                _marker: PhantomData,
            },
            #[cfg(any(feature = "notimplemented", feature = "everything"))]
            SPDIF: SPDIF {
                _marker: PhantomData,
            },
            #[cfg(any(feature = "notimplemented", feature = "everything"))]
            SAI1: SAI1 {
                _marker: PhantomData,
            },
            #[cfg(any(feature = "notimplemented", feature = "everything"))]
            SAI2: SAI2 {
                _marker: PhantomData,
            },
            #[cfg(any(feature = "notimplemented", feature = "everything"))]
            SAI3: SAI3 {
                _marker: PhantomData,
            },
            #[cfg(any(feature = "notimplemented", feature = "everything"))]
            LPSPI1: LPSPI1 {
                _marker: PhantomData,
            },
            #[cfg(any(feature = "notimplemented", feature = "everything"))]
            LPSPI2: LPSPI2 {
                _marker: PhantomData,
            },
            #[cfg(any(feature = "notimplemented", feature = "everything"))]
            LPSPI3: LPSPI3 {
                _marker: PhantomData,
            },
            #[cfg(any(feature = "notimplemented", feature = "everything"))]
            LPSPI4: LPSPI4 {
                _marker: PhantomData,
            },
            #[cfg(any(feature = "notimplemented", feature = "everything"))]
            ADC_ETC: ADC_ETC {
                _marker: PhantomData,
            },
            #[cfg(any(feature = "notimplemented", feature = "everything"))]
            AOI1: AOI1 {
                _marker: PhantomData,
            },
            #[cfg(any(feature = "notimplemented", feature = "everything"))]
            AOI2: AOI2 {
                _marker: PhantomData,
            },
            #[cfg(any(feature = "notimplemented", feature = "everything"))]
            XBARA1: XBARA1 {
                _marker: PhantomData,
            },
            #[cfg(any(feature = "notimplemented", feature = "everything"))]
            XBARB2: XBARB2 {
                _marker: PhantomData,
            },
            #[cfg(any(feature = "notimplemented", feature = "everything"))]
            XBARB3: XBARB3 {
                _marker: PhantomData,
            },
            #[cfg(any(feature = "notimplemented", feature = "everything"))]
            ENC1: ENC1 {
                _marker: PhantomData,
            },
            #[cfg(any(feature = "notimplemented", feature = "everything"))]
            ENC2: ENC2 {
                _marker: PhantomData,
            },
            #[cfg(any(feature = "notimplemented", feature = "everything"))]
            ENC3: ENC3 {
                _marker: PhantomData,
            },
            #[cfg(any(feature = "notimplemented", feature = "everything"))]
            ENC4: ENC4 {
                _marker: PhantomData,
            },
            #[cfg(any(feature = "notimplemented", feature = "everything"))]
            PWM1: PWM1 {
                _marker: PhantomData,
            },
            #[cfg(any(feature = "notimplemented", feature = "everything"))]
            PWM2: PWM2 {
                _marker: PhantomData,
            },
            #[cfg(any(feature = "notimplemented", feature = "everything"))]
            PWM3: PWM3 {
                _marker: PhantomData,
            },
            #[cfg(any(feature = "notimplemented", feature = "everything"))]
            PWM4: PWM4 {
                _marker: PhantomData,
            },
            #[cfg(any(feature = "notimplemented", feature = "everything"))]
            BEE: BEE {
                _marker: PhantomData,
            },
            #[cfg(any(feature = "notimplemented", feature = "everything"))]
            LPI2C1: LPI2C1 {
                _marker: PhantomData,
            },
            #[cfg(any(feature = "notimplemented", feature = "everything"))]
            LPI2C2: LPI2C2 {
                _marker: PhantomData,
            },
            #[cfg(any(feature = "notimplemented", feature = "everything"))]
            LPI2C3: LPI2C3 {
                _marker: PhantomData,
            },
            #[cfg(any(feature = "notimplemented", feature = "everything"))]
            LPI2C4: LPI2C4 {
                _marker: PhantomData,
            },
            #[cfg(any(feature = "notimplemented", feature = "everything"))]
            SYSTEMCONTROL: SYSTEMCONTROL {
                _marker: PhantomData,
            },
        }
    }
}
