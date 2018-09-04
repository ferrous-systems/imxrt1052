#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CSSELR {
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
#[doc = "Possible values of the field `IND`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INDR {
    #[doc = "Data or unified cache."]
    IND_0,
    #[doc = "Instruction cache."]
    IND_1,
}
impl INDR {
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
            INDR::IND_0 => false,
            INDR::IND_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INDR {
        match value {
            false => INDR::IND_0,
            true => INDR::IND_1,
        }
    }
    #[doc = "Checks if the value of the field is `IND_0`"]
    #[inline]
    pub fn is_ind_0(&self) -> bool {
        *self == INDR::IND_0
    }
    #[doc = "Checks if the value of the field is `IND_1`"]
    #[inline]
    pub fn is_ind_1(&self) -> bool {
        *self == INDR::IND_1
    }
}
#[doc = "Possible values of the field `LEVEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LEVELR {
    #[doc = "Level 1 cache."]
    LEVEL_0,
    #[doc = "Level 2 cache."]
    LEVEL_1,
    #[doc = "Level 3 cache."]
    LEVEL_2,
    #[doc = "Level 4 cache."]
    LEVEL_3,
    #[doc = "Level 5 cache."]
    LEVEL_4,
    #[doc = "Level 6 cache."]
    LEVEL_5,
    #[doc = "Level 7 cache."]
    LEVEL_6,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl LEVELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LEVELR::LEVEL_0 => 0,
            LEVELR::LEVEL_1 => 1,
            LEVELR::LEVEL_2 => 2,
            LEVELR::LEVEL_3 => 3,
            LEVELR::LEVEL_4 => 4,
            LEVELR::LEVEL_5 => 5,
            LEVELR::LEVEL_6 => 6,
            LEVELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LEVELR {
        match value {
            0 => LEVELR::LEVEL_0,
            1 => LEVELR::LEVEL_1,
            2 => LEVELR::LEVEL_2,
            3 => LEVELR::LEVEL_3,
            4 => LEVELR::LEVEL_4,
            5 => LEVELR::LEVEL_5,
            6 => LEVELR::LEVEL_6,
            i => LEVELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL_0`"]
    #[inline]
    pub fn is_level_0(&self) -> bool {
        *self == LEVELR::LEVEL_0
    }
    #[doc = "Checks if the value of the field is `LEVEL_1`"]
    #[inline]
    pub fn is_level_1(&self) -> bool {
        *self == LEVELR::LEVEL_1
    }
    #[doc = "Checks if the value of the field is `LEVEL_2`"]
    #[inline]
    pub fn is_level_2(&self) -> bool {
        *self == LEVELR::LEVEL_2
    }
    #[doc = "Checks if the value of the field is `LEVEL_3`"]
    #[inline]
    pub fn is_level_3(&self) -> bool {
        *self == LEVELR::LEVEL_3
    }
    #[doc = "Checks if the value of the field is `LEVEL_4`"]
    #[inline]
    pub fn is_level_4(&self) -> bool {
        *self == LEVELR::LEVEL_4
    }
    #[doc = "Checks if the value of the field is `LEVEL_5`"]
    #[inline]
    pub fn is_level_5(&self) -> bool {
        *self == LEVELR::LEVEL_5
    }
    #[doc = "Checks if the value of the field is `LEVEL_6`"]
    #[inline]
    pub fn is_level_6(&self) -> bool {
        *self == LEVELR::LEVEL_6
    }
}
#[doc = "Values that can be written to the field `IND`"]
pub enum INDW {
    #[doc = "Data or unified cache."]
    IND_0,
    #[doc = "Instruction cache."]
    IND_1,
}
impl INDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INDW::IND_0 => false,
            INDW::IND_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INDW<'a> {
    w: &'a mut W,
}
impl<'a> _INDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Data or unified cache."]
    #[inline]
    pub fn ind_0(self) -> &'a mut W {
        self.variant(INDW::IND_0)
    }
    #[doc = "Instruction cache."]
    #[inline]
    pub fn ind_1(self) -> &'a mut W {
        self.variant(INDW::IND_1)
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
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LEVEL`"]
pub enum LEVELW {
    #[doc = "Level 1 cache."]
    LEVEL_0,
    #[doc = "Level 2 cache."]
    LEVEL_1,
    #[doc = "Level 3 cache."]
    LEVEL_2,
    #[doc = "Level 4 cache."]
    LEVEL_3,
    #[doc = "Level 5 cache."]
    LEVEL_4,
    #[doc = "Level 6 cache."]
    LEVEL_5,
    #[doc = "Level 7 cache."]
    LEVEL_6,
}
impl LEVELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LEVELW::LEVEL_0 => 0,
            LEVELW::LEVEL_1 => 1,
            LEVELW::LEVEL_2 => 2,
            LEVELW::LEVEL_3 => 3,
            LEVELW::LEVEL_4 => 4,
            LEVELW::LEVEL_5 => 5,
            LEVELW::LEVEL_6 => 6,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LEVELW<'a> {
    w: &'a mut W,
}
impl<'a> _LEVELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LEVELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Level 1 cache."]
    #[inline]
    pub fn level_0(self) -> &'a mut W {
        self.variant(LEVELW::LEVEL_0)
    }
    #[doc = "Level 2 cache."]
    #[inline]
    pub fn level_1(self) -> &'a mut W {
        self.variant(LEVELW::LEVEL_1)
    }
    #[doc = "Level 3 cache."]
    #[inline]
    pub fn level_2(self) -> &'a mut W {
        self.variant(LEVELW::LEVEL_2)
    }
    #[doc = "Level 4 cache."]
    #[inline]
    pub fn level_3(self) -> &'a mut W {
        self.variant(LEVELW::LEVEL_3)
    }
    #[doc = "Level 5 cache."]
    #[inline]
    pub fn level_4(self) -> &'a mut W {
        self.variant(LEVELW::LEVEL_4)
    }
    #[doc = "Level 6 cache."]
    #[inline]
    pub fn level_5(self) -> &'a mut W {
        self.variant(LEVELW::LEVEL_5)
    }
    #[doc = "Level 7 cache."]
    #[inline]
    pub fn level_6(self) -> &'a mut W {
        self.variant(LEVELW::LEVEL_6)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 1;
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
    #[doc = "Bit 0 - Instruction not data bit"]
    #[inline]
    pub fn ind(&self) -> INDR {
        INDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 1:3 - Cache level of required cache"]
    #[inline]
    pub fn level(&self) -> LEVELR {
        LEVELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 1;
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
    #[doc = "Bit 0 - Instruction not data bit"]
    #[inline]
    pub fn ind(&mut self) -> _INDW {
        _INDW { w: self }
    }
    #[doc = "Bits 1:3 - Cache level of required cache"]
    #[inline]
    pub fn level(&mut self) -> _LEVELW {
        _LEVELW { w: self }
    }
}
