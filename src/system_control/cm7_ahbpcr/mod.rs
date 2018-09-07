#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CM7_AHBPCR {
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
#[doc = "Possible values of the field `EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENR {
    #[doc = "AHBP disabled. When disabled all accesses are made to the AXIM interface."]
    EN_0,
    #[doc = "AHBP enabled."]
    EN_1,
}
impl ENR {
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
            ENR::EN_0 => false,
            ENR::EN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENR {
        match value {
            false => ENR::EN_0,
            true => ENR::EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `EN_0`"]
    #[inline]
    pub fn is_en_0(&self) -> bool {
        *self == ENR::EN_0
    }
    #[doc = "Checks if the value of the field is `EN_1`"]
    #[inline]
    pub fn is_en_1(&self) -> bool {
        *self == ENR::EN_1
    }
}
#[doc = "Possible values of the field `SZ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SZR {
    #[doc = "0MB. AHBP disabled."]
    SZ_0,
    #[doc = "64MB."]
    SZ_1,
    #[doc = "128MB."]
    SZ_2,
    #[doc = "256MB."]
    SZ_3,
    #[doc = "512MB."]
    SZ_4,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SZR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SZR::SZ_0 => 0,
            SZR::SZ_1 => 1,
            SZR::SZ_2 => 2,
            SZR::SZ_3 => 3,
            SZR::SZ_4 => 4,
            SZR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SZR {
        match value {
            0 => SZR::SZ_0,
            1 => SZR::SZ_1,
            2 => SZR::SZ_2,
            3 => SZR::SZ_3,
            4 => SZR::SZ_4,
            i => SZR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SZ_0`"]
    #[inline]
    pub fn is_sz_0(&self) -> bool {
        *self == SZR::SZ_0
    }
    #[doc = "Checks if the value of the field is `SZ_1`"]
    #[inline]
    pub fn is_sz_1(&self) -> bool {
        *self == SZR::SZ_1
    }
    #[doc = "Checks if the value of the field is `SZ_2`"]
    #[inline]
    pub fn is_sz_2(&self) -> bool {
        *self == SZR::SZ_2
    }
    #[doc = "Checks if the value of the field is `SZ_3`"]
    #[inline]
    pub fn is_sz_3(&self) -> bool {
        *self == SZR::SZ_3
    }
    #[doc = "Checks if the value of the field is `SZ_4`"]
    #[inline]
    pub fn is_sz_4(&self) -> bool {
        *self == SZR::SZ_4
    }
}
#[doc = "Values that can be written to the field `EN`"]
pub enum ENW {
    #[doc = "AHBP disabled. When disabled all accesses are made to the AXIM interface."]
    EN_0,
    #[doc = "AHBP enabled."]
    EN_1,
}
impl ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENW::EN_0 => false,
            ENW::EN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENW<'a> {
    w: &'a mut W,
}
impl<'a> _ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "AHBP disabled. When disabled all accesses are made to the AXIM interface."]
    #[inline]
    pub fn en_0(self) -> &'a mut W {
        self.variant(ENW::EN_0)
    }
    #[doc = "AHBP enabled."]
    #[inline]
    pub fn en_1(self) -> &'a mut W {
        self.variant(ENW::EN_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - AHBP enable."]
    #[inline]
    pub fn en(&self) -> ENR {
        ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 1:3 - AHBP size."]
    #[inline]
    pub fn sz(&self) -> SZR {
        SZR::_from({
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
    #[doc = "Bit 0 - AHBP enable."]
    #[inline]
    pub fn en(&mut self) -> _ENW {
        _ENW { w: self }
    }
}
