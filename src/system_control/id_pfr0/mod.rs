#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::ID_PFR0 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `STATE0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STATE0R {
    #[doc = "ARMv7-M unused"]
    STATE0_0,
    #[doc = "ARMv7-M unused"]
    STATE0_1,
    #[doc = "ARMv7-M unused"]
    STATE0_2,
    #[doc = "Support for Thumb encoding including Thumb-2 technology, with all basic 16-bit and 32-bit instructions."]
    STATE0_3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl STATE0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            STATE0R::STATE0_0 => 0,
            STATE0R::STATE0_1 => 1,
            STATE0R::STATE0_2 => 2,
            STATE0R::STATE0_3 => 3,
            STATE0R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> STATE0R {
        match value {
            0 => STATE0R::STATE0_0,
            1 => STATE0R::STATE0_1,
            2 => STATE0R::STATE0_2,
            3 => STATE0R::STATE0_3,
            i => STATE0R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `STATE0_0`"]
    #[inline]
    pub fn is_state0_0(&self) -> bool {
        *self == STATE0R::STATE0_0
    }
    #[doc = "Checks if the value of the field is `STATE0_1`"]
    #[inline]
    pub fn is_state0_1(&self) -> bool {
        *self == STATE0R::STATE0_1
    }
    #[doc = "Checks if the value of the field is `STATE0_2`"]
    #[inline]
    pub fn is_state0_2(&self) -> bool {
        *self == STATE0R::STATE0_2
    }
    #[doc = "Checks if the value of the field is `STATE0_3`"]
    #[inline]
    pub fn is_state0_3(&self) -> bool {
        *self == STATE0R::STATE0_3
    }
}
#[doc = "Possible values of the field `STATE1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STATE1R {
    #[doc = "The processor does not support the ARM instruction set."]
    STATE1_0,
    #[doc = "ARMv7-M unused"]
    STATE1_1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl STATE1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            STATE1R::STATE1_0 => 0,
            STATE1R::STATE1_1 => 1,
            STATE1R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> STATE1R {
        match value {
            0 => STATE1R::STATE1_0,
            1 => STATE1R::STATE1_1,
            i => STATE1R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `STATE1_0`"]
    #[inline]
    pub fn is_state1_0(&self) -> bool {
        *self == STATE1R::STATE1_0
    }
    #[doc = "Checks if the value of the field is `STATE1_1`"]
    #[inline]
    pub fn is_state1_1(&self) -> bool {
        *self == STATE1R::STATE1_1
    }
}
#[doc = r" Value of the field"]
pub struct STATE2R {
    bits: u8,
}
impl STATE2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct STATE3R {
    bits: u8,
}
impl STATE3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - ARM instruction set support"]
    #[inline]
    pub fn state0(&self) -> STATE0R {
        STATE0R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:7 - Thumb instruction set support"]
    #[inline]
    pub fn state1(&self) -> STATE1R {
        STATE1R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:11 - ARMv7-M unused"]
    #[inline]
    pub fn state2(&self) -> STATE2R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        STATE2R { bits }
    }
    #[doc = "Bits 12:15 - ARMv7-M unused"]
    #[inline]
    pub fn state3(&self) -> STATE3R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        STATE3R { bits }
    }
}
