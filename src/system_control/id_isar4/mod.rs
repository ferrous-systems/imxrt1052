#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::ID_ISAR4 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `UNPRIV_INSTRS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UNPRIV_INSTRSR {
    #[doc = "None supported, ARMv7-M unused."]
    UNPRIV_INSTRS_0,
    #[doc = "Adds support for the LDRBT, LDRT, STRBT, and STRT instructions."]
    UNPRIV_INSTRS_1,
    #[doc = "As for 1, and adds support for the LDRHT, LDRSBT, LDRSHT, and STRHT instructions."]
    UNPRIV_INSTRS_2,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl UNPRIV_INSTRSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            UNPRIV_INSTRSR::UNPRIV_INSTRS_0 => 0,
            UNPRIV_INSTRSR::UNPRIV_INSTRS_1 => 1,
            UNPRIV_INSTRSR::UNPRIV_INSTRS_2 => 2,
            UNPRIV_INSTRSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> UNPRIV_INSTRSR {
        match value {
            0 => UNPRIV_INSTRSR::UNPRIV_INSTRS_0,
            1 => UNPRIV_INSTRSR::UNPRIV_INSTRS_1,
            2 => UNPRIV_INSTRSR::UNPRIV_INSTRS_2,
            i => UNPRIV_INSTRSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `UNPRIV_INSTRS_0`"]
    #[inline]
    pub fn is_unpriv_instrs_0(&self) -> bool {
        *self == UNPRIV_INSTRSR::UNPRIV_INSTRS_0
    }
    #[doc = "Checks if the value of the field is `UNPRIV_INSTRS_1`"]
    #[inline]
    pub fn is_unpriv_instrs_1(&self) -> bool {
        *self == UNPRIV_INSTRSR::UNPRIV_INSTRS_1
    }
    #[doc = "Checks if the value of the field is `UNPRIV_INSTRS_2`"]
    #[inline]
    pub fn is_unpriv_instrs_2(&self) -> bool {
        *self == UNPRIV_INSTRSR::UNPRIV_INSTRS_2
    }
}
#[doc = "Possible values of the field `WITHSHIFTS_INSTRS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WITHSHIFTS_INSTRSR {
    #[doc = "Nonzero shifts supported only in MOV and shift instructions."]
    WITHSHIFTS_INSTRS_0,
    #[doc = "Adds support for shifts of loads and stores over the range LSL 0-3."]
    WITHSHIFTS_INSTRS_1,
    #[doc = "As for 1, and adds support for other constant shift options, on loads, stores, and other instructions."]
    WITHSHIFTS_INSTRS_3,
    #[doc = "ARMv7-M unused."]
    WITHSHIFTS_INSTRS_4,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl WITHSHIFTS_INSTRSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WITHSHIFTS_INSTRSR::WITHSHIFTS_INSTRS_0 => 0,
            WITHSHIFTS_INSTRSR::WITHSHIFTS_INSTRS_1 => 1,
            WITHSHIFTS_INSTRSR::WITHSHIFTS_INSTRS_3 => 3,
            WITHSHIFTS_INSTRSR::WITHSHIFTS_INSTRS_4 => 4,
            WITHSHIFTS_INSTRSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WITHSHIFTS_INSTRSR {
        match value {
            0 => WITHSHIFTS_INSTRSR::WITHSHIFTS_INSTRS_0,
            1 => WITHSHIFTS_INSTRSR::WITHSHIFTS_INSTRS_1,
            3 => WITHSHIFTS_INSTRSR::WITHSHIFTS_INSTRS_3,
            4 => WITHSHIFTS_INSTRSR::WITHSHIFTS_INSTRS_4,
            i => WITHSHIFTS_INSTRSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `WITHSHIFTS_INSTRS_0`"]
    #[inline]
    pub fn is_withshifts_instrs_0(&self) -> bool {
        *self == WITHSHIFTS_INSTRSR::WITHSHIFTS_INSTRS_0
    }
    #[doc = "Checks if the value of the field is `WITHSHIFTS_INSTRS_1`"]
    #[inline]
    pub fn is_withshifts_instrs_1(&self) -> bool {
        *self == WITHSHIFTS_INSTRSR::WITHSHIFTS_INSTRS_1
    }
    #[doc = "Checks if the value of the field is `WITHSHIFTS_INSTRS_3`"]
    #[inline]
    pub fn is_withshifts_instrs_3(&self) -> bool {
        *self == WITHSHIFTS_INSTRSR::WITHSHIFTS_INSTRS_3
    }
    #[doc = "Checks if the value of the field is `WITHSHIFTS_INSTRS_4`"]
    #[inline]
    pub fn is_withshifts_instrs_4(&self) -> bool {
        *self == WITHSHIFTS_INSTRSR::WITHSHIFTS_INSTRS_4
    }
}
#[doc = "Possible values of the field `WRITEBACK_INSTRS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WRITEBACK_INSTRSR {
    #[doc = "Basic support. Only the LDM, STM, PUSH, and POP instructions support writeback addressing modes. ARMv7-M unused."]
    WRITEBACK_INSTRS_0,
    #[doc = "Adds support for all of the writeback addressing modes defined in the ARMv7-M architecture."]
    WRITEBACK_INSTRS_1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl WRITEBACK_INSTRSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WRITEBACK_INSTRSR::WRITEBACK_INSTRS_0 => 0,
            WRITEBACK_INSTRSR::WRITEBACK_INSTRS_1 => 1,
            WRITEBACK_INSTRSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WRITEBACK_INSTRSR {
        match value {
            0 => WRITEBACK_INSTRSR::WRITEBACK_INSTRS_0,
            1 => WRITEBACK_INSTRSR::WRITEBACK_INSTRS_1,
            i => WRITEBACK_INSTRSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `WRITEBACK_INSTRS_0`"]
    #[inline]
    pub fn is_writeback_instrs_0(&self) -> bool {
        *self == WRITEBACK_INSTRSR::WRITEBACK_INSTRS_0
    }
    #[doc = "Checks if the value of the field is `WRITEBACK_INSTRS_1`"]
    #[inline]
    pub fn is_writeback_instrs_1(&self) -> bool {
        *self == WRITEBACK_INSTRSR::WRITEBACK_INSTRS_1
    }
}
#[doc = "Possible values of the field `BARRIER_INSTRS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BARRIER_INSTRSR {
    #[doc = "None supported, ARMv7-M unused."]
    BARRIER_INSTRS_0,
    #[doc = "Adds support for the DMB, DSB, and ISB barrier instructions."]
    BARRIER_INSTRS_1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl BARRIER_INSTRSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BARRIER_INSTRSR::BARRIER_INSTRS_0 => 0,
            BARRIER_INSTRSR::BARRIER_INSTRS_1 => 1,
            BARRIER_INSTRSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BARRIER_INSTRSR {
        match value {
            0 => BARRIER_INSTRSR::BARRIER_INSTRS_0,
            1 => BARRIER_INSTRSR::BARRIER_INSTRS_1,
            i => BARRIER_INSTRSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `BARRIER_INSTRS_0`"]
    #[inline]
    pub fn is_barrier_instrs_0(&self) -> bool {
        *self == BARRIER_INSTRSR::BARRIER_INSTRS_0
    }
    #[doc = "Checks if the value of the field is `BARRIER_INSTRS_1`"]
    #[inline]
    pub fn is_barrier_instrs_1(&self) -> bool {
        *self == BARRIER_INSTRSR::BARRIER_INSTRS_1
    }
}
#[doc = r" Value of the field"]
pub struct SYNCHPRIM_INSTRS_FRACR {
    bits: u8,
}
impl SYNCHPRIM_INSTRS_FRACR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `PSR_M_INSTRS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PSR_M_INSTRSR {
    #[doc = "None supported, ARMv7-M unused."]
    PSR_M_INSTRS_0,
    #[doc = "Adds support for the M-profile forms of the CPS, MRS, and MSR instructions, to access the PSRs."]
    PSR_M_INSTRS_1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PSR_M_INSTRSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PSR_M_INSTRSR::PSR_M_INSTRS_0 => 0,
            PSR_M_INSTRSR::PSR_M_INSTRS_1 => 1,
            PSR_M_INSTRSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PSR_M_INSTRSR {
        match value {
            0 => PSR_M_INSTRSR::PSR_M_INSTRS_0,
            1 => PSR_M_INSTRSR::PSR_M_INSTRS_1,
            i => PSR_M_INSTRSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PSR_M_INSTRS_0`"]
    #[inline]
    pub fn is_psr_m_instrs_0(&self) -> bool {
        *self == PSR_M_INSTRSR::PSR_M_INSTRS_0
    }
    #[doc = "Checks if the value of the field is `PSR_M_INSTRS_1`"]
    #[inline]
    pub fn is_psr_m_instrs_1(&self) -> bool {
        *self == PSR_M_INSTRSR::PSR_M_INSTRS_1
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - Indicates the supported unprivileged instructions. These are the instruction variants indicated by a T suffix."]
    #[inline]
    pub fn unpriv_instrs(&self) -> UNPRIV_INSTRSR {
        UNPRIV_INSTRSR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:7 - Indicates the support for instructions with shifts"]
    #[inline]
    pub fn withshifts_instrs(&self) -> WITHSHIFTS_INSTRSR {
        WITHSHIFTS_INSTRSR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:11 - Indicates the support for Writeback addressing modes"]
    #[inline]
    pub fn writeback_instrs(&self) -> WRITEBACK_INSTRSR {
        WRITEBACK_INSTRSR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:19 - Indicates the supported Barrier instructions"]
    #[inline]
    pub fn barrier_instrs(&self) -> BARRIER_INSTRSR {
        BARRIER_INSTRSR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:23 - Together with the ID_ISAR3[SYNCHPRIM_INSTRS] indicates the supported Synchronization Primitives"]
    #[inline]
    pub fn synchprim_instrs_frac(&self) -> SYNCHPRIM_INSTRS_FRACR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SYNCHPRIM_INSTRS_FRACR { bits }
    }
    #[doc = "Bits 24:27 - Indicates the supported M profile instructions to modify the PSRs"]
    #[inline]
    pub fn psr_m_instrs(&self) -> PSR_M_INSTRSR {
        PSR_M_INSTRSR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
