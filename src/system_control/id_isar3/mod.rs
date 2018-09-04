#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::ID_ISAR3 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `SATURATE_INSTRS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SATURATE_INSTRSR {
    #[doc = "None supported"]
    SATURATE_INSTRS_0,
    #[doc = "Adds support for the QADD, QDADD, QDSUB, and QSUB instructions, and for the Q bit in the PSRs."]
    SATURATE_INSTRS_1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SATURATE_INSTRSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SATURATE_INSTRSR::SATURATE_INSTRS_0 => 0,
            SATURATE_INSTRSR::SATURATE_INSTRS_1 => 1,
            SATURATE_INSTRSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SATURATE_INSTRSR {
        match value {
            0 => SATURATE_INSTRSR::SATURATE_INSTRS_0,
            1 => SATURATE_INSTRSR::SATURATE_INSTRS_1,
            i => SATURATE_INSTRSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SATURATE_INSTRS_0`"]
    #[inline]
    pub fn is_saturate_instrs_0(&self) -> bool {
        *self == SATURATE_INSTRSR::SATURATE_INSTRS_0
    }
    #[doc = "Checks if the value of the field is `SATURATE_INSTRS_1`"]
    #[inline]
    pub fn is_saturate_instrs_1(&self) -> bool {
        *self == SATURATE_INSTRSR::SATURATE_INSTRS_1
    }
}
#[doc = "Possible values of the field `SIMD_INSTRS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SIMD_INSTRSR {
    #[doc = "None supported, ARMv7-M unused."]
    SIMD_INSTRS_0,
    #[doc = "Adds support for the SSAT and USAT instructions, and for the Q bit in the PSRs."]
    SIMD_INSTRS_1,
    #[doc = "As for 1, and adds support for the PKHBT, PKHTB, QADD16, QADD8, QASX, QSUB16, QSUB8, QSAX, SADD16, SADD8, SASX, SEL, SHADD16, SHADD8, SHASX, SHSUB16, SHSUB8, SHSAX, SSAT16, SSUB16, SSUB8, SSAX, SXTAB16, SXTB16, UADD16, UADD8, UASX, UHADD16, UHADD8, UHASX, UHSUB16, UHSUB8, UHSAX, UQADD16, UQADD8, UQASX, UQSUB16, UQSUB8, UQSAX, USAD8, USADA8, USAT16, USUB16, USUB8, USAX, UXTAB16, and UXTB16 instructions. Also adds support for the GE[3:0] bits in the PSRs."]
    SIMD_INSTRS_3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SIMD_INSTRSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SIMD_INSTRSR::SIMD_INSTRS_0 => 0,
            SIMD_INSTRSR::SIMD_INSTRS_1 => 1,
            SIMD_INSTRSR::SIMD_INSTRS_3 => 3,
            SIMD_INSTRSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SIMD_INSTRSR {
        match value {
            0 => SIMD_INSTRSR::SIMD_INSTRS_0,
            1 => SIMD_INSTRSR::SIMD_INSTRS_1,
            3 => SIMD_INSTRSR::SIMD_INSTRS_3,
            i => SIMD_INSTRSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SIMD_INSTRS_0`"]
    #[inline]
    pub fn is_simd_instrs_0(&self) -> bool {
        *self == SIMD_INSTRSR::SIMD_INSTRS_0
    }
    #[doc = "Checks if the value of the field is `SIMD_INSTRS_1`"]
    #[inline]
    pub fn is_simd_instrs_1(&self) -> bool {
        *self == SIMD_INSTRSR::SIMD_INSTRS_1
    }
    #[doc = "Checks if the value of the field is `SIMD_INSTRS_3`"]
    #[inline]
    pub fn is_simd_instrs_3(&self) -> bool {
        *self == SIMD_INSTRSR::SIMD_INSTRS_3
    }
}
#[doc = "Possible values of the field `SVC_INSTRS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SVC_INSTRSR {
    #[doc = "None supported, ARMv7-M unused."]
    SVC_INSTRS_0,
    #[doc = "Adds support for the SVC instruction."]
    SVC_INSTRS_1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SVC_INSTRSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SVC_INSTRSR::SVC_INSTRS_0 => 0,
            SVC_INSTRSR::SVC_INSTRS_1 => 1,
            SVC_INSTRSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SVC_INSTRSR {
        match value {
            0 => SVC_INSTRSR::SVC_INSTRS_0,
            1 => SVC_INSTRSR::SVC_INSTRS_1,
            i => SVC_INSTRSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SVC_INSTRS_0`"]
    #[inline]
    pub fn is_svc_instrs_0(&self) -> bool {
        *self == SVC_INSTRSR::SVC_INSTRS_0
    }
    #[doc = "Checks if the value of the field is `SVC_INSTRS_1`"]
    #[inline]
    pub fn is_svc_instrs_1(&self) -> bool {
        *self == SVC_INSTRSR::SVC_INSTRS_1
    }
}
#[doc = r" Value of the field"]
pub struct SYNCHPRIM_INSTRSR {
    bits: u8,
}
impl SYNCHPRIM_INSTRSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `TABBRANCH_INSTRS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TABBRANCH_INSTRSR {
    #[doc = "None supported, ARMv7-M unused."]
    TABBRANCH_INSTRS_0,
    #[doc = "Adds support for the TBB and TBH instructions."]
    TABBRANCH_INSTRS_1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TABBRANCH_INSTRSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TABBRANCH_INSTRSR::TABBRANCH_INSTRS_0 => 0,
            TABBRANCH_INSTRSR::TABBRANCH_INSTRS_1 => 1,
            TABBRANCH_INSTRSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TABBRANCH_INSTRSR {
        match value {
            0 => TABBRANCH_INSTRSR::TABBRANCH_INSTRS_0,
            1 => TABBRANCH_INSTRSR::TABBRANCH_INSTRS_1,
            i => TABBRANCH_INSTRSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `TABBRANCH_INSTRS_0`"]
    #[inline]
    pub fn is_tabbranch_instrs_0(&self) -> bool {
        *self == TABBRANCH_INSTRSR::TABBRANCH_INSTRS_0
    }
    #[doc = "Checks if the value of the field is `TABBRANCH_INSTRS_1`"]
    #[inline]
    pub fn is_tabbranch_instrs_1(&self) -> bool {
        *self == TABBRANCH_INSTRSR::TABBRANCH_INSTRS_1
    }
}
#[doc = "Possible values of the field `THUMBCOPY_INSTRS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum THUMBCOPY_INSTRSR {
    #[doc = "None supported, ARMv7-M unused."]
    THUMBCOPY_INSTRS_0,
    #[doc = "Adds support for encoding T1 of the MOV (register) instruction copying from a low register to a low register."]
    THUMBCOPY_INSTRS_1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl THUMBCOPY_INSTRSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            THUMBCOPY_INSTRSR::THUMBCOPY_INSTRS_0 => 0,
            THUMBCOPY_INSTRSR::THUMBCOPY_INSTRS_1 => 1,
            THUMBCOPY_INSTRSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> THUMBCOPY_INSTRSR {
        match value {
            0 => THUMBCOPY_INSTRSR::THUMBCOPY_INSTRS_0,
            1 => THUMBCOPY_INSTRSR::THUMBCOPY_INSTRS_1,
            i => THUMBCOPY_INSTRSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `THUMBCOPY_INSTRS_0`"]
    #[inline]
    pub fn is_thumbcopy_instrs_0(&self) -> bool {
        *self == THUMBCOPY_INSTRSR::THUMBCOPY_INSTRS_0
    }
    #[doc = "Checks if the value of the field is `THUMBCOPY_INSTRS_1`"]
    #[inline]
    pub fn is_thumbcopy_instrs_1(&self) -> bool {
        *self == THUMBCOPY_INSTRSR::THUMBCOPY_INSTRS_1
    }
}
#[doc = "Possible values of the field `TRUENOP_INSTRS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRUENOP_INSTRSR {
    #[doc = "None supported, ARMv7-M unused."]
    TRUENOP_INSTRS_0,
    #[doc = "Adds support for encoding T1 of the MOV (register) instruction copying from a low register to a low register."]
    TRUENOP_INSTRS_1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TRUENOP_INSTRSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TRUENOP_INSTRSR::TRUENOP_INSTRS_0 => 0,
            TRUENOP_INSTRSR::TRUENOP_INSTRS_1 => 1,
            TRUENOP_INSTRSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TRUENOP_INSTRSR {
        match value {
            0 => TRUENOP_INSTRSR::TRUENOP_INSTRS_0,
            1 => TRUENOP_INSTRSR::TRUENOP_INSTRS_1,
            i => TRUENOP_INSTRSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `TRUENOP_INSTRS_0`"]
    #[inline]
    pub fn is_truenop_instrs_0(&self) -> bool {
        *self == TRUENOP_INSTRSR::TRUENOP_INSTRS_0
    }
    #[doc = "Checks if the value of the field is `TRUENOP_INSTRS_1`"]
    #[inline]
    pub fn is_truenop_instrs_1(&self) -> bool {
        *self == TRUENOP_INSTRSR::TRUENOP_INSTRS_1
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - Indicates the supported Saturate instructions"]
    #[inline]
    pub fn saturate_instrs(&self) -> SATURATE_INSTRSR {
        SATURATE_INSTRSR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:7 - Indicates the supported SIMD instructions"]
    #[inline]
    pub fn simd_instrs(&self) -> SIMD_INSTRSR {
        SIMD_INSTRSR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:11 - Indicates the supported SVC instructions"]
    #[inline]
    pub fn svc_instrs(&self) -> SVC_INSTRSR {
        SVC_INSTRSR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:15 - Together with the ID_ISAR4[SYNCHPRIM_INSTRS_FRAC] indicates the supported Synchronization Primitives"]
    #[inline]
    pub fn synchprim_instrs(&self) -> SYNCHPRIM_INSTRSR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SYNCHPRIM_INSTRSR { bits }
    }
    #[doc = "Bits 16:19 - Indicates the supported Table Branch instructions"]
    #[inline]
    pub fn tabbranch_instrs(&self) -> TABBRANCH_INSTRSR {
        TABBRANCH_INSTRSR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:23 - Indicates the supported non flag-setting MOV instructions"]
    #[inline]
    pub fn thumbcopy_instrs(&self) -> THUMBCOPY_INSTRSR {
        THUMBCOPY_INSTRSR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:27 - Indicates the supported non flag-setting MOV instructions"]
    #[inline]
    pub fn truenop_instrs(&self) -> TRUENOP_INSTRSR {
        TRUENOP_INSTRSR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
