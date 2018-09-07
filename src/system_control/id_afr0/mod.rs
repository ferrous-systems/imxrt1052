#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::ID_AFR0 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct IMPLEMENTATION_DEFINED0R {
    bits: u8,
}
impl IMPLEMENTATION_DEFINED0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct IMPLEMENTATION_DEFINED1R {
    bits: u8,
}
impl IMPLEMENTATION_DEFINED1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct IMPLEMENTATION_DEFINED2R {
    bits: u8,
}
impl IMPLEMENTATION_DEFINED2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct IMPLEMENTATION_DEFINED3R {
    bits: u8,
}
impl IMPLEMENTATION_DEFINED3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - Gives information about the IMPLEMENTATION DEFINED features of a processor implementation."]
    #[inline]
    pub fn implementation_defined0(&self) -> IMPLEMENTATION_DEFINED0R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        IMPLEMENTATION_DEFINED0R { bits }
    }
    #[doc = "Bits 4:7 - Gives information about the IMPLEMENTATION DEFINED features of a processor implementation."]
    #[inline]
    pub fn implementation_defined1(&self) -> IMPLEMENTATION_DEFINED1R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        IMPLEMENTATION_DEFINED1R { bits }
    }
    #[doc = "Bits 8:11 - Gives information about the IMPLEMENTATION DEFINED features of a processor implementation."]
    #[inline]
    pub fn implementation_defined2(&self) -> IMPLEMENTATION_DEFINED2R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        IMPLEMENTATION_DEFINED2R { bits }
    }
    #[doc = "Bits 12:15 - Gives information about the IMPLEMENTATION DEFINED features of a processor implementation."]
    #[inline]
    pub fn implementation_defined3(&self) -> IMPLEMENTATION_DEFINED3R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        IMPLEMENTATION_DEFINED3R { bits }
    }
}
