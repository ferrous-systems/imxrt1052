#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CM7_DTCMCR {
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
    #[doc = "TCM disabled."]
    EN_0,
    #[doc = "TCM enabled."]
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
#[doc = "Possible values of the field `RMW`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RMWR {
    #[doc = "RMW disabled."]
    RMW_0,
    #[doc = "RMW enabled."]
    RMW_1,
}
impl RMWR {
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
            RMWR::RMW_0 => false,
            RMWR::RMW_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RMWR {
        match value {
            false => RMWR::RMW_0,
            true => RMWR::RMW_1,
        }
    }
    #[doc = "Checks if the value of the field is `RMW_0`"]
    #[inline]
    pub fn is_rmw_0(&self) -> bool {
        *self == RMWR::RMW_0
    }
    #[doc = "Checks if the value of the field is `RMW_1`"]
    #[inline]
    pub fn is_rmw_1(&self) -> bool {
        *self == RMWR::RMW_1
    }
}
#[doc = "Possible values of the field `RETEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RETENR {
    #[doc = "Retry phase disabled."]
    RETEN_0,
    #[doc = "Retry phase enabled."]
    RETEN_1,
}
impl RETENR {
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
            RETENR::RETEN_0 => false,
            RETENR::RETEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RETENR {
        match value {
            false => RETENR::RETEN_0,
            true => RETENR::RETEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `RETEN_0`"]
    #[inline]
    pub fn is_reten_0(&self) -> bool {
        *self == RETENR::RETEN_0
    }
    #[doc = "Checks if the value of the field is `RETEN_1`"]
    #[inline]
    pub fn is_reten_1(&self) -> bool {
        *self == RETENR::RETEN_1
    }
}
#[doc = "Possible values of the field `SZ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SZR {
    #[doc = "No TCM implemented."]
    SZ_0,
    #[doc = "4KB."]
    SZ_3,
    #[doc = "8KB."]
    SZ_4,
    #[doc = "16KB."]
    SZ_5,
    #[doc = "32KB."]
    SZ_6,
    #[doc = "64KB."]
    SZ_7,
    #[doc = "128KB."]
    SZ_8,
    #[doc = "256KB."]
    SZ_9,
    #[doc = "512KB."]
    SZ_10,
    #[doc = "1MB."]
    SZ_11,
    #[doc = "2MB."]
    SZ_12,
    #[doc = "4MB."]
    SZ_13,
    #[doc = "8MB."]
    SZ_14,
    #[doc = "16MB."]
    SZ_15,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SZR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SZR::SZ_0 => 0,
            SZR::SZ_3 => 3,
            SZR::SZ_4 => 4,
            SZR::SZ_5 => 5,
            SZR::SZ_6 => 6,
            SZR::SZ_7 => 7,
            SZR::SZ_8 => 8,
            SZR::SZ_9 => 9,
            SZR::SZ_10 => 10,
            SZR::SZ_11 => 11,
            SZR::SZ_12 => 12,
            SZR::SZ_13 => 13,
            SZR::SZ_14 => 14,
            SZR::SZ_15 => 15,
            SZR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SZR {
        match value {
            0 => SZR::SZ_0,
            3 => SZR::SZ_3,
            4 => SZR::SZ_4,
            5 => SZR::SZ_5,
            6 => SZR::SZ_6,
            7 => SZR::SZ_7,
            8 => SZR::SZ_8,
            9 => SZR::SZ_9,
            10 => SZR::SZ_10,
            11 => SZR::SZ_11,
            12 => SZR::SZ_12,
            13 => SZR::SZ_13,
            14 => SZR::SZ_14,
            15 => SZR::SZ_15,
            i => SZR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SZ_0`"]
    #[inline]
    pub fn is_sz_0(&self) -> bool {
        *self == SZR::SZ_0
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
    #[doc = "Checks if the value of the field is `SZ_5`"]
    #[inline]
    pub fn is_sz_5(&self) -> bool {
        *self == SZR::SZ_5
    }
    #[doc = "Checks if the value of the field is `SZ_6`"]
    #[inline]
    pub fn is_sz_6(&self) -> bool {
        *self == SZR::SZ_6
    }
    #[doc = "Checks if the value of the field is `SZ_7`"]
    #[inline]
    pub fn is_sz_7(&self) -> bool {
        *self == SZR::SZ_7
    }
    #[doc = "Checks if the value of the field is `SZ_8`"]
    #[inline]
    pub fn is_sz_8(&self) -> bool {
        *self == SZR::SZ_8
    }
    #[doc = "Checks if the value of the field is `SZ_9`"]
    #[inline]
    pub fn is_sz_9(&self) -> bool {
        *self == SZR::SZ_9
    }
    #[doc = "Checks if the value of the field is `SZ_10`"]
    #[inline]
    pub fn is_sz_10(&self) -> bool {
        *self == SZR::SZ_10
    }
    #[doc = "Checks if the value of the field is `SZ_11`"]
    #[inline]
    pub fn is_sz_11(&self) -> bool {
        *self == SZR::SZ_11
    }
    #[doc = "Checks if the value of the field is `SZ_12`"]
    #[inline]
    pub fn is_sz_12(&self) -> bool {
        *self == SZR::SZ_12
    }
    #[doc = "Checks if the value of the field is `SZ_13`"]
    #[inline]
    pub fn is_sz_13(&self) -> bool {
        *self == SZR::SZ_13
    }
    #[doc = "Checks if the value of the field is `SZ_14`"]
    #[inline]
    pub fn is_sz_14(&self) -> bool {
        *self == SZR::SZ_14
    }
    #[doc = "Checks if the value of the field is `SZ_15`"]
    #[inline]
    pub fn is_sz_15(&self) -> bool {
        *self == SZR::SZ_15
    }
}
#[doc = "Values that can be written to the field `EN`"]
pub enum ENW {
    #[doc = "TCM disabled."]
    EN_0,
    #[doc = "TCM enabled."]
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
    #[doc = "TCM disabled."]
    #[inline]
    pub fn en_0(self) -> &'a mut W {
        self.variant(ENW::EN_0)
    }
    #[doc = "TCM enabled."]
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
#[doc = "Values that can be written to the field `RMW`"]
pub enum RMWW {
    #[doc = "RMW disabled."]
    RMW_0,
    #[doc = "RMW enabled."]
    RMW_1,
}
impl RMWW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RMWW::RMW_0 => false,
            RMWW::RMW_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RMWW<'a> {
    w: &'a mut W,
}
impl<'a> _RMWW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RMWW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "RMW disabled."]
    #[inline]
    pub fn rmw_0(self) -> &'a mut W {
        self.variant(RMWW::RMW_0)
    }
    #[doc = "RMW enabled."]
    #[inline]
    pub fn rmw_1(self) -> &'a mut W {
        self.variant(RMWW::RMW_1)
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
#[doc = "Values that can be written to the field `RETEN`"]
pub enum RETENW {
    #[doc = "Retry phase disabled."]
    RETEN_0,
    #[doc = "Retry phase enabled."]
    RETEN_1,
}
impl RETENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RETENW::RETEN_0 => false,
            RETENW::RETEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RETENW<'a> {
    w: &'a mut W,
}
impl<'a> _RETENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RETENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Retry phase disabled."]
    #[inline]
    pub fn reten_0(self) -> &'a mut W {
        self.variant(RETENW::RETEN_0)
    }
    #[doc = "Retry phase enabled."]
    #[inline]
    pub fn reten_1(self) -> &'a mut W {
        self.variant(RETENW::RETEN_1)
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
    #[doc = "Bit 0 - TCM enable. When a TCM is disabled all accesses are made to the AXIM interface."]
    #[inline]
    pub fn en(&self) -> ENR {
        ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Read-Modify-Write (RMW) enable. Indicates that all writes to TCM, that are not the full width of the TCM RAM, use a RMW sequence."]
    #[inline]
    pub fn rmw(&self) -> RMWR {
        RMWR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Retry phase enable. When enabled the processor guarantees to honor the retry output on the corresponding TCM interface, re-executing the instruction which carried out the TCM access."]
    #[inline]
    pub fn reten(&self) -> RETENR {
        RETENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 3:6 - TCM size. Indicates the size of the relevant TCM."]
    #[inline]
    pub fn sz(&self) -> SZR {
        SZR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 3;
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
    #[doc = "Bit 0 - TCM enable. When a TCM is disabled all accesses are made to the AXIM interface."]
    #[inline]
    pub fn en(&mut self) -> _ENW {
        _ENW { w: self }
    }
    #[doc = "Bit 1 - Read-Modify-Write (RMW) enable. Indicates that all writes to TCM, that are not the full width of the TCM RAM, use a RMW sequence."]
    #[inline]
    pub fn rmw(&mut self) -> _RMWW {
        _RMWW { w: self }
    }
    #[doc = "Bit 2 - Retry phase enable. When enabled the processor guarantees to honor the retry output on the corresponding TCM interface, re-executing the instruction which carried out the TCM access."]
    #[inline]
    pub fn reten(&mut self) -> _RETENW {
        _RETENW { w: self }
    }
}
