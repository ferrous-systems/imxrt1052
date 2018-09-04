#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::REG_CORE_TOG {
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
#[doc = "Possible values of the field `REG0_TARG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG0_TARGR {
    #[doc = "Power gated off"]
    REG0_TARG_0,
    #[doc = "Target core voltage = 0.725V"]
    REG0_TARG_1,
    #[doc = "Target core voltage = 0.750V"]
    REG0_TARG_2,
    #[doc = "Target core voltage = 0.775V"]
    REG0_TARG_3,
    #[doc = "Target core voltage = 1.100V"]
    REG0_TARG_16,
    #[doc = "Target core voltage = 1.450V"]
    REG0_TARG_30,
    #[doc = "Power FET switched full on. No regulation."]
    REG0_TARG_31,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl REG0_TARGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            REG0_TARGR::REG0_TARG_0 => 0,
            REG0_TARGR::REG0_TARG_1 => 1,
            REG0_TARGR::REG0_TARG_2 => 2,
            REG0_TARGR::REG0_TARG_3 => 3,
            REG0_TARGR::REG0_TARG_16 => 16,
            REG0_TARGR::REG0_TARG_30 => 30,
            REG0_TARGR::REG0_TARG_31 => 31,
            REG0_TARGR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> REG0_TARGR {
        match value {
            0 => REG0_TARGR::REG0_TARG_0,
            1 => REG0_TARGR::REG0_TARG_1,
            2 => REG0_TARGR::REG0_TARG_2,
            3 => REG0_TARGR::REG0_TARG_3,
            16 => REG0_TARGR::REG0_TARG_16,
            30 => REG0_TARGR::REG0_TARG_30,
            31 => REG0_TARGR::REG0_TARG_31,
            i => REG0_TARGR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `REG0_TARG_0`"]
    #[inline]
    pub fn is_reg0_targ_0(&self) -> bool {
        *self == REG0_TARGR::REG0_TARG_0
    }
    #[doc = "Checks if the value of the field is `REG0_TARG_1`"]
    #[inline]
    pub fn is_reg0_targ_1(&self) -> bool {
        *self == REG0_TARGR::REG0_TARG_1
    }
    #[doc = "Checks if the value of the field is `REG0_TARG_2`"]
    #[inline]
    pub fn is_reg0_targ_2(&self) -> bool {
        *self == REG0_TARGR::REG0_TARG_2
    }
    #[doc = "Checks if the value of the field is `REG0_TARG_3`"]
    #[inline]
    pub fn is_reg0_targ_3(&self) -> bool {
        *self == REG0_TARGR::REG0_TARG_3
    }
    #[doc = "Checks if the value of the field is `REG0_TARG_16`"]
    #[inline]
    pub fn is_reg0_targ_16(&self) -> bool {
        *self == REG0_TARGR::REG0_TARG_16
    }
    #[doc = "Checks if the value of the field is `REG0_TARG_30`"]
    #[inline]
    pub fn is_reg0_targ_30(&self) -> bool {
        *self == REG0_TARGR::REG0_TARG_30
    }
    #[doc = "Checks if the value of the field is `REG0_TARG_31`"]
    #[inline]
    pub fn is_reg0_targ_31(&self) -> bool {
        *self == REG0_TARGR::REG0_TARG_31
    }
}
#[doc = "Possible values of the field `REG0_ADJ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG0_ADJR {
    #[doc = "No adjustment"]
    REG0_ADJ_0,
    #[doc = "+ 0.25%"]
    REG0_ADJ_1,
    #[doc = "+ 0.50%"]
    REG0_ADJ_2,
    #[doc = "+ 0.75%"]
    REG0_ADJ_3,
    #[doc = "+ 1.00%"]
    REG0_ADJ_4,
    #[doc = "+ 1.25%"]
    REG0_ADJ_5,
    #[doc = "+ 1.50%"]
    REG0_ADJ_6,
    #[doc = "+ 1.75%"]
    REG0_ADJ_7,
    #[doc = "- 0.25%"]
    REG0_ADJ_8,
    #[doc = "- 0.50%"]
    REG0_ADJ_9,
    #[doc = "- 0.75%"]
    REG0_ADJ_10,
    #[doc = "- 1.00%"]
    REG0_ADJ_11,
    #[doc = "- 1.25%"]
    REG0_ADJ_12,
    #[doc = "- 1.50%"]
    REG0_ADJ_13,
    #[doc = "- 1.75%"]
    REG0_ADJ_14,
    #[doc = "- 2.00%"]
    REG0_ADJ_15,
}
impl REG0_ADJR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            REG0_ADJR::REG0_ADJ_0 => 0,
            REG0_ADJR::REG0_ADJ_1 => 1,
            REG0_ADJR::REG0_ADJ_2 => 2,
            REG0_ADJR::REG0_ADJ_3 => 3,
            REG0_ADJR::REG0_ADJ_4 => 4,
            REG0_ADJR::REG0_ADJ_5 => 5,
            REG0_ADJR::REG0_ADJ_6 => 6,
            REG0_ADJR::REG0_ADJ_7 => 7,
            REG0_ADJR::REG0_ADJ_8 => 8,
            REG0_ADJR::REG0_ADJ_9 => 9,
            REG0_ADJR::REG0_ADJ_10 => 10,
            REG0_ADJR::REG0_ADJ_11 => 11,
            REG0_ADJR::REG0_ADJ_12 => 12,
            REG0_ADJR::REG0_ADJ_13 => 13,
            REG0_ADJR::REG0_ADJ_14 => 14,
            REG0_ADJR::REG0_ADJ_15 => 15,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> REG0_ADJR {
        match value {
            0 => REG0_ADJR::REG0_ADJ_0,
            1 => REG0_ADJR::REG0_ADJ_1,
            2 => REG0_ADJR::REG0_ADJ_2,
            3 => REG0_ADJR::REG0_ADJ_3,
            4 => REG0_ADJR::REG0_ADJ_4,
            5 => REG0_ADJR::REG0_ADJ_5,
            6 => REG0_ADJR::REG0_ADJ_6,
            7 => REG0_ADJR::REG0_ADJ_7,
            8 => REG0_ADJR::REG0_ADJ_8,
            9 => REG0_ADJR::REG0_ADJ_9,
            10 => REG0_ADJR::REG0_ADJ_10,
            11 => REG0_ADJR::REG0_ADJ_11,
            12 => REG0_ADJR::REG0_ADJ_12,
            13 => REG0_ADJR::REG0_ADJ_13,
            14 => REG0_ADJR::REG0_ADJ_14,
            15 => REG0_ADJR::REG0_ADJ_15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `REG0_ADJ_0`"]
    #[inline]
    pub fn is_reg0_adj_0(&self) -> bool {
        *self == REG0_ADJR::REG0_ADJ_0
    }
    #[doc = "Checks if the value of the field is `REG0_ADJ_1`"]
    #[inline]
    pub fn is_reg0_adj_1(&self) -> bool {
        *self == REG0_ADJR::REG0_ADJ_1
    }
    #[doc = "Checks if the value of the field is `REG0_ADJ_2`"]
    #[inline]
    pub fn is_reg0_adj_2(&self) -> bool {
        *self == REG0_ADJR::REG0_ADJ_2
    }
    #[doc = "Checks if the value of the field is `REG0_ADJ_3`"]
    #[inline]
    pub fn is_reg0_adj_3(&self) -> bool {
        *self == REG0_ADJR::REG0_ADJ_3
    }
    #[doc = "Checks if the value of the field is `REG0_ADJ_4`"]
    #[inline]
    pub fn is_reg0_adj_4(&self) -> bool {
        *self == REG0_ADJR::REG0_ADJ_4
    }
    #[doc = "Checks if the value of the field is `REG0_ADJ_5`"]
    #[inline]
    pub fn is_reg0_adj_5(&self) -> bool {
        *self == REG0_ADJR::REG0_ADJ_5
    }
    #[doc = "Checks if the value of the field is `REG0_ADJ_6`"]
    #[inline]
    pub fn is_reg0_adj_6(&self) -> bool {
        *self == REG0_ADJR::REG0_ADJ_6
    }
    #[doc = "Checks if the value of the field is `REG0_ADJ_7`"]
    #[inline]
    pub fn is_reg0_adj_7(&self) -> bool {
        *self == REG0_ADJR::REG0_ADJ_7
    }
    #[doc = "Checks if the value of the field is `REG0_ADJ_8`"]
    #[inline]
    pub fn is_reg0_adj_8(&self) -> bool {
        *self == REG0_ADJR::REG0_ADJ_8
    }
    #[doc = "Checks if the value of the field is `REG0_ADJ_9`"]
    #[inline]
    pub fn is_reg0_adj_9(&self) -> bool {
        *self == REG0_ADJR::REG0_ADJ_9
    }
    #[doc = "Checks if the value of the field is `REG0_ADJ_10`"]
    #[inline]
    pub fn is_reg0_adj_10(&self) -> bool {
        *self == REG0_ADJR::REG0_ADJ_10
    }
    #[doc = "Checks if the value of the field is `REG0_ADJ_11`"]
    #[inline]
    pub fn is_reg0_adj_11(&self) -> bool {
        *self == REG0_ADJR::REG0_ADJ_11
    }
    #[doc = "Checks if the value of the field is `REG0_ADJ_12`"]
    #[inline]
    pub fn is_reg0_adj_12(&self) -> bool {
        *self == REG0_ADJR::REG0_ADJ_12
    }
    #[doc = "Checks if the value of the field is `REG0_ADJ_13`"]
    #[inline]
    pub fn is_reg0_adj_13(&self) -> bool {
        *self == REG0_ADJR::REG0_ADJ_13
    }
    #[doc = "Checks if the value of the field is `REG0_ADJ_14`"]
    #[inline]
    pub fn is_reg0_adj_14(&self) -> bool {
        *self == REG0_ADJR::REG0_ADJ_14
    }
    #[doc = "Checks if the value of the field is `REG0_ADJ_15`"]
    #[inline]
    pub fn is_reg0_adj_15(&self) -> bool {
        *self == REG0_ADJR::REG0_ADJ_15
    }
}
#[doc = "Possible values of the field `REG1_TARG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG1_TARGR {
    #[doc = "Power gated off"]
    REG1_TARG_0,
    #[doc = "Target core voltage = 0.725V"]
    REG1_TARG_1,
    #[doc = "Target core voltage = 0.750V"]
    REG1_TARG_2,
    #[doc = "Target core voltage = 0.775V"]
    REG1_TARG_3,
    #[doc = "Target core voltage = 1.100V"]
    REG1_TARG_16,
    #[doc = "Target core voltage = 1.450V"]
    REG1_TARG_30,
    #[doc = "Power FET switched full on. No regulation."]
    REG1_TARG_31,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl REG1_TARGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            REG1_TARGR::REG1_TARG_0 => 0,
            REG1_TARGR::REG1_TARG_1 => 1,
            REG1_TARGR::REG1_TARG_2 => 2,
            REG1_TARGR::REG1_TARG_3 => 3,
            REG1_TARGR::REG1_TARG_16 => 16,
            REG1_TARGR::REG1_TARG_30 => 30,
            REG1_TARGR::REG1_TARG_31 => 31,
            REG1_TARGR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> REG1_TARGR {
        match value {
            0 => REG1_TARGR::REG1_TARG_0,
            1 => REG1_TARGR::REG1_TARG_1,
            2 => REG1_TARGR::REG1_TARG_2,
            3 => REG1_TARGR::REG1_TARG_3,
            16 => REG1_TARGR::REG1_TARG_16,
            30 => REG1_TARGR::REG1_TARG_30,
            31 => REG1_TARGR::REG1_TARG_31,
            i => REG1_TARGR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `REG1_TARG_0`"]
    #[inline]
    pub fn is_reg1_targ_0(&self) -> bool {
        *self == REG1_TARGR::REG1_TARG_0
    }
    #[doc = "Checks if the value of the field is `REG1_TARG_1`"]
    #[inline]
    pub fn is_reg1_targ_1(&self) -> bool {
        *self == REG1_TARGR::REG1_TARG_1
    }
    #[doc = "Checks if the value of the field is `REG1_TARG_2`"]
    #[inline]
    pub fn is_reg1_targ_2(&self) -> bool {
        *self == REG1_TARGR::REG1_TARG_2
    }
    #[doc = "Checks if the value of the field is `REG1_TARG_3`"]
    #[inline]
    pub fn is_reg1_targ_3(&self) -> bool {
        *self == REG1_TARGR::REG1_TARG_3
    }
    #[doc = "Checks if the value of the field is `REG1_TARG_16`"]
    #[inline]
    pub fn is_reg1_targ_16(&self) -> bool {
        *self == REG1_TARGR::REG1_TARG_16
    }
    #[doc = "Checks if the value of the field is `REG1_TARG_30`"]
    #[inline]
    pub fn is_reg1_targ_30(&self) -> bool {
        *self == REG1_TARGR::REG1_TARG_30
    }
    #[doc = "Checks if the value of the field is `REG1_TARG_31`"]
    #[inline]
    pub fn is_reg1_targ_31(&self) -> bool {
        *self == REG1_TARGR::REG1_TARG_31
    }
}
#[doc = "Possible values of the field `REG1_ADJ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG1_ADJR {
    #[doc = "No adjustment"]
    REG1_ADJ_0,
    #[doc = "+ 0.25%"]
    REG1_ADJ_1,
    #[doc = "+ 0.50%"]
    REG1_ADJ_2,
    #[doc = "+ 0.75%"]
    REG1_ADJ_3,
    #[doc = "+ 1.00%"]
    REG1_ADJ_4,
    #[doc = "+ 1.25%"]
    REG1_ADJ_5,
    #[doc = "+ 1.50%"]
    REG1_ADJ_6,
    #[doc = "+ 1.75%"]
    REG1_ADJ_7,
    #[doc = "- 0.25%"]
    REG1_ADJ_8,
    #[doc = "- 0.50%"]
    REG1_ADJ_9,
    #[doc = "- 0.75%"]
    REG1_ADJ_10,
    #[doc = "- 1.00%"]
    REG1_ADJ_11,
    #[doc = "- 1.25%"]
    REG1_ADJ_12,
    #[doc = "- 1.50%"]
    REG1_ADJ_13,
    #[doc = "- 1.75%"]
    REG1_ADJ_14,
    #[doc = "- 2.00%"]
    REG1_ADJ_15,
}
impl REG1_ADJR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            REG1_ADJR::REG1_ADJ_0 => 0,
            REG1_ADJR::REG1_ADJ_1 => 1,
            REG1_ADJR::REG1_ADJ_2 => 2,
            REG1_ADJR::REG1_ADJ_3 => 3,
            REG1_ADJR::REG1_ADJ_4 => 4,
            REG1_ADJR::REG1_ADJ_5 => 5,
            REG1_ADJR::REG1_ADJ_6 => 6,
            REG1_ADJR::REG1_ADJ_7 => 7,
            REG1_ADJR::REG1_ADJ_8 => 8,
            REG1_ADJR::REG1_ADJ_9 => 9,
            REG1_ADJR::REG1_ADJ_10 => 10,
            REG1_ADJR::REG1_ADJ_11 => 11,
            REG1_ADJR::REG1_ADJ_12 => 12,
            REG1_ADJR::REG1_ADJ_13 => 13,
            REG1_ADJR::REG1_ADJ_14 => 14,
            REG1_ADJR::REG1_ADJ_15 => 15,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> REG1_ADJR {
        match value {
            0 => REG1_ADJR::REG1_ADJ_0,
            1 => REG1_ADJR::REG1_ADJ_1,
            2 => REG1_ADJR::REG1_ADJ_2,
            3 => REG1_ADJR::REG1_ADJ_3,
            4 => REG1_ADJR::REG1_ADJ_4,
            5 => REG1_ADJR::REG1_ADJ_5,
            6 => REG1_ADJR::REG1_ADJ_6,
            7 => REG1_ADJR::REG1_ADJ_7,
            8 => REG1_ADJR::REG1_ADJ_8,
            9 => REG1_ADJR::REG1_ADJ_9,
            10 => REG1_ADJR::REG1_ADJ_10,
            11 => REG1_ADJR::REG1_ADJ_11,
            12 => REG1_ADJR::REG1_ADJ_12,
            13 => REG1_ADJR::REG1_ADJ_13,
            14 => REG1_ADJR::REG1_ADJ_14,
            15 => REG1_ADJR::REG1_ADJ_15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `REG1_ADJ_0`"]
    #[inline]
    pub fn is_reg1_adj_0(&self) -> bool {
        *self == REG1_ADJR::REG1_ADJ_0
    }
    #[doc = "Checks if the value of the field is `REG1_ADJ_1`"]
    #[inline]
    pub fn is_reg1_adj_1(&self) -> bool {
        *self == REG1_ADJR::REG1_ADJ_1
    }
    #[doc = "Checks if the value of the field is `REG1_ADJ_2`"]
    #[inline]
    pub fn is_reg1_adj_2(&self) -> bool {
        *self == REG1_ADJR::REG1_ADJ_2
    }
    #[doc = "Checks if the value of the field is `REG1_ADJ_3`"]
    #[inline]
    pub fn is_reg1_adj_3(&self) -> bool {
        *self == REG1_ADJR::REG1_ADJ_3
    }
    #[doc = "Checks if the value of the field is `REG1_ADJ_4`"]
    #[inline]
    pub fn is_reg1_adj_4(&self) -> bool {
        *self == REG1_ADJR::REG1_ADJ_4
    }
    #[doc = "Checks if the value of the field is `REG1_ADJ_5`"]
    #[inline]
    pub fn is_reg1_adj_5(&self) -> bool {
        *self == REG1_ADJR::REG1_ADJ_5
    }
    #[doc = "Checks if the value of the field is `REG1_ADJ_6`"]
    #[inline]
    pub fn is_reg1_adj_6(&self) -> bool {
        *self == REG1_ADJR::REG1_ADJ_6
    }
    #[doc = "Checks if the value of the field is `REG1_ADJ_7`"]
    #[inline]
    pub fn is_reg1_adj_7(&self) -> bool {
        *self == REG1_ADJR::REG1_ADJ_7
    }
    #[doc = "Checks if the value of the field is `REG1_ADJ_8`"]
    #[inline]
    pub fn is_reg1_adj_8(&self) -> bool {
        *self == REG1_ADJR::REG1_ADJ_8
    }
    #[doc = "Checks if the value of the field is `REG1_ADJ_9`"]
    #[inline]
    pub fn is_reg1_adj_9(&self) -> bool {
        *self == REG1_ADJR::REG1_ADJ_9
    }
    #[doc = "Checks if the value of the field is `REG1_ADJ_10`"]
    #[inline]
    pub fn is_reg1_adj_10(&self) -> bool {
        *self == REG1_ADJR::REG1_ADJ_10
    }
    #[doc = "Checks if the value of the field is `REG1_ADJ_11`"]
    #[inline]
    pub fn is_reg1_adj_11(&self) -> bool {
        *self == REG1_ADJR::REG1_ADJ_11
    }
    #[doc = "Checks if the value of the field is `REG1_ADJ_12`"]
    #[inline]
    pub fn is_reg1_adj_12(&self) -> bool {
        *self == REG1_ADJR::REG1_ADJ_12
    }
    #[doc = "Checks if the value of the field is `REG1_ADJ_13`"]
    #[inline]
    pub fn is_reg1_adj_13(&self) -> bool {
        *self == REG1_ADJR::REG1_ADJ_13
    }
    #[doc = "Checks if the value of the field is `REG1_ADJ_14`"]
    #[inline]
    pub fn is_reg1_adj_14(&self) -> bool {
        *self == REG1_ADJR::REG1_ADJ_14
    }
    #[doc = "Checks if the value of the field is `REG1_ADJ_15`"]
    #[inline]
    pub fn is_reg1_adj_15(&self) -> bool {
        *self == REG1_ADJR::REG1_ADJ_15
    }
}
#[doc = "Possible values of the field `REG2_TARG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG2_TARGR {
    #[doc = "Power gated off"]
    REG2_TARG_0,
    #[doc = "Target core voltage = 0.725V"]
    REG2_TARG_1,
    #[doc = "Target core voltage = 0.750V"]
    REG2_TARG_2,
    #[doc = "Target core voltage = 0.775V"]
    REG2_TARG_3,
    #[doc = "Target core voltage = 1.100V"]
    REG2_TARG_16,
    #[doc = "Target core voltage = 1.450V"]
    REG2_TARG_30,
    #[doc = "Power FET switched full on. No regulation."]
    REG2_TARG_31,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl REG2_TARGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            REG2_TARGR::REG2_TARG_0 => 0,
            REG2_TARGR::REG2_TARG_1 => 1,
            REG2_TARGR::REG2_TARG_2 => 2,
            REG2_TARGR::REG2_TARG_3 => 3,
            REG2_TARGR::REG2_TARG_16 => 16,
            REG2_TARGR::REG2_TARG_30 => 30,
            REG2_TARGR::REG2_TARG_31 => 31,
            REG2_TARGR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> REG2_TARGR {
        match value {
            0 => REG2_TARGR::REG2_TARG_0,
            1 => REG2_TARGR::REG2_TARG_1,
            2 => REG2_TARGR::REG2_TARG_2,
            3 => REG2_TARGR::REG2_TARG_3,
            16 => REG2_TARGR::REG2_TARG_16,
            30 => REG2_TARGR::REG2_TARG_30,
            31 => REG2_TARGR::REG2_TARG_31,
            i => REG2_TARGR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `REG2_TARG_0`"]
    #[inline]
    pub fn is_reg2_targ_0(&self) -> bool {
        *self == REG2_TARGR::REG2_TARG_0
    }
    #[doc = "Checks if the value of the field is `REG2_TARG_1`"]
    #[inline]
    pub fn is_reg2_targ_1(&self) -> bool {
        *self == REG2_TARGR::REG2_TARG_1
    }
    #[doc = "Checks if the value of the field is `REG2_TARG_2`"]
    #[inline]
    pub fn is_reg2_targ_2(&self) -> bool {
        *self == REG2_TARGR::REG2_TARG_2
    }
    #[doc = "Checks if the value of the field is `REG2_TARG_3`"]
    #[inline]
    pub fn is_reg2_targ_3(&self) -> bool {
        *self == REG2_TARGR::REG2_TARG_3
    }
    #[doc = "Checks if the value of the field is `REG2_TARG_16`"]
    #[inline]
    pub fn is_reg2_targ_16(&self) -> bool {
        *self == REG2_TARGR::REG2_TARG_16
    }
    #[doc = "Checks if the value of the field is `REG2_TARG_30`"]
    #[inline]
    pub fn is_reg2_targ_30(&self) -> bool {
        *self == REG2_TARGR::REG2_TARG_30
    }
    #[doc = "Checks if the value of the field is `REG2_TARG_31`"]
    #[inline]
    pub fn is_reg2_targ_31(&self) -> bool {
        *self == REG2_TARGR::REG2_TARG_31
    }
}
#[doc = "Possible values of the field `REG2_ADJ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG2_ADJR {
    #[doc = "No adjustment"]
    REG2_ADJ_0,
    #[doc = "+ 0.25%"]
    REG2_ADJ_1,
    #[doc = "+ 0.50%"]
    REG2_ADJ_2,
    #[doc = "+ 0.75%"]
    REG2_ADJ_3,
    #[doc = "+ 1.00%"]
    REG2_ADJ_4,
    #[doc = "+ 1.25%"]
    REG2_ADJ_5,
    #[doc = "+ 1.50%"]
    REG2_ADJ_6,
    #[doc = "+ 1.75%"]
    REG2_ADJ_7,
    #[doc = "- 0.25%"]
    REG2_ADJ_8,
    #[doc = "- 0.50%"]
    REG2_ADJ_9,
    #[doc = "- 0.75%"]
    REG2_ADJ_10,
    #[doc = "- 1.00%"]
    REG2_ADJ_11,
    #[doc = "- 1.25%"]
    REG2_ADJ_12,
    #[doc = "- 1.50%"]
    REG2_ADJ_13,
    #[doc = "- 1.75%"]
    REG2_ADJ_14,
    #[doc = "- 2.00%"]
    REG2_ADJ_15,
}
impl REG2_ADJR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            REG2_ADJR::REG2_ADJ_0 => 0,
            REG2_ADJR::REG2_ADJ_1 => 1,
            REG2_ADJR::REG2_ADJ_2 => 2,
            REG2_ADJR::REG2_ADJ_3 => 3,
            REG2_ADJR::REG2_ADJ_4 => 4,
            REG2_ADJR::REG2_ADJ_5 => 5,
            REG2_ADJR::REG2_ADJ_6 => 6,
            REG2_ADJR::REG2_ADJ_7 => 7,
            REG2_ADJR::REG2_ADJ_8 => 8,
            REG2_ADJR::REG2_ADJ_9 => 9,
            REG2_ADJR::REG2_ADJ_10 => 10,
            REG2_ADJR::REG2_ADJ_11 => 11,
            REG2_ADJR::REG2_ADJ_12 => 12,
            REG2_ADJR::REG2_ADJ_13 => 13,
            REG2_ADJR::REG2_ADJ_14 => 14,
            REG2_ADJR::REG2_ADJ_15 => 15,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> REG2_ADJR {
        match value {
            0 => REG2_ADJR::REG2_ADJ_0,
            1 => REG2_ADJR::REG2_ADJ_1,
            2 => REG2_ADJR::REG2_ADJ_2,
            3 => REG2_ADJR::REG2_ADJ_3,
            4 => REG2_ADJR::REG2_ADJ_4,
            5 => REG2_ADJR::REG2_ADJ_5,
            6 => REG2_ADJR::REG2_ADJ_6,
            7 => REG2_ADJR::REG2_ADJ_7,
            8 => REG2_ADJR::REG2_ADJ_8,
            9 => REG2_ADJR::REG2_ADJ_9,
            10 => REG2_ADJR::REG2_ADJ_10,
            11 => REG2_ADJR::REG2_ADJ_11,
            12 => REG2_ADJR::REG2_ADJ_12,
            13 => REG2_ADJR::REG2_ADJ_13,
            14 => REG2_ADJR::REG2_ADJ_14,
            15 => REG2_ADJR::REG2_ADJ_15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `REG2_ADJ_0`"]
    #[inline]
    pub fn is_reg2_adj_0(&self) -> bool {
        *self == REG2_ADJR::REG2_ADJ_0
    }
    #[doc = "Checks if the value of the field is `REG2_ADJ_1`"]
    #[inline]
    pub fn is_reg2_adj_1(&self) -> bool {
        *self == REG2_ADJR::REG2_ADJ_1
    }
    #[doc = "Checks if the value of the field is `REG2_ADJ_2`"]
    #[inline]
    pub fn is_reg2_adj_2(&self) -> bool {
        *self == REG2_ADJR::REG2_ADJ_2
    }
    #[doc = "Checks if the value of the field is `REG2_ADJ_3`"]
    #[inline]
    pub fn is_reg2_adj_3(&self) -> bool {
        *self == REG2_ADJR::REG2_ADJ_3
    }
    #[doc = "Checks if the value of the field is `REG2_ADJ_4`"]
    #[inline]
    pub fn is_reg2_adj_4(&self) -> bool {
        *self == REG2_ADJR::REG2_ADJ_4
    }
    #[doc = "Checks if the value of the field is `REG2_ADJ_5`"]
    #[inline]
    pub fn is_reg2_adj_5(&self) -> bool {
        *self == REG2_ADJR::REG2_ADJ_5
    }
    #[doc = "Checks if the value of the field is `REG2_ADJ_6`"]
    #[inline]
    pub fn is_reg2_adj_6(&self) -> bool {
        *self == REG2_ADJR::REG2_ADJ_6
    }
    #[doc = "Checks if the value of the field is `REG2_ADJ_7`"]
    #[inline]
    pub fn is_reg2_adj_7(&self) -> bool {
        *self == REG2_ADJR::REG2_ADJ_7
    }
    #[doc = "Checks if the value of the field is `REG2_ADJ_8`"]
    #[inline]
    pub fn is_reg2_adj_8(&self) -> bool {
        *self == REG2_ADJR::REG2_ADJ_8
    }
    #[doc = "Checks if the value of the field is `REG2_ADJ_9`"]
    #[inline]
    pub fn is_reg2_adj_9(&self) -> bool {
        *self == REG2_ADJR::REG2_ADJ_9
    }
    #[doc = "Checks if the value of the field is `REG2_ADJ_10`"]
    #[inline]
    pub fn is_reg2_adj_10(&self) -> bool {
        *self == REG2_ADJR::REG2_ADJ_10
    }
    #[doc = "Checks if the value of the field is `REG2_ADJ_11`"]
    #[inline]
    pub fn is_reg2_adj_11(&self) -> bool {
        *self == REG2_ADJR::REG2_ADJ_11
    }
    #[doc = "Checks if the value of the field is `REG2_ADJ_12`"]
    #[inline]
    pub fn is_reg2_adj_12(&self) -> bool {
        *self == REG2_ADJR::REG2_ADJ_12
    }
    #[doc = "Checks if the value of the field is `REG2_ADJ_13`"]
    #[inline]
    pub fn is_reg2_adj_13(&self) -> bool {
        *self == REG2_ADJR::REG2_ADJ_13
    }
    #[doc = "Checks if the value of the field is `REG2_ADJ_14`"]
    #[inline]
    pub fn is_reg2_adj_14(&self) -> bool {
        *self == REG2_ADJR::REG2_ADJ_14
    }
    #[doc = "Checks if the value of the field is `REG2_ADJ_15`"]
    #[inline]
    pub fn is_reg2_adj_15(&self) -> bool {
        *self == REG2_ADJR::REG2_ADJ_15
    }
}
#[doc = "Possible values of the field `RAMP_RATE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAMP_RATER {
    #[doc = "Fast"]
    RAMP_RATE_0,
    #[doc = "Medium Fast"]
    RAMP_RATE_1,
    #[doc = "Medium Slow"]
    RAMP_RATE_2,
    #[doc = "Slow"]
    RAMP_RATE_3,
}
impl RAMP_RATER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RAMP_RATER::RAMP_RATE_0 => 0,
            RAMP_RATER::RAMP_RATE_1 => 1,
            RAMP_RATER::RAMP_RATE_2 => 2,
            RAMP_RATER::RAMP_RATE_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RAMP_RATER {
        match value {
            0 => RAMP_RATER::RAMP_RATE_0,
            1 => RAMP_RATER::RAMP_RATE_1,
            2 => RAMP_RATER::RAMP_RATE_2,
            3 => RAMP_RATER::RAMP_RATE_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RAMP_RATE_0`"]
    #[inline]
    pub fn is_ramp_rate_0(&self) -> bool {
        *self == RAMP_RATER::RAMP_RATE_0
    }
    #[doc = "Checks if the value of the field is `RAMP_RATE_1`"]
    #[inline]
    pub fn is_ramp_rate_1(&self) -> bool {
        *self == RAMP_RATER::RAMP_RATE_1
    }
    #[doc = "Checks if the value of the field is `RAMP_RATE_2`"]
    #[inline]
    pub fn is_ramp_rate_2(&self) -> bool {
        *self == RAMP_RATER::RAMP_RATE_2
    }
    #[doc = "Checks if the value of the field is `RAMP_RATE_3`"]
    #[inline]
    pub fn is_ramp_rate_3(&self) -> bool {
        *self == RAMP_RATER::RAMP_RATE_3
    }
}
#[doc = r" Value of the field"]
pub struct FET_ODRIVER {
    bits: bool,
}
impl FET_ODRIVER {
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
#[doc = "Values that can be written to the field `REG0_TARG`"]
pub enum REG0_TARGW {
    #[doc = "Power gated off"]
    REG0_TARG_0,
    #[doc = "Target core voltage = 0.725V"]
    REG0_TARG_1,
    #[doc = "Target core voltage = 0.750V"]
    REG0_TARG_2,
    #[doc = "Target core voltage = 0.775V"]
    REG0_TARG_3,
    #[doc = "Target core voltage = 1.100V"]
    REG0_TARG_16,
    #[doc = "Target core voltage = 1.450V"]
    REG0_TARG_30,
    #[doc = "Power FET switched full on. No regulation."]
    REG0_TARG_31,
}
impl REG0_TARGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            REG0_TARGW::REG0_TARG_0 => 0,
            REG0_TARGW::REG0_TARG_1 => 1,
            REG0_TARGW::REG0_TARG_2 => 2,
            REG0_TARGW::REG0_TARG_3 => 3,
            REG0_TARGW::REG0_TARG_16 => 16,
            REG0_TARGW::REG0_TARG_30 => 30,
            REG0_TARGW::REG0_TARG_31 => 31,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REG0_TARGW<'a> {
    w: &'a mut W,
}
impl<'a> _REG0_TARGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REG0_TARGW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Power gated off"]
    #[inline]
    pub fn reg0_targ_0(self) -> &'a mut W {
        self.variant(REG0_TARGW::REG0_TARG_0)
    }
    #[doc = "Target core voltage = 0.725V"]
    #[inline]
    pub fn reg0_targ_1(self) -> &'a mut W {
        self.variant(REG0_TARGW::REG0_TARG_1)
    }
    #[doc = "Target core voltage = 0.750V"]
    #[inline]
    pub fn reg0_targ_2(self) -> &'a mut W {
        self.variant(REG0_TARGW::REG0_TARG_2)
    }
    #[doc = "Target core voltage = 0.775V"]
    #[inline]
    pub fn reg0_targ_3(self) -> &'a mut W {
        self.variant(REG0_TARGW::REG0_TARG_3)
    }
    #[doc = "Target core voltage = 1.100V"]
    #[inline]
    pub fn reg0_targ_16(self) -> &'a mut W {
        self.variant(REG0_TARGW::REG0_TARG_16)
    }
    #[doc = "Target core voltage = 1.450V"]
    #[inline]
    pub fn reg0_targ_30(self) -> &'a mut W {
        self.variant(REG0_TARGW::REG0_TARG_30)
    }
    #[doc = "Power FET switched full on. No regulation."]
    #[inline]
    pub fn reg0_targ_31(self) -> &'a mut W {
        self.variant(REG0_TARGW::REG0_TARG_31)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `REG0_ADJ`"]
pub enum REG0_ADJW {
    #[doc = "No adjustment"]
    REG0_ADJ_0,
    #[doc = "+ 0.25%"]
    REG0_ADJ_1,
    #[doc = "+ 0.50%"]
    REG0_ADJ_2,
    #[doc = "+ 0.75%"]
    REG0_ADJ_3,
    #[doc = "+ 1.00%"]
    REG0_ADJ_4,
    #[doc = "+ 1.25%"]
    REG0_ADJ_5,
    #[doc = "+ 1.50%"]
    REG0_ADJ_6,
    #[doc = "+ 1.75%"]
    REG0_ADJ_7,
    #[doc = "- 0.25%"]
    REG0_ADJ_8,
    #[doc = "- 0.50%"]
    REG0_ADJ_9,
    #[doc = "- 0.75%"]
    REG0_ADJ_10,
    #[doc = "- 1.00%"]
    REG0_ADJ_11,
    #[doc = "- 1.25%"]
    REG0_ADJ_12,
    #[doc = "- 1.50%"]
    REG0_ADJ_13,
    #[doc = "- 1.75%"]
    REG0_ADJ_14,
    #[doc = "- 2.00%"]
    REG0_ADJ_15,
}
impl REG0_ADJW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            REG0_ADJW::REG0_ADJ_0 => 0,
            REG0_ADJW::REG0_ADJ_1 => 1,
            REG0_ADJW::REG0_ADJ_2 => 2,
            REG0_ADJW::REG0_ADJ_3 => 3,
            REG0_ADJW::REG0_ADJ_4 => 4,
            REG0_ADJW::REG0_ADJ_5 => 5,
            REG0_ADJW::REG0_ADJ_6 => 6,
            REG0_ADJW::REG0_ADJ_7 => 7,
            REG0_ADJW::REG0_ADJ_8 => 8,
            REG0_ADJW::REG0_ADJ_9 => 9,
            REG0_ADJW::REG0_ADJ_10 => 10,
            REG0_ADJW::REG0_ADJ_11 => 11,
            REG0_ADJW::REG0_ADJ_12 => 12,
            REG0_ADJW::REG0_ADJ_13 => 13,
            REG0_ADJW::REG0_ADJ_14 => 14,
            REG0_ADJW::REG0_ADJ_15 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REG0_ADJW<'a> {
    w: &'a mut W,
}
impl<'a> _REG0_ADJW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REG0_ADJW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No adjustment"]
    #[inline]
    pub fn reg0_adj_0(self) -> &'a mut W {
        self.variant(REG0_ADJW::REG0_ADJ_0)
    }
    #[doc = "+ 0.25%"]
    #[inline]
    pub fn reg0_adj_1(self) -> &'a mut W {
        self.variant(REG0_ADJW::REG0_ADJ_1)
    }
    #[doc = "+ 0.50%"]
    #[inline]
    pub fn reg0_adj_2(self) -> &'a mut W {
        self.variant(REG0_ADJW::REG0_ADJ_2)
    }
    #[doc = "+ 0.75%"]
    #[inline]
    pub fn reg0_adj_3(self) -> &'a mut W {
        self.variant(REG0_ADJW::REG0_ADJ_3)
    }
    #[doc = "+ 1.00%"]
    #[inline]
    pub fn reg0_adj_4(self) -> &'a mut W {
        self.variant(REG0_ADJW::REG0_ADJ_4)
    }
    #[doc = "+ 1.25%"]
    #[inline]
    pub fn reg0_adj_5(self) -> &'a mut W {
        self.variant(REG0_ADJW::REG0_ADJ_5)
    }
    #[doc = "+ 1.50%"]
    #[inline]
    pub fn reg0_adj_6(self) -> &'a mut W {
        self.variant(REG0_ADJW::REG0_ADJ_6)
    }
    #[doc = "+ 1.75%"]
    #[inline]
    pub fn reg0_adj_7(self) -> &'a mut W {
        self.variant(REG0_ADJW::REG0_ADJ_7)
    }
    #[doc = "- 0.25%"]
    #[inline]
    pub fn reg0_adj_8(self) -> &'a mut W {
        self.variant(REG0_ADJW::REG0_ADJ_8)
    }
    #[doc = "- 0.50%"]
    #[inline]
    pub fn reg0_adj_9(self) -> &'a mut W {
        self.variant(REG0_ADJW::REG0_ADJ_9)
    }
    #[doc = "- 0.75%"]
    #[inline]
    pub fn reg0_adj_10(self) -> &'a mut W {
        self.variant(REG0_ADJW::REG0_ADJ_10)
    }
    #[doc = "- 1.00%"]
    #[inline]
    pub fn reg0_adj_11(self) -> &'a mut W {
        self.variant(REG0_ADJW::REG0_ADJ_11)
    }
    #[doc = "- 1.25%"]
    #[inline]
    pub fn reg0_adj_12(self) -> &'a mut W {
        self.variant(REG0_ADJW::REG0_ADJ_12)
    }
    #[doc = "- 1.50%"]
    #[inline]
    pub fn reg0_adj_13(self) -> &'a mut W {
        self.variant(REG0_ADJW::REG0_ADJ_13)
    }
    #[doc = "- 1.75%"]
    #[inline]
    pub fn reg0_adj_14(self) -> &'a mut W {
        self.variant(REG0_ADJW::REG0_ADJ_14)
    }
    #[doc = "- 2.00%"]
    #[inline]
    pub fn reg0_adj_15(self) -> &'a mut W {
        self.variant(REG0_ADJW::REG0_ADJ_15)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `REG1_TARG`"]
pub enum REG1_TARGW {
    #[doc = "Power gated off"]
    REG1_TARG_0,
    #[doc = "Target core voltage = 0.725V"]
    REG1_TARG_1,
    #[doc = "Target core voltage = 0.750V"]
    REG1_TARG_2,
    #[doc = "Target core voltage = 0.775V"]
    REG1_TARG_3,
    #[doc = "Target core voltage = 1.100V"]
    REG1_TARG_16,
    #[doc = "Target core voltage = 1.450V"]
    REG1_TARG_30,
    #[doc = "Power FET switched full on. No regulation."]
    REG1_TARG_31,
}
impl REG1_TARGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            REG1_TARGW::REG1_TARG_0 => 0,
            REG1_TARGW::REG1_TARG_1 => 1,
            REG1_TARGW::REG1_TARG_2 => 2,
            REG1_TARGW::REG1_TARG_3 => 3,
            REG1_TARGW::REG1_TARG_16 => 16,
            REG1_TARGW::REG1_TARG_30 => 30,
            REG1_TARGW::REG1_TARG_31 => 31,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REG1_TARGW<'a> {
    w: &'a mut W,
}
impl<'a> _REG1_TARGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REG1_TARGW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Power gated off"]
    #[inline]
    pub fn reg1_targ_0(self) -> &'a mut W {
        self.variant(REG1_TARGW::REG1_TARG_0)
    }
    #[doc = "Target core voltage = 0.725V"]
    #[inline]
    pub fn reg1_targ_1(self) -> &'a mut W {
        self.variant(REG1_TARGW::REG1_TARG_1)
    }
    #[doc = "Target core voltage = 0.750V"]
    #[inline]
    pub fn reg1_targ_2(self) -> &'a mut W {
        self.variant(REG1_TARGW::REG1_TARG_2)
    }
    #[doc = "Target core voltage = 0.775V"]
    #[inline]
    pub fn reg1_targ_3(self) -> &'a mut W {
        self.variant(REG1_TARGW::REG1_TARG_3)
    }
    #[doc = "Target core voltage = 1.100V"]
    #[inline]
    pub fn reg1_targ_16(self) -> &'a mut W {
        self.variant(REG1_TARGW::REG1_TARG_16)
    }
    #[doc = "Target core voltage = 1.450V"]
    #[inline]
    pub fn reg1_targ_30(self) -> &'a mut W {
        self.variant(REG1_TARGW::REG1_TARG_30)
    }
    #[doc = "Power FET switched full on. No regulation."]
    #[inline]
    pub fn reg1_targ_31(self) -> &'a mut W {
        self.variant(REG1_TARGW::REG1_TARG_31)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `REG1_ADJ`"]
pub enum REG1_ADJW {
    #[doc = "No adjustment"]
    REG1_ADJ_0,
    #[doc = "+ 0.25%"]
    REG1_ADJ_1,
    #[doc = "+ 0.50%"]
    REG1_ADJ_2,
    #[doc = "+ 0.75%"]
    REG1_ADJ_3,
    #[doc = "+ 1.00%"]
    REG1_ADJ_4,
    #[doc = "+ 1.25%"]
    REG1_ADJ_5,
    #[doc = "+ 1.50%"]
    REG1_ADJ_6,
    #[doc = "+ 1.75%"]
    REG1_ADJ_7,
    #[doc = "- 0.25%"]
    REG1_ADJ_8,
    #[doc = "- 0.50%"]
    REG1_ADJ_9,
    #[doc = "- 0.75%"]
    REG1_ADJ_10,
    #[doc = "- 1.00%"]
    REG1_ADJ_11,
    #[doc = "- 1.25%"]
    REG1_ADJ_12,
    #[doc = "- 1.50%"]
    REG1_ADJ_13,
    #[doc = "- 1.75%"]
    REG1_ADJ_14,
    #[doc = "- 2.00%"]
    REG1_ADJ_15,
}
impl REG1_ADJW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            REG1_ADJW::REG1_ADJ_0 => 0,
            REG1_ADJW::REG1_ADJ_1 => 1,
            REG1_ADJW::REG1_ADJ_2 => 2,
            REG1_ADJW::REG1_ADJ_3 => 3,
            REG1_ADJW::REG1_ADJ_4 => 4,
            REG1_ADJW::REG1_ADJ_5 => 5,
            REG1_ADJW::REG1_ADJ_6 => 6,
            REG1_ADJW::REG1_ADJ_7 => 7,
            REG1_ADJW::REG1_ADJ_8 => 8,
            REG1_ADJW::REG1_ADJ_9 => 9,
            REG1_ADJW::REG1_ADJ_10 => 10,
            REG1_ADJW::REG1_ADJ_11 => 11,
            REG1_ADJW::REG1_ADJ_12 => 12,
            REG1_ADJW::REG1_ADJ_13 => 13,
            REG1_ADJW::REG1_ADJ_14 => 14,
            REG1_ADJW::REG1_ADJ_15 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REG1_ADJW<'a> {
    w: &'a mut W,
}
impl<'a> _REG1_ADJW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REG1_ADJW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No adjustment"]
    #[inline]
    pub fn reg1_adj_0(self) -> &'a mut W {
        self.variant(REG1_ADJW::REG1_ADJ_0)
    }
    #[doc = "+ 0.25%"]
    #[inline]
    pub fn reg1_adj_1(self) -> &'a mut W {
        self.variant(REG1_ADJW::REG1_ADJ_1)
    }
    #[doc = "+ 0.50%"]
    #[inline]
    pub fn reg1_adj_2(self) -> &'a mut W {
        self.variant(REG1_ADJW::REG1_ADJ_2)
    }
    #[doc = "+ 0.75%"]
    #[inline]
    pub fn reg1_adj_3(self) -> &'a mut W {
        self.variant(REG1_ADJW::REG1_ADJ_3)
    }
    #[doc = "+ 1.00%"]
    #[inline]
    pub fn reg1_adj_4(self) -> &'a mut W {
        self.variant(REG1_ADJW::REG1_ADJ_4)
    }
    #[doc = "+ 1.25%"]
    #[inline]
    pub fn reg1_adj_5(self) -> &'a mut W {
        self.variant(REG1_ADJW::REG1_ADJ_5)
    }
    #[doc = "+ 1.50%"]
    #[inline]
    pub fn reg1_adj_6(self) -> &'a mut W {
        self.variant(REG1_ADJW::REG1_ADJ_6)
    }
    #[doc = "+ 1.75%"]
    #[inline]
    pub fn reg1_adj_7(self) -> &'a mut W {
        self.variant(REG1_ADJW::REG1_ADJ_7)
    }
    #[doc = "- 0.25%"]
    #[inline]
    pub fn reg1_adj_8(self) -> &'a mut W {
        self.variant(REG1_ADJW::REG1_ADJ_8)
    }
    #[doc = "- 0.50%"]
    #[inline]
    pub fn reg1_adj_9(self) -> &'a mut W {
        self.variant(REG1_ADJW::REG1_ADJ_9)
    }
    #[doc = "- 0.75%"]
    #[inline]
    pub fn reg1_adj_10(self) -> &'a mut W {
        self.variant(REG1_ADJW::REG1_ADJ_10)
    }
    #[doc = "- 1.00%"]
    #[inline]
    pub fn reg1_adj_11(self) -> &'a mut W {
        self.variant(REG1_ADJW::REG1_ADJ_11)
    }
    #[doc = "- 1.25%"]
    #[inline]
    pub fn reg1_adj_12(self) -> &'a mut W {
        self.variant(REG1_ADJW::REG1_ADJ_12)
    }
    #[doc = "- 1.50%"]
    #[inline]
    pub fn reg1_adj_13(self) -> &'a mut W {
        self.variant(REG1_ADJW::REG1_ADJ_13)
    }
    #[doc = "- 1.75%"]
    #[inline]
    pub fn reg1_adj_14(self) -> &'a mut W {
        self.variant(REG1_ADJW::REG1_ADJ_14)
    }
    #[doc = "- 2.00%"]
    #[inline]
    pub fn reg1_adj_15(self) -> &'a mut W {
        self.variant(REG1_ADJW::REG1_ADJ_15)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `REG2_TARG`"]
pub enum REG2_TARGW {
    #[doc = "Power gated off"]
    REG2_TARG_0,
    #[doc = "Target core voltage = 0.725V"]
    REG2_TARG_1,
    #[doc = "Target core voltage = 0.750V"]
    REG2_TARG_2,
    #[doc = "Target core voltage = 0.775V"]
    REG2_TARG_3,
    #[doc = "Target core voltage = 1.100V"]
    REG2_TARG_16,
    #[doc = "Target core voltage = 1.450V"]
    REG2_TARG_30,
    #[doc = "Power FET switched full on. No regulation."]
    REG2_TARG_31,
}
impl REG2_TARGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            REG2_TARGW::REG2_TARG_0 => 0,
            REG2_TARGW::REG2_TARG_1 => 1,
            REG2_TARGW::REG2_TARG_2 => 2,
            REG2_TARGW::REG2_TARG_3 => 3,
            REG2_TARGW::REG2_TARG_16 => 16,
            REG2_TARGW::REG2_TARG_30 => 30,
            REG2_TARGW::REG2_TARG_31 => 31,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REG2_TARGW<'a> {
    w: &'a mut W,
}
impl<'a> _REG2_TARGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REG2_TARGW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Power gated off"]
    #[inline]
    pub fn reg2_targ_0(self) -> &'a mut W {
        self.variant(REG2_TARGW::REG2_TARG_0)
    }
    #[doc = "Target core voltage = 0.725V"]
    #[inline]
    pub fn reg2_targ_1(self) -> &'a mut W {
        self.variant(REG2_TARGW::REG2_TARG_1)
    }
    #[doc = "Target core voltage = 0.750V"]
    #[inline]
    pub fn reg2_targ_2(self) -> &'a mut W {
        self.variant(REG2_TARGW::REG2_TARG_2)
    }
    #[doc = "Target core voltage = 0.775V"]
    #[inline]
    pub fn reg2_targ_3(self) -> &'a mut W {
        self.variant(REG2_TARGW::REG2_TARG_3)
    }
    #[doc = "Target core voltage = 1.100V"]
    #[inline]
    pub fn reg2_targ_16(self) -> &'a mut W {
        self.variant(REG2_TARGW::REG2_TARG_16)
    }
    #[doc = "Target core voltage = 1.450V"]
    #[inline]
    pub fn reg2_targ_30(self) -> &'a mut W {
        self.variant(REG2_TARGW::REG2_TARG_30)
    }
    #[doc = "Power FET switched full on. No regulation."]
    #[inline]
    pub fn reg2_targ_31(self) -> &'a mut W {
        self.variant(REG2_TARGW::REG2_TARG_31)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `REG2_ADJ`"]
pub enum REG2_ADJW {
    #[doc = "No adjustment"]
    REG2_ADJ_0,
    #[doc = "+ 0.25%"]
    REG2_ADJ_1,
    #[doc = "+ 0.50%"]
    REG2_ADJ_2,
    #[doc = "+ 0.75%"]
    REG2_ADJ_3,
    #[doc = "+ 1.00%"]
    REG2_ADJ_4,
    #[doc = "+ 1.25%"]
    REG2_ADJ_5,
    #[doc = "+ 1.50%"]
    REG2_ADJ_6,
    #[doc = "+ 1.75%"]
    REG2_ADJ_7,
    #[doc = "- 0.25%"]
    REG2_ADJ_8,
    #[doc = "- 0.50%"]
    REG2_ADJ_9,
    #[doc = "- 0.75%"]
    REG2_ADJ_10,
    #[doc = "- 1.00%"]
    REG2_ADJ_11,
    #[doc = "- 1.25%"]
    REG2_ADJ_12,
    #[doc = "- 1.50%"]
    REG2_ADJ_13,
    #[doc = "- 1.75%"]
    REG2_ADJ_14,
    #[doc = "- 2.00%"]
    REG2_ADJ_15,
}
impl REG2_ADJW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            REG2_ADJW::REG2_ADJ_0 => 0,
            REG2_ADJW::REG2_ADJ_1 => 1,
            REG2_ADJW::REG2_ADJ_2 => 2,
            REG2_ADJW::REG2_ADJ_3 => 3,
            REG2_ADJW::REG2_ADJ_4 => 4,
            REG2_ADJW::REG2_ADJ_5 => 5,
            REG2_ADJW::REG2_ADJ_6 => 6,
            REG2_ADJW::REG2_ADJ_7 => 7,
            REG2_ADJW::REG2_ADJ_8 => 8,
            REG2_ADJW::REG2_ADJ_9 => 9,
            REG2_ADJW::REG2_ADJ_10 => 10,
            REG2_ADJW::REG2_ADJ_11 => 11,
            REG2_ADJW::REG2_ADJ_12 => 12,
            REG2_ADJW::REG2_ADJ_13 => 13,
            REG2_ADJW::REG2_ADJ_14 => 14,
            REG2_ADJW::REG2_ADJ_15 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REG2_ADJW<'a> {
    w: &'a mut W,
}
impl<'a> _REG2_ADJW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REG2_ADJW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No adjustment"]
    #[inline]
    pub fn reg2_adj_0(self) -> &'a mut W {
        self.variant(REG2_ADJW::REG2_ADJ_0)
    }
    #[doc = "+ 0.25%"]
    #[inline]
    pub fn reg2_adj_1(self) -> &'a mut W {
        self.variant(REG2_ADJW::REG2_ADJ_1)
    }
    #[doc = "+ 0.50%"]
    #[inline]
    pub fn reg2_adj_2(self) -> &'a mut W {
        self.variant(REG2_ADJW::REG2_ADJ_2)
    }
    #[doc = "+ 0.75%"]
    #[inline]
    pub fn reg2_adj_3(self) -> &'a mut W {
        self.variant(REG2_ADJW::REG2_ADJ_3)
    }
    #[doc = "+ 1.00%"]
    #[inline]
    pub fn reg2_adj_4(self) -> &'a mut W {
        self.variant(REG2_ADJW::REG2_ADJ_4)
    }
    #[doc = "+ 1.25%"]
    #[inline]
    pub fn reg2_adj_5(self) -> &'a mut W {
        self.variant(REG2_ADJW::REG2_ADJ_5)
    }
    #[doc = "+ 1.50%"]
    #[inline]
    pub fn reg2_adj_6(self) -> &'a mut W {
        self.variant(REG2_ADJW::REG2_ADJ_6)
    }
    #[doc = "+ 1.75%"]
    #[inline]
    pub fn reg2_adj_7(self) -> &'a mut W {
        self.variant(REG2_ADJW::REG2_ADJ_7)
    }
    #[doc = "- 0.25%"]
    #[inline]
    pub fn reg2_adj_8(self) -> &'a mut W {
        self.variant(REG2_ADJW::REG2_ADJ_8)
    }
    #[doc = "- 0.50%"]
    #[inline]
    pub fn reg2_adj_9(self) -> &'a mut W {
        self.variant(REG2_ADJW::REG2_ADJ_9)
    }
    #[doc = "- 0.75%"]
    #[inline]
    pub fn reg2_adj_10(self) -> &'a mut W {
        self.variant(REG2_ADJW::REG2_ADJ_10)
    }
    #[doc = "- 1.00%"]
    #[inline]
    pub fn reg2_adj_11(self) -> &'a mut W {
        self.variant(REG2_ADJW::REG2_ADJ_11)
    }
    #[doc = "- 1.25%"]
    #[inline]
    pub fn reg2_adj_12(self) -> &'a mut W {
        self.variant(REG2_ADJW::REG2_ADJ_12)
    }
    #[doc = "- 1.50%"]
    #[inline]
    pub fn reg2_adj_13(self) -> &'a mut W {
        self.variant(REG2_ADJW::REG2_ADJ_13)
    }
    #[doc = "- 1.75%"]
    #[inline]
    pub fn reg2_adj_14(self) -> &'a mut W {
        self.variant(REG2_ADJW::REG2_ADJ_14)
    }
    #[doc = "- 2.00%"]
    #[inline]
    pub fn reg2_adj_15(self) -> &'a mut W {
        self.variant(REG2_ADJW::REG2_ADJ_15)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RAMP_RATE`"]
pub enum RAMP_RATEW {
    #[doc = "Fast"]
    RAMP_RATE_0,
    #[doc = "Medium Fast"]
    RAMP_RATE_1,
    #[doc = "Medium Slow"]
    RAMP_RATE_2,
    #[doc = "Slow"]
    RAMP_RATE_3,
}
impl RAMP_RATEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RAMP_RATEW::RAMP_RATE_0 => 0,
            RAMP_RATEW::RAMP_RATE_1 => 1,
            RAMP_RATEW::RAMP_RATE_2 => 2,
            RAMP_RATEW::RAMP_RATE_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RAMP_RATEW<'a> {
    w: &'a mut W,
}
impl<'a> _RAMP_RATEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RAMP_RATEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Fast"]
    #[inline]
    pub fn ramp_rate_0(self) -> &'a mut W {
        self.variant(RAMP_RATEW::RAMP_RATE_0)
    }
    #[doc = "Medium Fast"]
    #[inline]
    pub fn ramp_rate_1(self) -> &'a mut W {
        self.variant(RAMP_RATEW::RAMP_RATE_1)
    }
    #[doc = "Medium Slow"]
    #[inline]
    pub fn ramp_rate_2(self) -> &'a mut W {
        self.variant(RAMP_RATEW::RAMP_RATE_2)
    }
    #[doc = "Slow"]
    #[inline]
    pub fn ramp_rate_3(self) -> &'a mut W {
        self.variant(RAMP_RATEW::RAMP_RATE_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FET_ODRIVEW<'a> {
    w: &'a mut W,
}
impl<'a> _FET_ODRIVEW<'a> {
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
        const OFFSET: u8 = 29;
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
    #[doc = "Bits 0:4 - This field defines the target voltage for the ARM core power domain"]
    #[inline]
    pub fn reg0_targ(&self) -> REG0_TARGR {
        REG0_TARGR::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 5:8 - This bit field defines the adjustment bits to calibrate the target value of Reg0. The adjustment is applied on top on any adjustment applied to the global reference in the misc0 register."]
    #[inline]
    pub fn reg0_adj(&self) -> REG0_ADJR {
        REG0_ADJR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 9:13 - This bit field defines the target voltage for the vpu/gpu power domain. Single bit increments reflect 25mV core voltage steps. Not all steps will make sense to use either because of input supply limitations or load operation."]
    #[inline]
    pub fn reg1_targ(&self) -> REG1_TARGR {
        REG1_TARGR::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 14:17 - This bit field defines the adjustment bits to calibrate the target value of Reg1. The adjustment is applied on top on any adjustment applied to the global reference in the misc0 register."]
    #[inline]
    pub fn reg1_adj(&self) -> REG1_ADJR {
        REG1_ADJR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 18:22 - This field defines the target voltage for the SOC power domain"]
    #[inline]
    pub fn reg2_targ(&self) -> REG2_TARGR {
        REG2_TARGR::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 23:26 - This bit field defines the adjustment bits to calibrate the target value of Reg2. The adjustment is applied on top on any adjustment applied to the global reference in the misc0 register."]
    #[inline]
    pub fn reg2_adj(&self) -> REG2_ADJR {
        REG2_ADJR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 27:28 - Regulator voltage ramp rate."]
    #[inline]
    pub fn ramp_rate(&self) -> RAMP_RATER {
        RAMP_RATER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 29 - If set, increases the gate drive on power gating FETs to reduce leakage in the off state"]
    #[inline]
    pub fn fet_odrive(&self) -> FET_ODRIVER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FET_ODRIVER { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 4726802 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:4 - This field defines the target voltage for the ARM core power domain"]
    #[inline]
    pub fn reg0_targ(&mut self) -> _REG0_TARGW {
        _REG0_TARGW { w: self }
    }
    #[doc = "Bits 5:8 - This bit field defines the adjustment bits to calibrate the target value of Reg0. The adjustment is applied on top on any adjustment applied to the global reference in the misc0 register."]
    #[inline]
    pub fn reg0_adj(&mut self) -> _REG0_ADJW {
        _REG0_ADJW { w: self }
    }
    #[doc = "Bits 9:13 - This bit field defines the target voltage for the vpu/gpu power domain. Single bit increments reflect 25mV core voltage steps. Not all steps will make sense to use either because of input supply limitations or load operation."]
    #[inline]
    pub fn reg1_targ(&mut self) -> _REG1_TARGW {
        _REG1_TARGW { w: self }
    }
    #[doc = "Bits 14:17 - This bit field defines the adjustment bits to calibrate the target value of Reg1. The adjustment is applied on top on any adjustment applied to the global reference in the misc0 register."]
    #[inline]
    pub fn reg1_adj(&mut self) -> _REG1_ADJW {
        _REG1_ADJW { w: self }
    }
    #[doc = "Bits 18:22 - This field defines the target voltage for the SOC power domain"]
    #[inline]
    pub fn reg2_targ(&mut self) -> _REG2_TARGW {
        _REG2_TARGW { w: self }
    }
    #[doc = "Bits 23:26 - This bit field defines the adjustment bits to calibrate the target value of Reg2. The adjustment is applied on top on any adjustment applied to the global reference in the misc0 register."]
    #[inline]
    pub fn reg2_adj(&mut self) -> _REG2_ADJW {
        _REG2_ADJW { w: self }
    }
    #[doc = "Bits 27:28 - Regulator voltage ramp rate."]
    #[inline]
    pub fn ramp_rate(&mut self) -> _RAMP_RATEW {
        _RAMP_RATEW { w: self }
    }
    #[doc = "Bit 29 - If set, increases the gate drive on power gating FETs to reduce leakage in the off state"]
    #[inline]
    pub fn fet_odrive(&mut self) -> _FET_ODRIVEW {
        _FET_ODRIVEW { w: self }
    }
}
