#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CCR {
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
#[doc = "Possible values of the field `NONBASETHRDENA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NONBASETHRDENAR {
    #[doc = "processor can enter Thread mode only when no exception is active"]
    NONBASETHRDENA_0,
    #[doc = "processor can enter Thread mode from any level under the control of an EXC_RETURN value"]
    NONBASETHRDENA_1,
}
impl NONBASETHRDENAR {
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
            NONBASETHRDENAR::NONBASETHRDENA_0 => false,
            NONBASETHRDENAR::NONBASETHRDENA_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NONBASETHRDENAR {
        match value {
            false => NONBASETHRDENAR::NONBASETHRDENA_0,
            true => NONBASETHRDENAR::NONBASETHRDENA_1,
        }
    }
    #[doc = "Checks if the value of the field is `NONBASETHRDENA_0`"]
    #[inline]
    pub fn is_nonbasethrdena_0(&self) -> bool {
        *self == NONBASETHRDENAR::NONBASETHRDENA_0
    }
    #[doc = "Checks if the value of the field is `NONBASETHRDENA_1`"]
    #[inline]
    pub fn is_nonbasethrdena_1(&self) -> bool {
        *self == NONBASETHRDENAR::NONBASETHRDENA_1
    }
}
#[doc = "Possible values of the field `USERSETMPEND`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USERSETMPENDR {
    #[doc = "disable"]
    USERSETMPEND_0,
    #[doc = "enable"]
    USERSETMPEND_1,
}
impl USERSETMPENDR {
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
            USERSETMPENDR::USERSETMPEND_0 => false,
            USERSETMPENDR::USERSETMPEND_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USERSETMPENDR {
        match value {
            false => USERSETMPENDR::USERSETMPEND_0,
            true => USERSETMPENDR::USERSETMPEND_1,
        }
    }
    #[doc = "Checks if the value of the field is `USERSETMPEND_0`"]
    #[inline]
    pub fn is_usersetmpend_0(&self) -> bool {
        *self == USERSETMPENDR::USERSETMPEND_0
    }
    #[doc = "Checks if the value of the field is `USERSETMPEND_1`"]
    #[inline]
    pub fn is_usersetmpend_1(&self) -> bool {
        *self == USERSETMPENDR::USERSETMPEND_1
    }
}
#[doc = "Possible values of the field `UNALIGN_TRP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UNALIGN_TRPR {
    #[doc = "do not trap unaligned halfword and word accesses"]
    UNALIGN_TRP_0,
    #[doc = "trap unaligned halfword and word accesses"]
    UNALIGN_TRP_1,
}
impl UNALIGN_TRPR {
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
            UNALIGN_TRPR::UNALIGN_TRP_0 => false,
            UNALIGN_TRPR::UNALIGN_TRP_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UNALIGN_TRPR {
        match value {
            false => UNALIGN_TRPR::UNALIGN_TRP_0,
            true => UNALIGN_TRPR::UNALIGN_TRP_1,
        }
    }
    #[doc = "Checks if the value of the field is `UNALIGN_TRP_0`"]
    #[inline]
    pub fn is_unalign_trp_0(&self) -> bool {
        *self == UNALIGN_TRPR::UNALIGN_TRP_0
    }
    #[doc = "Checks if the value of the field is `UNALIGN_TRP_1`"]
    #[inline]
    pub fn is_unalign_trp_1(&self) -> bool {
        *self == UNALIGN_TRPR::UNALIGN_TRP_1
    }
}
#[doc = "Possible values of the field `DIV_0_TRP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIV_0_TRPR {
    #[doc = "do not trap divide by 0"]
    DIV_0_TRP_0,
    #[doc = "trap divide by 0"]
    DIV_0_TRP_1,
}
impl DIV_0_TRPR {
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
            DIV_0_TRPR::DIV_0_TRP_0 => false,
            DIV_0_TRPR::DIV_0_TRP_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DIV_0_TRPR {
        match value {
            false => DIV_0_TRPR::DIV_0_TRP_0,
            true => DIV_0_TRPR::DIV_0_TRP_1,
        }
    }
    #[doc = "Checks if the value of the field is `DIV_0_TRP_0`"]
    #[inline]
    pub fn is_div_0_trp_0(&self) -> bool {
        *self == DIV_0_TRPR::DIV_0_TRP_0
    }
    #[doc = "Checks if the value of the field is `DIV_0_TRP_1`"]
    #[inline]
    pub fn is_div_0_trp_1(&self) -> bool {
        *self == DIV_0_TRPR::DIV_0_TRP_1
    }
}
#[doc = "Possible values of the field `BFHFNMIGN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BFHFNMIGNR {
    #[doc = "data bus faults caused by load and store instructions cause a lock-up"]
    BFHFNMIGN_0,
    #[doc = "handlers running at priority -1 and -2 ignore data bus faults caused by load and store instructions"]
    BFHFNMIGN_1,
}
impl BFHFNMIGNR {
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
            BFHFNMIGNR::BFHFNMIGN_0 => false,
            BFHFNMIGNR::BFHFNMIGN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BFHFNMIGNR {
        match value {
            false => BFHFNMIGNR::BFHFNMIGN_0,
            true => BFHFNMIGNR::BFHFNMIGN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BFHFNMIGN_0`"]
    #[inline]
    pub fn is_bfhfnmign_0(&self) -> bool {
        *self == BFHFNMIGNR::BFHFNMIGN_0
    }
    #[doc = "Checks if the value of the field is `BFHFNMIGN_1`"]
    #[inline]
    pub fn is_bfhfnmign_1(&self) -> bool {
        *self == BFHFNMIGNR::BFHFNMIGN_1
    }
}
#[doc = "Possible values of the field `STKALIGN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STKALIGNR {
    #[doc = "4-byte aligned"]
    STKALIGN_0,
    #[doc = "8-byte aligned"]
    STKALIGN_1,
}
impl STKALIGNR {
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
            STKALIGNR::STKALIGN_0 => false,
            STKALIGNR::STKALIGN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STKALIGNR {
        match value {
            false => STKALIGNR::STKALIGN_0,
            true => STKALIGNR::STKALIGN_1,
        }
    }
    #[doc = "Checks if the value of the field is `STKALIGN_0`"]
    #[inline]
    pub fn is_stkalign_0(&self) -> bool {
        *self == STKALIGNR::STKALIGN_0
    }
    #[doc = "Checks if the value of the field is `STKALIGN_1`"]
    #[inline]
    pub fn is_stkalign_1(&self) -> bool {
        *self == STKALIGNR::STKALIGN_1
    }
}
#[doc = "Possible values of the field `DC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCR {
    #[doc = "L1 data cache disabled"]
    DC_0,
    #[doc = "L1 data cache enabled"]
    DC_1,
}
impl DCR {
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
            DCR::DC_0 => false,
            DCR::DC_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DCR {
        match value {
            false => DCR::DC_0,
            true => DCR::DC_1,
        }
    }
    #[doc = "Checks if the value of the field is `DC_0`"]
    #[inline]
    pub fn is_dc_0(&self) -> bool {
        *self == DCR::DC_0
    }
    #[doc = "Checks if the value of the field is `DC_1`"]
    #[inline]
    pub fn is_dc_1(&self) -> bool {
        *self == DCR::DC_1
    }
}
#[doc = "Possible values of the field `IC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICR {
    #[doc = "L1 instruction cache disabled"]
    IC_0,
    #[doc = "L1 instruction cache enabled"]
    IC_1,
}
impl ICR {
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
            ICR::IC_0 => false,
            ICR::IC_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ICR {
        match value {
            false => ICR::IC_0,
            true => ICR::IC_1,
        }
    }
    #[doc = "Checks if the value of the field is `IC_0`"]
    #[inline]
    pub fn is_ic_0(&self) -> bool {
        *self == ICR::IC_0
    }
    #[doc = "Checks if the value of the field is `IC_1`"]
    #[inline]
    pub fn is_ic_1(&self) -> bool {
        *self == ICR::IC_1
    }
}
#[doc = r" Value of the field"]
pub struct BPR {
    bits: bool,
}
impl BPR {
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
#[doc = "Values that can be written to the field `NONBASETHRDENA`"]
pub enum NONBASETHRDENAW {
    #[doc = "processor can enter Thread mode only when no exception is active"]
    NONBASETHRDENA_0,
    #[doc = "processor can enter Thread mode from any level under the control of an EXC_RETURN value"]
    NONBASETHRDENA_1,
}
impl NONBASETHRDENAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            NONBASETHRDENAW::NONBASETHRDENA_0 => false,
            NONBASETHRDENAW::NONBASETHRDENA_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NONBASETHRDENAW<'a> {
    w: &'a mut W,
}
impl<'a> _NONBASETHRDENAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NONBASETHRDENAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "processor can enter Thread mode only when no exception is active"]
    #[inline]
    pub fn nonbasethrdena_0(self) -> &'a mut W {
        self.variant(NONBASETHRDENAW::NONBASETHRDENA_0)
    }
    #[doc = "processor can enter Thread mode from any level under the control of an EXC_RETURN value"]
    #[inline]
    pub fn nonbasethrdena_1(self) -> &'a mut W {
        self.variant(NONBASETHRDENAW::NONBASETHRDENA_1)
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
#[doc = "Values that can be written to the field `USERSETMPEND`"]
pub enum USERSETMPENDW {
    #[doc = "disable"]
    USERSETMPEND_0,
    #[doc = "enable"]
    USERSETMPEND_1,
}
impl USERSETMPENDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USERSETMPENDW::USERSETMPEND_0 => false,
            USERSETMPENDW::USERSETMPEND_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USERSETMPENDW<'a> {
    w: &'a mut W,
}
impl<'a> _USERSETMPENDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USERSETMPENDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "disable"]
    #[inline]
    pub fn usersetmpend_0(self) -> &'a mut W {
        self.variant(USERSETMPENDW::USERSETMPEND_0)
    }
    #[doc = "enable"]
    #[inline]
    pub fn usersetmpend_1(self) -> &'a mut W {
        self.variant(USERSETMPENDW::USERSETMPEND_1)
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
#[doc = "Values that can be written to the field `UNALIGN_TRP`"]
pub enum UNALIGN_TRPW {
    #[doc = "do not trap unaligned halfword and word accesses"]
    UNALIGN_TRP_0,
    #[doc = "trap unaligned halfword and word accesses"]
    UNALIGN_TRP_1,
}
impl UNALIGN_TRPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            UNALIGN_TRPW::UNALIGN_TRP_0 => false,
            UNALIGN_TRPW::UNALIGN_TRP_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UNALIGN_TRPW<'a> {
    w: &'a mut W,
}
impl<'a> _UNALIGN_TRPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UNALIGN_TRPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "do not trap unaligned halfword and word accesses"]
    #[inline]
    pub fn unalign_trp_0(self) -> &'a mut W {
        self.variant(UNALIGN_TRPW::UNALIGN_TRP_0)
    }
    #[doc = "trap unaligned halfword and word accesses"]
    #[inline]
    pub fn unalign_trp_1(self) -> &'a mut W {
        self.variant(UNALIGN_TRPW::UNALIGN_TRP_1)
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
#[doc = "Values that can be written to the field `DIV_0_TRP`"]
pub enum DIV_0_TRPW {
    #[doc = "do not trap divide by 0"]
    DIV_0_TRP_0,
    #[doc = "trap divide by 0"]
    DIV_0_TRP_1,
}
impl DIV_0_TRPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DIV_0_TRPW::DIV_0_TRP_0 => false,
            DIV_0_TRPW::DIV_0_TRP_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DIV_0_TRPW<'a> {
    w: &'a mut W,
}
impl<'a> _DIV_0_TRPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DIV_0_TRPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "do not trap divide by 0"]
    #[inline]
    pub fn div_0_trp_0(self) -> &'a mut W {
        self.variant(DIV_0_TRPW::DIV_0_TRP_0)
    }
    #[doc = "trap divide by 0"]
    #[inline]
    pub fn div_0_trp_1(self) -> &'a mut W {
        self.variant(DIV_0_TRPW::DIV_0_TRP_1)
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BFHFNMIGN`"]
pub enum BFHFNMIGNW {
    #[doc = "data bus faults caused by load and store instructions cause a lock-up"]
    BFHFNMIGN_0,
    #[doc = "handlers running at priority -1 and -2 ignore data bus faults caused by load and store instructions"]
    BFHFNMIGN_1,
}
impl BFHFNMIGNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BFHFNMIGNW::BFHFNMIGN_0 => false,
            BFHFNMIGNW::BFHFNMIGN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BFHFNMIGNW<'a> {
    w: &'a mut W,
}
impl<'a> _BFHFNMIGNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BFHFNMIGNW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "data bus faults caused by load and store instructions cause a lock-up"]
    #[inline]
    pub fn bfhfnmign_0(self) -> &'a mut W {
        self.variant(BFHFNMIGNW::BFHFNMIGN_0)
    }
    #[doc = "handlers running at priority -1 and -2 ignore data bus faults caused by load and store instructions"]
    #[inline]
    pub fn bfhfnmign_1(self) -> &'a mut W {
        self.variant(BFHFNMIGNW::BFHFNMIGN_1)
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
#[doc = "Values that can be written to the field `STKALIGN`"]
pub enum STKALIGNW {
    #[doc = "4-byte aligned"]
    STKALIGN_0,
    #[doc = "8-byte aligned"]
    STKALIGN_1,
}
impl STKALIGNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STKALIGNW::STKALIGN_0 => false,
            STKALIGNW::STKALIGN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STKALIGNW<'a> {
    w: &'a mut W,
}
impl<'a> _STKALIGNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STKALIGNW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "4-byte aligned"]
    #[inline]
    pub fn stkalign_0(self) -> &'a mut W {
        self.variant(STKALIGNW::STKALIGN_0)
    }
    #[doc = "8-byte aligned"]
    #[inline]
    pub fn stkalign_1(self) -> &'a mut W {
        self.variant(STKALIGNW::STKALIGN_1)
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
#[doc = "Values that can be written to the field `DC`"]
pub enum DCW {
    #[doc = "L1 data cache disabled"]
    DC_0,
    #[doc = "L1 data cache enabled"]
    DC_1,
}
impl DCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DCW::DC_0 => false,
            DCW::DC_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DCW<'a> {
    w: &'a mut W,
}
impl<'a> _DCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "L1 data cache disabled"]
    #[inline]
    pub fn dc_0(self) -> &'a mut W {
        self.variant(DCW::DC_0)
    }
    #[doc = "L1 data cache enabled"]
    #[inline]
    pub fn dc_1(self) -> &'a mut W {
        self.variant(DCW::DC_1)
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
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `IC`"]
pub enum ICW {
    #[doc = "L1 instruction cache disabled"]
    IC_0,
    #[doc = "L1 instruction cache enabled"]
    IC_1,
}
impl ICW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ICW::IC_0 => false,
            ICW::IC_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ICW<'a> {
    w: &'a mut W,
}
impl<'a> _ICW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ICW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "L1 instruction cache disabled"]
    #[inline]
    pub fn ic_0(self) -> &'a mut W {
        self.variant(ICW::IC_0)
    }
    #[doc = "L1 instruction cache enabled"]
    #[inline]
    pub fn ic_1(self) -> &'a mut W {
        self.variant(ICW::IC_1)
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
        const OFFSET: u8 = 17;
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
    #[doc = "Bit 0 - Indicates how the processor enters Thread mode"]
    #[inline]
    pub fn nonbasethrdena(&self) -> NONBASETHRDENAR {
        NONBASETHRDENAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Enables unprivileged software access to the STIR"]
    #[inline]
    pub fn usersetmpend(&self) -> USERSETMPENDR {
        USERSETMPENDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Enables unaligned access traps"]
    #[inline]
    pub fn unalign_trp(&self) -> UNALIGN_TRPR {
        UNALIGN_TRPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Enables faulting or halting when the processor executes an SDIV or UDIV instruction with a divisor of 0"]
    #[inline]
    pub fn div_0_trp(&self) -> DIV_0_TRPR {
        DIV_0_TRPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Enables handlers with priority -1 or -2 to ignore data BusFaults caused by load and store instructions."]
    #[inline]
    pub fn bfhfnmign(&self) -> BFHFNMIGNR {
        BFHFNMIGNR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Indicates stack alignment on exception entry"]
    #[inline]
    pub fn stkalign(&self) -> STKALIGNR {
        STKALIGNR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Enables L1 data cache."]
    #[inline]
    pub fn dc(&self) -> DCR {
        DCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Enables L1 instruction cache."]
    #[inline]
    pub fn ic(&self) -> ICR {
        ICR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Always reads-as-one. It indicates branch prediction is enabled."]
    #[inline]
    pub fn bp(&self) -> BPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BPR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 262144 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Indicates how the processor enters Thread mode"]
    #[inline]
    pub fn nonbasethrdena(&mut self) -> _NONBASETHRDENAW {
        _NONBASETHRDENAW { w: self }
    }
    #[doc = "Bit 1 - Enables unprivileged software access to the STIR"]
    #[inline]
    pub fn usersetmpend(&mut self) -> _USERSETMPENDW {
        _USERSETMPENDW { w: self }
    }
    #[doc = "Bit 3 - Enables unaligned access traps"]
    #[inline]
    pub fn unalign_trp(&mut self) -> _UNALIGN_TRPW {
        _UNALIGN_TRPW { w: self }
    }
    #[doc = "Bit 4 - Enables faulting or halting when the processor executes an SDIV or UDIV instruction with a divisor of 0"]
    #[inline]
    pub fn div_0_trp(&mut self) -> _DIV_0_TRPW {
        _DIV_0_TRPW { w: self }
    }
    #[doc = "Bit 8 - Enables handlers with priority -1 or -2 to ignore data BusFaults caused by load and store instructions."]
    #[inline]
    pub fn bfhfnmign(&mut self) -> _BFHFNMIGNW {
        _BFHFNMIGNW { w: self }
    }
    #[doc = "Bit 9 - Indicates stack alignment on exception entry"]
    #[inline]
    pub fn stkalign(&mut self) -> _STKALIGNW {
        _STKALIGNW { w: self }
    }
    #[doc = "Bit 16 - Enables L1 data cache."]
    #[inline]
    pub fn dc(&mut self) -> _DCW {
        _DCW { w: self }
    }
    #[doc = "Bit 17 - Enables L1 instruction cache."]
    #[inline]
    pub fn ic(&mut self) -> _ICW {
        _ICW { w: self }
    }
}
