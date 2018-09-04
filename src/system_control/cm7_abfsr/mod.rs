#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CM7_ABFSR {
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
#[doc = r" Value of the field"]
pub struct ITCMR {
    bits: bool,
}
impl ITCMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct DTCMR {
    bits: bool,
}
impl DTCMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct AHBPR {
    bits: bool,
}
impl AHBPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct AXIMR {
    bits: bool,
}
impl AXIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct EPPBR {
    bits: bool,
}
impl EPPBR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = "Possible values of the field `AXIMTYPE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AXIMTYPER {
    #[doc = "OKAY."]
    AXIMTYPE_0,
    #[doc = "EXOKAY."]
    AXIMTYPE_1,
    #[doc = "SLVERR."]
    AXIMTYPE_2,
    #[doc = "DECERR."]
    AXIMTYPE_3,
}
impl AXIMTYPER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            AXIMTYPER::AXIMTYPE_0 => 0,
            AXIMTYPER::AXIMTYPE_1 => 1,
            AXIMTYPER::AXIMTYPE_2 => 2,
            AXIMTYPER::AXIMTYPE_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> AXIMTYPER {
        match value {
            0 => AXIMTYPER::AXIMTYPE_0,
            1 => AXIMTYPER::AXIMTYPE_1,
            2 => AXIMTYPER::AXIMTYPE_2,
            3 => AXIMTYPER::AXIMTYPE_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `AXIMTYPE_0`"]
    #[inline]
    pub fn is_aximtype_0(&self) -> bool {
        *self == AXIMTYPER::AXIMTYPE_0
    }
    #[doc = "Checks if the value of the field is `AXIMTYPE_1`"]
    #[inline]
    pub fn is_aximtype_1(&self) -> bool {
        *self == AXIMTYPER::AXIMTYPE_1
    }
    #[doc = "Checks if the value of the field is `AXIMTYPE_2`"]
    #[inline]
    pub fn is_aximtype_2(&self) -> bool {
        *self == AXIMTYPER::AXIMTYPE_2
    }
    #[doc = "Checks if the value of the field is `AXIMTYPE_3`"]
    #[inline]
    pub fn is_aximtype_3(&self) -> bool {
        *self == AXIMTYPER::AXIMTYPE_3
    }
}
#[doc = r" Proxy"]
pub struct _ITCMW<'a> {
    w: &'a mut W,
}
impl<'a> _ITCMW<'a> {
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
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DTCMW<'a> {
    w: &'a mut W,
}
impl<'a> _DTCMW<'a> {
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
#[doc = r" Proxy"]
pub struct _AHBPW<'a> {
    w: &'a mut W,
}
impl<'a> _AHBPW<'a> {
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _AXIMW<'a> {
    w: &'a mut W,
}
impl<'a> _AXIMW<'a> {
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _EPPBW<'a> {
    w: &'a mut W,
}
impl<'a> _EPPBW<'a> {
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AXIMTYPE`"]
pub enum AXIMTYPEW {
    #[doc = "OKAY."]
    AXIMTYPE_0,
    #[doc = "EXOKAY."]
    AXIMTYPE_1,
    #[doc = "SLVERR."]
    AXIMTYPE_2,
    #[doc = "DECERR."]
    AXIMTYPE_3,
}
impl AXIMTYPEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            AXIMTYPEW::AXIMTYPE_0 => 0,
            AXIMTYPEW::AXIMTYPE_1 => 1,
            AXIMTYPEW::AXIMTYPE_2 => 2,
            AXIMTYPEW::AXIMTYPE_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AXIMTYPEW<'a> {
    w: &'a mut W,
}
impl<'a> _AXIMTYPEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AXIMTYPEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "OKAY."]
    #[inline]
    pub fn aximtype_0(self) -> &'a mut W {
        self.variant(AXIMTYPEW::AXIMTYPE_0)
    }
    #[doc = "EXOKAY."]
    #[inline]
    pub fn aximtype_1(self) -> &'a mut W {
        self.variant(AXIMTYPEW::AXIMTYPE_1)
    }
    #[doc = "SLVERR."]
    #[inline]
    pub fn aximtype_2(self) -> &'a mut W {
        self.variant(AXIMTYPEW::AXIMTYPE_2)
    }
    #[doc = "DECERR."]
    #[inline]
    pub fn aximtype_3(self) -> &'a mut W {
        self.variant(AXIMTYPEW::AXIMTYPE_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
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
    #[doc = "Bit 0 - Asynchronous fault on ITCM interface."]
    #[inline]
    pub fn itcm(&self) -> ITCMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ITCMR { bits }
    }
    #[doc = "Bit 1 - Asynchronous fault on DTCM interface."]
    #[inline]
    pub fn dtcm(&self) -> DTCMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DTCMR { bits }
    }
    #[doc = "Bit 2 - Asynchronous fault on AHBP interface."]
    #[inline]
    pub fn ahbp(&self) -> AHBPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AHBPR { bits }
    }
    #[doc = "Bit 3 - Asynchronous fault on AXIM interface."]
    #[inline]
    pub fn axim(&self) -> AXIMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AXIMR { bits }
    }
    #[doc = "Bit 4 - Asynchronous fault on EPPB interface."]
    #[inline]
    pub fn eppb(&self) -> EPPBR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EPPBR { bits }
    }
    #[doc = "Bits 8:9 - Indicates the type of fault on the AXIM interface. Only valid when AXIM is 1."]
    #[inline]
    pub fn aximtype(&self) -> AXIMTYPER {
        AXIMTYPER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
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
    #[doc = "Bit 0 - Asynchronous fault on ITCM interface."]
    #[inline]
    pub fn itcm(&mut self) -> _ITCMW {
        _ITCMW { w: self }
    }
    #[doc = "Bit 1 - Asynchronous fault on DTCM interface."]
    #[inline]
    pub fn dtcm(&mut self) -> _DTCMW {
        _DTCMW { w: self }
    }
    #[doc = "Bit 2 - Asynchronous fault on AHBP interface."]
    #[inline]
    pub fn ahbp(&mut self) -> _AHBPW {
        _AHBPW { w: self }
    }
    #[doc = "Bit 3 - Asynchronous fault on AXIM interface."]
    #[inline]
    pub fn axim(&mut self) -> _AXIMW {
        _AXIMW { w: self }
    }
    #[doc = "Bit 4 - Asynchronous fault on EPPB interface."]
    #[inline]
    pub fn eppb(&mut self) -> _EPPBW {
        _EPPBW { w: self }
    }
    #[doc = "Bits 8:9 - Indicates the type of fault on the AXIM interface. Only valid when AXIM is 1."]
    #[inline]
    pub fn aximtype(&mut self) -> _AXIMTYPEW {
        _AXIMTYPEW { w: self }
    }
}
