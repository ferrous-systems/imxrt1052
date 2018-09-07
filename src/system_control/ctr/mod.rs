#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::CTR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct IMINLINER {
    bits: u8,
}
impl IMINLINER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DMINLINER {
    bits: u8,
}
impl DMINLINER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ERGR {
    bits: u8,
}
impl ERGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CWGR {
    bits: u8,
}
impl CWGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `FORMAT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FORMATR {
    #[doc = "ARMv7 format."]
    FORMAT_4,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl FORMATR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FORMATR::FORMAT_4 => 4,
            FORMATR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FORMATR {
        match value {
            4 => FORMATR::FORMAT_4,
            i => FORMATR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `FORMAT_4`"]
    #[inline]
    pub fn is_format_4(&self) -> bool {
        *self == FORMATR::FORMAT_4
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - Log2 of the number of words in the smallest cache line of all the instruction caches that are controlled by the processor."]
    #[inline]
    pub fn iminline(&self) -> IMINLINER {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        IMINLINER { bits }
    }
    #[doc = "Bits 16:19 - Log2 of the number of words in the smallest cache line of all the data caches and unified caches that are controlled by the processor."]
    #[inline]
    pub fn dminline(&self) -> DMINLINER {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DMINLINER { bits }
    }
    #[doc = "Bits 20:23 - Exclusives Reservation Granule. The maximum size of the reservation granule that has been implemented for the Load-Exclusive and Store-Exclusive instructions, encoded as Log2 of the number of words."]
    #[inline]
    pub fn erg(&self) -> ERGR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ERGR { bits }
    }
    #[doc = "Bits 24:27 - Cache Write-back Granule. The maximum size of memory that can be overwritten as a result of the eviction of a cache entry that has had a memory location in it modified, encoded as Log2 of the number of words."]
    #[inline]
    pub fn cwg(&self) -> CWGR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CWGR { bits }
    }
    #[doc = "Bits 29:31 - Indicates the implemented CTR format."]
    #[inline]
    pub fn format(&self) -> FORMATR {
        FORMATR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
