#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::AIRCR {
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
pub struct PRIGROUPR {
    bits: u8,
}
impl PRIGROUPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `ENDIANNESS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDIANNESSR {
    #[doc = "Little-endian"]
    ENDIANNESS_0,
    #[doc = "Big-endian"]
    ENDIANNESS_1,
}
impl ENDIANNESSR {
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
            ENDIANNESSR::ENDIANNESS_0 => false,
            ENDIANNESSR::ENDIANNESS_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENDIANNESSR {
        match value {
            false => ENDIANNESSR::ENDIANNESS_0,
            true => ENDIANNESSR::ENDIANNESS_1,
        }
    }
    #[doc = "Checks if the value of the field is `ENDIANNESS_0`"]
    #[inline]
    pub fn is_endianness_0(&self) -> bool {
        *self == ENDIANNESSR::ENDIANNESS_0
    }
    #[doc = "Checks if the value of the field is `ENDIANNESS_1`"]
    #[inline]
    pub fn is_endianness_1(&self) -> bool {
        *self == ENDIANNESSR::ENDIANNESS_1
    }
}
#[doc = r" Value of the field"]
pub struct VECTKEYR {
    bits: u16,
}
impl VECTKEYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `VECTRESET`"]
pub enum VECTRESETW {
    #[doc = "No change"]
    VECTRESET_0,
    #[doc = "Causes a local system reset"]
    VECTRESET_1,
}
impl VECTRESETW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            VECTRESETW::VECTRESET_0 => false,
            VECTRESETW::VECTRESET_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VECTRESETW<'a> {
    w: &'a mut W,
}
impl<'a> _VECTRESETW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VECTRESETW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No change"]
    #[inline]
    pub fn vectreset_0(self) -> &'a mut W {
        self.variant(VECTRESETW::VECTRESET_0)
    }
    #[doc = "Causes a local system reset"]
    #[inline]
    pub fn vectreset_1(self) -> &'a mut W {
        self.variant(VECTRESETW::VECTRESET_1)
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
#[doc = "Values that can be written to the field `VECTCLRACTIVE`"]
pub enum VECTCLRACTIVEW {
    #[doc = "No change"]
    VECTCLRACTIVE_0,
    #[doc = "Clears all active state information for fixed and configurable exceptions"]
    VECTCLRACTIVE_1,
}
impl VECTCLRACTIVEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            VECTCLRACTIVEW::VECTCLRACTIVE_0 => false,
            VECTCLRACTIVEW::VECTCLRACTIVE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VECTCLRACTIVEW<'a> {
    w: &'a mut W,
}
impl<'a> _VECTCLRACTIVEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VECTCLRACTIVEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No change"]
    #[inline]
    pub fn vectclractive_0(self) -> &'a mut W {
        self.variant(VECTCLRACTIVEW::VECTCLRACTIVE_0)
    }
    #[doc = "Clears all active state information for fixed and configurable exceptions"]
    #[inline]
    pub fn vectclractive_1(self) -> &'a mut W {
        self.variant(VECTCLRACTIVEW::VECTCLRACTIVE_1)
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
#[doc = "Values that can be written to the field `SYSRESETREQ`"]
pub enum SYSRESETREQW {
    #[doc = "no system reset request"]
    SYSRESETREQ_0,
    #[doc = "asserts a signal to the outer system that requests a reset"]
    SYSRESETREQ_1,
}
impl SYSRESETREQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SYSRESETREQW::SYSRESETREQ_0 => false,
            SYSRESETREQW::SYSRESETREQ_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SYSRESETREQW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSRESETREQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SYSRESETREQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no system reset request"]
    #[inline]
    pub fn sysresetreq_0(self) -> &'a mut W {
        self.variant(SYSRESETREQW::SYSRESETREQ_0)
    }
    #[doc = "asserts a signal to the outer system that requests a reset"]
    #[inline]
    pub fn sysresetreq_1(self) -> &'a mut W {
        self.variant(SYSRESETREQW::SYSRESETREQ_1)
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PRIGROUPW<'a> {
    w: &'a mut W,
}
impl<'a> _PRIGROUPW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _VECTKEYW<'a> {
    w: &'a mut W,
}
impl<'a> _VECTKEYW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 65535;
        const OFFSET: u8 = 16;
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
    #[doc = "Bits 8:10 - Interrupt priority grouping field. This field determines the split of group priority from subpriority."]
    #[inline]
    pub fn prigroup(&self) -> PRIGROUPR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PRIGROUPR { bits }
    }
    #[doc = "Bit 15 - Data endianness"]
    #[inline]
    pub fn endianness(&self) -> ENDIANNESSR {
        ENDIANNESSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:31 - Register key"]
    #[inline]
    pub fn vectkey(&self) -> VECTKEYR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        VECTKEYR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 4194631680 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Writing 1 to this bit causes a local system reset"]
    #[inline]
    pub fn vectreset(&mut self) -> _VECTRESETW {
        _VECTRESETW { w: self }
    }
    #[doc = "Bit 1 - Writing 1 to this bit clears all active state information for fixed and configurable exceptions."]
    #[inline]
    pub fn vectclractive(&mut self) -> _VECTCLRACTIVEW {
        _VECTCLRACTIVEW { w: self }
    }
    #[doc = "Bit 2 - System reset request"]
    #[inline]
    pub fn sysresetreq(&mut self) -> _SYSRESETREQW {
        _SYSRESETREQW { w: self }
    }
    #[doc = "Bits 8:10 - Interrupt priority grouping field. This field determines the split of group priority from subpriority."]
    #[inline]
    pub fn prigroup(&mut self) -> _PRIGROUPW {
        _PRIGROUPW { w: self }
    }
    #[doc = "Bits 16:31 - Register key"]
    #[inline]
    pub fn vectkey(&mut self) -> _VECTKEYW {
        _VECTKEYW { w: self }
    }
}
