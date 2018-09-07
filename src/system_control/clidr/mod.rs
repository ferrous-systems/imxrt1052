#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::CLIDR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `CL1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CL1R {
    #[doc = "No cache"]
    CL1_0,
    #[doc = "Instruction cache only"]
    CL1_1,
    #[doc = "Data cache only"]
    CL1_2,
    #[doc = "Separate instruction and data caches"]
    CL1_3,
    #[doc = "Unified cache"]
    CL1_4,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CL1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CL1R::CL1_0 => 0,
            CL1R::CL1_1 => 1,
            CL1R::CL1_2 => 2,
            CL1R::CL1_3 => 3,
            CL1R::CL1_4 => 4,
            CL1R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CL1R {
        match value {
            0 => CL1R::CL1_0,
            1 => CL1R::CL1_1,
            2 => CL1R::CL1_2,
            3 => CL1R::CL1_3,
            4 => CL1R::CL1_4,
            i => CL1R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CL1_0`"]
    #[inline]
    pub fn is_cl1_0(&self) -> bool {
        *self == CL1R::CL1_0
    }
    #[doc = "Checks if the value of the field is `CL1_1`"]
    #[inline]
    pub fn is_cl1_1(&self) -> bool {
        *self == CL1R::CL1_1
    }
    #[doc = "Checks if the value of the field is `CL1_2`"]
    #[inline]
    pub fn is_cl1_2(&self) -> bool {
        *self == CL1R::CL1_2
    }
    #[doc = "Checks if the value of the field is `CL1_3`"]
    #[inline]
    pub fn is_cl1_3(&self) -> bool {
        *self == CL1R::CL1_3
    }
    #[doc = "Checks if the value of the field is `CL1_4`"]
    #[inline]
    pub fn is_cl1_4(&self) -> bool {
        *self == CL1R::CL1_4
    }
}
#[doc = "Possible values of the field `CL2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CL2R {
    #[doc = "No cache"]
    CL2_0,
    #[doc = "Instruction cache only"]
    CL2_1,
    #[doc = "Data cache only"]
    CL2_2,
    #[doc = "Separate instruction and data caches"]
    CL2_3,
    #[doc = "Unified cache"]
    CL2_4,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CL2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CL2R::CL2_0 => 0,
            CL2R::CL2_1 => 1,
            CL2R::CL2_2 => 2,
            CL2R::CL2_3 => 3,
            CL2R::CL2_4 => 4,
            CL2R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CL2R {
        match value {
            0 => CL2R::CL2_0,
            1 => CL2R::CL2_1,
            2 => CL2R::CL2_2,
            3 => CL2R::CL2_3,
            4 => CL2R::CL2_4,
            i => CL2R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CL2_0`"]
    #[inline]
    pub fn is_cl2_0(&self) -> bool {
        *self == CL2R::CL2_0
    }
    #[doc = "Checks if the value of the field is `CL2_1`"]
    #[inline]
    pub fn is_cl2_1(&self) -> bool {
        *self == CL2R::CL2_1
    }
    #[doc = "Checks if the value of the field is `CL2_2`"]
    #[inline]
    pub fn is_cl2_2(&self) -> bool {
        *self == CL2R::CL2_2
    }
    #[doc = "Checks if the value of the field is `CL2_3`"]
    #[inline]
    pub fn is_cl2_3(&self) -> bool {
        *self == CL2R::CL2_3
    }
    #[doc = "Checks if the value of the field is `CL2_4`"]
    #[inline]
    pub fn is_cl2_4(&self) -> bool {
        *self == CL2R::CL2_4
    }
}
#[doc = "Possible values of the field `CL3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CL3R {
    #[doc = "No cache"]
    CL3_0,
    #[doc = "Instruction cache only"]
    CL3_1,
    #[doc = "Data cache only"]
    CL3_2,
    #[doc = "Separate instruction and data caches"]
    CL3_3,
    #[doc = "Unified cache"]
    CL3_4,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CL3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CL3R::CL3_0 => 0,
            CL3R::CL3_1 => 1,
            CL3R::CL3_2 => 2,
            CL3R::CL3_3 => 3,
            CL3R::CL3_4 => 4,
            CL3R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CL3R {
        match value {
            0 => CL3R::CL3_0,
            1 => CL3R::CL3_1,
            2 => CL3R::CL3_2,
            3 => CL3R::CL3_3,
            4 => CL3R::CL3_4,
            i => CL3R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CL3_0`"]
    #[inline]
    pub fn is_cl3_0(&self) -> bool {
        *self == CL3R::CL3_0
    }
    #[doc = "Checks if the value of the field is `CL3_1`"]
    #[inline]
    pub fn is_cl3_1(&self) -> bool {
        *self == CL3R::CL3_1
    }
    #[doc = "Checks if the value of the field is `CL3_2`"]
    #[inline]
    pub fn is_cl3_2(&self) -> bool {
        *self == CL3R::CL3_2
    }
    #[doc = "Checks if the value of the field is `CL3_3`"]
    #[inline]
    pub fn is_cl3_3(&self) -> bool {
        *self == CL3R::CL3_3
    }
    #[doc = "Checks if the value of the field is `CL3_4`"]
    #[inline]
    pub fn is_cl3_4(&self) -> bool {
        *self == CL3R::CL3_4
    }
}
#[doc = "Possible values of the field `CL4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CL4R {
    #[doc = "No cache"]
    CL4_0,
    #[doc = "Instruction cache only"]
    CL4_1,
    #[doc = "Data cache only"]
    CL4_2,
    #[doc = "Separate instruction and data caches"]
    CL4_3,
    #[doc = "Unified cache"]
    CL4_4,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CL4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CL4R::CL4_0 => 0,
            CL4R::CL4_1 => 1,
            CL4R::CL4_2 => 2,
            CL4R::CL4_3 => 3,
            CL4R::CL4_4 => 4,
            CL4R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CL4R {
        match value {
            0 => CL4R::CL4_0,
            1 => CL4R::CL4_1,
            2 => CL4R::CL4_2,
            3 => CL4R::CL4_3,
            4 => CL4R::CL4_4,
            i => CL4R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CL4_0`"]
    #[inline]
    pub fn is_cl4_0(&self) -> bool {
        *self == CL4R::CL4_0
    }
    #[doc = "Checks if the value of the field is `CL4_1`"]
    #[inline]
    pub fn is_cl4_1(&self) -> bool {
        *self == CL4R::CL4_1
    }
    #[doc = "Checks if the value of the field is `CL4_2`"]
    #[inline]
    pub fn is_cl4_2(&self) -> bool {
        *self == CL4R::CL4_2
    }
    #[doc = "Checks if the value of the field is `CL4_3`"]
    #[inline]
    pub fn is_cl4_3(&self) -> bool {
        *self == CL4R::CL4_3
    }
    #[doc = "Checks if the value of the field is `CL4_4`"]
    #[inline]
    pub fn is_cl4_4(&self) -> bool {
        *self == CL4R::CL4_4
    }
}
#[doc = "Possible values of the field `CL5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CL5R {
    #[doc = "No cache"]
    CL5_0,
    #[doc = "Instruction cache only"]
    CL5_1,
    #[doc = "Data cache only"]
    CL5_2,
    #[doc = "Separate instruction and data caches"]
    CL5_3,
    #[doc = "Unified cache"]
    CL5_4,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CL5R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CL5R::CL5_0 => 0,
            CL5R::CL5_1 => 1,
            CL5R::CL5_2 => 2,
            CL5R::CL5_3 => 3,
            CL5R::CL5_4 => 4,
            CL5R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CL5R {
        match value {
            0 => CL5R::CL5_0,
            1 => CL5R::CL5_1,
            2 => CL5R::CL5_2,
            3 => CL5R::CL5_3,
            4 => CL5R::CL5_4,
            i => CL5R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CL5_0`"]
    #[inline]
    pub fn is_cl5_0(&self) -> bool {
        *self == CL5R::CL5_0
    }
    #[doc = "Checks if the value of the field is `CL5_1`"]
    #[inline]
    pub fn is_cl5_1(&self) -> bool {
        *self == CL5R::CL5_1
    }
    #[doc = "Checks if the value of the field is `CL5_2`"]
    #[inline]
    pub fn is_cl5_2(&self) -> bool {
        *self == CL5R::CL5_2
    }
    #[doc = "Checks if the value of the field is `CL5_3`"]
    #[inline]
    pub fn is_cl5_3(&self) -> bool {
        *self == CL5R::CL5_3
    }
    #[doc = "Checks if the value of the field is `CL5_4`"]
    #[inline]
    pub fn is_cl5_4(&self) -> bool {
        *self == CL5R::CL5_4
    }
}
#[doc = "Possible values of the field `CL6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CL6R {
    #[doc = "No cache"]
    CL6_0,
    #[doc = "Instruction cache only"]
    CL6_1,
    #[doc = "Data cache only"]
    CL6_2,
    #[doc = "Separate instruction and data caches"]
    CL6_3,
    #[doc = "Unified cache"]
    CL6_4,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CL6R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CL6R::CL6_0 => 0,
            CL6R::CL6_1 => 1,
            CL6R::CL6_2 => 2,
            CL6R::CL6_3 => 3,
            CL6R::CL6_4 => 4,
            CL6R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CL6R {
        match value {
            0 => CL6R::CL6_0,
            1 => CL6R::CL6_1,
            2 => CL6R::CL6_2,
            3 => CL6R::CL6_3,
            4 => CL6R::CL6_4,
            i => CL6R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CL6_0`"]
    #[inline]
    pub fn is_cl6_0(&self) -> bool {
        *self == CL6R::CL6_0
    }
    #[doc = "Checks if the value of the field is `CL6_1`"]
    #[inline]
    pub fn is_cl6_1(&self) -> bool {
        *self == CL6R::CL6_1
    }
    #[doc = "Checks if the value of the field is `CL6_2`"]
    #[inline]
    pub fn is_cl6_2(&self) -> bool {
        *self == CL6R::CL6_2
    }
    #[doc = "Checks if the value of the field is `CL6_3`"]
    #[inline]
    pub fn is_cl6_3(&self) -> bool {
        *self == CL6R::CL6_3
    }
    #[doc = "Checks if the value of the field is `CL6_4`"]
    #[inline]
    pub fn is_cl6_4(&self) -> bool {
        *self == CL6R::CL6_4
    }
}
#[doc = "Possible values of the field `CL7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CL7R {
    #[doc = "No cache"]
    CL7_0,
    #[doc = "Instruction cache only"]
    CL7_1,
    #[doc = "Data cache only"]
    CL7_2,
    #[doc = "Separate instruction and data caches"]
    CL7_3,
    #[doc = "Unified cache"]
    CL7_4,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CL7R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CL7R::CL7_0 => 0,
            CL7R::CL7_1 => 1,
            CL7R::CL7_2 => 2,
            CL7R::CL7_3 => 3,
            CL7R::CL7_4 => 4,
            CL7R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CL7R {
        match value {
            0 => CL7R::CL7_0,
            1 => CL7R::CL7_1,
            2 => CL7R::CL7_2,
            3 => CL7R::CL7_3,
            4 => CL7R::CL7_4,
            i => CL7R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CL7_0`"]
    #[inline]
    pub fn is_cl7_0(&self) -> bool {
        *self == CL7R::CL7_0
    }
    #[doc = "Checks if the value of the field is `CL7_1`"]
    #[inline]
    pub fn is_cl7_1(&self) -> bool {
        *self == CL7R::CL7_1
    }
    #[doc = "Checks if the value of the field is `CL7_2`"]
    #[inline]
    pub fn is_cl7_2(&self) -> bool {
        *self == CL7R::CL7_2
    }
    #[doc = "Checks if the value of the field is `CL7_3`"]
    #[inline]
    pub fn is_cl7_3(&self) -> bool {
        *self == CL7R::CL7_3
    }
    #[doc = "Checks if the value of the field is `CL7_4`"]
    #[inline]
    pub fn is_cl7_4(&self) -> bool {
        *self == CL7R::CL7_4
    }
}
#[doc = "Possible values of the field `LOUIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOUISR {
    #[doc = "0"]
    LOUIS_0,
    #[doc = "1"]
    LOUIS_1,
    #[doc = "2"]
    LOUIS_2,
    #[doc = "3"]
    LOUIS_3,
    #[doc = "4"]
    LOUIS_4,
    #[doc = "5"]
    LOUIS_5,
    #[doc = "6"]
    LOUIS_6,
    #[doc = "7"]
    LOUIS_7,
}
impl LOUISR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LOUISR::LOUIS_0 => 0,
            LOUISR::LOUIS_1 => 1,
            LOUISR::LOUIS_2 => 2,
            LOUISR::LOUIS_3 => 3,
            LOUISR::LOUIS_4 => 4,
            LOUISR::LOUIS_5 => 5,
            LOUISR::LOUIS_6 => 6,
            LOUISR::LOUIS_7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LOUISR {
        match value {
            0 => LOUISR::LOUIS_0,
            1 => LOUISR::LOUIS_1,
            2 => LOUISR::LOUIS_2,
            3 => LOUISR::LOUIS_3,
            4 => LOUISR::LOUIS_4,
            5 => LOUISR::LOUIS_5,
            6 => LOUISR::LOUIS_6,
            7 => LOUISR::LOUIS_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOUIS_0`"]
    #[inline]
    pub fn is_louis_0(&self) -> bool {
        *self == LOUISR::LOUIS_0
    }
    #[doc = "Checks if the value of the field is `LOUIS_1`"]
    #[inline]
    pub fn is_louis_1(&self) -> bool {
        *self == LOUISR::LOUIS_1
    }
    #[doc = "Checks if the value of the field is `LOUIS_2`"]
    #[inline]
    pub fn is_louis_2(&self) -> bool {
        *self == LOUISR::LOUIS_2
    }
    #[doc = "Checks if the value of the field is `LOUIS_3`"]
    #[inline]
    pub fn is_louis_3(&self) -> bool {
        *self == LOUISR::LOUIS_3
    }
    #[doc = "Checks if the value of the field is `LOUIS_4`"]
    #[inline]
    pub fn is_louis_4(&self) -> bool {
        *self == LOUISR::LOUIS_4
    }
    #[doc = "Checks if the value of the field is `LOUIS_5`"]
    #[inline]
    pub fn is_louis_5(&self) -> bool {
        *self == LOUISR::LOUIS_5
    }
    #[doc = "Checks if the value of the field is `LOUIS_6`"]
    #[inline]
    pub fn is_louis_6(&self) -> bool {
        *self == LOUISR::LOUIS_6
    }
    #[doc = "Checks if the value of the field is `LOUIS_7`"]
    #[inline]
    pub fn is_louis_7(&self) -> bool {
        *self == LOUISR::LOUIS_7
    }
}
#[doc = "Possible values of the field `LOC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCR {
    #[doc = "0"]
    LOC_0,
    #[doc = "1"]
    LOC_1,
    #[doc = "2"]
    LOC_2,
    #[doc = "3"]
    LOC_3,
    #[doc = "4"]
    LOC_4,
    #[doc = "5"]
    LOC_5,
    #[doc = "6"]
    LOC_6,
    #[doc = "7"]
    LOC_7,
}
impl LOCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LOCR::LOC_0 => 0,
            LOCR::LOC_1 => 1,
            LOCR::LOC_2 => 2,
            LOCR::LOC_3 => 3,
            LOCR::LOC_4 => 4,
            LOCR::LOC_5 => 5,
            LOCR::LOC_6 => 6,
            LOCR::LOC_7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LOCR {
        match value {
            0 => LOCR::LOC_0,
            1 => LOCR::LOC_1,
            2 => LOCR::LOC_2,
            3 => LOCR::LOC_3,
            4 => LOCR::LOC_4,
            5 => LOCR::LOC_5,
            6 => LOCR::LOC_6,
            7 => LOCR::LOC_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOC_0`"]
    #[inline]
    pub fn is_loc_0(&self) -> bool {
        *self == LOCR::LOC_0
    }
    #[doc = "Checks if the value of the field is `LOC_1`"]
    #[inline]
    pub fn is_loc_1(&self) -> bool {
        *self == LOCR::LOC_1
    }
    #[doc = "Checks if the value of the field is `LOC_2`"]
    #[inline]
    pub fn is_loc_2(&self) -> bool {
        *self == LOCR::LOC_2
    }
    #[doc = "Checks if the value of the field is `LOC_3`"]
    #[inline]
    pub fn is_loc_3(&self) -> bool {
        *self == LOCR::LOC_3
    }
    #[doc = "Checks if the value of the field is `LOC_4`"]
    #[inline]
    pub fn is_loc_4(&self) -> bool {
        *self == LOCR::LOC_4
    }
    #[doc = "Checks if the value of the field is `LOC_5`"]
    #[inline]
    pub fn is_loc_5(&self) -> bool {
        *self == LOCR::LOC_5
    }
    #[doc = "Checks if the value of the field is `LOC_6`"]
    #[inline]
    pub fn is_loc_6(&self) -> bool {
        *self == LOCR::LOC_6
    }
    #[doc = "Checks if the value of the field is `LOC_7`"]
    #[inline]
    pub fn is_loc_7(&self) -> bool {
        *self == LOCR::LOC_7
    }
}
#[doc = "Possible values of the field `LOU`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOUR {
    #[doc = "0"]
    LOU_0,
    #[doc = "1"]
    LOU_1,
    #[doc = "2"]
    LOU_2,
    #[doc = "3"]
    LOU_3,
    #[doc = "4"]
    LOU_4,
    #[doc = "5"]
    LOU_5,
    #[doc = "6"]
    LOU_6,
    #[doc = "7"]
    LOU_7,
}
impl LOUR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LOUR::LOU_0 => 0,
            LOUR::LOU_1 => 1,
            LOUR::LOU_2 => 2,
            LOUR::LOU_3 => 3,
            LOUR::LOU_4 => 4,
            LOUR::LOU_5 => 5,
            LOUR::LOU_6 => 6,
            LOUR::LOU_7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LOUR {
        match value {
            0 => LOUR::LOU_0,
            1 => LOUR::LOU_1,
            2 => LOUR::LOU_2,
            3 => LOUR::LOU_3,
            4 => LOUR::LOU_4,
            5 => LOUR::LOU_5,
            6 => LOUR::LOU_6,
            7 => LOUR::LOU_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOU_0`"]
    #[inline]
    pub fn is_lou_0(&self) -> bool {
        *self == LOUR::LOU_0
    }
    #[doc = "Checks if the value of the field is `LOU_1`"]
    #[inline]
    pub fn is_lou_1(&self) -> bool {
        *self == LOUR::LOU_1
    }
    #[doc = "Checks if the value of the field is `LOU_2`"]
    #[inline]
    pub fn is_lou_2(&self) -> bool {
        *self == LOUR::LOU_2
    }
    #[doc = "Checks if the value of the field is `LOU_3`"]
    #[inline]
    pub fn is_lou_3(&self) -> bool {
        *self == LOUR::LOU_3
    }
    #[doc = "Checks if the value of the field is `LOU_4`"]
    #[inline]
    pub fn is_lou_4(&self) -> bool {
        *self == LOUR::LOU_4
    }
    #[doc = "Checks if the value of the field is `LOU_5`"]
    #[inline]
    pub fn is_lou_5(&self) -> bool {
        *self == LOUR::LOU_5
    }
    #[doc = "Checks if the value of the field is `LOU_6`"]
    #[inline]
    pub fn is_lou_6(&self) -> bool {
        *self == LOUR::LOU_6
    }
    #[doc = "Checks if the value of the field is `LOU_7`"]
    #[inline]
    pub fn is_lou_7(&self) -> bool {
        *self == LOUR::LOU_7
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:2 - Indicate the type of cache implemented at level 1."]
    #[inline]
    pub fn cl1(&self) -> CL1R {
        CL1R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 3:5 - Indicate the type of cache implemented at level 2."]
    #[inline]
    pub fn cl2(&self) -> CL2R {
        CL2R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 6:8 - Indicate the type of cache implemented at level 3."]
    #[inline]
    pub fn cl3(&self) -> CL3R {
        CL3R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 9:11 - Indicate the type of cache implemented at level 4."]
    #[inline]
    pub fn cl4(&self) -> CL4R {
        CL4R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:14 - Indicate the type of cache implemented at level 5."]
    #[inline]
    pub fn cl5(&self) -> CL5R {
        CL5R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 15:17 - Indicate the type of cache implemented at level 6."]
    #[inline]
    pub fn cl6(&self) -> CL6R {
        CL6R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 18:20 - Indicate the type of cache implemented at level 7."]
    #[inline]
    pub fn cl7(&self) -> CL7R {
        CL7R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 21:23 - Level of Unification Inner Shareable for the cache hierarchy. This field is RAZ."]
    #[inline]
    pub fn louis(&self) -> LOUISR {
        LOUISR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:26 - Level of Coherency for the cache hierarchy"]
    #[inline]
    pub fn loc(&self) -> LOCR {
        LOCR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 27:29 - Level of Unification for the cache hierarchy"]
    #[inline]
    pub fn lou(&self) -> LOUR {
        LOUR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
