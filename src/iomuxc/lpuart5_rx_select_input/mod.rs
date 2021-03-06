#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::LPUART5_RX_SELECT_INPUT {
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
#[doc = "Possible values of the field `DAISY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DAISYR {
    #[doc = "Selecting Pad: GPIO_EMC_24 for Mode: ALT2"]
    GPIO_EMC_24_ALT2,
    #[doc = "Selecting Pad: GPIO_B1_13 for Mode: ALT1"]
    GPIO_B1_13_ALT1,
}
impl DAISYR {
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
            DAISYR::GPIO_EMC_24_ALT2 => false,
            DAISYR::GPIO_B1_13_ALT1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DAISYR {
        match value {
            false => DAISYR::GPIO_EMC_24_ALT2,
            true => DAISYR::GPIO_B1_13_ALT1,
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_EMC_24_ALT2`"]
    #[inline]
    pub fn is_gpio_emc_24_alt2(&self) -> bool {
        *self == DAISYR::GPIO_EMC_24_ALT2
    }
    #[doc = "Checks if the value of the field is `GPIO_B1_13_ALT1`"]
    #[inline]
    pub fn is_gpio_b1_13_alt1(&self) -> bool {
        *self == DAISYR::GPIO_B1_13_ALT1
    }
}
#[doc = "Values that can be written to the field `DAISY`"]
pub enum DAISYW {
    #[doc = "Selecting Pad: GPIO_EMC_24 for Mode: ALT2"]
    GPIO_EMC_24_ALT2,
    #[doc = "Selecting Pad: GPIO_B1_13 for Mode: ALT1"]
    GPIO_B1_13_ALT1,
}
impl DAISYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DAISYW::GPIO_EMC_24_ALT2 => false,
            DAISYW::GPIO_B1_13_ALT1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DAISYW<'a> {
    w: &'a mut W,
}
impl<'a> _DAISYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DAISYW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Selecting Pad: GPIO_EMC_24 for Mode: ALT2"]
    #[inline]
    pub fn gpio_emc_24_alt2(self) -> &'a mut W {
        self.variant(DAISYW::GPIO_EMC_24_ALT2)
    }
    #[doc = "Selecting Pad: GPIO_B1_13 for Mode: ALT1"]
    #[inline]
    pub fn gpio_b1_13_alt1(self) -> &'a mut W {
        self.variant(DAISYW::GPIO_B1_13_ALT1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Selecting Pads Involved in Daisy Chain."]
    #[inline]
    pub fn daisy(&self) -> DAISYR {
        DAISYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
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
    #[doc = "Bit 0 - Selecting Pads Involved in Daisy Chain."]
    #[inline]
    pub fn daisy(&mut self) -> _DAISYW {
        _DAISYW { w: self }
    }
}
