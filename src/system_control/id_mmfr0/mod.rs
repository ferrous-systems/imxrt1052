#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::ID_MMFR0 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `PMSASUPPORT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PMSASUPPORTR {
    #[doc = "Not supported"]
    PMSASUPPORT_0,
    #[doc = "ARMv7-M unused"]
    PMSASUPPORT_1,
    #[doc = "ARMv7-M unused"]
    PMSASUPPORT_2,
    #[doc = "PMSAv7, providing support for a base region and subregions."]
    PMSASUPPORT_3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PMSASUPPORTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PMSASUPPORTR::PMSASUPPORT_0 => 0,
            PMSASUPPORTR::PMSASUPPORT_1 => 1,
            PMSASUPPORTR::PMSASUPPORT_2 => 2,
            PMSASUPPORTR::PMSASUPPORT_3 => 3,
            PMSASUPPORTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PMSASUPPORTR {
        match value {
            0 => PMSASUPPORTR::PMSASUPPORT_0,
            1 => PMSASUPPORTR::PMSASUPPORT_1,
            2 => PMSASUPPORTR::PMSASUPPORT_2,
            3 => PMSASUPPORTR::PMSASUPPORT_3,
            i => PMSASUPPORTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PMSASUPPORT_0`"]
    #[inline]
    pub fn is_pmsasupport_0(&self) -> bool {
        *self == PMSASUPPORTR::PMSASUPPORT_0
    }
    #[doc = "Checks if the value of the field is `PMSASUPPORT_1`"]
    #[inline]
    pub fn is_pmsasupport_1(&self) -> bool {
        *self == PMSASUPPORTR::PMSASUPPORT_1
    }
    #[doc = "Checks if the value of the field is `PMSASUPPORT_2`"]
    #[inline]
    pub fn is_pmsasupport_2(&self) -> bool {
        *self == PMSASUPPORTR::PMSASUPPORT_2
    }
    #[doc = "Checks if the value of the field is `PMSASUPPORT_3`"]
    #[inline]
    pub fn is_pmsasupport_3(&self) -> bool {
        *self == PMSASUPPORTR::PMSASUPPORT_3
    }
}
#[doc = "Possible values of the field `OUTERMOST_SHAREABILITY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTERMOST_SHAREABILITYR {
    #[doc = "Implemented as Non-cacheable"]
    OUTERMOST_SHAREABILITY_0,
    #[doc = "ARMv7-M unused"]
    OUTERMOST_SHAREABILITY_1,
    #[doc = "ARMv7-M unused"]
    OUTERMOST_SHAREABILITY_2,
    #[doc = "ARMv7-M unused"]
    OUTERMOST_SHAREABILITY_3,
    #[doc = "ARMv7-M unused"]
    OUTERMOST_SHAREABILITY_4,
    #[doc = "ARMv7-M unused"]
    OUTERMOST_SHAREABILITY_5,
    #[doc = "ARMv7-M unused"]
    OUTERMOST_SHAREABILITY_6,
    #[doc = "ARMv7-M unused"]
    OUTERMOST_SHAREABILITY_7,
    #[doc = "ARMv7-M unused"]
    OUTERMOST_SHAREABILITY_8,
    #[doc = "ARMv7-M unused"]
    OUTERMOST_SHAREABILITY_9,
    #[doc = "ARMv7-M unused"]
    OUTERMOST_SHAREABILITY_10,
    #[doc = "ARMv7-M unused"]
    OUTERMOST_SHAREABILITY_11,
    #[doc = "ARMv7-M unused"]
    OUTERMOST_SHAREABILITY_12,
    #[doc = "ARMv7-M unused"]
    OUTERMOST_SHAREABILITY_13,
    #[doc = "ARMv7-M unused"]
    OUTERMOST_SHAREABILITY_14,
    #[doc = "Shareability ignored."]
    OUTERMOST_SHAREABILITY_15,
}
impl OUTERMOST_SHAREABILITYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            OUTERMOST_SHAREABILITYR::OUTERMOST_SHAREABILITY_0 => 0,
            OUTERMOST_SHAREABILITYR::OUTERMOST_SHAREABILITY_1 => 1,
            OUTERMOST_SHAREABILITYR::OUTERMOST_SHAREABILITY_2 => 2,
            OUTERMOST_SHAREABILITYR::OUTERMOST_SHAREABILITY_3 => 3,
            OUTERMOST_SHAREABILITYR::OUTERMOST_SHAREABILITY_4 => 4,
            OUTERMOST_SHAREABILITYR::OUTERMOST_SHAREABILITY_5 => 5,
            OUTERMOST_SHAREABILITYR::OUTERMOST_SHAREABILITY_6 => 6,
            OUTERMOST_SHAREABILITYR::OUTERMOST_SHAREABILITY_7 => 7,
            OUTERMOST_SHAREABILITYR::OUTERMOST_SHAREABILITY_8 => 8,
            OUTERMOST_SHAREABILITYR::OUTERMOST_SHAREABILITY_9 => 9,
            OUTERMOST_SHAREABILITYR::OUTERMOST_SHAREABILITY_10 => 10,
            OUTERMOST_SHAREABILITYR::OUTERMOST_SHAREABILITY_11 => 11,
            OUTERMOST_SHAREABILITYR::OUTERMOST_SHAREABILITY_12 => 12,
            OUTERMOST_SHAREABILITYR::OUTERMOST_SHAREABILITY_13 => 13,
            OUTERMOST_SHAREABILITYR::OUTERMOST_SHAREABILITY_14 => 14,
            OUTERMOST_SHAREABILITYR::OUTERMOST_SHAREABILITY_15 => 15,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> OUTERMOST_SHAREABILITYR {
        match value {
            0 => OUTERMOST_SHAREABILITYR::OUTERMOST_SHAREABILITY_0,
            1 => OUTERMOST_SHAREABILITYR::OUTERMOST_SHAREABILITY_1,
            2 => OUTERMOST_SHAREABILITYR::OUTERMOST_SHAREABILITY_2,
            3 => OUTERMOST_SHAREABILITYR::OUTERMOST_SHAREABILITY_3,
            4 => OUTERMOST_SHAREABILITYR::OUTERMOST_SHAREABILITY_4,
            5 => OUTERMOST_SHAREABILITYR::OUTERMOST_SHAREABILITY_5,
            6 => OUTERMOST_SHAREABILITYR::OUTERMOST_SHAREABILITY_6,
            7 => OUTERMOST_SHAREABILITYR::OUTERMOST_SHAREABILITY_7,
            8 => OUTERMOST_SHAREABILITYR::OUTERMOST_SHAREABILITY_8,
            9 => OUTERMOST_SHAREABILITYR::OUTERMOST_SHAREABILITY_9,
            10 => OUTERMOST_SHAREABILITYR::OUTERMOST_SHAREABILITY_10,
            11 => OUTERMOST_SHAREABILITYR::OUTERMOST_SHAREABILITY_11,
            12 => OUTERMOST_SHAREABILITYR::OUTERMOST_SHAREABILITY_12,
            13 => OUTERMOST_SHAREABILITYR::OUTERMOST_SHAREABILITY_13,
            14 => OUTERMOST_SHAREABILITYR::OUTERMOST_SHAREABILITY_14,
            15 => OUTERMOST_SHAREABILITYR::OUTERMOST_SHAREABILITY_15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OUTERMOST_SHAREABILITY_0`"]
    #[inline]
    pub fn is_outermost_shareability_0(&self) -> bool {
        *self == OUTERMOST_SHAREABILITYR::OUTERMOST_SHAREABILITY_0
    }
    #[doc = "Checks if the value of the field is `OUTERMOST_SHAREABILITY_1`"]
    #[inline]
    pub fn is_outermost_shareability_1(&self) -> bool {
        *self == OUTERMOST_SHAREABILITYR::OUTERMOST_SHAREABILITY_1
    }
    #[doc = "Checks if the value of the field is `OUTERMOST_SHAREABILITY_2`"]
    #[inline]
    pub fn is_outermost_shareability_2(&self) -> bool {
        *self == OUTERMOST_SHAREABILITYR::OUTERMOST_SHAREABILITY_2
    }
    #[doc = "Checks if the value of the field is `OUTERMOST_SHAREABILITY_3`"]
    #[inline]
    pub fn is_outermost_shareability_3(&self) -> bool {
        *self == OUTERMOST_SHAREABILITYR::OUTERMOST_SHAREABILITY_3
    }
    #[doc = "Checks if the value of the field is `OUTERMOST_SHAREABILITY_4`"]
    #[inline]
    pub fn is_outermost_shareability_4(&self) -> bool {
        *self == OUTERMOST_SHAREABILITYR::OUTERMOST_SHAREABILITY_4
    }
    #[doc = "Checks if the value of the field is `OUTERMOST_SHAREABILITY_5`"]
    #[inline]
    pub fn is_outermost_shareability_5(&self) -> bool {
        *self == OUTERMOST_SHAREABILITYR::OUTERMOST_SHAREABILITY_5
    }
    #[doc = "Checks if the value of the field is `OUTERMOST_SHAREABILITY_6`"]
    #[inline]
    pub fn is_outermost_shareability_6(&self) -> bool {
        *self == OUTERMOST_SHAREABILITYR::OUTERMOST_SHAREABILITY_6
    }
    #[doc = "Checks if the value of the field is `OUTERMOST_SHAREABILITY_7`"]
    #[inline]
    pub fn is_outermost_shareability_7(&self) -> bool {
        *self == OUTERMOST_SHAREABILITYR::OUTERMOST_SHAREABILITY_7
    }
    #[doc = "Checks if the value of the field is `OUTERMOST_SHAREABILITY_8`"]
    #[inline]
    pub fn is_outermost_shareability_8(&self) -> bool {
        *self == OUTERMOST_SHAREABILITYR::OUTERMOST_SHAREABILITY_8
    }
    #[doc = "Checks if the value of the field is `OUTERMOST_SHAREABILITY_9`"]
    #[inline]
    pub fn is_outermost_shareability_9(&self) -> bool {
        *self == OUTERMOST_SHAREABILITYR::OUTERMOST_SHAREABILITY_9
    }
    #[doc = "Checks if the value of the field is `OUTERMOST_SHAREABILITY_10`"]
    #[inline]
    pub fn is_outermost_shareability_10(&self) -> bool {
        *self == OUTERMOST_SHAREABILITYR::OUTERMOST_SHAREABILITY_10
    }
    #[doc = "Checks if the value of the field is `OUTERMOST_SHAREABILITY_11`"]
    #[inline]
    pub fn is_outermost_shareability_11(&self) -> bool {
        *self == OUTERMOST_SHAREABILITYR::OUTERMOST_SHAREABILITY_11
    }
    #[doc = "Checks if the value of the field is `OUTERMOST_SHAREABILITY_12`"]
    #[inline]
    pub fn is_outermost_shareability_12(&self) -> bool {
        *self == OUTERMOST_SHAREABILITYR::OUTERMOST_SHAREABILITY_12
    }
    #[doc = "Checks if the value of the field is `OUTERMOST_SHAREABILITY_13`"]
    #[inline]
    pub fn is_outermost_shareability_13(&self) -> bool {
        *self == OUTERMOST_SHAREABILITYR::OUTERMOST_SHAREABILITY_13
    }
    #[doc = "Checks if the value of the field is `OUTERMOST_SHAREABILITY_14`"]
    #[inline]
    pub fn is_outermost_shareability_14(&self) -> bool {
        *self == OUTERMOST_SHAREABILITYR::OUTERMOST_SHAREABILITY_14
    }
    #[doc = "Checks if the value of the field is `OUTERMOST_SHAREABILITY_15`"]
    #[inline]
    pub fn is_outermost_shareability_15(&self) -> bool {
        *self == OUTERMOST_SHAREABILITYR::OUTERMOST_SHAREABILITY_15
    }
}
#[doc = "Possible values of the field `SHAREABILITY_LEVELS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SHAREABILITY_LEVELSR {
    #[doc = "One level of shareability implemented"]
    SHAREABILITY_LEVELS_0,
    #[doc = "ARMv7-M unused"]
    SHAREABILITY_LEVELS_1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SHAREABILITY_LEVELSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SHAREABILITY_LEVELSR::SHAREABILITY_LEVELS_0 => 0,
            SHAREABILITY_LEVELSR::SHAREABILITY_LEVELS_1 => 1,
            SHAREABILITY_LEVELSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SHAREABILITY_LEVELSR {
        match value {
            0 => SHAREABILITY_LEVELSR::SHAREABILITY_LEVELS_0,
            1 => SHAREABILITY_LEVELSR::SHAREABILITY_LEVELS_1,
            i => SHAREABILITY_LEVELSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SHAREABILITY_LEVELS_0`"]
    #[inline]
    pub fn is_shareability_levels_0(&self) -> bool {
        *self == SHAREABILITY_LEVELSR::SHAREABILITY_LEVELS_0
    }
    #[doc = "Checks if the value of the field is `SHAREABILITY_LEVELS_1`"]
    #[inline]
    pub fn is_shareability_levels_1(&self) -> bool {
        *self == SHAREABILITY_LEVELSR::SHAREABILITY_LEVELS_1
    }
}
#[doc = "Possible values of the field `TCM_SUPPORT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCM_SUPPORTR {
    #[doc = "No tightly coupled memories implemented."]
    TCM_SUPPORT_0,
    #[doc = "Tightly coupled memories implemented with IMPLEMENTATION DEFINED control."]
    TCM_SUPPORT_1,
    #[doc = "ARMv7-M unused"]
    TCM_SUPPORT_2,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TCM_SUPPORTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TCM_SUPPORTR::TCM_SUPPORT_0 => 0,
            TCM_SUPPORTR::TCM_SUPPORT_1 => 1,
            TCM_SUPPORTR::TCM_SUPPORT_2 => 2,
            TCM_SUPPORTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TCM_SUPPORTR {
        match value {
            0 => TCM_SUPPORTR::TCM_SUPPORT_0,
            1 => TCM_SUPPORTR::TCM_SUPPORT_1,
            2 => TCM_SUPPORTR::TCM_SUPPORT_2,
            i => TCM_SUPPORTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `TCM_SUPPORT_0`"]
    #[inline]
    pub fn is_tcm_support_0(&self) -> bool {
        *self == TCM_SUPPORTR::TCM_SUPPORT_0
    }
    #[doc = "Checks if the value of the field is `TCM_SUPPORT_1`"]
    #[inline]
    pub fn is_tcm_support_1(&self) -> bool {
        *self == TCM_SUPPORTR::TCM_SUPPORT_1
    }
    #[doc = "Checks if the value of the field is `TCM_SUPPORT_2`"]
    #[inline]
    pub fn is_tcm_support_2(&self) -> bool {
        *self == TCM_SUPPORTR::TCM_SUPPORT_2
    }
}
#[doc = "Possible values of the field `AUXILIARY_REGISTERS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUXILIARY_REGISTERSR {
    #[doc = "Not supported"]
    AUXILIARY_REGISTERS_0,
    #[doc = "Support for Auxiliary Control Register only."]
    AUXILIARY_REGISTERS_1,
    #[doc = "ARMv7-M unused"]
    AUXILIARY_REGISTERS_2,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl AUXILIARY_REGISTERSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            AUXILIARY_REGISTERSR::AUXILIARY_REGISTERS_0 => 0,
            AUXILIARY_REGISTERSR::AUXILIARY_REGISTERS_1 => 1,
            AUXILIARY_REGISTERSR::AUXILIARY_REGISTERS_2 => 2,
            AUXILIARY_REGISTERSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> AUXILIARY_REGISTERSR {
        match value {
            0 => AUXILIARY_REGISTERSR::AUXILIARY_REGISTERS_0,
            1 => AUXILIARY_REGISTERSR::AUXILIARY_REGISTERS_1,
            2 => AUXILIARY_REGISTERSR::AUXILIARY_REGISTERS_2,
            i => AUXILIARY_REGISTERSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `AUXILIARY_REGISTERS_0`"]
    #[inline]
    pub fn is_auxiliary_registers_0(&self) -> bool {
        *self == AUXILIARY_REGISTERSR::AUXILIARY_REGISTERS_0
    }
    #[doc = "Checks if the value of the field is `AUXILIARY_REGISTERS_1`"]
    #[inline]
    pub fn is_auxiliary_registers_1(&self) -> bool {
        *self == AUXILIARY_REGISTERSR::AUXILIARY_REGISTERS_1
    }
    #[doc = "Checks if the value of the field is `AUXILIARY_REGISTERS_2`"]
    #[inline]
    pub fn is_auxiliary_registers_2(&self) -> bool {
        *self == AUXILIARY_REGISTERSR::AUXILIARY_REGISTERS_2
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 4:7 - Indicates support for a PMSA"]
    #[inline]
    pub fn pmsasupport(&self) -> PMSASUPPORTR {
        PMSASUPPORTR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:11 - Indicates the outermost shareability domain implemented"]
    #[inline]
    pub fn outermost_shareability(&self) -> OUTERMOST_SHAREABILITYR {
        OUTERMOST_SHAREABILITYR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:15 - Indicates the number of shareability levels implemented"]
    #[inline]
    pub fn shareability_levels(&self) -> SHAREABILITY_LEVELSR {
        SHAREABILITY_LEVELSR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:19 - Indicates the support for Tightly Coupled Memory"]
    #[inline]
    pub fn tcm_support(&self) -> TCM_SUPPORTR {
        TCM_SUPPORTR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:23 - Indicates the support for Auxiliary registers"]
    #[inline]
    pub fn auxiliary_registers(&self) -> AUXILIARY_REGISTERSR {
        AUXILIARY_REGISTERSR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
