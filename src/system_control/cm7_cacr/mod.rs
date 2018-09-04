#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CM7_CACR {
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
#[doc = "Possible values of the field `SIWT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SIWTR {
    #[doc = "Normal Cacheable Shared locations are treated as being Non-cacheable. Default mode of operation for Shared memory."]
    SIWT_0,
    #[doc = "Normal Cacheable shared locations are treated as Write-Through."]
    SIWT_1,
}
impl SIWTR {
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
            SIWTR::SIWT_0 => false,
            SIWTR::SIWT_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SIWTR {
        match value {
            false => SIWTR::SIWT_0,
            true => SIWTR::SIWT_1,
        }
    }
    #[doc = "Checks if the value of the field is `SIWT_0`"]
    #[inline]
    pub fn is_siwt_0(&self) -> bool {
        *self == SIWTR::SIWT_0
    }
    #[doc = "Checks if the value of the field is `SIWT_1`"]
    #[inline]
    pub fn is_siwt_1(&self) -> bool {
        *self == SIWTR::SIWT_1
    }
}
#[doc = "Possible values of the field `ECCDIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ECCDISR {
    #[doc = "Enables ECC in the instruction and data cache."]
    ECCDIS_0,
    #[doc = "Disables ECC in the instruction and data cache."]
    ECCDIS_1,
}
impl ECCDISR {
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
            ECCDISR::ECCDIS_0 => false,
            ECCDISR::ECCDIS_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ECCDISR {
        match value {
            false => ECCDISR::ECCDIS_0,
            true => ECCDISR::ECCDIS_1,
        }
    }
    #[doc = "Checks if the value of the field is `ECCDIS_0`"]
    #[inline]
    pub fn is_eccdis_0(&self) -> bool {
        *self == ECCDISR::ECCDIS_0
    }
    #[doc = "Checks if the value of the field is `ECCDIS_1`"]
    #[inline]
    pub fn is_eccdis_1(&self) -> bool {
        *self == ECCDISR::ECCDIS_1
    }
}
#[doc = "Possible values of the field `FORCEWT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FORCEWTR {
    #[doc = "Disables Force Write-Through."]
    FORCEWT_0,
    #[doc = "Enables Force Write-Through. All Cacheable memory regions are treated as Write-Through."]
    FORCEWT_1,
}
impl FORCEWTR {
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
            FORCEWTR::FORCEWT_0 => false,
            FORCEWTR::FORCEWT_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FORCEWTR {
        match value {
            false => FORCEWTR::FORCEWT_0,
            true => FORCEWTR::FORCEWT_1,
        }
    }
    #[doc = "Checks if the value of the field is `FORCEWT_0`"]
    #[inline]
    pub fn is_forcewt_0(&self) -> bool {
        *self == FORCEWTR::FORCEWT_0
    }
    #[doc = "Checks if the value of the field is `FORCEWT_1`"]
    #[inline]
    pub fn is_forcewt_1(&self) -> bool {
        *self == FORCEWTR::FORCEWT_1
    }
}
#[doc = "Values that can be written to the field `SIWT`"]
pub enum SIWTW {
    #[doc = "Normal Cacheable Shared locations are treated as being Non-cacheable. Default mode of operation for Shared memory."]
    SIWT_0,
    #[doc = "Normal Cacheable shared locations are treated as Write-Through."]
    SIWT_1,
}
impl SIWTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SIWTW::SIWT_0 => false,
            SIWTW::SIWT_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SIWTW<'a> {
    w: &'a mut W,
}
impl<'a> _SIWTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SIWTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal Cacheable Shared locations are treated as being Non-cacheable. Default mode of operation for Shared memory."]
    #[inline]
    pub fn siwt_0(self) -> &'a mut W {
        self.variant(SIWTW::SIWT_0)
    }
    #[doc = "Normal Cacheable shared locations are treated as Write-Through."]
    #[inline]
    pub fn siwt_1(self) -> &'a mut W {
        self.variant(SIWTW::SIWT_1)
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
#[doc = "Values that can be written to the field `ECCDIS`"]
pub enum ECCDISW {
    #[doc = "Enables ECC in the instruction and data cache."]
    ECCDIS_0,
    #[doc = "Disables ECC in the instruction and data cache."]
    ECCDIS_1,
}
impl ECCDISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ECCDISW::ECCDIS_0 => false,
            ECCDISW::ECCDIS_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ECCDISW<'a> {
    w: &'a mut W,
}
impl<'a> _ECCDISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ECCDISW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enables ECC in the instruction and data cache."]
    #[inline]
    pub fn eccdis_0(self) -> &'a mut W {
        self.variant(ECCDISW::ECCDIS_0)
    }
    #[doc = "Disables ECC in the instruction and data cache."]
    #[inline]
    pub fn eccdis_1(self) -> &'a mut W {
        self.variant(ECCDISW::ECCDIS_1)
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
#[doc = "Values that can be written to the field `FORCEWT`"]
pub enum FORCEWTW {
    #[doc = "Disables Force Write-Through."]
    FORCEWT_0,
    #[doc = "Enables Force Write-Through. All Cacheable memory regions are treated as Write-Through."]
    FORCEWT_1,
}
impl FORCEWTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FORCEWTW::FORCEWT_0 => false,
            FORCEWTW::FORCEWT_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FORCEWTW<'a> {
    w: &'a mut W,
}
impl<'a> _FORCEWTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FORCEWTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disables Force Write-Through."]
    #[inline]
    pub fn forcewt_0(self) -> &'a mut W {
        self.variant(FORCEWTW::FORCEWT_0)
    }
    #[doc = "Enables Force Write-Through. All Cacheable memory regions are treated as Write-Through."]
    #[inline]
    pub fn forcewt_1(self) -> &'a mut W {
        self.variant(FORCEWTW::FORCEWT_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Shared cacheable-is-WT for data cache. Enables limited cache coherency usage."]
    #[inline]
    pub fn siwt(&self) -> SIWTR {
        SIWTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Enables ECC in the instruction and data cache."]
    #[inline]
    pub fn eccdis(&self) -> ECCDISR {
        ECCDISR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Enables Force Write-Through in the data cache."]
    #[inline]
    pub fn forcewt(&self) -> FORCEWTR {
        FORCEWTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
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
    #[doc = "Bit 0 - Shared cacheable-is-WT for data cache. Enables limited cache coherency usage."]
    #[inline]
    pub fn siwt(&mut self) -> _SIWTW {
        _SIWTW { w: self }
    }
    #[doc = "Bit 1 - Enables ECC in the instruction and data cache."]
    #[inline]
    pub fn eccdis(&mut self) -> _ECCDISW {
        _ECCDISW { w: self }
    }
    #[doc = "Bit 2 - Enables Force Write-Through in the data cache."]
    #[inline]
    pub fn forcewt(&mut self) -> _FORCEWTW {
        _FORCEWTW { w: self }
    }
}
