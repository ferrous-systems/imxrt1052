#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CR {
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
#[doc = "Possible values of the field `MEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MENR {
    #[doc = "Module is disabled"]
    MEN_0,
    #[doc = "Module is enabled"]
    MEN_1,
}
impl MENR {
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
            MENR::MEN_0 => false,
            MENR::MEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MENR {
        match value {
            false => MENR::MEN_0,
            true => MENR::MEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `MEN_0`"]
    #[inline]
    pub fn is_men_0(&self) -> bool {
        *self == MENR::MEN_0
    }
    #[doc = "Checks if the value of the field is `MEN_1`"]
    #[inline]
    pub fn is_men_1(&self) -> bool {
        *self == MENR::MEN_1
    }
}
#[doc = "Possible values of the field `RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSTR {
    #[doc = "Module is not reset"]
    RST_0,
    #[doc = "Module is reset"]
    RST_1,
}
impl RSTR {
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
            RSTR::RST_0 => false,
            RSTR::RST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RSTR {
        match value {
            false => RSTR::RST_0,
            true => RSTR::RST_1,
        }
    }
    #[doc = "Checks if the value of the field is `RST_0`"]
    #[inline]
    pub fn is_rst_0(&self) -> bool {
        *self == RSTR::RST_0
    }
    #[doc = "Checks if the value of the field is `RST_1`"]
    #[inline]
    pub fn is_rst_1(&self) -> bool {
        *self == RSTR::RST_1
    }
}
#[doc = "Possible values of the field `DOZEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DOZENR {
    #[doc = "Module is enabled in Doze mode"]
    DOZEN_0,
    #[doc = "Module is disabled in Doze mode"]
    DOZEN_1,
}
impl DOZENR {
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
            DOZENR::DOZEN_0 => false,
            DOZENR::DOZEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DOZENR {
        match value {
            false => DOZENR::DOZEN_0,
            true => DOZENR::DOZEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `DOZEN_0`"]
    #[inline]
    pub fn is_dozen_0(&self) -> bool {
        *self == DOZENR::DOZEN_0
    }
    #[doc = "Checks if the value of the field is `DOZEN_1`"]
    #[inline]
    pub fn is_dozen_1(&self) -> bool {
        *self == DOZENR::DOZEN_1
    }
}
#[doc = "Possible values of the field `DBGEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBGENR {
    #[doc = "Module is disabled in debug mode"]
    DBGEN_0,
    #[doc = "Module is enabled in debug mode"]
    DBGEN_1,
}
impl DBGENR {
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
            DBGENR::DBGEN_0 => false,
            DBGENR::DBGEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DBGENR {
        match value {
            false => DBGENR::DBGEN_0,
            true => DBGENR::DBGEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `DBGEN_0`"]
    #[inline]
    pub fn is_dbgen_0(&self) -> bool {
        *self == DBGENR::DBGEN_0
    }
    #[doc = "Checks if the value of the field is `DBGEN_1`"]
    #[inline]
    pub fn is_dbgen_1(&self) -> bool {
        *self == DBGENR::DBGEN_1
    }
}
#[doc = "Values that can be written to the field `MEN`"]
pub enum MENW {
    #[doc = "Module is disabled"]
    MEN_0,
    #[doc = "Module is enabled"]
    MEN_1,
}
impl MENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MENW::MEN_0 => false,
            MENW::MEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MENW<'a> {
    w: &'a mut W,
}
impl<'a> _MENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Module is disabled"]
    #[inline]
    pub fn men_0(self) -> &'a mut W {
        self.variant(MENW::MEN_0)
    }
    #[doc = "Module is enabled"]
    #[inline]
    pub fn men_1(self) -> &'a mut W {
        self.variant(MENW::MEN_1)
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
#[doc = "Values that can be written to the field `RST`"]
pub enum RSTW {
    #[doc = "Module is not reset"]
    RST_0,
    #[doc = "Module is reset"]
    RST_1,
}
impl RSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RSTW::RST_0 => false,
            RSTW::RST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Module is not reset"]
    #[inline]
    pub fn rst_0(self) -> &'a mut W {
        self.variant(RSTW::RST_0)
    }
    #[doc = "Module is reset"]
    #[inline]
    pub fn rst_1(self) -> &'a mut W {
        self.variant(RSTW::RST_1)
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
#[doc = "Values that can be written to the field `DOZEN`"]
pub enum DOZENW {
    #[doc = "Module is enabled in Doze mode"]
    DOZEN_0,
    #[doc = "Module is disabled in Doze mode"]
    DOZEN_1,
}
impl DOZENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DOZENW::DOZEN_0 => false,
            DOZENW::DOZEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DOZENW<'a> {
    w: &'a mut W,
}
impl<'a> _DOZENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DOZENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Module is enabled in Doze mode"]
    #[inline]
    pub fn dozen_0(self) -> &'a mut W {
        self.variant(DOZENW::DOZEN_0)
    }
    #[doc = "Module is disabled in Doze mode"]
    #[inline]
    pub fn dozen_1(self) -> &'a mut W {
        self.variant(DOZENW::DOZEN_1)
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
#[doc = "Values that can be written to the field `DBGEN`"]
pub enum DBGENW {
    #[doc = "Module is disabled in debug mode"]
    DBGEN_0,
    #[doc = "Module is enabled in debug mode"]
    DBGEN_1,
}
impl DBGENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DBGENW::DBGEN_0 => false,
            DBGENW::DBGEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DBGENW<'a> {
    w: &'a mut W,
}
impl<'a> _DBGENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DBGENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Module is disabled in debug mode"]
    #[inline]
    pub fn dbgen_0(self) -> &'a mut W {
        self.variant(DBGENW::DBGEN_0)
    }
    #[doc = "Module is enabled in debug mode"]
    #[inline]
    pub fn dbgen_1(self) -> &'a mut W {
        self.variant(DBGENW::DBGEN_1)
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RTF`"]
pub enum RTFW {
    #[doc = "No effect"]
    RTF_0,
    #[doc = "Transmit FIFO is reset"]
    RTF_1,
}
impl RTFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RTFW::RTF_0 => false,
            RTFW::RTF_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RTFW<'a> {
    w: &'a mut W,
}
impl<'a> _RTFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RTFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn rtf_0(self) -> &'a mut W {
        self.variant(RTFW::RTF_0)
    }
    #[doc = "Transmit FIFO is reset"]
    #[inline]
    pub fn rtf_1(self) -> &'a mut W {
        self.variant(RTFW::RTF_1)
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RRF`"]
pub enum RRFW {
    #[doc = "No effect"]
    RRF_0,
    #[doc = "Receive FIFO is reset"]
    RRF_1,
}
impl RRFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RRFW::RRF_0 => false,
            RRFW::RRF_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RRFW<'a> {
    w: &'a mut W,
}
impl<'a> _RRFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RRFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn rrf_0(self) -> &'a mut W {
        self.variant(RRFW::RRF_0)
    }
    #[doc = "Receive FIFO is reset"]
    #[inline]
    pub fn rrf_1(self) -> &'a mut W {
        self.variant(RRFW::RRF_1)
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
        const OFFSET: u8 = 9;
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
    #[doc = "Bit 0 - Module Enable"]
    #[inline]
    pub fn men(&self) -> MENR {
        MENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Software Reset"]
    #[inline]
    pub fn rst(&self) -> RSTR {
        RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Doze mode enable"]
    #[inline]
    pub fn dozen(&self) -> DOZENR {
        DOZENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Debug Enable"]
    #[inline]
    pub fn dbgen(&self) -> DBGENR {
        DBGENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
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
    #[doc = "Bit 0 - Module Enable"]
    #[inline]
    pub fn men(&mut self) -> _MENW {
        _MENW { w: self }
    }
    #[doc = "Bit 1 - Software Reset"]
    #[inline]
    pub fn rst(&mut self) -> _RSTW {
        _RSTW { w: self }
    }
    #[doc = "Bit 2 - Doze mode enable"]
    #[inline]
    pub fn dozen(&mut self) -> _DOZENW {
        _DOZENW { w: self }
    }
    #[doc = "Bit 3 - Debug Enable"]
    #[inline]
    pub fn dbgen(&mut self) -> _DBGENW {
        _DBGENW { w: self }
    }
    #[doc = "Bit 8 - Reset Transmit FIFO"]
    #[inline]
    pub fn rtf(&mut self) -> _RTFW {
        _RTFW { w: self }
    }
    #[doc = "Bit 9 - Reset Receive FIFO"]
    #[inline]
    pub fn rrf(&mut self) -> _RRFW {
        _RRFW { w: self }
    }
}
