#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SHCSR {
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
#[doc = "Possible values of the field `MEMFAULTACT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MEMFAULTACTR {
    #[doc = "exception is not active"]
    MEMFAULTACT_0,
    #[doc = "exception is active"]
    MEMFAULTACT_1,
}
impl MEMFAULTACTR {
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
            MEMFAULTACTR::MEMFAULTACT_0 => false,
            MEMFAULTACTR::MEMFAULTACT_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MEMFAULTACTR {
        match value {
            false => MEMFAULTACTR::MEMFAULTACT_0,
            true => MEMFAULTACTR::MEMFAULTACT_1,
        }
    }
    #[doc = "Checks if the value of the field is `MEMFAULTACT_0`"]
    #[inline]
    pub fn is_memfaultact_0(&self) -> bool {
        *self == MEMFAULTACTR::MEMFAULTACT_0
    }
    #[doc = "Checks if the value of the field is `MEMFAULTACT_1`"]
    #[inline]
    pub fn is_memfaultact_1(&self) -> bool {
        *self == MEMFAULTACTR::MEMFAULTACT_1
    }
}
#[doc = "Possible values of the field `BUSFAULTACT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUSFAULTACTR {
    #[doc = "exception is not active"]
    BUSFAULTACT_0,
    #[doc = "exception is active"]
    BUSFAULTACT_1,
}
impl BUSFAULTACTR {
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
            BUSFAULTACTR::BUSFAULTACT_0 => false,
            BUSFAULTACTR::BUSFAULTACT_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BUSFAULTACTR {
        match value {
            false => BUSFAULTACTR::BUSFAULTACT_0,
            true => BUSFAULTACTR::BUSFAULTACT_1,
        }
    }
    #[doc = "Checks if the value of the field is `BUSFAULTACT_0`"]
    #[inline]
    pub fn is_busfaultact_0(&self) -> bool {
        *self == BUSFAULTACTR::BUSFAULTACT_0
    }
    #[doc = "Checks if the value of the field is `BUSFAULTACT_1`"]
    #[inline]
    pub fn is_busfaultact_1(&self) -> bool {
        *self == BUSFAULTACTR::BUSFAULTACT_1
    }
}
#[doc = "Possible values of the field `USGFAULTACT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USGFAULTACTR {
    #[doc = "exception is not active"]
    USGFAULTACT_0,
    #[doc = "exception is active"]
    USGFAULTACT_1,
}
impl USGFAULTACTR {
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
            USGFAULTACTR::USGFAULTACT_0 => false,
            USGFAULTACTR::USGFAULTACT_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USGFAULTACTR {
        match value {
            false => USGFAULTACTR::USGFAULTACT_0,
            true => USGFAULTACTR::USGFAULTACT_1,
        }
    }
    #[doc = "Checks if the value of the field is `USGFAULTACT_0`"]
    #[inline]
    pub fn is_usgfaultact_0(&self) -> bool {
        *self == USGFAULTACTR::USGFAULTACT_0
    }
    #[doc = "Checks if the value of the field is `USGFAULTACT_1`"]
    #[inline]
    pub fn is_usgfaultact_1(&self) -> bool {
        *self == USGFAULTACTR::USGFAULTACT_1
    }
}
#[doc = "Possible values of the field `SVCALLACT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SVCALLACTR {
    #[doc = "exception is not active"]
    SVCALLACT_0,
    #[doc = "exception is active"]
    SVCALLACT_1,
}
impl SVCALLACTR {
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
            SVCALLACTR::SVCALLACT_0 => false,
            SVCALLACTR::SVCALLACT_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SVCALLACTR {
        match value {
            false => SVCALLACTR::SVCALLACT_0,
            true => SVCALLACTR::SVCALLACT_1,
        }
    }
    #[doc = "Checks if the value of the field is `SVCALLACT_0`"]
    #[inline]
    pub fn is_svcallact_0(&self) -> bool {
        *self == SVCALLACTR::SVCALLACT_0
    }
    #[doc = "Checks if the value of the field is `SVCALLACT_1`"]
    #[inline]
    pub fn is_svcallact_1(&self) -> bool {
        *self == SVCALLACTR::SVCALLACT_1
    }
}
#[doc = "Possible values of the field `MONITORACT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MONITORACTR {
    #[doc = "exception is not active"]
    MONITORACT_0,
    #[doc = "exception is active"]
    MONITORACT_1,
}
impl MONITORACTR {
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
            MONITORACTR::MONITORACT_0 => false,
            MONITORACTR::MONITORACT_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MONITORACTR {
        match value {
            false => MONITORACTR::MONITORACT_0,
            true => MONITORACTR::MONITORACT_1,
        }
    }
    #[doc = "Checks if the value of the field is `MONITORACT_0`"]
    #[inline]
    pub fn is_monitoract_0(&self) -> bool {
        *self == MONITORACTR::MONITORACT_0
    }
    #[doc = "Checks if the value of the field is `MONITORACT_1`"]
    #[inline]
    pub fn is_monitoract_1(&self) -> bool {
        *self == MONITORACTR::MONITORACT_1
    }
}
#[doc = "Possible values of the field `PENDSVACT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PENDSVACTR {
    #[doc = "exception is not active"]
    PENDSVACT_0,
    #[doc = "exception is active"]
    PENDSVACT_1,
}
impl PENDSVACTR {
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
            PENDSVACTR::PENDSVACT_0 => false,
            PENDSVACTR::PENDSVACT_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PENDSVACTR {
        match value {
            false => PENDSVACTR::PENDSVACT_0,
            true => PENDSVACTR::PENDSVACT_1,
        }
    }
    #[doc = "Checks if the value of the field is `PENDSVACT_0`"]
    #[inline]
    pub fn is_pendsvact_0(&self) -> bool {
        *self == PENDSVACTR::PENDSVACT_0
    }
    #[doc = "Checks if the value of the field is `PENDSVACT_1`"]
    #[inline]
    pub fn is_pendsvact_1(&self) -> bool {
        *self == PENDSVACTR::PENDSVACT_1
    }
}
#[doc = "Possible values of the field `SYSTICKACT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSTICKACTR {
    #[doc = "exception is not active"]
    SYSTICKACT_0,
    #[doc = "exception is active"]
    SYSTICKACT_1,
}
impl SYSTICKACTR {
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
            SYSTICKACTR::SYSTICKACT_0 => false,
            SYSTICKACTR::SYSTICKACT_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SYSTICKACTR {
        match value {
            false => SYSTICKACTR::SYSTICKACT_0,
            true => SYSTICKACTR::SYSTICKACT_1,
        }
    }
    #[doc = "Checks if the value of the field is `SYSTICKACT_0`"]
    #[inline]
    pub fn is_systickact_0(&self) -> bool {
        *self == SYSTICKACTR::SYSTICKACT_0
    }
    #[doc = "Checks if the value of the field is `SYSTICKACT_1`"]
    #[inline]
    pub fn is_systickact_1(&self) -> bool {
        *self == SYSTICKACTR::SYSTICKACT_1
    }
}
#[doc = "Possible values of the field `USGFAULTPENDED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USGFAULTPENDEDR {
    #[doc = "exception is not pending"]
    USGFAULTPENDED_0,
    #[doc = "exception is pending"]
    USGFAULTPENDED_1,
}
impl USGFAULTPENDEDR {
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
            USGFAULTPENDEDR::USGFAULTPENDED_0 => false,
            USGFAULTPENDEDR::USGFAULTPENDED_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USGFAULTPENDEDR {
        match value {
            false => USGFAULTPENDEDR::USGFAULTPENDED_0,
            true => USGFAULTPENDEDR::USGFAULTPENDED_1,
        }
    }
    #[doc = "Checks if the value of the field is `USGFAULTPENDED_0`"]
    #[inline]
    pub fn is_usgfaultpended_0(&self) -> bool {
        *self == USGFAULTPENDEDR::USGFAULTPENDED_0
    }
    #[doc = "Checks if the value of the field is `USGFAULTPENDED_1`"]
    #[inline]
    pub fn is_usgfaultpended_1(&self) -> bool {
        *self == USGFAULTPENDEDR::USGFAULTPENDED_1
    }
}
#[doc = "Possible values of the field `MEMFAULTPENDED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MEMFAULTPENDEDR {
    #[doc = "exception is not pending"]
    MEMFAULTPENDED_0,
    #[doc = "exception is pending"]
    MEMFAULTPENDED_1,
}
impl MEMFAULTPENDEDR {
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
            MEMFAULTPENDEDR::MEMFAULTPENDED_0 => false,
            MEMFAULTPENDEDR::MEMFAULTPENDED_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MEMFAULTPENDEDR {
        match value {
            false => MEMFAULTPENDEDR::MEMFAULTPENDED_0,
            true => MEMFAULTPENDEDR::MEMFAULTPENDED_1,
        }
    }
    #[doc = "Checks if the value of the field is `MEMFAULTPENDED_0`"]
    #[inline]
    pub fn is_memfaultpended_0(&self) -> bool {
        *self == MEMFAULTPENDEDR::MEMFAULTPENDED_0
    }
    #[doc = "Checks if the value of the field is `MEMFAULTPENDED_1`"]
    #[inline]
    pub fn is_memfaultpended_1(&self) -> bool {
        *self == MEMFAULTPENDEDR::MEMFAULTPENDED_1
    }
}
#[doc = "Possible values of the field `BUSFAULTPENDED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUSFAULTPENDEDR {
    #[doc = "exception is not pending"]
    BUSFAULTPENDED_0,
    #[doc = "exception is pending"]
    BUSFAULTPENDED_1,
}
impl BUSFAULTPENDEDR {
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
            BUSFAULTPENDEDR::BUSFAULTPENDED_0 => false,
            BUSFAULTPENDEDR::BUSFAULTPENDED_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BUSFAULTPENDEDR {
        match value {
            false => BUSFAULTPENDEDR::BUSFAULTPENDED_0,
            true => BUSFAULTPENDEDR::BUSFAULTPENDED_1,
        }
    }
    #[doc = "Checks if the value of the field is `BUSFAULTPENDED_0`"]
    #[inline]
    pub fn is_busfaultpended_0(&self) -> bool {
        *self == BUSFAULTPENDEDR::BUSFAULTPENDED_0
    }
    #[doc = "Checks if the value of the field is `BUSFAULTPENDED_1`"]
    #[inline]
    pub fn is_busfaultpended_1(&self) -> bool {
        *self == BUSFAULTPENDEDR::BUSFAULTPENDED_1
    }
}
#[doc = "Possible values of the field `SVCALLPENDED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SVCALLPENDEDR {
    #[doc = "exception is not pending"]
    SVCALLPENDED_0,
    #[doc = "exception is pending"]
    SVCALLPENDED_1,
}
impl SVCALLPENDEDR {
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
            SVCALLPENDEDR::SVCALLPENDED_0 => false,
            SVCALLPENDEDR::SVCALLPENDED_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SVCALLPENDEDR {
        match value {
            false => SVCALLPENDEDR::SVCALLPENDED_0,
            true => SVCALLPENDEDR::SVCALLPENDED_1,
        }
    }
    #[doc = "Checks if the value of the field is `SVCALLPENDED_0`"]
    #[inline]
    pub fn is_svcallpended_0(&self) -> bool {
        *self == SVCALLPENDEDR::SVCALLPENDED_0
    }
    #[doc = "Checks if the value of the field is `SVCALLPENDED_1`"]
    #[inline]
    pub fn is_svcallpended_1(&self) -> bool {
        *self == SVCALLPENDEDR::SVCALLPENDED_1
    }
}
#[doc = "Possible values of the field `MEMFAULTENA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MEMFAULTENAR {
    #[doc = "disable the exception"]
    MEMFAULTENA_0,
    #[doc = "enable the exception"]
    MEMFAULTENA_1,
}
impl MEMFAULTENAR {
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
            MEMFAULTENAR::MEMFAULTENA_0 => false,
            MEMFAULTENAR::MEMFAULTENA_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MEMFAULTENAR {
        match value {
            false => MEMFAULTENAR::MEMFAULTENA_0,
            true => MEMFAULTENAR::MEMFAULTENA_1,
        }
    }
    #[doc = "Checks if the value of the field is `MEMFAULTENA_0`"]
    #[inline]
    pub fn is_memfaultena_0(&self) -> bool {
        *self == MEMFAULTENAR::MEMFAULTENA_0
    }
    #[doc = "Checks if the value of the field is `MEMFAULTENA_1`"]
    #[inline]
    pub fn is_memfaultena_1(&self) -> bool {
        *self == MEMFAULTENAR::MEMFAULTENA_1
    }
}
#[doc = "Possible values of the field `BUSFAULTENA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUSFAULTENAR {
    #[doc = "disable the exception"]
    BUSFAULTENA_0,
    #[doc = "enable the exception"]
    BUSFAULTENA_1,
}
impl BUSFAULTENAR {
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
            BUSFAULTENAR::BUSFAULTENA_0 => false,
            BUSFAULTENAR::BUSFAULTENA_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BUSFAULTENAR {
        match value {
            false => BUSFAULTENAR::BUSFAULTENA_0,
            true => BUSFAULTENAR::BUSFAULTENA_1,
        }
    }
    #[doc = "Checks if the value of the field is `BUSFAULTENA_0`"]
    #[inline]
    pub fn is_busfaultena_0(&self) -> bool {
        *self == BUSFAULTENAR::BUSFAULTENA_0
    }
    #[doc = "Checks if the value of the field is `BUSFAULTENA_1`"]
    #[inline]
    pub fn is_busfaultena_1(&self) -> bool {
        *self == BUSFAULTENAR::BUSFAULTENA_1
    }
}
#[doc = "Possible values of the field `USGFAULTENA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USGFAULTENAR {
    #[doc = "disable the exception"]
    USGFAULTENA_0,
    #[doc = "enable the exception"]
    USGFAULTENA_1,
}
impl USGFAULTENAR {
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
            USGFAULTENAR::USGFAULTENA_0 => false,
            USGFAULTENAR::USGFAULTENA_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USGFAULTENAR {
        match value {
            false => USGFAULTENAR::USGFAULTENA_0,
            true => USGFAULTENAR::USGFAULTENA_1,
        }
    }
    #[doc = "Checks if the value of the field is `USGFAULTENA_0`"]
    #[inline]
    pub fn is_usgfaultena_0(&self) -> bool {
        *self == USGFAULTENAR::USGFAULTENA_0
    }
    #[doc = "Checks if the value of the field is `USGFAULTENA_1`"]
    #[inline]
    pub fn is_usgfaultena_1(&self) -> bool {
        *self == USGFAULTENAR::USGFAULTENA_1
    }
}
#[doc = "Values that can be written to the field `MEMFAULTACT`"]
pub enum MEMFAULTACTW {
    #[doc = "exception is not active"]
    MEMFAULTACT_0,
    #[doc = "exception is active"]
    MEMFAULTACT_1,
}
impl MEMFAULTACTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MEMFAULTACTW::MEMFAULTACT_0 => false,
            MEMFAULTACTW::MEMFAULTACT_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MEMFAULTACTW<'a> {
    w: &'a mut W,
}
impl<'a> _MEMFAULTACTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MEMFAULTACTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "exception is not active"]
    #[inline]
    pub fn memfaultact_0(self) -> &'a mut W {
        self.variant(MEMFAULTACTW::MEMFAULTACT_0)
    }
    #[doc = "exception is active"]
    #[inline]
    pub fn memfaultact_1(self) -> &'a mut W {
        self.variant(MEMFAULTACTW::MEMFAULTACT_1)
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
#[doc = "Values that can be written to the field `BUSFAULTACT`"]
pub enum BUSFAULTACTW {
    #[doc = "exception is not active"]
    BUSFAULTACT_0,
    #[doc = "exception is active"]
    BUSFAULTACT_1,
}
impl BUSFAULTACTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BUSFAULTACTW::BUSFAULTACT_0 => false,
            BUSFAULTACTW::BUSFAULTACT_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BUSFAULTACTW<'a> {
    w: &'a mut W,
}
impl<'a> _BUSFAULTACTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BUSFAULTACTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "exception is not active"]
    #[inline]
    pub fn busfaultact_0(self) -> &'a mut W {
        self.variant(BUSFAULTACTW::BUSFAULTACT_0)
    }
    #[doc = "exception is active"]
    #[inline]
    pub fn busfaultact_1(self) -> &'a mut W {
        self.variant(BUSFAULTACTW::BUSFAULTACT_1)
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
#[doc = "Values that can be written to the field `USGFAULTACT`"]
pub enum USGFAULTACTW {
    #[doc = "exception is not active"]
    USGFAULTACT_0,
    #[doc = "exception is active"]
    USGFAULTACT_1,
}
impl USGFAULTACTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USGFAULTACTW::USGFAULTACT_0 => false,
            USGFAULTACTW::USGFAULTACT_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USGFAULTACTW<'a> {
    w: &'a mut W,
}
impl<'a> _USGFAULTACTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USGFAULTACTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "exception is not active"]
    #[inline]
    pub fn usgfaultact_0(self) -> &'a mut W {
        self.variant(USGFAULTACTW::USGFAULTACT_0)
    }
    #[doc = "exception is active"]
    #[inline]
    pub fn usgfaultact_1(self) -> &'a mut W {
        self.variant(USGFAULTACTW::USGFAULTACT_1)
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
#[doc = "Values that can be written to the field `SVCALLACT`"]
pub enum SVCALLACTW {
    #[doc = "exception is not active"]
    SVCALLACT_0,
    #[doc = "exception is active"]
    SVCALLACT_1,
}
impl SVCALLACTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SVCALLACTW::SVCALLACT_0 => false,
            SVCALLACTW::SVCALLACT_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SVCALLACTW<'a> {
    w: &'a mut W,
}
impl<'a> _SVCALLACTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SVCALLACTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "exception is not active"]
    #[inline]
    pub fn svcallact_0(self) -> &'a mut W {
        self.variant(SVCALLACTW::SVCALLACT_0)
    }
    #[doc = "exception is active"]
    #[inline]
    pub fn svcallact_1(self) -> &'a mut W {
        self.variant(SVCALLACTW::SVCALLACT_1)
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MONITORACT`"]
pub enum MONITORACTW {
    #[doc = "exception is not active"]
    MONITORACT_0,
    #[doc = "exception is active"]
    MONITORACT_1,
}
impl MONITORACTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MONITORACTW::MONITORACT_0 => false,
            MONITORACTW::MONITORACT_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MONITORACTW<'a> {
    w: &'a mut W,
}
impl<'a> _MONITORACTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MONITORACTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "exception is not active"]
    #[inline]
    pub fn monitoract_0(self) -> &'a mut W {
        self.variant(MONITORACTW::MONITORACT_0)
    }
    #[doc = "exception is active"]
    #[inline]
    pub fn monitoract_1(self) -> &'a mut W {
        self.variant(MONITORACTW::MONITORACT_1)
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
#[doc = "Values that can be written to the field `PENDSVACT`"]
pub enum PENDSVACTW {
    #[doc = "exception is not active"]
    PENDSVACT_0,
    #[doc = "exception is active"]
    PENDSVACT_1,
}
impl PENDSVACTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PENDSVACTW::PENDSVACT_0 => false,
            PENDSVACTW::PENDSVACT_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PENDSVACTW<'a> {
    w: &'a mut W,
}
impl<'a> _PENDSVACTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PENDSVACTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "exception is not active"]
    #[inline]
    pub fn pendsvact_0(self) -> &'a mut W {
        self.variant(PENDSVACTW::PENDSVACT_0)
    }
    #[doc = "exception is active"]
    #[inline]
    pub fn pendsvact_1(self) -> &'a mut W {
        self.variant(PENDSVACTW::PENDSVACT_1)
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SYSTICKACT`"]
pub enum SYSTICKACTW {
    #[doc = "exception is not active"]
    SYSTICKACT_0,
    #[doc = "exception is active"]
    SYSTICKACT_1,
}
impl SYSTICKACTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SYSTICKACTW::SYSTICKACT_0 => false,
            SYSTICKACTW::SYSTICKACT_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SYSTICKACTW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSTICKACTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SYSTICKACTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "exception is not active"]
    #[inline]
    pub fn systickact_0(self) -> &'a mut W {
        self.variant(SYSTICKACTW::SYSTICKACT_0)
    }
    #[doc = "exception is active"]
    #[inline]
    pub fn systickact_1(self) -> &'a mut W {
        self.variant(SYSTICKACTW::SYSTICKACT_1)
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `USGFAULTPENDED`"]
pub enum USGFAULTPENDEDW {
    #[doc = "exception is not pending"]
    USGFAULTPENDED_0,
    #[doc = "exception is pending"]
    USGFAULTPENDED_1,
}
impl USGFAULTPENDEDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USGFAULTPENDEDW::USGFAULTPENDED_0 => false,
            USGFAULTPENDEDW::USGFAULTPENDED_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USGFAULTPENDEDW<'a> {
    w: &'a mut W,
}
impl<'a> _USGFAULTPENDEDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USGFAULTPENDEDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "exception is not pending"]
    #[inline]
    pub fn usgfaultpended_0(self) -> &'a mut W {
        self.variant(USGFAULTPENDEDW::USGFAULTPENDED_0)
    }
    #[doc = "exception is pending"]
    #[inline]
    pub fn usgfaultpended_1(self) -> &'a mut W {
        self.variant(USGFAULTPENDEDW::USGFAULTPENDED_1)
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MEMFAULTPENDED`"]
pub enum MEMFAULTPENDEDW {
    #[doc = "exception is not pending"]
    MEMFAULTPENDED_0,
    #[doc = "exception is pending"]
    MEMFAULTPENDED_1,
}
impl MEMFAULTPENDEDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MEMFAULTPENDEDW::MEMFAULTPENDED_0 => false,
            MEMFAULTPENDEDW::MEMFAULTPENDED_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MEMFAULTPENDEDW<'a> {
    w: &'a mut W,
}
impl<'a> _MEMFAULTPENDEDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MEMFAULTPENDEDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "exception is not pending"]
    #[inline]
    pub fn memfaultpended_0(self) -> &'a mut W {
        self.variant(MEMFAULTPENDEDW::MEMFAULTPENDED_0)
    }
    #[doc = "exception is pending"]
    #[inline]
    pub fn memfaultpended_1(self) -> &'a mut W {
        self.variant(MEMFAULTPENDEDW::MEMFAULTPENDED_1)
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BUSFAULTPENDED`"]
pub enum BUSFAULTPENDEDW {
    #[doc = "exception is not pending"]
    BUSFAULTPENDED_0,
    #[doc = "exception is pending"]
    BUSFAULTPENDED_1,
}
impl BUSFAULTPENDEDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BUSFAULTPENDEDW::BUSFAULTPENDED_0 => false,
            BUSFAULTPENDEDW::BUSFAULTPENDED_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BUSFAULTPENDEDW<'a> {
    w: &'a mut W,
}
impl<'a> _BUSFAULTPENDEDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BUSFAULTPENDEDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "exception is not pending"]
    #[inline]
    pub fn busfaultpended_0(self) -> &'a mut W {
        self.variant(BUSFAULTPENDEDW::BUSFAULTPENDED_0)
    }
    #[doc = "exception is pending"]
    #[inline]
    pub fn busfaultpended_1(self) -> &'a mut W {
        self.variant(BUSFAULTPENDEDW::BUSFAULTPENDED_1)
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
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SVCALLPENDED`"]
pub enum SVCALLPENDEDW {
    #[doc = "exception is not pending"]
    SVCALLPENDED_0,
    #[doc = "exception is pending"]
    SVCALLPENDED_1,
}
impl SVCALLPENDEDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SVCALLPENDEDW::SVCALLPENDED_0 => false,
            SVCALLPENDEDW::SVCALLPENDED_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SVCALLPENDEDW<'a> {
    w: &'a mut W,
}
impl<'a> _SVCALLPENDEDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SVCALLPENDEDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "exception is not pending"]
    #[inline]
    pub fn svcallpended_0(self) -> &'a mut W {
        self.variant(SVCALLPENDEDW::SVCALLPENDED_0)
    }
    #[doc = "exception is pending"]
    #[inline]
    pub fn svcallpended_1(self) -> &'a mut W {
        self.variant(SVCALLPENDEDW::SVCALLPENDED_1)
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
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MEMFAULTENA`"]
pub enum MEMFAULTENAW {
    #[doc = "disable the exception"]
    MEMFAULTENA_0,
    #[doc = "enable the exception"]
    MEMFAULTENA_1,
}
impl MEMFAULTENAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MEMFAULTENAW::MEMFAULTENA_0 => false,
            MEMFAULTENAW::MEMFAULTENA_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MEMFAULTENAW<'a> {
    w: &'a mut W,
}
impl<'a> _MEMFAULTENAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MEMFAULTENAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "disable the exception"]
    #[inline]
    pub fn memfaultena_0(self) -> &'a mut W {
        self.variant(MEMFAULTENAW::MEMFAULTENA_0)
    }
    #[doc = "enable the exception"]
    #[inline]
    pub fn memfaultena_1(self) -> &'a mut W {
        self.variant(MEMFAULTENAW::MEMFAULTENA_1)
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
#[doc = "Values that can be written to the field `BUSFAULTENA`"]
pub enum BUSFAULTENAW {
    #[doc = "disable the exception"]
    BUSFAULTENA_0,
    #[doc = "enable the exception"]
    BUSFAULTENA_1,
}
impl BUSFAULTENAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BUSFAULTENAW::BUSFAULTENA_0 => false,
            BUSFAULTENAW::BUSFAULTENA_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BUSFAULTENAW<'a> {
    w: &'a mut W,
}
impl<'a> _BUSFAULTENAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BUSFAULTENAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "disable the exception"]
    #[inline]
    pub fn busfaultena_0(self) -> &'a mut W {
        self.variant(BUSFAULTENAW::BUSFAULTENA_0)
    }
    #[doc = "enable the exception"]
    #[inline]
    pub fn busfaultena_1(self) -> &'a mut W {
        self.variant(BUSFAULTENAW::BUSFAULTENA_1)
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
#[doc = "Values that can be written to the field `USGFAULTENA`"]
pub enum USGFAULTENAW {
    #[doc = "disable the exception"]
    USGFAULTENA_0,
    #[doc = "enable the exception"]
    USGFAULTENA_1,
}
impl USGFAULTENAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USGFAULTENAW::USGFAULTENA_0 => false,
            USGFAULTENAW::USGFAULTENA_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USGFAULTENAW<'a> {
    w: &'a mut W,
}
impl<'a> _USGFAULTENAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USGFAULTENAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "disable the exception"]
    #[inline]
    pub fn usgfaultena_0(self) -> &'a mut W {
        self.variant(USGFAULTENAW::USGFAULTENA_0)
    }
    #[doc = "enable the exception"]
    #[inline]
    pub fn usgfaultena_1(self) -> &'a mut W {
        self.variant(USGFAULTENAW::USGFAULTENA_1)
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
        const OFFSET: u8 = 18;
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
    #[doc = "Bit 0 - MemManage exception active bit"]
    #[inline]
    pub fn memfaultact(&self) -> MEMFAULTACTR {
        MEMFAULTACTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - BusFault exception active bit"]
    #[inline]
    pub fn busfaultact(&self) -> BUSFAULTACTR {
        BUSFAULTACTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - UsageFault exception active bit"]
    #[inline]
    pub fn usgfaultact(&self) -> USGFAULTACTR {
        USGFAULTACTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - SVCall active bit"]
    #[inline]
    pub fn svcallact(&self) -> SVCALLACTR {
        SVCALLACTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Debug monitor active bit"]
    #[inline]
    pub fn monitoract(&self) -> MONITORACTR {
        MONITORACTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - PendSV exception active bit"]
    #[inline]
    pub fn pendsvact(&self) -> PENDSVACTR {
        PENDSVACTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - SysTick exception active bit"]
    #[inline]
    pub fn systickact(&self) -> SYSTICKACTR {
        SYSTICKACTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - UsageFault exception pending bit"]
    #[inline]
    pub fn usgfaultpended(&self) -> USGFAULTPENDEDR {
        USGFAULTPENDEDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - MemManage exception pending bit"]
    #[inline]
    pub fn memfaultpended(&self) -> MEMFAULTPENDEDR {
        MEMFAULTPENDEDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - BusFault exception pending bit"]
    #[inline]
    pub fn busfaultpended(&self) -> BUSFAULTPENDEDR {
        BUSFAULTPENDEDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - SVCall pending bit"]
    #[inline]
    pub fn svcallpended(&self) -> SVCALLPENDEDR {
        SVCALLPENDEDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - MemManage enable bit"]
    #[inline]
    pub fn memfaultena(&self) -> MEMFAULTENAR {
        MEMFAULTENAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - BusFault enable bit"]
    #[inline]
    pub fn busfaultena(&self) -> BUSFAULTENAR {
        BUSFAULTENAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - UsageFault enable bit"]
    #[inline]
    pub fn usgfaultena(&self) -> USGFAULTENAR {
        USGFAULTENAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
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
    #[doc = "Bit 0 - MemManage exception active bit"]
    #[inline]
    pub fn memfaultact(&mut self) -> _MEMFAULTACTW {
        _MEMFAULTACTW { w: self }
    }
    #[doc = "Bit 1 - BusFault exception active bit"]
    #[inline]
    pub fn busfaultact(&mut self) -> _BUSFAULTACTW {
        _BUSFAULTACTW { w: self }
    }
    #[doc = "Bit 3 - UsageFault exception active bit"]
    #[inline]
    pub fn usgfaultact(&mut self) -> _USGFAULTACTW {
        _USGFAULTACTW { w: self }
    }
    #[doc = "Bit 7 - SVCall active bit"]
    #[inline]
    pub fn svcallact(&mut self) -> _SVCALLACTW {
        _SVCALLACTW { w: self }
    }
    #[doc = "Bit 8 - Debug monitor active bit"]
    #[inline]
    pub fn monitoract(&mut self) -> _MONITORACTW {
        _MONITORACTW { w: self }
    }
    #[doc = "Bit 10 - PendSV exception active bit"]
    #[inline]
    pub fn pendsvact(&mut self) -> _PENDSVACTW {
        _PENDSVACTW { w: self }
    }
    #[doc = "Bit 11 - SysTick exception active bit"]
    #[inline]
    pub fn systickact(&mut self) -> _SYSTICKACTW {
        _SYSTICKACTW { w: self }
    }
    #[doc = "Bit 12 - UsageFault exception pending bit"]
    #[inline]
    pub fn usgfaultpended(&mut self) -> _USGFAULTPENDEDW {
        _USGFAULTPENDEDW { w: self }
    }
    #[doc = "Bit 13 - MemManage exception pending bit"]
    #[inline]
    pub fn memfaultpended(&mut self) -> _MEMFAULTPENDEDW {
        _MEMFAULTPENDEDW { w: self }
    }
    #[doc = "Bit 14 - BusFault exception pending bit"]
    #[inline]
    pub fn busfaultpended(&mut self) -> _BUSFAULTPENDEDW {
        _BUSFAULTPENDEDW { w: self }
    }
    #[doc = "Bit 15 - SVCall pending bit"]
    #[inline]
    pub fn svcallpended(&mut self) -> _SVCALLPENDEDW {
        _SVCALLPENDEDW { w: self }
    }
    #[doc = "Bit 16 - MemManage enable bit"]
    #[inline]
    pub fn memfaultena(&mut self) -> _MEMFAULTENAW {
        _MEMFAULTENAW { w: self }
    }
    #[doc = "Bit 17 - BusFault enable bit"]
    #[inline]
    pub fn busfaultena(&mut self) -> _BUSFAULTENAW {
        _BUSFAULTENAW { w: self }
    }
    #[doc = "Bit 18 - UsageFault enable bit"]
    #[inline]
    pub fn usgfaultena(&mut self) -> _USGFAULTENAW {
        _USGFAULTENAW { w: self }
    }
}
