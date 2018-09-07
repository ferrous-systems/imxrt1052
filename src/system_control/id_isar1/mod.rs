#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::ID_ISAR1 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `EXTEND_INSTRS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTEND_INSTRSR {
    #[doc = "None supported, ARMv7-M unused"]
    EXTEND_INSTRS_0,
    #[doc = "Adds support for the SXTB, SXTH, UXTB, and UXTH instructions"]
    EXTEND_INSTRS_1,
    #[doc = "As for 1, and adds support for the SXTAB, SXTAB16, SXTAH, SXTB16, UXTAB, UXTAB16, UXTAH, and UXTB16 instructions"]
    EXTEND_INSTRS_2,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl EXTEND_INSTRSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EXTEND_INSTRSR::EXTEND_INSTRS_0 => 0,
            EXTEND_INSTRSR::EXTEND_INSTRS_1 => 1,
            EXTEND_INSTRSR::EXTEND_INSTRS_2 => 2,
            EXTEND_INSTRSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EXTEND_INSTRSR {
        match value {
            0 => EXTEND_INSTRSR::EXTEND_INSTRS_0,
            1 => EXTEND_INSTRSR::EXTEND_INSTRS_1,
            2 => EXTEND_INSTRSR::EXTEND_INSTRS_2,
            i => EXTEND_INSTRSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `EXTEND_INSTRS_0`"]
    #[inline]
    pub fn is_extend_instrs_0(&self) -> bool {
        *self == EXTEND_INSTRSR::EXTEND_INSTRS_0
    }
    #[doc = "Checks if the value of the field is `EXTEND_INSTRS_1`"]
    #[inline]
    pub fn is_extend_instrs_1(&self) -> bool {
        *self == EXTEND_INSTRSR::EXTEND_INSTRS_1
    }
    #[doc = "Checks if the value of the field is `EXTEND_INSTRS_2`"]
    #[inline]
    pub fn is_extend_instrs_2(&self) -> bool {
        *self == EXTEND_INSTRSR::EXTEND_INSTRS_2
    }
}
#[doc = "Possible values of the field `IFTHEN_INSTRS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IFTHEN_INSTRSR {
    #[doc = "None supported, ARMv7-M unused"]
    IFTHEN_INSTRS_0,
    #[doc = "Adds support for the IT instructions, and for the IT bits in the PSRs"]
    IFTHEN_INSTRS_1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl IFTHEN_INSTRSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            IFTHEN_INSTRSR::IFTHEN_INSTRS_0 => 0,
            IFTHEN_INSTRSR::IFTHEN_INSTRS_1 => 1,
            IFTHEN_INSTRSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> IFTHEN_INSTRSR {
        match value {
            0 => IFTHEN_INSTRSR::IFTHEN_INSTRS_0,
            1 => IFTHEN_INSTRSR::IFTHEN_INSTRS_1,
            i => IFTHEN_INSTRSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `IFTHEN_INSTRS_0`"]
    #[inline]
    pub fn is_ifthen_instrs_0(&self) -> bool {
        *self == IFTHEN_INSTRSR::IFTHEN_INSTRS_0
    }
    #[doc = "Checks if the value of the field is `IFTHEN_INSTRS_1`"]
    #[inline]
    pub fn is_ifthen_instrs_1(&self) -> bool {
        *self == IFTHEN_INSTRSR::IFTHEN_INSTRS_1
    }
}
#[doc = "Possible values of the field `IMMEDIATE_INSTRS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IMMEDIATE_INSTRSR {
    #[doc = "None supported, ARMv7-M unused"]
    IMMEDIATE_INSTRS_0,
    #[doc = "Adds support for the ADDW, MOVW, MOVT, and SUBW instructions"]
    IMMEDIATE_INSTRS_1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl IMMEDIATE_INSTRSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            IMMEDIATE_INSTRSR::IMMEDIATE_INSTRS_0 => 0,
            IMMEDIATE_INSTRSR::IMMEDIATE_INSTRS_1 => 1,
            IMMEDIATE_INSTRSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> IMMEDIATE_INSTRSR {
        match value {
            0 => IMMEDIATE_INSTRSR::IMMEDIATE_INSTRS_0,
            1 => IMMEDIATE_INSTRSR::IMMEDIATE_INSTRS_1,
            i => IMMEDIATE_INSTRSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `IMMEDIATE_INSTRS_0`"]
    #[inline]
    pub fn is_immediate_instrs_0(&self) -> bool {
        *self == IMMEDIATE_INSTRSR::IMMEDIATE_INSTRS_0
    }
    #[doc = "Checks if the value of the field is `IMMEDIATE_INSTRS_1`"]
    #[inline]
    pub fn is_immediate_instrs_1(&self) -> bool {
        *self == IMMEDIATE_INSTRSR::IMMEDIATE_INSTRS_1
    }
}
#[doc = "Possible values of the field `INTERWORK_INSTRS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTERWORK_INSTRSR {
    #[doc = "None supported, ARMv7-M unused"]
    INTERWORK_INSTRS_0,
    #[doc = "Adds support for the BX instruction, and the T bit in the PSR"]
    INTERWORK_INSTRS_1,
    #[doc = "As for 1, and adds support for the BLX instruction, and PC loads have BX-like behavior"]
    INTERWORK_INSTRS_2,
    #[doc = "ARMv7-M unused"]
    INTERWORK_INSTRS_3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl INTERWORK_INSTRSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            INTERWORK_INSTRSR::INTERWORK_INSTRS_0 => 0,
            INTERWORK_INSTRSR::INTERWORK_INSTRS_1 => 1,
            INTERWORK_INSTRSR::INTERWORK_INSTRS_2 => 2,
            INTERWORK_INSTRSR::INTERWORK_INSTRS_3 => 3,
            INTERWORK_INSTRSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> INTERWORK_INSTRSR {
        match value {
            0 => INTERWORK_INSTRSR::INTERWORK_INSTRS_0,
            1 => INTERWORK_INSTRSR::INTERWORK_INSTRS_1,
            2 => INTERWORK_INSTRSR::INTERWORK_INSTRS_2,
            3 => INTERWORK_INSTRSR::INTERWORK_INSTRS_3,
            i => INTERWORK_INSTRSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `INTERWORK_INSTRS_0`"]
    #[inline]
    pub fn is_interwork_instrs_0(&self) -> bool {
        *self == INTERWORK_INSTRSR::INTERWORK_INSTRS_0
    }
    #[doc = "Checks if the value of the field is `INTERWORK_INSTRS_1`"]
    #[inline]
    pub fn is_interwork_instrs_1(&self) -> bool {
        *self == INTERWORK_INSTRSR::INTERWORK_INSTRS_1
    }
    #[doc = "Checks if the value of the field is `INTERWORK_INSTRS_2`"]
    #[inline]
    pub fn is_interwork_instrs_2(&self) -> bool {
        *self == INTERWORK_INSTRSR::INTERWORK_INSTRS_2
    }
    #[doc = "Checks if the value of the field is `INTERWORK_INSTRS_3`"]
    #[inline]
    pub fn is_interwork_instrs_3(&self) -> bool {
        *self == INTERWORK_INSTRSR::INTERWORK_INSTRS_3
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 12:15 - Indicates the supported Extend instructions"]
    #[inline]
    pub fn extend_instrs(&self) -> EXTEND_INSTRSR {
        EXTEND_INSTRSR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:19 - Indicates the supported IfThen instructions"]
    #[inline]
    pub fn ifthen_instrs(&self) -> IFTHEN_INSTRSR {
        IFTHEN_INSTRSR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:23 - Indicates the support for data-processing instructions with long immediate"]
    #[inline]
    pub fn immediate_instrs(&self) -> IMMEDIATE_INSTRSR {
        IMMEDIATE_INSTRSR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:27 - Indicates the supported Interworking instructions"]
    #[inline]
    pub fn interwork_instrs(&self) -> INTERWORK_INSTRSR {
        INTERWORK_INSTRSR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
