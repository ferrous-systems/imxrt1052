#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CFSR {
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
#[doc = "Possible values of the field `IACCVIOL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IACCVIOLR {
    #[doc = "no instruction access violation fault"]
    IACCVIOL_0,
    #[doc = "the processor attempted an instruction fetch from a location that does not permit execution"]
    IACCVIOL_1,
}
impl IACCVIOLR {
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
            IACCVIOLR::IACCVIOL_0 => false,
            IACCVIOLR::IACCVIOL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IACCVIOLR {
        match value {
            false => IACCVIOLR::IACCVIOL_0,
            true => IACCVIOLR::IACCVIOL_1,
        }
    }
    #[doc = "Checks if the value of the field is `IACCVIOL_0`"]
    #[inline]
    pub fn is_iaccviol_0(&self) -> bool {
        *self == IACCVIOLR::IACCVIOL_0
    }
    #[doc = "Checks if the value of the field is `IACCVIOL_1`"]
    #[inline]
    pub fn is_iaccviol_1(&self) -> bool {
        *self == IACCVIOLR::IACCVIOL_1
    }
}
#[doc = "Possible values of the field `DACCVIOL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DACCVIOLR {
    #[doc = "no data access violation fault"]
    DACCVIOL_0,
    #[doc = "the processor attempted a load or store at a location that does not permit the operation"]
    DACCVIOL_1,
}
impl DACCVIOLR {
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
            DACCVIOLR::DACCVIOL_0 => false,
            DACCVIOLR::DACCVIOL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DACCVIOLR {
        match value {
            false => DACCVIOLR::DACCVIOL_0,
            true => DACCVIOLR::DACCVIOL_1,
        }
    }
    #[doc = "Checks if the value of the field is `DACCVIOL_0`"]
    #[inline]
    pub fn is_daccviol_0(&self) -> bool {
        *self == DACCVIOLR::DACCVIOL_0
    }
    #[doc = "Checks if the value of the field is `DACCVIOL_1`"]
    #[inline]
    pub fn is_daccviol_1(&self) -> bool {
        *self == DACCVIOLR::DACCVIOL_1
    }
}
#[doc = "Possible values of the field `MUNSTKERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MUNSTKERRR {
    #[doc = "no unstacking fault"]
    MUNSTKERR_0,
    #[doc = "unstack for an exception return has caused one or more access violations"]
    MUNSTKERR_1,
}
impl MUNSTKERRR {
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
            MUNSTKERRR::MUNSTKERR_0 => false,
            MUNSTKERRR::MUNSTKERR_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MUNSTKERRR {
        match value {
            false => MUNSTKERRR::MUNSTKERR_0,
            true => MUNSTKERRR::MUNSTKERR_1,
        }
    }
    #[doc = "Checks if the value of the field is `MUNSTKERR_0`"]
    #[inline]
    pub fn is_munstkerr_0(&self) -> bool {
        *self == MUNSTKERRR::MUNSTKERR_0
    }
    #[doc = "Checks if the value of the field is `MUNSTKERR_1`"]
    #[inline]
    pub fn is_munstkerr_1(&self) -> bool {
        *self == MUNSTKERRR::MUNSTKERR_1
    }
}
#[doc = "Possible values of the field `MSTKERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTKERRR {
    #[doc = "no stacking fault"]
    MSTKERR_0,
    #[doc = "stacking for an exception entry has caused one or more access violations"]
    MSTKERR_1,
}
impl MSTKERRR {
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
            MSTKERRR::MSTKERR_0 => false,
            MSTKERRR::MSTKERR_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MSTKERRR {
        match value {
            false => MSTKERRR::MSTKERR_0,
            true => MSTKERRR::MSTKERR_1,
        }
    }
    #[doc = "Checks if the value of the field is `MSTKERR_0`"]
    #[inline]
    pub fn is_mstkerr_0(&self) -> bool {
        *self == MSTKERRR::MSTKERR_0
    }
    #[doc = "Checks if the value of the field is `MSTKERR_1`"]
    #[inline]
    pub fn is_mstkerr_1(&self) -> bool {
        *self == MSTKERRR::MSTKERR_1
    }
}
#[doc = "Possible values of the field `MLSPERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MLSPERRR {
    #[doc = "No MemManage fault occurred during floating-point lazy state preservation"]
    MLSPERR_0,
    #[doc = "A MemManage fault occurred during floating-point lazy state preservation"]
    MLSPERR_1,
}
impl MLSPERRR {
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
            MLSPERRR::MLSPERR_0 => false,
            MLSPERRR::MLSPERR_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MLSPERRR {
        match value {
            false => MLSPERRR::MLSPERR_0,
            true => MLSPERRR::MLSPERR_1,
        }
    }
    #[doc = "Checks if the value of the field is `MLSPERR_0`"]
    #[inline]
    pub fn is_mlsperr_0(&self) -> bool {
        *self == MLSPERRR::MLSPERR_0
    }
    #[doc = "Checks if the value of the field is `MLSPERR_1`"]
    #[inline]
    pub fn is_mlsperr_1(&self) -> bool {
        *self == MLSPERRR::MLSPERR_1
    }
}
#[doc = "Possible values of the field `MMARVALID`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MMARVALIDR {
    #[doc = "value in MMAR is not a valid fault address"]
    MMARVALID_0,
    #[doc = "MMAR holds a valid fault address"]
    MMARVALID_1,
}
impl MMARVALIDR {
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
            MMARVALIDR::MMARVALID_0 => false,
            MMARVALIDR::MMARVALID_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MMARVALIDR {
        match value {
            false => MMARVALIDR::MMARVALID_0,
            true => MMARVALIDR::MMARVALID_1,
        }
    }
    #[doc = "Checks if the value of the field is `MMARVALID_0`"]
    #[inline]
    pub fn is_mmarvalid_0(&self) -> bool {
        *self == MMARVALIDR::MMARVALID_0
    }
    #[doc = "Checks if the value of the field is `MMARVALID_1`"]
    #[inline]
    pub fn is_mmarvalid_1(&self) -> bool {
        *self == MMARVALIDR::MMARVALID_1
    }
}
#[doc = "Possible values of the field `IBUSERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IBUSERRR {
    #[doc = "no instruction bus error"]
    IBUSERR_0,
    #[doc = "instruction bus error"]
    IBUSERR_1,
}
impl IBUSERRR {
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
            IBUSERRR::IBUSERR_0 => false,
            IBUSERRR::IBUSERR_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IBUSERRR {
        match value {
            false => IBUSERRR::IBUSERR_0,
            true => IBUSERRR::IBUSERR_1,
        }
    }
    #[doc = "Checks if the value of the field is `IBUSERR_0`"]
    #[inline]
    pub fn is_ibuserr_0(&self) -> bool {
        *self == IBUSERRR::IBUSERR_0
    }
    #[doc = "Checks if the value of the field is `IBUSERR_1`"]
    #[inline]
    pub fn is_ibuserr_1(&self) -> bool {
        *self == IBUSERRR::IBUSERR_1
    }
}
#[doc = "Possible values of the field `PRECISERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRECISERRR {
    #[doc = "no precise data bus error"]
    PRECISERR_0,
    #[doc = "a data bus error has occurred, and the PC value stacked for the exception return points to the instruction that caused the fault"]
    PRECISERR_1,
}
impl PRECISERRR {
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
            PRECISERRR::PRECISERR_0 => false,
            PRECISERRR::PRECISERR_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PRECISERRR {
        match value {
            false => PRECISERRR::PRECISERR_0,
            true => PRECISERRR::PRECISERR_1,
        }
    }
    #[doc = "Checks if the value of the field is `PRECISERR_0`"]
    #[inline]
    pub fn is_preciserr_0(&self) -> bool {
        *self == PRECISERRR::PRECISERR_0
    }
    #[doc = "Checks if the value of the field is `PRECISERR_1`"]
    #[inline]
    pub fn is_preciserr_1(&self) -> bool {
        *self == PRECISERRR::PRECISERR_1
    }
}
#[doc = "Possible values of the field `IMPRECISERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IMPRECISERRR {
    #[doc = "no imprecise data bus error"]
    IMPRECISERR_0,
    #[doc = "a data bus error has occurred, but the return address in the stack frame is not related to the instruction that caused the error"]
    IMPRECISERR_1,
}
impl IMPRECISERRR {
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
            IMPRECISERRR::IMPRECISERR_0 => false,
            IMPRECISERRR::IMPRECISERR_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IMPRECISERRR {
        match value {
            false => IMPRECISERRR::IMPRECISERR_0,
            true => IMPRECISERRR::IMPRECISERR_1,
        }
    }
    #[doc = "Checks if the value of the field is `IMPRECISERR_0`"]
    #[inline]
    pub fn is_impreciserr_0(&self) -> bool {
        *self == IMPRECISERRR::IMPRECISERR_0
    }
    #[doc = "Checks if the value of the field is `IMPRECISERR_1`"]
    #[inline]
    pub fn is_impreciserr_1(&self) -> bool {
        *self == IMPRECISERRR::IMPRECISERR_1
    }
}
#[doc = "Possible values of the field `UNSTKERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UNSTKERRR {
    #[doc = "no unstacking fault"]
    UNSTKERR_0,
    #[doc = "unstack for an exception return has caused one or more BusFaults"]
    UNSTKERR_1,
}
impl UNSTKERRR {
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
            UNSTKERRR::UNSTKERR_0 => false,
            UNSTKERRR::UNSTKERR_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UNSTKERRR {
        match value {
            false => UNSTKERRR::UNSTKERR_0,
            true => UNSTKERRR::UNSTKERR_1,
        }
    }
    #[doc = "Checks if the value of the field is `UNSTKERR_0`"]
    #[inline]
    pub fn is_unstkerr_0(&self) -> bool {
        *self == UNSTKERRR::UNSTKERR_0
    }
    #[doc = "Checks if the value of the field is `UNSTKERR_1`"]
    #[inline]
    pub fn is_unstkerr_1(&self) -> bool {
        *self == UNSTKERRR::UNSTKERR_1
    }
}
#[doc = "Possible values of the field `STKERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STKERRR {
    #[doc = "no stacking fault"]
    STKERR_0,
    #[doc = "stacking for an exception entry has caused one or more BusFaults"]
    STKERR_1,
}
impl STKERRR {
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
            STKERRR::STKERR_0 => false,
            STKERRR::STKERR_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STKERRR {
        match value {
            false => STKERRR::STKERR_0,
            true => STKERRR::STKERR_1,
        }
    }
    #[doc = "Checks if the value of the field is `STKERR_0`"]
    #[inline]
    pub fn is_stkerr_0(&self) -> bool {
        *self == STKERRR::STKERR_0
    }
    #[doc = "Checks if the value of the field is `STKERR_1`"]
    #[inline]
    pub fn is_stkerr_1(&self) -> bool {
        *self == STKERRR::STKERR_1
    }
}
#[doc = "Possible values of the field `LSPERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSPERRR {
    #[doc = "No bus fault occurred during floating-point lazy state preservation"]
    LSPERR_0,
    #[doc = "A bus fault occurred during floating-point lazy state preservation"]
    LSPERR_1,
}
impl LSPERRR {
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
            LSPERRR::LSPERR_0 => false,
            LSPERRR::LSPERR_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LSPERRR {
        match value {
            false => LSPERRR::LSPERR_0,
            true => LSPERRR::LSPERR_1,
        }
    }
    #[doc = "Checks if the value of the field is `LSPERR_0`"]
    #[inline]
    pub fn is_lsperr_0(&self) -> bool {
        *self == LSPERRR::LSPERR_0
    }
    #[doc = "Checks if the value of the field is `LSPERR_1`"]
    #[inline]
    pub fn is_lsperr_1(&self) -> bool {
        *self == LSPERRR::LSPERR_1
    }
}
#[doc = "Possible values of the field `BFARVALID`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BFARVALIDR {
    #[doc = "value in BFAR is not a valid fault address"]
    BFARVALID_0,
    #[doc = "BFAR holds a valid fault address"]
    BFARVALID_1,
}
impl BFARVALIDR {
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
            BFARVALIDR::BFARVALID_0 => false,
            BFARVALIDR::BFARVALID_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BFARVALIDR {
        match value {
            false => BFARVALIDR::BFARVALID_0,
            true => BFARVALIDR::BFARVALID_1,
        }
    }
    #[doc = "Checks if the value of the field is `BFARVALID_0`"]
    #[inline]
    pub fn is_bfarvalid_0(&self) -> bool {
        *self == BFARVALIDR::BFARVALID_0
    }
    #[doc = "Checks if the value of the field is `BFARVALID_1`"]
    #[inline]
    pub fn is_bfarvalid_1(&self) -> bool {
        *self == BFARVALIDR::BFARVALID_1
    }
}
#[doc = "Possible values of the field `UNDEFINSTR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UNDEFINSTRR {
    #[doc = "no undefined instruction UsageFault"]
    UNDEFINSTR_0,
    #[doc = "the processor has attempted to execute an undefined instruction"]
    UNDEFINSTR_1,
}
impl UNDEFINSTRR {
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
            UNDEFINSTRR::UNDEFINSTR_0 => false,
            UNDEFINSTRR::UNDEFINSTR_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UNDEFINSTRR {
        match value {
            false => UNDEFINSTRR::UNDEFINSTR_0,
            true => UNDEFINSTRR::UNDEFINSTR_1,
        }
    }
    #[doc = "Checks if the value of the field is `UNDEFINSTR_0`"]
    #[inline]
    pub fn is_undefinstr_0(&self) -> bool {
        *self == UNDEFINSTRR::UNDEFINSTR_0
    }
    #[doc = "Checks if the value of the field is `UNDEFINSTR_1`"]
    #[inline]
    pub fn is_undefinstr_1(&self) -> bool {
        *self == UNDEFINSTRR::UNDEFINSTR_1
    }
}
#[doc = "Possible values of the field `INVSTATE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INVSTATER {
    #[doc = "no invalid state UsageFault"]
    INVSTATE_0,
    #[doc = "the processor has attempted to execute an instruction that makes illegal use of the EPSR"]
    INVSTATE_1,
}
impl INVSTATER {
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
            INVSTATER::INVSTATE_0 => false,
            INVSTATER::INVSTATE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INVSTATER {
        match value {
            false => INVSTATER::INVSTATE_0,
            true => INVSTATER::INVSTATE_1,
        }
    }
    #[doc = "Checks if the value of the field is `INVSTATE_0`"]
    #[inline]
    pub fn is_invstate_0(&self) -> bool {
        *self == INVSTATER::INVSTATE_0
    }
    #[doc = "Checks if the value of the field is `INVSTATE_1`"]
    #[inline]
    pub fn is_invstate_1(&self) -> bool {
        *self == INVSTATER::INVSTATE_1
    }
}
#[doc = "Possible values of the field `INVPC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INVPCR {
    #[doc = "no invalid PC load UsageFault"]
    INVPC_0,
    #[doc = "the processor has attempted an illegal load of EXC_RETURN to the PC"]
    INVPC_1,
}
impl INVPCR {
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
            INVPCR::INVPC_0 => false,
            INVPCR::INVPC_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INVPCR {
        match value {
            false => INVPCR::INVPC_0,
            true => INVPCR::INVPC_1,
        }
    }
    #[doc = "Checks if the value of the field is `INVPC_0`"]
    #[inline]
    pub fn is_invpc_0(&self) -> bool {
        *self == INVPCR::INVPC_0
    }
    #[doc = "Checks if the value of the field is `INVPC_1`"]
    #[inline]
    pub fn is_invpc_1(&self) -> bool {
        *self == INVPCR::INVPC_1
    }
}
#[doc = "Possible values of the field `NOCP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NOCPR {
    #[doc = "no UsageFault caused by attempting to access a coprocessor"]
    NOCP_0,
    #[doc = "the processor has attempted to access a coprocessor"]
    NOCP_1,
}
impl NOCPR {
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
            NOCPR::NOCP_0 => false,
            NOCPR::NOCP_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NOCPR {
        match value {
            false => NOCPR::NOCP_0,
            true => NOCPR::NOCP_1,
        }
    }
    #[doc = "Checks if the value of the field is `NOCP_0`"]
    #[inline]
    pub fn is_nocp_0(&self) -> bool {
        *self == NOCPR::NOCP_0
    }
    #[doc = "Checks if the value of the field is `NOCP_1`"]
    #[inline]
    pub fn is_nocp_1(&self) -> bool {
        *self == NOCPR::NOCP_1
    }
}
#[doc = "Possible values of the field `UNALIGNED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UNALIGNEDR {
    #[doc = "no unaligned access fault, or unaligned access trapping not enabled"]
    UNALIGNED_0,
    #[doc = "the processor has made an unaligned memory access"]
    UNALIGNED_1,
}
impl UNALIGNEDR {
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
            UNALIGNEDR::UNALIGNED_0 => false,
            UNALIGNEDR::UNALIGNED_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UNALIGNEDR {
        match value {
            false => UNALIGNEDR::UNALIGNED_0,
            true => UNALIGNEDR::UNALIGNED_1,
        }
    }
    #[doc = "Checks if the value of the field is `UNALIGNED_0`"]
    #[inline]
    pub fn is_unaligned_0(&self) -> bool {
        *self == UNALIGNEDR::UNALIGNED_0
    }
    #[doc = "Checks if the value of the field is `UNALIGNED_1`"]
    #[inline]
    pub fn is_unaligned_1(&self) -> bool {
        *self == UNALIGNEDR::UNALIGNED_1
    }
}
#[doc = "Possible values of the field `DIVBYZERO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIVBYZEROR {
    #[doc = "no divide by zero fault, or divide by zero trapping not enabled"]
    DIVBYZERO_0,
    #[doc = "the processor has executed an SDIV or UDIV instruction with a divisor of 0"]
    DIVBYZERO_1,
}
impl DIVBYZEROR {
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
            DIVBYZEROR::DIVBYZERO_0 => false,
            DIVBYZEROR::DIVBYZERO_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DIVBYZEROR {
        match value {
            false => DIVBYZEROR::DIVBYZERO_0,
            true => DIVBYZEROR::DIVBYZERO_1,
        }
    }
    #[doc = "Checks if the value of the field is `DIVBYZERO_0`"]
    #[inline]
    pub fn is_divbyzero_0(&self) -> bool {
        *self == DIVBYZEROR::DIVBYZERO_0
    }
    #[doc = "Checks if the value of the field is `DIVBYZERO_1`"]
    #[inline]
    pub fn is_divbyzero_1(&self) -> bool {
        *self == DIVBYZEROR::DIVBYZERO_1
    }
}
#[doc = "Values that can be written to the field `IACCVIOL`"]
pub enum IACCVIOLW {
    #[doc = "no instruction access violation fault"]
    IACCVIOL_0,
    #[doc = "the processor attempted an instruction fetch from a location that does not permit execution"]
    IACCVIOL_1,
}
impl IACCVIOLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IACCVIOLW::IACCVIOL_0 => false,
            IACCVIOLW::IACCVIOL_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IACCVIOLW<'a> {
    w: &'a mut W,
}
impl<'a> _IACCVIOLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IACCVIOLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no instruction access violation fault"]
    #[inline]
    pub fn iaccviol_0(self) -> &'a mut W {
        self.variant(IACCVIOLW::IACCVIOL_0)
    }
    #[doc = "the processor attempted an instruction fetch from a location that does not permit execution"]
    #[inline]
    pub fn iaccviol_1(self) -> &'a mut W {
        self.variant(IACCVIOLW::IACCVIOL_1)
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
#[doc = "Values that can be written to the field `DACCVIOL`"]
pub enum DACCVIOLW {
    #[doc = "no data access violation fault"]
    DACCVIOL_0,
    #[doc = "the processor attempted a load or store at a location that does not permit the operation"]
    DACCVIOL_1,
}
impl DACCVIOLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DACCVIOLW::DACCVIOL_0 => false,
            DACCVIOLW::DACCVIOL_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DACCVIOLW<'a> {
    w: &'a mut W,
}
impl<'a> _DACCVIOLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DACCVIOLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no data access violation fault"]
    #[inline]
    pub fn daccviol_0(self) -> &'a mut W {
        self.variant(DACCVIOLW::DACCVIOL_0)
    }
    #[doc = "the processor attempted a load or store at a location that does not permit the operation"]
    #[inline]
    pub fn daccviol_1(self) -> &'a mut W {
        self.variant(DACCVIOLW::DACCVIOL_1)
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
#[doc = "Values that can be written to the field `MUNSTKERR`"]
pub enum MUNSTKERRW {
    #[doc = "no unstacking fault"]
    MUNSTKERR_0,
    #[doc = "unstack for an exception return has caused one or more access violations"]
    MUNSTKERR_1,
}
impl MUNSTKERRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MUNSTKERRW::MUNSTKERR_0 => false,
            MUNSTKERRW::MUNSTKERR_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MUNSTKERRW<'a> {
    w: &'a mut W,
}
impl<'a> _MUNSTKERRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MUNSTKERRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no unstacking fault"]
    #[inline]
    pub fn munstkerr_0(self) -> &'a mut W {
        self.variant(MUNSTKERRW::MUNSTKERR_0)
    }
    #[doc = "unstack for an exception return has caused one or more access violations"]
    #[inline]
    pub fn munstkerr_1(self) -> &'a mut W {
        self.variant(MUNSTKERRW::MUNSTKERR_1)
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
#[doc = "Values that can be written to the field `MSTKERR`"]
pub enum MSTKERRW {
    #[doc = "no stacking fault"]
    MSTKERR_0,
    #[doc = "stacking for an exception entry has caused one or more access violations"]
    MSTKERR_1,
}
impl MSTKERRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MSTKERRW::MSTKERR_0 => false,
            MSTKERRW::MSTKERR_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MSTKERRW<'a> {
    w: &'a mut W,
}
impl<'a> _MSTKERRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MSTKERRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no stacking fault"]
    #[inline]
    pub fn mstkerr_0(self) -> &'a mut W {
        self.variant(MSTKERRW::MSTKERR_0)
    }
    #[doc = "stacking for an exception entry has caused one or more access violations"]
    #[inline]
    pub fn mstkerr_1(self) -> &'a mut W {
        self.variant(MSTKERRW::MSTKERR_1)
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
#[doc = "Values that can be written to the field `MLSPERR`"]
pub enum MLSPERRW {
    #[doc = "No MemManage fault occurred during floating-point lazy state preservation"]
    MLSPERR_0,
    #[doc = "A MemManage fault occurred during floating-point lazy state preservation"]
    MLSPERR_1,
}
impl MLSPERRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MLSPERRW::MLSPERR_0 => false,
            MLSPERRW::MLSPERR_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MLSPERRW<'a> {
    w: &'a mut W,
}
impl<'a> _MLSPERRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MLSPERRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No MemManage fault occurred during floating-point lazy state preservation"]
    #[inline]
    pub fn mlsperr_0(self) -> &'a mut W {
        self.variant(MLSPERRW::MLSPERR_0)
    }
    #[doc = "A MemManage fault occurred during floating-point lazy state preservation"]
    #[inline]
    pub fn mlsperr_1(self) -> &'a mut W {
        self.variant(MLSPERRW::MLSPERR_1)
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MMARVALID`"]
pub enum MMARVALIDW {
    #[doc = "value in MMAR is not a valid fault address"]
    MMARVALID_0,
    #[doc = "MMAR holds a valid fault address"]
    MMARVALID_1,
}
impl MMARVALIDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MMARVALIDW::MMARVALID_0 => false,
            MMARVALIDW::MMARVALID_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MMARVALIDW<'a> {
    w: &'a mut W,
}
impl<'a> _MMARVALIDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MMARVALIDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "value in MMAR is not a valid fault address"]
    #[inline]
    pub fn mmarvalid_0(self) -> &'a mut W {
        self.variant(MMARVALIDW::MMARVALID_0)
    }
    #[doc = "MMAR holds a valid fault address"]
    #[inline]
    pub fn mmarvalid_1(self) -> &'a mut W {
        self.variant(MMARVALIDW::MMARVALID_1)
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
#[doc = "Values that can be written to the field `IBUSERR`"]
pub enum IBUSERRW {
    #[doc = "no instruction bus error"]
    IBUSERR_0,
    #[doc = "instruction bus error"]
    IBUSERR_1,
}
impl IBUSERRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IBUSERRW::IBUSERR_0 => false,
            IBUSERRW::IBUSERR_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IBUSERRW<'a> {
    w: &'a mut W,
}
impl<'a> _IBUSERRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IBUSERRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no instruction bus error"]
    #[inline]
    pub fn ibuserr_0(self) -> &'a mut W {
        self.variant(IBUSERRW::IBUSERR_0)
    }
    #[doc = "instruction bus error"]
    #[inline]
    pub fn ibuserr_1(self) -> &'a mut W {
        self.variant(IBUSERRW::IBUSERR_1)
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
#[doc = "Values that can be written to the field `PRECISERR`"]
pub enum PRECISERRW {
    #[doc = "no precise data bus error"]
    PRECISERR_0,
    #[doc = "a data bus error has occurred, and the PC value stacked for the exception return points to the instruction that caused the fault"]
    PRECISERR_1,
}
impl PRECISERRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PRECISERRW::PRECISERR_0 => false,
            PRECISERRW::PRECISERR_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PRECISERRW<'a> {
    w: &'a mut W,
}
impl<'a> _PRECISERRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PRECISERRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no precise data bus error"]
    #[inline]
    pub fn preciserr_0(self) -> &'a mut W {
        self.variant(PRECISERRW::PRECISERR_0)
    }
    #[doc = "a data bus error has occurred, and the PC value stacked for the exception return points to the instruction that caused the fault"]
    #[inline]
    pub fn preciserr_1(self) -> &'a mut W {
        self.variant(PRECISERRW::PRECISERR_1)
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
#[doc = "Values that can be written to the field `IMPRECISERR`"]
pub enum IMPRECISERRW {
    #[doc = "no imprecise data bus error"]
    IMPRECISERR_0,
    #[doc = "a data bus error has occurred, but the return address in the stack frame is not related to the instruction that caused the error"]
    IMPRECISERR_1,
}
impl IMPRECISERRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IMPRECISERRW::IMPRECISERR_0 => false,
            IMPRECISERRW::IMPRECISERR_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IMPRECISERRW<'a> {
    w: &'a mut W,
}
impl<'a> _IMPRECISERRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IMPRECISERRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no imprecise data bus error"]
    #[inline]
    pub fn impreciserr_0(self) -> &'a mut W {
        self.variant(IMPRECISERRW::IMPRECISERR_0)
    }
    #[doc = "a data bus error has occurred, but the return address in the stack frame is not related to the instruction that caused the error"]
    #[inline]
    pub fn impreciserr_1(self) -> &'a mut W {
        self.variant(IMPRECISERRW::IMPRECISERR_1)
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
#[doc = "Values that can be written to the field `UNSTKERR`"]
pub enum UNSTKERRW {
    #[doc = "no unstacking fault"]
    UNSTKERR_0,
    #[doc = "unstack for an exception return has caused one or more BusFaults"]
    UNSTKERR_1,
}
impl UNSTKERRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            UNSTKERRW::UNSTKERR_0 => false,
            UNSTKERRW::UNSTKERR_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UNSTKERRW<'a> {
    w: &'a mut W,
}
impl<'a> _UNSTKERRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UNSTKERRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no unstacking fault"]
    #[inline]
    pub fn unstkerr_0(self) -> &'a mut W {
        self.variant(UNSTKERRW::UNSTKERR_0)
    }
    #[doc = "unstack for an exception return has caused one or more BusFaults"]
    #[inline]
    pub fn unstkerr_1(self) -> &'a mut W {
        self.variant(UNSTKERRW::UNSTKERR_1)
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
#[doc = "Values that can be written to the field `STKERR`"]
pub enum STKERRW {
    #[doc = "no stacking fault"]
    STKERR_0,
    #[doc = "stacking for an exception entry has caused one or more BusFaults"]
    STKERR_1,
}
impl STKERRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STKERRW::STKERR_0 => false,
            STKERRW::STKERR_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STKERRW<'a> {
    w: &'a mut W,
}
impl<'a> _STKERRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STKERRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no stacking fault"]
    #[inline]
    pub fn stkerr_0(self) -> &'a mut W {
        self.variant(STKERRW::STKERR_0)
    }
    #[doc = "stacking for an exception entry has caused one or more BusFaults"]
    #[inline]
    pub fn stkerr_1(self) -> &'a mut W {
        self.variant(STKERRW::STKERR_1)
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
#[doc = "Values that can be written to the field `LSPERR`"]
pub enum LSPERRW {
    #[doc = "No bus fault occurred during floating-point lazy state preservation"]
    LSPERR_0,
    #[doc = "A bus fault occurred during floating-point lazy state preservation"]
    LSPERR_1,
}
impl LSPERRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LSPERRW::LSPERR_0 => false,
            LSPERRW::LSPERR_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LSPERRW<'a> {
    w: &'a mut W,
}
impl<'a> _LSPERRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LSPERRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No bus fault occurred during floating-point lazy state preservation"]
    #[inline]
    pub fn lsperr_0(self) -> &'a mut W {
        self.variant(LSPERRW::LSPERR_0)
    }
    #[doc = "A bus fault occurred during floating-point lazy state preservation"]
    #[inline]
    pub fn lsperr_1(self) -> &'a mut W {
        self.variant(LSPERRW::LSPERR_1)
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
#[doc = "Values that can be written to the field `BFARVALID`"]
pub enum BFARVALIDW {
    #[doc = "value in BFAR is not a valid fault address"]
    BFARVALID_0,
    #[doc = "BFAR holds a valid fault address"]
    BFARVALID_1,
}
impl BFARVALIDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BFARVALIDW::BFARVALID_0 => false,
            BFARVALIDW::BFARVALID_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BFARVALIDW<'a> {
    w: &'a mut W,
}
impl<'a> _BFARVALIDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BFARVALIDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "value in BFAR is not a valid fault address"]
    #[inline]
    pub fn bfarvalid_0(self) -> &'a mut W {
        self.variant(BFARVALIDW::BFARVALID_0)
    }
    #[doc = "BFAR holds a valid fault address"]
    #[inline]
    pub fn bfarvalid_1(self) -> &'a mut W {
        self.variant(BFARVALIDW::BFARVALID_1)
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
#[doc = "Values that can be written to the field `UNDEFINSTR`"]
pub enum UNDEFINSTRW {
    #[doc = "no undefined instruction UsageFault"]
    UNDEFINSTR_0,
    #[doc = "the processor has attempted to execute an undefined instruction"]
    UNDEFINSTR_1,
}
impl UNDEFINSTRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            UNDEFINSTRW::UNDEFINSTR_0 => false,
            UNDEFINSTRW::UNDEFINSTR_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UNDEFINSTRW<'a> {
    w: &'a mut W,
}
impl<'a> _UNDEFINSTRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UNDEFINSTRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no undefined instruction UsageFault"]
    #[inline]
    pub fn undefinstr_0(self) -> &'a mut W {
        self.variant(UNDEFINSTRW::UNDEFINSTR_0)
    }
    #[doc = "the processor has attempted to execute an undefined instruction"]
    #[inline]
    pub fn undefinstr_1(self) -> &'a mut W {
        self.variant(UNDEFINSTRW::UNDEFINSTR_1)
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
#[doc = "Values that can be written to the field `INVSTATE`"]
pub enum INVSTATEW {
    #[doc = "no invalid state UsageFault"]
    INVSTATE_0,
    #[doc = "the processor has attempted to execute an instruction that makes illegal use of the EPSR"]
    INVSTATE_1,
}
impl INVSTATEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INVSTATEW::INVSTATE_0 => false,
            INVSTATEW::INVSTATE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INVSTATEW<'a> {
    w: &'a mut W,
}
impl<'a> _INVSTATEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INVSTATEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no invalid state UsageFault"]
    #[inline]
    pub fn invstate_0(self) -> &'a mut W {
        self.variant(INVSTATEW::INVSTATE_0)
    }
    #[doc = "the processor has attempted to execute an instruction that makes illegal use of the EPSR"]
    #[inline]
    pub fn invstate_1(self) -> &'a mut W {
        self.variant(INVSTATEW::INVSTATE_1)
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
#[doc = "Values that can be written to the field `INVPC`"]
pub enum INVPCW {
    #[doc = "no invalid PC load UsageFault"]
    INVPC_0,
    #[doc = "the processor has attempted an illegal load of EXC_RETURN to the PC"]
    INVPC_1,
}
impl INVPCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INVPCW::INVPC_0 => false,
            INVPCW::INVPC_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INVPCW<'a> {
    w: &'a mut W,
}
impl<'a> _INVPCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INVPCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no invalid PC load UsageFault"]
    #[inline]
    pub fn invpc_0(self) -> &'a mut W {
        self.variant(INVPCW::INVPC_0)
    }
    #[doc = "the processor has attempted an illegal load of EXC_RETURN to the PC"]
    #[inline]
    pub fn invpc_1(self) -> &'a mut W {
        self.variant(INVPCW::INVPC_1)
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
#[doc = "Values that can be written to the field `NOCP`"]
pub enum NOCPW {
    #[doc = "no UsageFault caused by attempting to access a coprocessor"]
    NOCP_0,
    #[doc = "the processor has attempted to access a coprocessor"]
    NOCP_1,
}
impl NOCPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            NOCPW::NOCP_0 => false,
            NOCPW::NOCP_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NOCPW<'a> {
    w: &'a mut W,
}
impl<'a> _NOCPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NOCPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no UsageFault caused by attempting to access a coprocessor"]
    #[inline]
    pub fn nocp_0(self) -> &'a mut W {
        self.variant(NOCPW::NOCP_0)
    }
    #[doc = "the processor has attempted to access a coprocessor"]
    #[inline]
    pub fn nocp_1(self) -> &'a mut W {
        self.variant(NOCPW::NOCP_1)
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
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `UNALIGNED`"]
pub enum UNALIGNEDW {
    #[doc = "no unaligned access fault, or unaligned access trapping not enabled"]
    UNALIGNED_0,
    #[doc = "the processor has made an unaligned memory access"]
    UNALIGNED_1,
}
impl UNALIGNEDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            UNALIGNEDW::UNALIGNED_0 => false,
            UNALIGNEDW::UNALIGNED_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UNALIGNEDW<'a> {
    w: &'a mut W,
}
impl<'a> _UNALIGNEDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UNALIGNEDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no unaligned access fault, or unaligned access trapping not enabled"]
    #[inline]
    pub fn unaligned_0(self) -> &'a mut W {
        self.variant(UNALIGNEDW::UNALIGNED_0)
    }
    #[doc = "the processor has made an unaligned memory access"]
    #[inline]
    pub fn unaligned_1(self) -> &'a mut W {
        self.variant(UNALIGNEDW::UNALIGNED_1)
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
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DIVBYZERO`"]
pub enum DIVBYZEROW {
    #[doc = "no divide by zero fault, or divide by zero trapping not enabled"]
    DIVBYZERO_0,
    #[doc = "the processor has executed an SDIV or UDIV instruction with a divisor of 0"]
    DIVBYZERO_1,
}
impl DIVBYZEROW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DIVBYZEROW::DIVBYZERO_0 => false,
            DIVBYZEROW::DIVBYZERO_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DIVBYZEROW<'a> {
    w: &'a mut W,
}
impl<'a> _DIVBYZEROW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DIVBYZEROW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no divide by zero fault, or divide by zero trapping not enabled"]
    #[inline]
    pub fn divbyzero_0(self) -> &'a mut W {
        self.variant(DIVBYZEROW::DIVBYZERO_0)
    }
    #[doc = "the processor has executed an SDIV or UDIV instruction with a divisor of 0"]
    #[inline]
    pub fn divbyzero_1(self) -> &'a mut W {
        self.variant(DIVBYZEROW::DIVBYZERO_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Instruction access violation flag"]
    #[inline]
    pub fn iaccviol(&self) -> IACCVIOLR {
        IACCVIOLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Data access violation flag"]
    #[inline]
    pub fn daccviol(&self) -> DACCVIOLR {
        DACCVIOLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - MemManage fault on unstacking for a return from exception"]
    #[inline]
    pub fn munstkerr(&self) -> MUNSTKERRR {
        MUNSTKERRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - MemManage fault on stacking for exception entry"]
    #[inline]
    pub fn mstkerr(&self) -> MSTKERRR {
        MSTKERRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - MemManage fault occurred during floating-point lazy state preservation"]
    #[inline]
    pub fn mlsperr(&self) -> MLSPERRR {
        MLSPERRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - MemManage Fault Address Register (MMFAR) valid flag"]
    #[inline]
    pub fn mmarvalid(&self) -> MMARVALIDR {
        MMARVALIDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Instruction bus error"]
    #[inline]
    pub fn ibuserr(&self) -> IBUSERRR {
        IBUSERRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Precise data bus error"]
    #[inline]
    pub fn preciserr(&self) -> PRECISERRR {
        PRECISERRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Imprecise data bus error"]
    #[inline]
    pub fn impreciserr(&self) -> IMPRECISERRR {
        IMPRECISERRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - BusFault on unstacking for a return from exception"]
    #[inline]
    pub fn unstkerr(&self) -> UNSTKERRR {
        UNSTKERRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - BusFault on stacking for exception entry"]
    #[inline]
    pub fn stkerr(&self) -> STKERRR {
        STKERRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Bus fault occurred during floating-point lazy state preservation"]
    #[inline]
    pub fn lsperr(&self) -> LSPERRR {
        LSPERRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - BusFault Address Register (BFAR) valid flag"]
    #[inline]
    pub fn bfarvalid(&self) -> BFARVALIDR {
        BFARVALIDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Undefined instruction UsageFault"]
    #[inline]
    pub fn undefinstr(&self) -> UNDEFINSTRR {
        UNDEFINSTRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Invalid state UsageFault"]
    #[inline]
    pub fn invstate(&self) -> INVSTATER {
        INVSTATER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Invalid PC load UsageFault, caused by an invalid PC load by EXC_RETURN"]
    #[inline]
    pub fn invpc(&self) -> INVPCR {
        INVPCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - No coprocessor UsageFault"]
    #[inline]
    pub fn nocp(&self) -> NOCPR {
        NOCPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Unaligned access UsageFault"]
    #[inline]
    pub fn unaligned(&self) -> UNALIGNEDR {
        UNALIGNEDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Divide by zero UsageFault"]
    #[inline]
    pub fn divbyzero(&self) -> DIVBYZEROR {
        DIVBYZEROR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
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
    #[doc = "Bit 0 - Instruction access violation flag"]
    #[inline]
    pub fn iaccviol(&mut self) -> _IACCVIOLW {
        _IACCVIOLW { w: self }
    }
    #[doc = "Bit 1 - Data access violation flag"]
    #[inline]
    pub fn daccviol(&mut self) -> _DACCVIOLW {
        _DACCVIOLW { w: self }
    }
    #[doc = "Bit 3 - MemManage fault on unstacking for a return from exception"]
    #[inline]
    pub fn munstkerr(&mut self) -> _MUNSTKERRW {
        _MUNSTKERRW { w: self }
    }
    #[doc = "Bit 4 - MemManage fault on stacking for exception entry"]
    #[inline]
    pub fn mstkerr(&mut self) -> _MSTKERRW {
        _MSTKERRW { w: self }
    }
    #[doc = "Bit 5 - MemManage fault occurred during floating-point lazy state preservation"]
    #[inline]
    pub fn mlsperr(&mut self) -> _MLSPERRW {
        _MLSPERRW { w: self }
    }
    #[doc = "Bit 7 - MemManage Fault Address Register (MMFAR) valid flag"]
    #[inline]
    pub fn mmarvalid(&mut self) -> _MMARVALIDW {
        _MMARVALIDW { w: self }
    }
    #[doc = "Bit 8 - Instruction bus error"]
    #[inline]
    pub fn ibuserr(&mut self) -> _IBUSERRW {
        _IBUSERRW { w: self }
    }
    #[doc = "Bit 9 - Precise data bus error"]
    #[inline]
    pub fn preciserr(&mut self) -> _PRECISERRW {
        _PRECISERRW { w: self }
    }
    #[doc = "Bit 10 - Imprecise data bus error"]
    #[inline]
    pub fn impreciserr(&mut self) -> _IMPRECISERRW {
        _IMPRECISERRW { w: self }
    }
    #[doc = "Bit 11 - BusFault on unstacking for a return from exception"]
    #[inline]
    pub fn unstkerr(&mut self) -> _UNSTKERRW {
        _UNSTKERRW { w: self }
    }
    #[doc = "Bit 12 - BusFault on stacking for exception entry"]
    #[inline]
    pub fn stkerr(&mut self) -> _STKERRW {
        _STKERRW { w: self }
    }
    #[doc = "Bit 13 - Bus fault occurred during floating-point lazy state preservation"]
    #[inline]
    pub fn lsperr(&mut self) -> _LSPERRW {
        _LSPERRW { w: self }
    }
    #[doc = "Bit 15 - BusFault Address Register (BFAR) valid flag"]
    #[inline]
    pub fn bfarvalid(&mut self) -> _BFARVALIDW {
        _BFARVALIDW { w: self }
    }
    #[doc = "Bit 16 - Undefined instruction UsageFault"]
    #[inline]
    pub fn undefinstr(&mut self) -> _UNDEFINSTRW {
        _UNDEFINSTRW { w: self }
    }
    #[doc = "Bit 17 - Invalid state UsageFault"]
    #[inline]
    pub fn invstate(&mut self) -> _INVSTATEW {
        _INVSTATEW { w: self }
    }
    #[doc = "Bit 18 - Invalid PC load UsageFault, caused by an invalid PC load by EXC_RETURN"]
    #[inline]
    pub fn invpc(&mut self) -> _INVPCW {
        _INVPCW { w: self }
    }
    #[doc = "Bit 19 - No coprocessor UsageFault"]
    #[inline]
    pub fn nocp(&mut self) -> _NOCPW {
        _NOCPW { w: self }
    }
    #[doc = "Bit 24 - Unaligned access UsageFault"]
    #[inline]
    pub fn unaligned(&mut self) -> _UNALIGNEDW {
        _UNALIGNEDW { w: self }
    }
    #[doc = "Bit 25 - Divide by zero UsageFault"]
    #[inline]
    pub fn divbyzero(&mut self) -> _DIVBYZEROW {
        _DIVBYZEROW { w: self }
    }
}
