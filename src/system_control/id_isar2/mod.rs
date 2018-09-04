#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::ID_ISAR2 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `LOADSTORE_INSTRS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOADSTORE_INSTRSR {
    #[doc = "None supported, ARMv7-M unused"]
    LOADSTORE_INSTRS_0,
    #[doc = "Adds support for the LDRD and STRD instructions"]
    LOADSTORE_INSTRS_1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl LOADSTORE_INSTRSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LOADSTORE_INSTRSR::LOADSTORE_INSTRS_0 => 0,
            LOADSTORE_INSTRSR::LOADSTORE_INSTRS_1 => 1,
            LOADSTORE_INSTRSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LOADSTORE_INSTRSR {
        match value {
            0 => LOADSTORE_INSTRSR::LOADSTORE_INSTRS_0,
            1 => LOADSTORE_INSTRSR::LOADSTORE_INSTRS_1,
            i => LOADSTORE_INSTRSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOADSTORE_INSTRS_0`"]
    #[inline]
    pub fn is_loadstore_instrs_0(&self) -> bool {
        *self == LOADSTORE_INSTRSR::LOADSTORE_INSTRS_0
    }
    #[doc = "Checks if the value of the field is `LOADSTORE_INSTRS_1`"]
    #[inline]
    pub fn is_loadstore_instrs_1(&self) -> bool {
        *self == LOADSTORE_INSTRSR::LOADSTORE_INSTRS_1
    }
}
#[doc = "Possible values of the field `MEMHINT_INSTRS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MEMHINT_INSTRSR {
    #[doc = "None supported, ARMv7-M unused."]
    MEMHINT_INSTRS_0,
    #[doc = "Adds support for the PLD instruction, ARMv7-M unused."]
    MEMHINT_INSTRS_1,
    #[doc = "As for 1, ARMv7-M unused."]
    MEMHINT_INSTRS_2,
    #[doc = "As for 1 or 2, and adds support for the PLI instruction."]
    MEMHINT_INSTRS_3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MEMHINT_INSTRSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MEMHINT_INSTRSR::MEMHINT_INSTRS_0 => 0,
            MEMHINT_INSTRSR::MEMHINT_INSTRS_1 => 1,
            MEMHINT_INSTRSR::MEMHINT_INSTRS_2 => 2,
            MEMHINT_INSTRSR::MEMHINT_INSTRS_3 => 3,
            MEMHINT_INSTRSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MEMHINT_INSTRSR {
        match value {
            0 => MEMHINT_INSTRSR::MEMHINT_INSTRS_0,
            1 => MEMHINT_INSTRSR::MEMHINT_INSTRS_1,
            2 => MEMHINT_INSTRSR::MEMHINT_INSTRS_2,
            3 => MEMHINT_INSTRSR::MEMHINT_INSTRS_3,
            i => MEMHINT_INSTRSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `MEMHINT_INSTRS_0`"]
    #[inline]
    pub fn is_memhint_instrs_0(&self) -> bool {
        *self == MEMHINT_INSTRSR::MEMHINT_INSTRS_0
    }
    #[doc = "Checks if the value of the field is `MEMHINT_INSTRS_1`"]
    #[inline]
    pub fn is_memhint_instrs_1(&self) -> bool {
        *self == MEMHINT_INSTRSR::MEMHINT_INSTRS_1
    }
    #[doc = "Checks if the value of the field is `MEMHINT_INSTRS_2`"]
    #[inline]
    pub fn is_memhint_instrs_2(&self) -> bool {
        *self == MEMHINT_INSTRSR::MEMHINT_INSTRS_2
    }
    #[doc = "Checks if the value of the field is `MEMHINT_INSTRS_3`"]
    #[inline]
    pub fn is_memhint_instrs_3(&self) -> bool {
        *self == MEMHINT_INSTRSR::MEMHINT_INSTRS_3
    }
}
#[doc = "Possible values of the field `MULTIACCESSINT_INSTRS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MULTIACCESSINT_INSTRSR {
    #[doc = "None supported. This means the LDM and STM instructions are not interruptible. ARMv7-M unused."]
    MULTIACCESSINT_INSTRS_0,
    #[doc = "LDM and STM instructions are restartable."]
    MULTIACCESSINT_INSTRS_1,
    #[doc = "LDM and STM instructions are continuable."]
    MULTIACCESSINT_INSTRS_2,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MULTIACCESSINT_INSTRSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MULTIACCESSINT_INSTRSR::MULTIACCESSINT_INSTRS_0 => 0,
            MULTIACCESSINT_INSTRSR::MULTIACCESSINT_INSTRS_1 => 1,
            MULTIACCESSINT_INSTRSR::MULTIACCESSINT_INSTRS_2 => 2,
            MULTIACCESSINT_INSTRSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MULTIACCESSINT_INSTRSR {
        match value {
            0 => MULTIACCESSINT_INSTRSR::MULTIACCESSINT_INSTRS_0,
            1 => MULTIACCESSINT_INSTRSR::MULTIACCESSINT_INSTRS_1,
            2 => MULTIACCESSINT_INSTRSR::MULTIACCESSINT_INSTRS_2,
            i => MULTIACCESSINT_INSTRSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `MULTIACCESSINT_INSTRS_0`"]
    #[inline]
    pub fn is_multiaccessint_instrs_0(&self) -> bool {
        *self == MULTIACCESSINT_INSTRSR::MULTIACCESSINT_INSTRS_0
    }
    #[doc = "Checks if the value of the field is `MULTIACCESSINT_INSTRS_1`"]
    #[inline]
    pub fn is_multiaccessint_instrs_1(&self) -> bool {
        *self == MULTIACCESSINT_INSTRSR::MULTIACCESSINT_INSTRS_1
    }
    #[doc = "Checks if the value of the field is `MULTIACCESSINT_INSTRS_2`"]
    #[inline]
    pub fn is_multiaccessint_instrs_2(&self) -> bool {
        *self == MULTIACCESSINT_INSTRSR::MULTIACCESSINT_INSTRS_2
    }
}
#[doc = "Possible values of the field `MULT_INSTRS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MULT_INSTRSR {
    #[doc = "None supported. This means only MUL is supported. ARMv7-M unused."]
    MULT_INSTRS_0,
    #[doc = "Adds support for the MLA instruction, ARMv7-M unused."]
    MULT_INSTRS_1,
    #[doc = "As for 1, and adds support for the MLS instruction."]
    MULT_INSTRS_2,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MULT_INSTRSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MULT_INSTRSR::MULT_INSTRS_0 => 0,
            MULT_INSTRSR::MULT_INSTRS_1 => 1,
            MULT_INSTRSR::MULT_INSTRS_2 => 2,
            MULT_INSTRSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MULT_INSTRSR {
        match value {
            0 => MULT_INSTRSR::MULT_INSTRS_0,
            1 => MULT_INSTRSR::MULT_INSTRS_1,
            2 => MULT_INSTRSR::MULT_INSTRS_2,
            i => MULT_INSTRSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `MULT_INSTRS_0`"]
    #[inline]
    pub fn is_mult_instrs_0(&self) -> bool {
        *self == MULT_INSTRSR::MULT_INSTRS_0
    }
    #[doc = "Checks if the value of the field is `MULT_INSTRS_1`"]
    #[inline]
    pub fn is_mult_instrs_1(&self) -> bool {
        *self == MULT_INSTRSR::MULT_INSTRS_1
    }
    #[doc = "Checks if the value of the field is `MULT_INSTRS_2`"]
    #[inline]
    pub fn is_mult_instrs_2(&self) -> bool {
        *self == MULT_INSTRSR::MULT_INSTRS_2
    }
}
#[doc = "Possible values of the field `MULTS_INSTRS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MULTS_INSTRSR {
    #[doc = "None supported, ARMv7-M unused"]
    MULTS_INSTRS_0,
    #[doc = "Adds support for the SMULL and SMLAL instructions"]
    MULTS_INSTRS_1,
    #[doc = "As for 1, and adds support for the SMLABB, SMLABT, SMLALBB, SMLALBT, SMLALTB, SMLALTT, SMLATB, SMLATT, SMLAWB, SMLAWT, SMULBB, SMULBT, SMULTB, SMULTT, SMULWB, and SMULWT instructions."]
    MULTS_INSTRS_2,
    #[doc = "As for 2, and adds support for the SMLAD, SMLADX, SMLALD, SMLALDX, SMLSD, SMLSDX, SMLSLD, SMLSLDX, SMMLA, SMMLAR, SMMLS, SMMLSR, SMMUL, SMMULR, SMUAD, SMUADX, SMUSD, and SMUSDX instructions."]
    MULTS_INSTRS_3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MULTS_INSTRSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MULTS_INSTRSR::MULTS_INSTRS_0 => 0,
            MULTS_INSTRSR::MULTS_INSTRS_1 => 1,
            MULTS_INSTRSR::MULTS_INSTRS_2 => 2,
            MULTS_INSTRSR::MULTS_INSTRS_3 => 3,
            MULTS_INSTRSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MULTS_INSTRSR {
        match value {
            0 => MULTS_INSTRSR::MULTS_INSTRS_0,
            1 => MULTS_INSTRSR::MULTS_INSTRS_1,
            2 => MULTS_INSTRSR::MULTS_INSTRS_2,
            3 => MULTS_INSTRSR::MULTS_INSTRS_3,
            i => MULTS_INSTRSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `MULTS_INSTRS_0`"]
    #[inline]
    pub fn is_mults_instrs_0(&self) -> bool {
        *self == MULTS_INSTRSR::MULTS_INSTRS_0
    }
    #[doc = "Checks if the value of the field is `MULTS_INSTRS_1`"]
    #[inline]
    pub fn is_mults_instrs_1(&self) -> bool {
        *self == MULTS_INSTRSR::MULTS_INSTRS_1
    }
    #[doc = "Checks if the value of the field is `MULTS_INSTRS_2`"]
    #[inline]
    pub fn is_mults_instrs_2(&self) -> bool {
        *self == MULTS_INSTRSR::MULTS_INSTRS_2
    }
    #[doc = "Checks if the value of the field is `MULTS_INSTRS_3`"]
    #[inline]
    pub fn is_mults_instrs_3(&self) -> bool {
        *self == MULTS_INSTRSR::MULTS_INSTRS_3
    }
}
#[doc = "Possible values of the field `MULTU_INSTRS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MULTU_INSTRSR {
    #[doc = "None supported, ARMv7-M unused"]
    MULTU_INSTRS_0,
    #[doc = "Adds support for the UMULL and UMLAL instructions."]
    MULTU_INSTRS_1,
    #[doc = "As for 1, and adds support for the UMAAL instruction."]
    MULTU_INSTRS_2,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MULTU_INSTRSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MULTU_INSTRSR::MULTU_INSTRS_0 => 0,
            MULTU_INSTRSR::MULTU_INSTRS_1 => 1,
            MULTU_INSTRSR::MULTU_INSTRS_2 => 2,
            MULTU_INSTRSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MULTU_INSTRSR {
        match value {
            0 => MULTU_INSTRSR::MULTU_INSTRS_0,
            1 => MULTU_INSTRSR::MULTU_INSTRS_1,
            2 => MULTU_INSTRSR::MULTU_INSTRS_2,
            i => MULTU_INSTRSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `MULTU_INSTRS_0`"]
    #[inline]
    pub fn is_multu_instrs_0(&self) -> bool {
        *self == MULTU_INSTRSR::MULTU_INSTRS_0
    }
    #[doc = "Checks if the value of the field is `MULTU_INSTRS_1`"]
    #[inline]
    pub fn is_multu_instrs_1(&self) -> bool {
        *self == MULTU_INSTRSR::MULTU_INSTRS_1
    }
    #[doc = "Checks if the value of the field is `MULTU_INSTRS_2`"]
    #[inline]
    pub fn is_multu_instrs_2(&self) -> bool {
        *self == MULTU_INSTRSR::MULTU_INSTRS_2
    }
}
#[doc = "Possible values of the field `REVERSAL_INSTRS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REVERSAL_INSTRSR {
    #[doc = "None supported, ARMv7-M unused"]
    REVERSAL_INSTRS_0,
    #[doc = "Adds support for the REV, REV16, and REVSH instructions, ARMv7-M unused."]
    REVERSAL_INSTRS_1,
    #[doc = "As for 1, and adds support for the RBIT instruction."]
    REVERSAL_INSTRS_2,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl REVERSAL_INSTRSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            REVERSAL_INSTRSR::REVERSAL_INSTRS_0 => 0,
            REVERSAL_INSTRSR::REVERSAL_INSTRS_1 => 1,
            REVERSAL_INSTRSR::REVERSAL_INSTRS_2 => 2,
            REVERSAL_INSTRSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> REVERSAL_INSTRSR {
        match value {
            0 => REVERSAL_INSTRSR::REVERSAL_INSTRS_0,
            1 => REVERSAL_INSTRSR::REVERSAL_INSTRS_1,
            2 => REVERSAL_INSTRSR::REVERSAL_INSTRS_2,
            i => REVERSAL_INSTRSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `REVERSAL_INSTRS_0`"]
    #[inline]
    pub fn is_reversal_instrs_0(&self) -> bool {
        *self == REVERSAL_INSTRSR::REVERSAL_INSTRS_0
    }
    #[doc = "Checks if the value of the field is `REVERSAL_INSTRS_1`"]
    #[inline]
    pub fn is_reversal_instrs_1(&self) -> bool {
        *self == REVERSAL_INSTRSR::REVERSAL_INSTRS_1
    }
    #[doc = "Checks if the value of the field is `REVERSAL_INSTRS_2`"]
    #[inline]
    pub fn is_reversal_instrs_2(&self) -> bool {
        *self == REVERSAL_INSTRSR::REVERSAL_INSTRS_2
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - Indicates the supported additional load and store instructions"]
    #[inline]
    pub fn loadstore_instrs(&self) -> LOADSTORE_INSTRSR {
        LOADSTORE_INSTRSR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:7 - Indicates the supported Memory Hint instructions"]
    #[inline]
    pub fn memhint_instrs(&self) -> MEMHINT_INSTRSR {
        MEMHINT_INSTRSR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:11 - Indicates the support for multi-access interruptible instructions"]
    #[inline]
    pub fn multiaccessint_instrs(&self) -> MULTIACCESSINT_INSTRSR {
        MULTIACCESSINT_INSTRSR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:15 - Indicates the supported additional Multiply instructions"]
    #[inline]
    pub fn mult_instrs(&self) -> MULT_INSTRSR {
        MULT_INSTRSR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:19 - Indicates the supported advanced signed Multiply instructions"]
    #[inline]
    pub fn mults_instrs(&self) -> MULTS_INSTRSR {
        MULTS_INSTRSR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:23 - Indicates the supported advanced unsigned Multiply instructions"]
    #[inline]
    pub fn multu_instrs(&self) -> MULTU_INSTRSR {
        MULTU_INSTRSR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 28:31 - Indicates the supported Reversal instructions"]
    #[inline]
    pub fn reversal_instrs(&self) -> REVERSAL_INSTRSR {
        REVERSAL_INSTRSR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
