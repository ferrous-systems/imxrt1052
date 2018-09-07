#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::ID_PFR1 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `PROGMODEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PROGMODELR {
    #[doc = "ARMv7-M unused"]
    PROGMODEL_0,
    #[doc = "Two-stack programmers' model supported"]
    PROGMODEL_2,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PROGMODELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PROGMODELR::PROGMODEL_0 => 0,
            PROGMODELR::PROGMODEL_2 => 2,
            PROGMODELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PROGMODELR {
        match value {
            0 => PROGMODELR::PROGMODEL_0,
            2 => PROGMODELR::PROGMODEL_2,
            i => PROGMODELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PROGMODEL_0`"]
    #[inline]
    pub fn is_progmodel_0(&self) -> bool {
        *self == PROGMODELR::PROGMODEL_0
    }
    #[doc = "Checks if the value of the field is `PROGMODEL_2`"]
    #[inline]
    pub fn is_progmodel_2(&self) -> bool {
        *self == PROGMODELR::PROGMODEL_2
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 8:11 - M profile programmers' model"]
    #[inline]
    pub fn progmodel(&self) -> PROGMODELR {
        PROGMODELR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
