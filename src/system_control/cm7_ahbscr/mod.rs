#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CM7_AHBSCR {
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
#[doc = "Possible values of the field `CTL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTLR {
    #[doc = "AHBS access priority demoted. This is the reset value."]
    CTL_0,
    #[doc = "Software access priority demoted."]
    CTL_1,
    #[doc = "AHBS access priority demoted by initializing the fairness counter to the CM7_AHBSCR[INITCOUNT] value when the software execution priority is higher than or equal to the threshold level programed in CM7_AHBSCR[TPRI]."]
    CTL_2,
    #[doc = "AHBSPRI signal has control of access priority."]
    CTL_3,
}
impl CTLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CTLR::CTL_0 => 0,
            CTLR::CTL_1 => 1,
            CTLR::CTL_2 => 2,
            CTLR::CTL_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CTLR {
        match value {
            0 => CTLR::CTL_0,
            1 => CTLR::CTL_1,
            2 => CTLR::CTL_2,
            3 => CTLR::CTL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CTL_0`"]
    #[inline]
    pub fn is_ctl_0(&self) -> bool {
        *self == CTLR::CTL_0
    }
    #[doc = "Checks if the value of the field is `CTL_1`"]
    #[inline]
    pub fn is_ctl_1(&self) -> bool {
        *self == CTLR::CTL_1
    }
    #[doc = "Checks if the value of the field is `CTL_2`"]
    #[inline]
    pub fn is_ctl_2(&self) -> bool {
        *self == CTLR::CTL_2
    }
    #[doc = "Checks if the value of the field is `CTL_3`"]
    #[inline]
    pub fn is_ctl_3(&self) -> bool {
        *self == CTLR::CTL_3
    }
}
#[doc = r" Value of the field"]
pub struct TPRIR {
    bits: u16,
}
impl TPRIR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct INITCOUNTR {
    bits: u8,
}
impl INITCOUNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `CTL`"]
pub enum CTLW {
    #[doc = "AHBS access priority demoted. This is the reset value."]
    CTL_0,
    #[doc = "Software access priority demoted."]
    CTL_1,
    #[doc = "AHBS access priority demoted by initializing the fairness counter to the CM7_AHBSCR[INITCOUNT] value when the software execution priority is higher than or equal to the threshold level programed in CM7_AHBSCR[TPRI]."]
    CTL_2,
    #[doc = "AHBSPRI signal has control of access priority."]
    CTL_3,
}
impl CTLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CTLW::CTL_0 => 0,
            CTLW::CTL_1 => 1,
            CTLW::CTL_2 => 2,
            CTLW::CTL_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CTLW<'a> {
    w: &'a mut W,
}
impl<'a> _CTLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTLW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "AHBS access priority demoted. This is the reset value."]
    #[inline]
    pub fn ctl_0(self) -> &'a mut W {
        self.variant(CTLW::CTL_0)
    }
    #[doc = "Software access priority demoted."]
    #[inline]
    pub fn ctl_1(self) -> &'a mut W {
        self.variant(CTLW::CTL_1)
    }
    #[doc = "AHBS access priority demoted by initializing the fairness counter to the CM7_AHBSCR[INITCOUNT] value when the software execution priority is higher than or equal to the threshold level programed in CM7_AHBSCR[TPRI]."]
    #[inline]
    pub fn ctl_2(self) -> &'a mut W {
        self.variant(CTLW::CTL_2)
    }
    #[doc = "AHBSPRI signal has control of access priority."]
    #[inline]
    pub fn ctl_3(self) -> &'a mut W {
        self.variant(CTLW::CTL_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TPRIW<'a> {
    w: &'a mut W,
}
impl<'a> _TPRIW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 511;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _INITCOUNTW<'a> {
    w: &'a mut W,
}
impl<'a> _INITCOUNTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 11;
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
    #[doc = "Bits 0:1 - AHBS prioritization control."]
    #[inline]
    pub fn ctl(&self) -> CTLR {
        CTLR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:10 - Threshold execution priority for AHBS traffic demotion."]
    #[inline]
    pub fn tpri(&self) -> TPRIR {
        let bits = {
            const MASK: u16 = 511;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        TPRIR { bits }
    }
    #[doc = "Bits 11:15 - Fairness counter initialization value."]
    #[inline]
    pub fn initcount(&self) -> INITCOUNTR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        INITCOUNTR { bits }
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
    #[doc = "Bits 0:1 - AHBS prioritization control."]
    #[inline]
    pub fn ctl(&mut self) -> _CTLW {
        _CTLW { w: self }
    }
    #[doc = "Bits 2:10 - Threshold execution priority for AHBS traffic demotion."]
    #[inline]
    pub fn tpri(&mut self) -> _TPRIW {
        _TPRIW { w: self }
    }
    #[doc = "Bits 11:15 - Fairness counter initialization value."]
    #[inline]
    pub fn initcount(&mut self) -> _INITCOUNTW {
        _INITCOUNTW { w: self }
    }
}
