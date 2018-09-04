#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::ID_DFR0 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `DEBUGMODEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEBUGMODELR {
    #[doc = "Not supported"]
    DEBUGMODEL_0,
    #[doc = "Support for M profile Debug architecture, with memory-mapped access."]
    DEBUGMODEL_1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DEBUGMODELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DEBUGMODELR::DEBUGMODEL_0 => 0,
            DEBUGMODELR::DEBUGMODEL_1 => 1,
            DEBUGMODELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DEBUGMODELR {
        match value {
            0 => DEBUGMODELR::DEBUGMODEL_0,
            1 => DEBUGMODELR::DEBUGMODEL_1,
            i => DEBUGMODELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DEBUGMODEL_0`"]
    #[inline]
    pub fn is_debugmodel_0(&self) -> bool {
        *self == DEBUGMODELR::DEBUGMODEL_0
    }
    #[doc = "Checks if the value of the field is `DEBUGMODEL_1`"]
    #[inline]
    pub fn is_debugmodel_1(&self) -> bool {
        *self == DEBUGMODELR::DEBUGMODEL_1
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 20:23 - Support for memory-mapped debug model for M profile processors"]
    #[inline]
    pub fn debugmodel(&self) -> DEBUGMODELR {
        DEBUGMODELR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
