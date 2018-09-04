#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::ID_MMFR2 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `WFI_STALL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WFI_STALLR {
    #[doc = "Not supported"]
    WFI_STALL_0,
    #[doc = "Support for WFI stalling"]
    WFI_STALL_1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl WFI_STALLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WFI_STALLR::WFI_STALL_0 => 0,
            WFI_STALLR::WFI_STALL_1 => 1,
            WFI_STALLR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WFI_STALLR {
        match value {
            0 => WFI_STALLR::WFI_STALL_0,
            1 => WFI_STALLR::WFI_STALL_1,
            i => WFI_STALLR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `WFI_STALL_0`"]
    #[inline]
    pub fn is_wfi_stall_0(&self) -> bool {
        *self == WFI_STALLR::WFI_STALL_0
    }
    #[doc = "Checks if the value of the field is `WFI_STALL_1`"]
    #[inline]
    pub fn is_wfi_stall_1(&self) -> bool {
        *self == WFI_STALLR::WFI_STALL_1
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 24:27 - Indicates the support for Wait For Interrupt (WFI) stalling"]
    #[inline]
    pub fn wfi_stall(&self) -> WFI_STALLR {
        WFI_STALLR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
