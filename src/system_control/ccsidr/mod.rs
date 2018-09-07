#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::CCSIDR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `LINESIZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINESIZER {
    #[doc = "The line length of 4 words."]
    LINESIZE_0,
    #[doc = "The line length of 8 words."]
    LINESIZE_1,
    #[doc = "The line length of 16 words."]
    LINESIZE_2,
    #[doc = "The line length of 32 words."]
    LINESIZE_3,
    #[doc = "The line length of 64 words."]
    LINESIZE_4,
    #[doc = "The line length of 128 words."]
    LINESIZE_5,
    #[doc = "The line length of 256 words."]
    LINESIZE_6,
    #[doc = "The line length of 512 words."]
    LINESIZE_7,
}
impl LINESIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LINESIZER::LINESIZE_0 => 0,
            LINESIZER::LINESIZE_1 => 1,
            LINESIZER::LINESIZE_2 => 2,
            LINESIZER::LINESIZE_3 => 3,
            LINESIZER::LINESIZE_4 => 4,
            LINESIZER::LINESIZE_5 => 5,
            LINESIZER::LINESIZE_6 => 6,
            LINESIZER::LINESIZE_7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LINESIZER {
        match value {
            0 => LINESIZER::LINESIZE_0,
            1 => LINESIZER::LINESIZE_1,
            2 => LINESIZER::LINESIZE_2,
            3 => LINESIZER::LINESIZE_3,
            4 => LINESIZER::LINESIZE_4,
            5 => LINESIZER::LINESIZE_5,
            6 => LINESIZER::LINESIZE_6,
            7 => LINESIZER::LINESIZE_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LINESIZE_0`"]
    #[inline]
    pub fn is_linesize_0(&self) -> bool {
        *self == LINESIZER::LINESIZE_0
    }
    #[doc = "Checks if the value of the field is `LINESIZE_1`"]
    #[inline]
    pub fn is_linesize_1(&self) -> bool {
        *self == LINESIZER::LINESIZE_1
    }
    #[doc = "Checks if the value of the field is `LINESIZE_2`"]
    #[inline]
    pub fn is_linesize_2(&self) -> bool {
        *self == LINESIZER::LINESIZE_2
    }
    #[doc = "Checks if the value of the field is `LINESIZE_3`"]
    #[inline]
    pub fn is_linesize_3(&self) -> bool {
        *self == LINESIZER::LINESIZE_3
    }
    #[doc = "Checks if the value of the field is `LINESIZE_4`"]
    #[inline]
    pub fn is_linesize_4(&self) -> bool {
        *self == LINESIZER::LINESIZE_4
    }
    #[doc = "Checks if the value of the field is `LINESIZE_5`"]
    #[inline]
    pub fn is_linesize_5(&self) -> bool {
        *self == LINESIZER::LINESIZE_5
    }
    #[doc = "Checks if the value of the field is `LINESIZE_6`"]
    #[inline]
    pub fn is_linesize_6(&self) -> bool {
        *self == LINESIZER::LINESIZE_6
    }
    #[doc = "Checks if the value of the field is `LINESIZE_7`"]
    #[inline]
    pub fn is_linesize_7(&self) -> bool {
        *self == LINESIZER::LINESIZE_7
    }
}
#[doc = r" Value of the field"]
pub struct ASSOCIATIVITYR {
    bits: u16,
}
impl ASSOCIATIVITYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct NUMSETSR {
    bits: u16,
}
impl NUMSETSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Possible values of the field `WA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAR {
    #[doc = "Feature not supported"]
    WA_0,
    #[doc = "Feature supported"]
    WA_1,
}
impl WAR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            WAR::WA_0 => false,
            WAR::WA_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WAR {
        match value {
            false => WAR::WA_0,
            true => WAR::WA_1,
        }
    }
    #[doc = "Checks if the value of the field is `WA_0`"]
    #[inline]
    pub fn is_wa_0(&self) -> bool {
        *self == WAR::WA_0
    }
    #[doc = "Checks if the value of the field is `WA_1`"]
    #[inline]
    pub fn is_wa_1(&self) -> bool {
        *self == WAR::WA_1
    }
}
#[doc = "Possible values of the field `RA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAR {
    #[doc = "Feature not supported"]
    RA_0,
    #[doc = "Feature supported"]
    RA_1,
}
impl RAR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            RAR::RA_0 => false,
            RAR::RA_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RAR {
        match value {
            false => RAR::RA_0,
            true => RAR::RA_1,
        }
    }
    #[doc = "Checks if the value of the field is `RA_0`"]
    #[inline]
    pub fn is_ra_0(&self) -> bool {
        *self == RAR::RA_0
    }
    #[doc = "Checks if the value of the field is `RA_1`"]
    #[inline]
    pub fn is_ra_1(&self) -> bool {
        *self == RAR::RA_1
    }
}
#[doc = "Possible values of the field `WB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WBR {
    #[doc = "Feature not supported"]
    WB_0,
    #[doc = "Feature supported"]
    WB_1,
}
impl WBR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            WBR::WB_0 => false,
            WBR::WB_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WBR {
        match value {
            false => WBR::WB_0,
            true => WBR::WB_1,
        }
    }
    #[doc = "Checks if the value of the field is `WB_0`"]
    #[inline]
    pub fn is_wb_0(&self) -> bool {
        *self == WBR::WB_0
    }
    #[doc = "Checks if the value of the field is `WB_1`"]
    #[inline]
    pub fn is_wb_1(&self) -> bool {
        *self == WBR::WB_1
    }
}
#[doc = "Possible values of the field `WT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WTR {
    #[doc = "Feature not supported"]
    WT_0,
    #[doc = "Feature supported"]
    WT_1,
}
impl WTR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            WTR::WT_0 => false,
            WTR::WT_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WTR {
        match value {
            false => WTR::WT_0,
            true => WTR::WT_1,
        }
    }
    #[doc = "Checks if the value of the field is `WT_0`"]
    #[inline]
    pub fn is_wt_0(&self) -> bool {
        *self == WTR::WT_0
    }
    #[doc = "Checks if the value of the field is `WT_1`"]
    #[inline]
    pub fn is_wt_1(&self) -> bool {
        *self == WTR::WT_1
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:2 - (Log2(Number of words in cache line)) - 2."]
    #[inline]
    pub fn linesize(&self) -> LINESIZER {
        LINESIZER::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 3:12 - (Associativity of cache) - 1, therefore a value of 0 indicates an associativity of 1. The associativity does not have to be a power of 2."]
    #[inline]
    pub fn associativity(&self) -> ASSOCIATIVITYR {
        let bits = {
            const MASK: u16 = 1023;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        ASSOCIATIVITYR { bits }
    }
    #[doc = "Bits 13:27 - (Number of sets in cache) - 1, therefore a value of 0 indicates 1 set in the cache. The number of sets does not have to be a power of 2."]
    #[inline]
    pub fn numsets(&self) -> NUMSETSR {
        let bits = {
            const MASK: u16 = 32767;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        NUMSETSR { bits }
    }
    #[doc = "Bit 28 - Indicates whether the cache level supports write-allocation"]
    #[inline]
    pub fn wa(&self) -> WAR {
        WAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - Indicates whether the cache level supports read-allocation"]
    #[inline]
    pub fn ra(&self) -> RAR {
        RAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - Indicates whether the cache level supports write-back"]
    #[inline]
    pub fn wb(&self) -> WBR {
        WBR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Indicates whether the cache level supports write-through"]
    #[inline]
    pub fn wt(&self) -> WTR {
        WTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
