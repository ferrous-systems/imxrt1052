#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ICSR {
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
pub struct VECTACTIVER {
    bits: u16,
}
impl VECTACTIVER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Possible values of the field `RETTOBASE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RETTOBASER {
    #[doc = "there are preempted active exceptions to execute"]
    RETTOBASE_0,
    #[doc = "there are no active exceptions, or the currently-executing exception is the only active exception"]
    RETTOBASE_1,
}
impl RETTOBASER {
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
            RETTOBASER::RETTOBASE_0 => false,
            RETTOBASER::RETTOBASE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RETTOBASER {
        match value {
            false => RETTOBASER::RETTOBASE_0,
            true => RETTOBASER::RETTOBASE_1,
        }
    }
    #[doc = "Checks if the value of the field is `RETTOBASE_0`"]
    #[inline]
    pub fn is_rettobase_0(&self) -> bool {
        *self == RETTOBASER::RETTOBASE_0
    }
    #[doc = "Checks if the value of the field is `RETTOBASE_1`"]
    #[inline]
    pub fn is_rettobase_1(&self) -> bool {
        *self == RETTOBASER::RETTOBASE_1
    }
}
#[doc = r" Value of the field"]
pub struct VECTPENDINGR {
    bits: u16,
}
impl VECTPENDINGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Possible values of the field `ISRPENDING`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISRPENDINGR {
    #[doc = "No external interrupt pending."]
    ISRPENDING_0,
    #[doc = "External interrupt pending."]
    ISRPENDING_1,
}
impl ISRPENDINGR {
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
            ISRPENDINGR::ISRPENDING_0 => false,
            ISRPENDINGR::ISRPENDING_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ISRPENDINGR {
        match value {
            false => ISRPENDINGR::ISRPENDING_0,
            true => ISRPENDINGR::ISRPENDING_1,
        }
    }
    #[doc = "Checks if the value of the field is `ISRPENDING_0`"]
    #[inline]
    pub fn is_isrpending_0(&self) -> bool {
        *self == ISRPENDINGR::ISRPENDING_0
    }
    #[doc = "Checks if the value of the field is `ISRPENDING_1`"]
    #[inline]
    pub fn is_isrpending_1(&self) -> bool {
        *self == ISRPENDINGR::ISRPENDING_1
    }
}
#[doc = "Possible values of the field `PENDSTSET`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PENDSTSETR {
    #[doc = "write: no effect; read: SysTick exception is not pending"]
    PENDSTSET_0,
    #[doc = "write: changes SysTick exception state to pending; read: SysTick exception is pending"]
    PENDSTSET_1,
}
impl PENDSTSETR {
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
            PENDSTSETR::PENDSTSET_0 => false,
            PENDSTSETR::PENDSTSET_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PENDSTSETR {
        match value {
            false => PENDSTSETR::PENDSTSET_0,
            true => PENDSTSETR::PENDSTSET_1,
        }
    }
    #[doc = "Checks if the value of the field is `PENDSTSET_0`"]
    #[inline]
    pub fn is_pendstset_0(&self) -> bool {
        *self == PENDSTSETR::PENDSTSET_0
    }
    #[doc = "Checks if the value of the field is `PENDSTSET_1`"]
    #[inline]
    pub fn is_pendstset_1(&self) -> bool {
        *self == PENDSTSETR::PENDSTSET_1
    }
}
#[doc = "Possible values of the field `PENDSVSET`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PENDSVSETR {
    #[doc = "write: no effect; read: PendSV exception is not pending"]
    PENDSVSET_0,
    #[doc = "write: changes PendSV exception state to pending; read: PendSV exception is pending"]
    PENDSVSET_1,
}
impl PENDSVSETR {
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
            PENDSVSETR::PENDSVSET_0 => false,
            PENDSVSETR::PENDSVSET_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PENDSVSETR {
        match value {
            false => PENDSVSETR::PENDSVSET_0,
            true => PENDSVSETR::PENDSVSET_1,
        }
    }
    #[doc = "Checks if the value of the field is `PENDSVSET_0`"]
    #[inline]
    pub fn is_pendsvset_0(&self) -> bool {
        *self == PENDSVSETR::PENDSVSET_0
    }
    #[doc = "Checks if the value of the field is `PENDSVSET_1`"]
    #[inline]
    pub fn is_pendsvset_1(&self) -> bool {
        *self == PENDSVSETR::PENDSVSET_1
    }
}
#[doc = "Possible values of the field `NMIPENDSET`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NMIPENDSETR {
    #[doc = "write: no effect; read: NMI exception is not pending"]
    NMIPENDSET_0,
    #[doc = "write: changes NMI exception state to pending; read: NMI exception is pending"]
    NMIPENDSET_1,
}
impl NMIPENDSETR {
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
            NMIPENDSETR::NMIPENDSET_0 => false,
            NMIPENDSETR::NMIPENDSET_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NMIPENDSETR {
        match value {
            false => NMIPENDSETR::NMIPENDSET_0,
            true => NMIPENDSETR::NMIPENDSET_1,
        }
    }
    #[doc = "Checks if the value of the field is `NMIPENDSET_0`"]
    #[inline]
    pub fn is_nmipendset_0(&self) -> bool {
        *self == NMIPENDSETR::NMIPENDSET_0
    }
    #[doc = "Checks if the value of the field is `NMIPENDSET_1`"]
    #[inline]
    pub fn is_nmipendset_1(&self) -> bool {
        *self == NMIPENDSETR::NMIPENDSET_1
    }
}
#[doc = "Values that can be written to the field `PENDSTCLR`"]
pub enum PENDSTCLRW {
    #[doc = "no effect"]
    PENDSTCLR_0,
    #[doc = "removes the pending state from the SysTick exception"]
    PENDSTCLR_1,
}
impl PENDSTCLRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PENDSTCLRW::PENDSTCLR_0 => false,
            PENDSTCLRW::PENDSTCLR_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PENDSTCLRW<'a> {
    w: &'a mut W,
}
impl<'a> _PENDSTCLRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PENDSTCLRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no effect"]
    #[inline]
    pub fn pendstclr_0(self) -> &'a mut W {
        self.variant(PENDSTCLRW::PENDSTCLR_0)
    }
    #[doc = "removes the pending state from the SysTick exception"]
    #[inline]
    pub fn pendstclr_1(self) -> &'a mut W {
        self.variant(PENDSTCLRW::PENDSTCLR_1)
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
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PENDSTSET`"]
pub enum PENDSTSETW {
    #[doc = "write: no effect; read: SysTick exception is not pending"]
    PENDSTSET_0,
    #[doc = "write: changes SysTick exception state to pending; read: SysTick exception is pending"]
    PENDSTSET_1,
}
impl PENDSTSETW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PENDSTSETW::PENDSTSET_0 => false,
            PENDSTSETW::PENDSTSET_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PENDSTSETW<'a> {
    w: &'a mut W,
}
impl<'a> _PENDSTSETW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PENDSTSETW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "write: no effect; read: SysTick exception is not pending"]
    #[inline]
    pub fn pendstset_0(self) -> &'a mut W {
        self.variant(PENDSTSETW::PENDSTSET_0)
    }
    #[doc = "write: changes SysTick exception state to pending; read: SysTick exception is pending"]
    #[inline]
    pub fn pendstset_1(self) -> &'a mut W {
        self.variant(PENDSTSETW::PENDSTSET_1)
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
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PENDSVCLR`"]
pub enum PENDSVCLRW {
    #[doc = "no effect"]
    PENDSVCLR_0,
    #[doc = "removes the pending state from the PendSV exception"]
    PENDSVCLR_1,
}
impl PENDSVCLRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PENDSVCLRW::PENDSVCLR_0 => false,
            PENDSVCLRW::PENDSVCLR_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PENDSVCLRW<'a> {
    w: &'a mut W,
}
impl<'a> _PENDSVCLRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PENDSVCLRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no effect"]
    #[inline]
    pub fn pendsvclr_0(self) -> &'a mut W {
        self.variant(PENDSVCLRW::PENDSVCLR_0)
    }
    #[doc = "removes the pending state from the PendSV exception"]
    #[inline]
    pub fn pendsvclr_1(self) -> &'a mut W {
        self.variant(PENDSVCLRW::PENDSVCLR_1)
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
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PENDSVSET`"]
pub enum PENDSVSETW {
    #[doc = "write: no effect; read: PendSV exception is not pending"]
    PENDSVSET_0,
    #[doc = "write: changes PendSV exception state to pending; read: PendSV exception is pending"]
    PENDSVSET_1,
}
impl PENDSVSETW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PENDSVSETW::PENDSVSET_0 => false,
            PENDSVSETW::PENDSVSET_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PENDSVSETW<'a> {
    w: &'a mut W,
}
impl<'a> _PENDSVSETW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PENDSVSETW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "write: no effect; read: PendSV exception is not pending"]
    #[inline]
    pub fn pendsvset_0(self) -> &'a mut W {
        self.variant(PENDSVSETW::PENDSVSET_0)
    }
    #[doc = "write: changes PendSV exception state to pending; read: PendSV exception is pending"]
    #[inline]
    pub fn pendsvset_1(self) -> &'a mut W {
        self.variant(PENDSVSETW::PENDSVSET_1)
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
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `NMIPENDSET`"]
pub enum NMIPENDSETW {
    #[doc = "write: no effect; read: NMI exception is not pending"]
    NMIPENDSET_0,
    #[doc = "write: changes NMI exception state to pending; read: NMI exception is pending"]
    NMIPENDSET_1,
}
impl NMIPENDSETW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            NMIPENDSETW::NMIPENDSET_0 => false,
            NMIPENDSETW::NMIPENDSET_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NMIPENDSETW<'a> {
    w: &'a mut W,
}
impl<'a> _NMIPENDSETW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NMIPENDSETW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "write: no effect; read: NMI exception is not pending"]
    #[inline]
    pub fn nmipendset_0(self) -> &'a mut W {
        self.variant(NMIPENDSETW::NMIPENDSET_0)
    }
    #[doc = "write: changes NMI exception state to pending; read: NMI exception is pending"]
    #[inline]
    pub fn nmipendset_1(self) -> &'a mut W {
        self.variant(NMIPENDSETW::NMIPENDSET_1)
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
    #[doc = "Bits 0:8 - Active exception number"]
    #[inline]
    pub fn vectactive(&self) -> VECTACTIVER {
        let bits = {
            const MASK: u16 = 511;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        VECTACTIVER { bits }
    }
    #[doc = "Bit 11 - Indicates whether there are preempted active exceptions"]
    #[inline]
    pub fn rettobase(&self) -> RETTOBASER {
        RETTOBASER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 12:20 - Exception number of the highest priority pending enabled exception"]
    #[inline]
    pub fn vectpending(&self) -> VECTPENDINGR {
        let bits = {
            const MASK: u16 = 511;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        VECTPENDINGR { bits }
    }
    #[doc = "Bit 22 - Interrupt pending flag, excluding NMI and Faults"]
    #[inline]
    pub fn isrpending(&self) -> ISRPENDINGR {
        ISRPENDINGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - SysTick exception set-pending bit"]
    #[inline]
    pub fn pendstset(&self) -> PENDSTSETR {
        PENDSTSETR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - PendSV set-pending bit"]
    #[inline]
    pub fn pendsvset(&self) -> PENDSVSETR {
        PENDSVSETR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - NMI set-pending bit"]
    #[inline]
    pub fn nmipendset(&self) -> NMIPENDSETR {
        NMIPENDSETR::_from({
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
    #[doc = "Bit 25 - SysTick exception clear-pending bit"]
    #[inline]
    pub fn pendstclr(&mut self) -> _PENDSTCLRW {
        _PENDSTCLRW { w: self }
    }
    #[doc = "Bit 26 - SysTick exception set-pending bit"]
    #[inline]
    pub fn pendstset(&mut self) -> _PENDSTSETW {
        _PENDSTSETW { w: self }
    }
    #[doc = "Bit 27 - PendSV clear-pending bit"]
    #[inline]
    pub fn pendsvclr(&mut self) -> _PENDSVCLRW {
        _PENDSVCLRW { w: self }
    }
    #[doc = "Bit 28 - PendSV set-pending bit"]
    #[inline]
    pub fn pendsvset(&mut self) -> _PENDSVSETW {
        _PENDSVSETW { w: self }
    }
    #[doc = "Bit 31 - NMI set-pending bit"]
    #[inline]
    pub fn nmipendset(&mut self) -> _NMIPENDSETW {
        _NMIPENDSETW { w: self }
    }
}
