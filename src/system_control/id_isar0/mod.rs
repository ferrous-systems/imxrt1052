#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::ID_ISAR0 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `BITCOUNT_INSTRS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BITCOUNT_INSTRSR {
    #[doc = "None supported, ARMv7-M unused"]
    BITCOUNT_INSTRS_0,
    #[doc = "Adds support for the CLZ instruction"]
    BITCOUNT_INSTRS_1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl BITCOUNT_INSTRSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BITCOUNT_INSTRSR::BITCOUNT_INSTRS_0 => 0,
            BITCOUNT_INSTRSR::BITCOUNT_INSTRS_1 => 1,
            BITCOUNT_INSTRSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BITCOUNT_INSTRSR {
        match value {
            0 => BITCOUNT_INSTRSR::BITCOUNT_INSTRS_0,
            1 => BITCOUNT_INSTRSR::BITCOUNT_INSTRS_1,
            i => BITCOUNT_INSTRSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `BITCOUNT_INSTRS_0`"]
    #[inline]
    pub fn is_bitcount_instrs_0(&self) -> bool {
        *self == BITCOUNT_INSTRSR::BITCOUNT_INSTRS_0
    }
    #[doc = "Checks if the value of the field is `BITCOUNT_INSTRS_1`"]
    #[inline]
    pub fn is_bitcount_instrs_1(&self) -> bool {
        *self == BITCOUNT_INSTRSR::BITCOUNT_INSTRS_1
    }
}
#[doc = "Possible values of the field `BITFIELD_INSTRS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BITFIELD_INSTRSR {
    #[doc = "None supported, ARMv7-M unused"]
    BITFIELD_INSTRS_0,
    #[doc = "Adds support for the BFC, BFI, SBFX, and UBFX instructions"]
    BITFIELD_INSTRS_1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl BITFIELD_INSTRSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BITFIELD_INSTRSR::BITFIELD_INSTRS_0 => 0,
            BITFIELD_INSTRSR::BITFIELD_INSTRS_1 => 1,
            BITFIELD_INSTRSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BITFIELD_INSTRSR {
        match value {
            0 => BITFIELD_INSTRSR::BITFIELD_INSTRS_0,
            1 => BITFIELD_INSTRSR::BITFIELD_INSTRS_1,
            i => BITFIELD_INSTRSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `BITFIELD_INSTRS_0`"]
    #[inline]
    pub fn is_bitfield_instrs_0(&self) -> bool {
        *self == BITFIELD_INSTRSR::BITFIELD_INSTRS_0
    }
    #[doc = "Checks if the value of the field is `BITFIELD_INSTRS_1`"]
    #[inline]
    pub fn is_bitfield_instrs_1(&self) -> bool {
        *self == BITFIELD_INSTRSR::BITFIELD_INSTRS_1
    }
}
#[doc = "Possible values of the field `CMPBRANCH_INSTRS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMPBRANCH_INSTRSR {
    #[doc = "None supported, ARMv7-M unused"]
    CMPBRANCH_INSTRS_0,
    #[doc = "Adds support for the CBNZ and CBZ instructions"]
    CMPBRANCH_INSTRS_1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CMPBRANCH_INSTRSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CMPBRANCH_INSTRSR::CMPBRANCH_INSTRS_0 => 0,
            CMPBRANCH_INSTRSR::CMPBRANCH_INSTRS_1 => 1,
            CMPBRANCH_INSTRSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CMPBRANCH_INSTRSR {
        match value {
            0 => CMPBRANCH_INSTRSR::CMPBRANCH_INSTRS_0,
            1 => CMPBRANCH_INSTRSR::CMPBRANCH_INSTRS_1,
            i => CMPBRANCH_INSTRSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CMPBRANCH_INSTRS_0`"]
    #[inline]
    pub fn is_cmpbranch_instrs_0(&self) -> bool {
        *self == CMPBRANCH_INSTRSR::CMPBRANCH_INSTRS_0
    }
    #[doc = "Checks if the value of the field is `CMPBRANCH_INSTRS_1`"]
    #[inline]
    pub fn is_cmpbranch_instrs_1(&self) -> bool {
        *self == CMPBRANCH_INSTRSR::CMPBRANCH_INSTRS_1
    }
}
#[doc = "Possible values of the field `COPROC_INSTRS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COPROC_INSTRSR {
    #[doc = "None supported, except for separately attributed architectures, for example the Floating-point extension"]
    COPROC_INSTRS_0,
    #[doc = "Adds support for generic CDP, LDC, MCR, MRC, and STC instructions"]
    COPROC_INSTRS_1,
    #[doc = "As for 1, and adds support for generic CDP2, LDC2, MCR2, MRC2, and STC2 instructions"]
    COPROC_INSTRS_2,
    #[doc = "As for 2, and adds support for generic MCRR and MRRC instructions"]
    COPROC_INSTRS_3,
    #[doc = "As for 3, and adds support for generic MCRR2 and MRRC2 instructions"]
    COPROC_INSTRS_4,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl COPROC_INSTRSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            COPROC_INSTRSR::COPROC_INSTRS_0 => 0,
            COPROC_INSTRSR::COPROC_INSTRS_1 => 1,
            COPROC_INSTRSR::COPROC_INSTRS_2 => 2,
            COPROC_INSTRSR::COPROC_INSTRS_3 => 3,
            COPROC_INSTRSR::COPROC_INSTRS_4 => 4,
            COPROC_INSTRSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> COPROC_INSTRSR {
        match value {
            0 => COPROC_INSTRSR::COPROC_INSTRS_0,
            1 => COPROC_INSTRSR::COPROC_INSTRS_1,
            2 => COPROC_INSTRSR::COPROC_INSTRS_2,
            3 => COPROC_INSTRSR::COPROC_INSTRS_3,
            4 => COPROC_INSTRSR::COPROC_INSTRS_4,
            i => COPROC_INSTRSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `COPROC_INSTRS_0`"]
    #[inline]
    pub fn is_coproc_instrs_0(&self) -> bool {
        *self == COPROC_INSTRSR::COPROC_INSTRS_0
    }
    #[doc = "Checks if the value of the field is `COPROC_INSTRS_1`"]
    #[inline]
    pub fn is_coproc_instrs_1(&self) -> bool {
        *self == COPROC_INSTRSR::COPROC_INSTRS_1
    }
    #[doc = "Checks if the value of the field is `COPROC_INSTRS_2`"]
    #[inline]
    pub fn is_coproc_instrs_2(&self) -> bool {
        *self == COPROC_INSTRSR::COPROC_INSTRS_2
    }
    #[doc = "Checks if the value of the field is `COPROC_INSTRS_3`"]
    #[inline]
    pub fn is_coproc_instrs_3(&self) -> bool {
        *self == COPROC_INSTRSR::COPROC_INSTRS_3
    }
    #[doc = "Checks if the value of the field is `COPROC_INSTRS_4`"]
    #[inline]
    pub fn is_coproc_instrs_4(&self) -> bool {
        *self == COPROC_INSTRSR::COPROC_INSTRS_4
    }
}
#[doc = "Possible values of the field `DEBUG_INSTRS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEBUG_INSTRSR {
    #[doc = "None supported, ARMv7-M unused"]
    DEBUG_INSTRS_0,
    #[doc = "Adds support for the BKPT instruction"]
    DEBUG_INSTRS_1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DEBUG_INSTRSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DEBUG_INSTRSR::DEBUG_INSTRS_0 => 0,
            DEBUG_INSTRSR::DEBUG_INSTRS_1 => 1,
            DEBUG_INSTRSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DEBUG_INSTRSR {
        match value {
            0 => DEBUG_INSTRSR::DEBUG_INSTRS_0,
            1 => DEBUG_INSTRSR::DEBUG_INSTRS_1,
            i => DEBUG_INSTRSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DEBUG_INSTRS_0`"]
    #[inline]
    pub fn is_debug_instrs_0(&self) -> bool {
        *self == DEBUG_INSTRSR::DEBUG_INSTRS_0
    }
    #[doc = "Checks if the value of the field is `DEBUG_INSTRS_1`"]
    #[inline]
    pub fn is_debug_instrs_1(&self) -> bool {
        *self == DEBUG_INSTRSR::DEBUG_INSTRS_1
    }
}
#[doc = "Possible values of the field `DIVIDE_INSTRS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIVIDE_INSTRSR {
    #[doc = "None supported, ARMv7-M unused"]
    DIVIDE_INSTRS_0,
    #[doc = "Adds support for the SDIV and UDIV instructions"]
    DIVIDE_INSTRS_1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DIVIDE_INSTRSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DIVIDE_INSTRSR::DIVIDE_INSTRS_0 => 0,
            DIVIDE_INSTRSR::DIVIDE_INSTRS_1 => 1,
            DIVIDE_INSTRSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DIVIDE_INSTRSR {
        match value {
            0 => DIVIDE_INSTRSR::DIVIDE_INSTRS_0,
            1 => DIVIDE_INSTRSR::DIVIDE_INSTRS_1,
            i => DIVIDE_INSTRSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DIVIDE_INSTRS_0`"]
    #[inline]
    pub fn is_divide_instrs_0(&self) -> bool {
        *self == DIVIDE_INSTRSR::DIVIDE_INSTRS_0
    }
    #[doc = "Checks if the value of the field is `DIVIDE_INSTRS_1`"]
    #[inline]
    pub fn is_divide_instrs_1(&self) -> bool {
        *self == DIVIDE_INSTRSR::DIVIDE_INSTRS_1
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 4:7 - Indicates the supported Bit Counting instructions"]
    #[inline]
    pub fn bitcount_instrs(&self) -> BITCOUNT_INSTRSR {
        BITCOUNT_INSTRSR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:11 - Indicates the supported BitField instructions"]
    #[inline]
    pub fn bitfield_instrs(&self) -> BITFIELD_INSTRSR {
        BITFIELD_INSTRSR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:15 - Indicates the supported combined Compare and Branch instructions"]
    #[inline]
    pub fn cmpbranch_instrs(&self) -> CMPBRANCH_INSTRSR {
        CMPBRANCH_INSTRSR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:19 - Indicates the supported Coprocessor instructions"]
    #[inline]
    pub fn coproc_instrs(&self) -> COPROC_INSTRSR {
        COPROC_INSTRSR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:23 - Indicates the supported Debug instructions"]
    #[inline]
    pub fn debug_instrs(&self) -> DEBUG_INSTRSR {
        DEBUG_INSTRSR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:27 - Indicates the supported Divide instructions"]
    #[inline]
    pub fn divide_instrs(&self) -> DIVIDE_INSTRSR {
        DIVIDE_INSTRSR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
