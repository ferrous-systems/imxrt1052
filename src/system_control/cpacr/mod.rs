#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CPACR {
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
#[doc = "Possible values of the field `CP0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CP0R {
    #[doc = "Access denied. Any attempted access generates a NOCP UsageFault."]
    CP0_0,
    #[doc = "Privileged access only. An unprivileged access generates a NOCP UsageFault."]
    CP0_1,
    #[doc = "Full access."]
    CP0_3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CP0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CP0R::CP0_0 => 0,
            CP0R::CP0_1 => 1,
            CP0R::CP0_3 => 3,
            CP0R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CP0R {
        match value {
            0 => CP0R::CP0_0,
            1 => CP0R::CP0_1,
            3 => CP0R::CP0_3,
            i => CP0R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CP0_0`"]
    #[inline]
    pub fn is_cp0_0(&self) -> bool {
        *self == CP0R::CP0_0
    }
    #[doc = "Checks if the value of the field is `CP0_1`"]
    #[inline]
    pub fn is_cp0_1(&self) -> bool {
        *self == CP0R::CP0_1
    }
    #[doc = "Checks if the value of the field is `CP0_3`"]
    #[inline]
    pub fn is_cp0_3(&self) -> bool {
        *self == CP0R::CP0_3
    }
}
#[doc = "Possible values of the field `CP1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CP1R {
    #[doc = "Access denied. Any attempted access generates a NOCP UsageFault."]
    CP1_0,
    #[doc = "Privileged access only. An unprivileged access generates a NOCP UsageFault."]
    CP1_1,
    #[doc = "Full access."]
    CP1_3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CP1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CP1R::CP1_0 => 0,
            CP1R::CP1_1 => 1,
            CP1R::CP1_3 => 3,
            CP1R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CP1R {
        match value {
            0 => CP1R::CP1_0,
            1 => CP1R::CP1_1,
            3 => CP1R::CP1_3,
            i => CP1R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CP1_0`"]
    #[inline]
    pub fn is_cp1_0(&self) -> bool {
        *self == CP1R::CP1_0
    }
    #[doc = "Checks if the value of the field is `CP1_1`"]
    #[inline]
    pub fn is_cp1_1(&self) -> bool {
        *self == CP1R::CP1_1
    }
    #[doc = "Checks if the value of the field is `CP1_3`"]
    #[inline]
    pub fn is_cp1_3(&self) -> bool {
        *self == CP1R::CP1_3
    }
}
#[doc = "Possible values of the field `CP2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CP2R {
    #[doc = "Access denied. Any attempted access generates a NOCP UsageFault."]
    CP2_0,
    #[doc = "Privileged access only. An unprivileged access generates a NOCP UsageFault."]
    CP2_1,
    #[doc = "Full access."]
    CP2_3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CP2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CP2R::CP2_0 => 0,
            CP2R::CP2_1 => 1,
            CP2R::CP2_3 => 3,
            CP2R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CP2R {
        match value {
            0 => CP2R::CP2_0,
            1 => CP2R::CP2_1,
            3 => CP2R::CP2_3,
            i => CP2R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CP2_0`"]
    #[inline]
    pub fn is_cp2_0(&self) -> bool {
        *self == CP2R::CP2_0
    }
    #[doc = "Checks if the value of the field is `CP2_1`"]
    #[inline]
    pub fn is_cp2_1(&self) -> bool {
        *self == CP2R::CP2_1
    }
    #[doc = "Checks if the value of the field is `CP2_3`"]
    #[inline]
    pub fn is_cp2_3(&self) -> bool {
        *self == CP2R::CP2_3
    }
}
#[doc = "Possible values of the field `CP3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CP3R {
    #[doc = "Access denied. Any attempted access generates a NOCP UsageFault."]
    CP3_0,
    #[doc = "Privileged access only. An unprivileged access generates a NOCP UsageFault."]
    CP3_1,
    #[doc = "Full access."]
    CP3_3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CP3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CP3R::CP3_0 => 0,
            CP3R::CP3_1 => 1,
            CP3R::CP3_3 => 3,
            CP3R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CP3R {
        match value {
            0 => CP3R::CP3_0,
            1 => CP3R::CP3_1,
            3 => CP3R::CP3_3,
            i => CP3R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CP3_0`"]
    #[inline]
    pub fn is_cp3_0(&self) -> bool {
        *self == CP3R::CP3_0
    }
    #[doc = "Checks if the value of the field is `CP3_1`"]
    #[inline]
    pub fn is_cp3_1(&self) -> bool {
        *self == CP3R::CP3_1
    }
    #[doc = "Checks if the value of the field is `CP3_3`"]
    #[inline]
    pub fn is_cp3_3(&self) -> bool {
        *self == CP3R::CP3_3
    }
}
#[doc = "Possible values of the field `CP4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CP4R {
    #[doc = "Access denied. Any attempted access generates a NOCP UsageFault."]
    CP4_0,
    #[doc = "Privileged access only. An unprivileged access generates a NOCP UsageFault."]
    CP4_1,
    #[doc = "Full access."]
    CP4_3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CP4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CP4R::CP4_0 => 0,
            CP4R::CP4_1 => 1,
            CP4R::CP4_3 => 3,
            CP4R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CP4R {
        match value {
            0 => CP4R::CP4_0,
            1 => CP4R::CP4_1,
            3 => CP4R::CP4_3,
            i => CP4R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CP4_0`"]
    #[inline]
    pub fn is_cp4_0(&self) -> bool {
        *self == CP4R::CP4_0
    }
    #[doc = "Checks if the value of the field is `CP4_1`"]
    #[inline]
    pub fn is_cp4_1(&self) -> bool {
        *self == CP4R::CP4_1
    }
    #[doc = "Checks if the value of the field is `CP4_3`"]
    #[inline]
    pub fn is_cp4_3(&self) -> bool {
        *self == CP4R::CP4_3
    }
}
#[doc = "Possible values of the field `CP5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CP5R {
    #[doc = "Access denied. Any attempted access generates a NOCP UsageFault."]
    CP5_0,
    #[doc = "Privileged access only. An unprivileged access generates a NOCP UsageFault."]
    CP5_1,
    #[doc = "Full access."]
    CP5_3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CP5R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CP5R::CP5_0 => 0,
            CP5R::CP5_1 => 1,
            CP5R::CP5_3 => 3,
            CP5R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CP5R {
        match value {
            0 => CP5R::CP5_0,
            1 => CP5R::CP5_1,
            3 => CP5R::CP5_3,
            i => CP5R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CP5_0`"]
    #[inline]
    pub fn is_cp5_0(&self) -> bool {
        *self == CP5R::CP5_0
    }
    #[doc = "Checks if the value of the field is `CP5_1`"]
    #[inline]
    pub fn is_cp5_1(&self) -> bool {
        *self == CP5R::CP5_1
    }
    #[doc = "Checks if the value of the field is `CP5_3`"]
    #[inline]
    pub fn is_cp5_3(&self) -> bool {
        *self == CP5R::CP5_3
    }
}
#[doc = "Possible values of the field `CP6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CP6R {
    #[doc = "Access denied. Any attempted access generates a NOCP UsageFault."]
    CP6_0,
    #[doc = "Privileged access only. An unprivileged access generates a NOCP UsageFault."]
    CP6_1,
    #[doc = "Full access."]
    CP6_3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CP6R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CP6R::CP6_0 => 0,
            CP6R::CP6_1 => 1,
            CP6R::CP6_3 => 3,
            CP6R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CP6R {
        match value {
            0 => CP6R::CP6_0,
            1 => CP6R::CP6_1,
            3 => CP6R::CP6_3,
            i => CP6R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CP6_0`"]
    #[inline]
    pub fn is_cp6_0(&self) -> bool {
        *self == CP6R::CP6_0
    }
    #[doc = "Checks if the value of the field is `CP6_1`"]
    #[inline]
    pub fn is_cp6_1(&self) -> bool {
        *self == CP6R::CP6_1
    }
    #[doc = "Checks if the value of the field is `CP6_3`"]
    #[inline]
    pub fn is_cp6_3(&self) -> bool {
        *self == CP6R::CP6_3
    }
}
#[doc = "Possible values of the field `CP7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CP7R {
    #[doc = "Access denied. Any attempted access generates a NOCP UsageFault."]
    CP7_0,
    #[doc = "Privileged access only. An unprivileged access generates a NOCP UsageFault."]
    CP7_1,
    #[doc = "Full access."]
    CP7_3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CP7R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CP7R::CP7_0 => 0,
            CP7R::CP7_1 => 1,
            CP7R::CP7_3 => 3,
            CP7R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CP7R {
        match value {
            0 => CP7R::CP7_0,
            1 => CP7R::CP7_1,
            3 => CP7R::CP7_3,
            i => CP7R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CP7_0`"]
    #[inline]
    pub fn is_cp7_0(&self) -> bool {
        *self == CP7R::CP7_0
    }
    #[doc = "Checks if the value of the field is `CP7_1`"]
    #[inline]
    pub fn is_cp7_1(&self) -> bool {
        *self == CP7R::CP7_1
    }
    #[doc = "Checks if the value of the field is `CP7_3`"]
    #[inline]
    pub fn is_cp7_3(&self) -> bool {
        *self == CP7R::CP7_3
    }
}
#[doc = "Possible values of the field `CP10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CP10R {
    #[doc = "Access denied. Any attempted access generates a NOCP UsageFault."]
    CP10_0,
    #[doc = "Privileged access only. An unprivileged access generates a NOCP UsageFault."]
    CP10_1,
    #[doc = "Full access."]
    CP10_3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CP10R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CP10R::CP10_0 => 0,
            CP10R::CP10_1 => 1,
            CP10R::CP10_3 => 3,
            CP10R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CP10R {
        match value {
            0 => CP10R::CP10_0,
            1 => CP10R::CP10_1,
            3 => CP10R::CP10_3,
            i => CP10R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CP10_0`"]
    #[inline]
    pub fn is_cp10_0(&self) -> bool {
        *self == CP10R::CP10_0
    }
    #[doc = "Checks if the value of the field is `CP10_1`"]
    #[inline]
    pub fn is_cp10_1(&self) -> bool {
        *self == CP10R::CP10_1
    }
    #[doc = "Checks if the value of the field is `CP10_3`"]
    #[inline]
    pub fn is_cp10_3(&self) -> bool {
        *self == CP10R::CP10_3
    }
}
#[doc = "Possible values of the field `CP11`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CP11R {
    #[doc = "Access denied. Any attempted access generates a NOCP UsageFault."]
    CP11_0,
    #[doc = "Privileged access only. An unprivileged access generates a NOCP UsageFault."]
    CP11_1,
    #[doc = "Full access."]
    CP11_3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CP11R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CP11R::CP11_0 => 0,
            CP11R::CP11_1 => 1,
            CP11R::CP11_3 => 3,
            CP11R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CP11R {
        match value {
            0 => CP11R::CP11_0,
            1 => CP11R::CP11_1,
            3 => CP11R::CP11_3,
            i => CP11R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CP11_0`"]
    #[inline]
    pub fn is_cp11_0(&self) -> bool {
        *self == CP11R::CP11_0
    }
    #[doc = "Checks if the value of the field is `CP11_1`"]
    #[inline]
    pub fn is_cp11_1(&self) -> bool {
        *self == CP11R::CP11_1
    }
    #[doc = "Checks if the value of the field is `CP11_3`"]
    #[inline]
    pub fn is_cp11_3(&self) -> bool {
        *self == CP11R::CP11_3
    }
}
#[doc = "Values that can be written to the field `CP0`"]
pub enum CP0W {
    #[doc = "Access denied. Any attempted access generates a NOCP UsageFault."]
    CP0_0,
    #[doc = "Privileged access only. An unprivileged access generates a NOCP UsageFault."]
    CP0_1,
    #[doc = "Full access."]
    CP0_3,
}
impl CP0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CP0W::CP0_0 => 0,
            CP0W::CP0_1 => 1,
            CP0W::CP0_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CP0W<'a> {
    w: &'a mut W,
}
impl<'a> _CP0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CP0W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Access denied. Any attempted access generates a NOCP UsageFault."]
    #[inline]
    pub fn cp0_0(self) -> &'a mut W {
        self.variant(CP0W::CP0_0)
    }
    #[doc = "Privileged access only. An unprivileged access generates a NOCP UsageFault."]
    #[inline]
    pub fn cp0_1(self) -> &'a mut W {
        self.variant(CP0W::CP0_1)
    }
    #[doc = "Full access."]
    #[inline]
    pub fn cp0_3(self) -> &'a mut W {
        self.variant(CP0W::CP0_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CP1`"]
pub enum CP1W {
    #[doc = "Access denied. Any attempted access generates a NOCP UsageFault."]
    CP1_0,
    #[doc = "Privileged access only. An unprivileged access generates a NOCP UsageFault."]
    CP1_1,
    #[doc = "Full access."]
    CP1_3,
}
impl CP1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CP1W::CP1_0 => 0,
            CP1W::CP1_1 => 1,
            CP1W::CP1_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CP1W<'a> {
    w: &'a mut W,
}
impl<'a> _CP1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CP1W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Access denied. Any attempted access generates a NOCP UsageFault."]
    #[inline]
    pub fn cp1_0(self) -> &'a mut W {
        self.variant(CP1W::CP1_0)
    }
    #[doc = "Privileged access only. An unprivileged access generates a NOCP UsageFault."]
    #[inline]
    pub fn cp1_1(self) -> &'a mut W {
        self.variant(CP1W::CP1_1)
    }
    #[doc = "Full access."]
    #[inline]
    pub fn cp1_3(self) -> &'a mut W {
        self.variant(CP1W::CP1_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CP2`"]
pub enum CP2W {
    #[doc = "Access denied. Any attempted access generates a NOCP UsageFault."]
    CP2_0,
    #[doc = "Privileged access only. An unprivileged access generates a NOCP UsageFault."]
    CP2_1,
    #[doc = "Full access."]
    CP2_3,
}
impl CP2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CP2W::CP2_0 => 0,
            CP2W::CP2_1 => 1,
            CP2W::CP2_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CP2W<'a> {
    w: &'a mut W,
}
impl<'a> _CP2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CP2W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Access denied. Any attempted access generates a NOCP UsageFault."]
    #[inline]
    pub fn cp2_0(self) -> &'a mut W {
        self.variant(CP2W::CP2_0)
    }
    #[doc = "Privileged access only. An unprivileged access generates a NOCP UsageFault."]
    #[inline]
    pub fn cp2_1(self) -> &'a mut W {
        self.variant(CP2W::CP2_1)
    }
    #[doc = "Full access."]
    #[inline]
    pub fn cp2_3(self) -> &'a mut W {
        self.variant(CP2W::CP2_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CP3`"]
pub enum CP3W {
    #[doc = "Access denied. Any attempted access generates a NOCP UsageFault."]
    CP3_0,
    #[doc = "Privileged access only. An unprivileged access generates a NOCP UsageFault."]
    CP3_1,
    #[doc = "Full access."]
    CP3_3,
}
impl CP3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CP3W::CP3_0 => 0,
            CP3W::CP3_1 => 1,
            CP3W::CP3_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CP3W<'a> {
    w: &'a mut W,
}
impl<'a> _CP3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CP3W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Access denied. Any attempted access generates a NOCP UsageFault."]
    #[inline]
    pub fn cp3_0(self) -> &'a mut W {
        self.variant(CP3W::CP3_0)
    }
    #[doc = "Privileged access only. An unprivileged access generates a NOCP UsageFault."]
    #[inline]
    pub fn cp3_1(self) -> &'a mut W {
        self.variant(CP3W::CP3_1)
    }
    #[doc = "Full access."]
    #[inline]
    pub fn cp3_3(self) -> &'a mut W {
        self.variant(CP3W::CP3_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CP4`"]
pub enum CP4W {
    #[doc = "Access denied. Any attempted access generates a NOCP UsageFault."]
    CP4_0,
    #[doc = "Privileged access only. An unprivileged access generates a NOCP UsageFault."]
    CP4_1,
    #[doc = "Full access."]
    CP4_3,
}
impl CP4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CP4W::CP4_0 => 0,
            CP4W::CP4_1 => 1,
            CP4W::CP4_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CP4W<'a> {
    w: &'a mut W,
}
impl<'a> _CP4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CP4W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Access denied. Any attempted access generates a NOCP UsageFault."]
    #[inline]
    pub fn cp4_0(self) -> &'a mut W {
        self.variant(CP4W::CP4_0)
    }
    #[doc = "Privileged access only. An unprivileged access generates a NOCP UsageFault."]
    #[inline]
    pub fn cp4_1(self) -> &'a mut W {
        self.variant(CP4W::CP4_1)
    }
    #[doc = "Full access."]
    #[inline]
    pub fn cp4_3(self) -> &'a mut W {
        self.variant(CP4W::CP4_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CP5`"]
pub enum CP5W {
    #[doc = "Access denied. Any attempted access generates a NOCP UsageFault."]
    CP5_0,
    #[doc = "Privileged access only. An unprivileged access generates a NOCP UsageFault."]
    CP5_1,
    #[doc = "Full access."]
    CP5_3,
}
impl CP5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CP5W::CP5_0 => 0,
            CP5W::CP5_1 => 1,
            CP5W::CP5_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CP5W<'a> {
    w: &'a mut W,
}
impl<'a> _CP5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CP5W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Access denied. Any attempted access generates a NOCP UsageFault."]
    #[inline]
    pub fn cp5_0(self) -> &'a mut W {
        self.variant(CP5W::CP5_0)
    }
    #[doc = "Privileged access only. An unprivileged access generates a NOCP UsageFault."]
    #[inline]
    pub fn cp5_1(self) -> &'a mut W {
        self.variant(CP5W::CP5_1)
    }
    #[doc = "Full access."]
    #[inline]
    pub fn cp5_3(self) -> &'a mut W {
        self.variant(CP5W::CP5_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CP6`"]
pub enum CP6W {
    #[doc = "Access denied. Any attempted access generates a NOCP UsageFault."]
    CP6_0,
    #[doc = "Privileged access only. An unprivileged access generates a NOCP UsageFault."]
    CP6_1,
    #[doc = "Full access."]
    CP6_3,
}
impl CP6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CP6W::CP6_0 => 0,
            CP6W::CP6_1 => 1,
            CP6W::CP6_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CP6W<'a> {
    w: &'a mut W,
}
impl<'a> _CP6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CP6W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Access denied. Any attempted access generates a NOCP UsageFault."]
    #[inline]
    pub fn cp6_0(self) -> &'a mut W {
        self.variant(CP6W::CP6_0)
    }
    #[doc = "Privileged access only. An unprivileged access generates a NOCP UsageFault."]
    #[inline]
    pub fn cp6_1(self) -> &'a mut W {
        self.variant(CP6W::CP6_1)
    }
    #[doc = "Full access."]
    #[inline]
    pub fn cp6_3(self) -> &'a mut W {
        self.variant(CP6W::CP6_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CP7`"]
pub enum CP7W {
    #[doc = "Access denied. Any attempted access generates a NOCP UsageFault."]
    CP7_0,
    #[doc = "Privileged access only. An unprivileged access generates a NOCP UsageFault."]
    CP7_1,
    #[doc = "Full access."]
    CP7_3,
}
impl CP7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CP7W::CP7_0 => 0,
            CP7W::CP7_1 => 1,
            CP7W::CP7_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CP7W<'a> {
    w: &'a mut W,
}
impl<'a> _CP7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CP7W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Access denied. Any attempted access generates a NOCP UsageFault."]
    #[inline]
    pub fn cp7_0(self) -> &'a mut W {
        self.variant(CP7W::CP7_0)
    }
    #[doc = "Privileged access only. An unprivileged access generates a NOCP UsageFault."]
    #[inline]
    pub fn cp7_1(self) -> &'a mut W {
        self.variant(CP7W::CP7_1)
    }
    #[doc = "Full access."]
    #[inline]
    pub fn cp7_3(self) -> &'a mut W {
        self.variant(CP7W::CP7_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CP10`"]
pub enum CP10W {
    #[doc = "Access denied. Any attempted access generates a NOCP UsageFault."]
    CP10_0,
    #[doc = "Privileged access only. An unprivileged access generates a NOCP UsageFault."]
    CP10_1,
    #[doc = "Full access."]
    CP10_3,
}
impl CP10W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CP10W::CP10_0 => 0,
            CP10W::CP10_1 => 1,
            CP10W::CP10_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CP10W<'a> {
    w: &'a mut W,
}
impl<'a> _CP10W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CP10W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Access denied. Any attempted access generates a NOCP UsageFault."]
    #[inline]
    pub fn cp10_0(self) -> &'a mut W {
        self.variant(CP10W::CP10_0)
    }
    #[doc = "Privileged access only. An unprivileged access generates a NOCP UsageFault."]
    #[inline]
    pub fn cp10_1(self) -> &'a mut W {
        self.variant(CP10W::CP10_1)
    }
    #[doc = "Full access."]
    #[inline]
    pub fn cp10_3(self) -> &'a mut W {
        self.variant(CP10W::CP10_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CP11`"]
pub enum CP11W {
    #[doc = "Access denied. Any attempted access generates a NOCP UsageFault."]
    CP11_0,
    #[doc = "Privileged access only. An unprivileged access generates a NOCP UsageFault."]
    CP11_1,
    #[doc = "Full access."]
    CP11_3,
}
impl CP11W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CP11W::CP11_0 => 0,
            CP11W::CP11_1 => 1,
            CP11W::CP11_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CP11W<'a> {
    w: &'a mut W,
}
impl<'a> _CP11W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CP11W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Access denied. Any attempted access generates a NOCP UsageFault."]
    #[inline]
    pub fn cp11_0(self) -> &'a mut W {
        self.variant(CP11W::CP11_0)
    }
    #[doc = "Privileged access only. An unprivileged access generates a NOCP UsageFault."]
    #[inline]
    pub fn cp11_1(self) -> &'a mut W {
        self.variant(CP11W::CP11_1)
    }
    #[doc = "Full access."]
    #[inline]
    pub fn cp11_3(self) -> &'a mut W {
        self.variant(CP11W::CP11_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 22;
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
    #[doc = "Bits 0:1 - Access privileges for coprocessor 0."]
    #[inline]
    pub fn cp0(&self) -> CP0R {
        CP0R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:3 - Access privileges for coprocessor 1."]
    #[inline]
    pub fn cp1(&self) -> CP1R {
        CP1R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5 - Access privileges for coprocessor 2."]
    #[inline]
    pub fn cp2(&self) -> CP2R {
        CP2R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 6:7 - Access privileges for coprocessor 3."]
    #[inline]
    pub fn cp3(&self) -> CP3R {
        CP3R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9 - Access privileges for coprocessor 4."]
    #[inline]
    pub fn cp4(&self) -> CP4R {
        CP4R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 10:11 - Access privileges for coprocessor 5."]
    #[inline]
    pub fn cp5(&self) -> CP5R {
        CP5R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:13 - Access privileges for coprocessor 6."]
    #[inline]
    pub fn cp6(&self) -> CP6R {
        CP6R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 14:15 - Access privileges for coprocessor 7."]
    #[inline]
    pub fn cp7(&self) -> CP7R {
        CP7R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:21 - Access privileges for coprocessor 10."]
    #[inline]
    pub fn cp10(&self) -> CP10R {
        CP10R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 22:23 - Access privileges for coprocessor 11."]
    #[inline]
    pub fn cp11(&self) -> CP11R {
        CP11R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 22;
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
    #[doc = "Bits 0:1 - Access privileges for coprocessor 0."]
    #[inline]
    pub fn cp0(&mut self) -> _CP0W {
        _CP0W { w: self }
    }
    #[doc = "Bits 2:3 - Access privileges for coprocessor 1."]
    #[inline]
    pub fn cp1(&mut self) -> _CP1W {
        _CP1W { w: self }
    }
    #[doc = "Bits 4:5 - Access privileges for coprocessor 2."]
    #[inline]
    pub fn cp2(&mut self) -> _CP2W {
        _CP2W { w: self }
    }
    #[doc = "Bits 6:7 - Access privileges for coprocessor 3."]
    #[inline]
    pub fn cp3(&mut self) -> _CP3W {
        _CP3W { w: self }
    }
    #[doc = "Bits 8:9 - Access privileges for coprocessor 4."]
    #[inline]
    pub fn cp4(&mut self) -> _CP4W {
        _CP4W { w: self }
    }
    #[doc = "Bits 10:11 - Access privileges for coprocessor 5."]
    #[inline]
    pub fn cp5(&mut self) -> _CP5W {
        _CP5W { w: self }
    }
    #[doc = "Bits 12:13 - Access privileges for coprocessor 6."]
    #[inline]
    pub fn cp6(&mut self) -> _CP6W {
        _CP6W { w: self }
    }
    #[doc = "Bits 14:15 - Access privileges for coprocessor 7."]
    #[inline]
    pub fn cp7(&mut self) -> _CP7W {
        _CP7W { w: self }
    }
    #[doc = "Bits 20:21 - Access privileges for coprocessor 10."]
    #[inline]
    pub fn cp10(&mut self) -> _CP10W {
        _CP10W { w: self }
    }
    #[doc = "Bits 22:23 - Access privileges for coprocessor 11."]
    #[inline]
    pub fn cp11(&mut self) -> _CP11W {
        _CP11W { w: self }
    }
}
