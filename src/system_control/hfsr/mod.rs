#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::HFSR {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `VECTTBL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VECTTBLR {
    #[doc = "no BusFault on vector table read"]
    VECTTBL_0,
    #[doc = "BusFault on vector table read"]
    VECTTBL_1,
}
impl VECTTBLR {
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
            VECTTBLR::VECTTBL_0 => false,
            VECTTBLR::VECTTBL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VECTTBLR {
        match value {
            false => VECTTBLR::VECTTBL_0,
            true => VECTTBLR::VECTTBL_1,
        }
    }
    #[doc = "Checks if the value of the field is `VECTTBL_0`"]
    #[inline]
    pub fn is_vecttbl_0(&self) -> bool {
        *self == VECTTBLR::VECTTBL_0
    }
    #[doc = "Checks if the value of the field is `VECTTBL_1`"]
    #[inline]
    pub fn is_vecttbl_1(&self) -> bool {
        *self == VECTTBLR::VECTTBL_1
    }
}
#[doc = "Possible values of the field `FORCED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FORCEDR {
    #[doc = "no forced HardFault"]
    FORCED_0,
    #[doc = "forced HardFault"]
    FORCED_1,
}
impl FORCEDR {
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
            FORCEDR::FORCED_0 => false,
            FORCEDR::FORCED_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FORCEDR {
        match value {
            false => FORCEDR::FORCED_0,
            true => FORCEDR::FORCED_1,
        }
    }
    #[doc = "Checks if the value of the field is `FORCED_0`"]
    #[inline]
    pub fn is_forced_0(&self) -> bool {
        *self == FORCEDR::FORCED_0
    }
    #[doc = "Checks if the value of the field is `FORCED_1`"]
    #[inline]
    pub fn is_forced_1(&self) -> bool {
        *self == FORCEDR::FORCED_1
    }
}
#[doc = "Possible values of the field `DEBUGEVT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEBUGEVTR {
    #[doc = "No Debug event has occurred."]
    DEBUGEVT_0,
    #[doc = "Debug event has occurred. The Debug Fault Status Register has been updated."]
    DEBUGEVT_1,
}
impl DEBUGEVTR {
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
            DEBUGEVTR::DEBUGEVT_0 => false,
            DEBUGEVTR::DEBUGEVT_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DEBUGEVTR {
        match value {
            false => DEBUGEVTR::DEBUGEVT_0,
            true => DEBUGEVTR::DEBUGEVT_1,
        }
    }
    #[doc = "Checks if the value of the field is `DEBUGEVT_0`"]
    #[inline]
    pub fn is_debugevt_0(&self) -> bool {
        *self == DEBUGEVTR::DEBUGEVT_0
    }
    #[doc = "Checks if the value of the field is `DEBUGEVT_1`"]
    #[inline]
    pub fn is_debugevt_1(&self) -> bool {
        *self == DEBUGEVTR::DEBUGEVT_1
    }
}
#[doc = "Values that can be written to the field `VECTTBL`"]
pub enum VECTTBLW {
    #[doc = "no BusFault on vector table read"]
    VECTTBL_0,
    #[doc = "BusFault on vector table read"]
    VECTTBL_1,
}
impl VECTTBLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            VECTTBLW::VECTTBL_0 => false,
            VECTTBLW::VECTTBL_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VECTTBLW<'a> {
    w: &'a mut W,
}
impl<'a> _VECTTBLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VECTTBLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no BusFault on vector table read"]
    #[inline]
    pub fn vecttbl_0(self) -> &'a mut W {
        self.variant(VECTTBLW::VECTTBL_0)
    }
    #[doc = "BusFault on vector table read"]
    #[inline]
    pub fn vecttbl_1(self) -> &'a mut W {
        self.variant(VECTTBLW::VECTTBL_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FORCED`"]
pub enum FORCEDW {
    #[doc = "no forced HardFault"]
    FORCED_0,
    #[doc = "forced HardFault"]
    FORCED_1,
}
impl FORCEDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FORCEDW::FORCED_0 => false,
            FORCEDW::FORCED_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FORCEDW<'a> {
    w: &'a mut W,
}
impl<'a> _FORCEDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FORCEDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no forced HardFault"]
    #[inline]
    pub fn forced_0(self) -> &'a mut W {
        self.variant(FORCEDW::FORCED_0)
    }
    #[doc = "forced HardFault"]
    #[inline]
    pub fn forced_1(self) -> &'a mut W {
        self.variant(FORCEDW::FORCED_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DEBUGEVT`"]
pub enum DEBUGEVTW {
    #[doc = "No Debug event has occurred."]
    DEBUGEVT_0,
    #[doc = "Debug event has occurred. The Debug Fault Status Register has been updated."]
    DEBUGEVT_1,
}
impl DEBUGEVTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DEBUGEVTW::DEBUGEVT_0 => false,
            DEBUGEVTW::DEBUGEVT_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DEBUGEVTW<'a> {
    w: &'a mut W,
}
impl<'a> _DEBUGEVTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DEBUGEVTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No Debug event has occurred."]
    #[inline]
    pub fn debugevt_0(self) -> &'a mut W {
        self.variant(DEBUGEVTW::DEBUGEVT_0)
    }
    #[doc = "Debug event has occurred. The Debug Fault Status Register has been updated."]
    #[inline]
    pub fn debugevt_1(self) -> &'a mut W {
        self.variant(DEBUGEVTW::DEBUGEVT_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 31;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 1 - Indicates a BusFault on a vector table read during exception processing."]
    #[inline]
    pub fn vecttbl(&self) -> VECTTBLR {
        VECTTBLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - Indicates a forced hard fault, generated by escalation of a fault with configurable priority that cannot be handles, either because of priority or because it is disabled."]
    #[inline]
    pub fn forced(&self) -> FORCEDR {
        FORCEDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Reserved for Debug use. When writing to the register you must write 0 to this bit, otherwise behavior is Unpredictable."]
    #[inline]
    pub fn debugevt(&self) -> DEBUGEVTR {
        DEBUGEVTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 1 - Indicates a BusFault on a vector table read during exception processing."]
    #[inline]
    pub fn vecttbl(&mut self) -> _VECTTBLW {
        _VECTTBLW { w: self }
    }
    #[doc = "Bit 30 - Indicates a forced hard fault, generated by escalation of a fault with configurable priority that cannot be handles, either because of priority or because it is disabled."]
    #[inline]
    pub fn forced(&mut self) -> _FORCEDW {
        _FORCEDW { w: self }
    }
    #[doc = "Bit 31 - Reserved for Debug use. When writing to the register you must write 0 to this bit, otherwise behavior is Unpredictable."]
    #[inline]
    pub fn debugevt(&mut self) -> _DEBUGEVTW {
        _DEBUGEVTW { w: self }
    }
}
