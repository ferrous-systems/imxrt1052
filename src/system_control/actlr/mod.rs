#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ACTLR {
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
#[doc = "Possible values of the field `DISFOLD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DISFOLDR {
    #[doc = "Normal operation."]
    DISFOLD_0,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl DISFOLDR {
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
            DISFOLDR::DISFOLD_0 => false,
            DISFOLDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DISFOLDR {
        match value {
            false => DISFOLDR::DISFOLD_0,
            i => DISFOLDR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISFOLD_0`"]
    #[inline]
    pub fn is_disfold_0(&self) -> bool {
        *self == DISFOLDR::DISFOLD_0
    }
}
#[doc = "Possible values of the field `FPEXCODIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FPEXCODISR {
    #[doc = "Normal operation."]
    FPEXCODIS_0,
    #[doc = "FPU exception outputs are disabled."]
    FPEXCODIS_1,
}
impl FPEXCODISR {
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
            FPEXCODISR::FPEXCODIS_0 => false,
            FPEXCODISR::FPEXCODIS_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FPEXCODISR {
        match value {
            false => FPEXCODISR::FPEXCODIS_0,
            true => FPEXCODISR::FPEXCODIS_1,
        }
    }
    #[doc = "Checks if the value of the field is `FPEXCODIS_0`"]
    #[inline]
    pub fn is_fpexcodis_0(&self) -> bool {
        *self == FPEXCODISR::FPEXCODIS_0
    }
    #[doc = "Checks if the value of the field is `FPEXCODIS_1`"]
    #[inline]
    pub fn is_fpexcodis_1(&self) -> bool {
        *self == FPEXCODISR::FPEXCODIS_1
    }
}
#[doc = "Possible values of the field `DISRAMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DISRAMODER {
    #[doc = "Normal operation."]
    DISRAMODE_0,
    #[doc = "Dynamic disabled."]
    DISRAMODE_1,
}
impl DISRAMODER {
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
            DISRAMODER::DISRAMODE_0 => false,
            DISRAMODER::DISRAMODE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DISRAMODER {
        match value {
            false => DISRAMODER::DISRAMODE_0,
            true => DISRAMODER::DISRAMODE_1,
        }
    }
    #[doc = "Checks if the value of the field is `DISRAMODE_0`"]
    #[inline]
    pub fn is_disramode_0(&self) -> bool {
        *self == DISRAMODER::DISRAMODE_0
    }
    #[doc = "Checks if the value of the field is `DISRAMODE_1`"]
    #[inline]
    pub fn is_disramode_1(&self) -> bool {
        *self == DISRAMODER::DISRAMODE_1
    }
}
#[doc = "Possible values of the field `DISITMATBFLUSH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DISITMATBFLUSHR {
    #[doc = "ITM and DWT ATB flush disabled, this bit is always 1."]
    DISITMATBFLUSH_1,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl DISITMATBFLUSHR {
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
            DISITMATBFLUSHR::DISITMATBFLUSH_1 => true,
            DISITMATBFLUSHR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DISITMATBFLUSHR {
        match value {
            true => DISITMATBFLUSHR::DISITMATBFLUSH_1,
            i => DISITMATBFLUSHR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISITMATBFLUSH_1`"]
    #[inline]
    pub fn is_disitmatbflush_1(&self) -> bool {
        *self == DISITMATBFLUSHR::DISITMATBFLUSH_1
    }
}
#[doc = "Possible values of the field `DISBTACREAD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DISBTACREADR {
    #[doc = "Normal operation."]
    DISBTACREAD_0,
    #[doc = "BTAC is not used and only static branch prediction can occur."]
    DISBTACREAD_1,
}
impl DISBTACREADR {
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
            DISBTACREADR::DISBTACREAD_0 => false,
            DISBTACREADR::DISBTACREAD_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DISBTACREADR {
        match value {
            false => DISBTACREADR::DISBTACREAD_0,
            true => DISBTACREADR::DISBTACREAD_1,
        }
    }
    #[doc = "Checks if the value of the field is `DISBTACREAD_0`"]
    #[inline]
    pub fn is_disbtacread_0(&self) -> bool {
        *self == DISBTACREADR::DISBTACREAD_0
    }
    #[doc = "Checks if the value of the field is `DISBTACREAD_1`"]
    #[inline]
    pub fn is_disbtacread_1(&self) -> bool {
        *self == DISBTACREADR::DISBTACREAD_1
    }
}
#[doc = "Possible values of the field `DISBTACALLOC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DISBTACALLOCR {
    #[doc = "Normal operation."]
    DISBTACALLOC_0,
    #[doc = "No new entries are allocated in Branch Target Address Cache (BTAC), but existing entries can be updated."]
    DISBTACALLOC_1,
}
impl DISBTACALLOCR {
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
            DISBTACALLOCR::DISBTACALLOC_0 => false,
            DISBTACALLOCR::DISBTACALLOC_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DISBTACALLOCR {
        match value {
            false => DISBTACALLOCR::DISBTACALLOC_0,
            true => DISBTACALLOCR::DISBTACALLOC_1,
        }
    }
    #[doc = "Checks if the value of the field is `DISBTACALLOC_0`"]
    #[inline]
    pub fn is_disbtacalloc_0(&self) -> bool {
        *self == DISBTACALLOCR::DISBTACALLOC_0
    }
    #[doc = "Checks if the value of the field is `DISBTACALLOC_1`"]
    #[inline]
    pub fn is_disbtacalloc_1(&self) -> bool {
        *self == DISBTACALLOCR::DISBTACALLOC_1
    }
}
#[doc = "Possible values of the field `DISCRITAXIRUR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DISCRITAXIRURR {
    #[doc = "Normal operation."]
    DISCRITAXIRUR_0,
    #[doc = "An AXI read to Strongly-Ordered or Device memory, or an LDREX to Shareable memory, is not put on AXI if there are any outstanding reads on AXI. Transactions on AXI cannot be interrupted. This bit might reduce the time that these transactions are in progress and might improve worst case interrupt latency. Performance is decreased when this bit is set."]
    DISCRITAXIRUR_1,
}
impl DISCRITAXIRURR {
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
            DISCRITAXIRURR::DISCRITAXIRUR_0 => false,
            DISCRITAXIRURR::DISCRITAXIRUR_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DISCRITAXIRURR {
        match value {
            false => DISCRITAXIRURR::DISCRITAXIRUR_0,
            true => DISCRITAXIRURR::DISCRITAXIRUR_1,
        }
    }
    #[doc = "Checks if the value of the field is `DISCRITAXIRUR_0`"]
    #[inline]
    pub fn is_discritaxirur_0(&self) -> bool {
        *self == DISCRITAXIRURR::DISCRITAXIRUR_0
    }
    #[doc = "Checks if the value of the field is `DISCRITAXIRUR_1`"]
    #[inline]
    pub fn is_discritaxirur_1(&self) -> bool {
        *self == DISCRITAXIRURR::DISCRITAXIRUR_1
    }
}
#[doc = "Possible values of the field `DISDI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DISDIR {
    #[doc = "Normal operation."]
    DISDI_0,
    #[doc = "Nothing can be dual-issued when this instruction type is in channel 0."]
    DISDI_1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DISDIR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DISDIR::DISDI_0 => 0,
            DISDIR::DISDI_1 => 1,
            DISDIR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DISDIR {
        match value {
            0 => DISDIR::DISDI_0,
            1 => DISDIR::DISDI_1,
            i => DISDIR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISDI_0`"]
    #[inline]
    pub fn is_disdi_0(&self) -> bool {
        *self == DISDIR::DISDI_0
    }
    #[doc = "Checks if the value of the field is `DISDI_1`"]
    #[inline]
    pub fn is_disdi_1(&self) -> bool {
        *self == DISDIR::DISDI_1
    }
}
#[doc = "Possible values of the field `DISISSCH1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DISISSCH1R {
    #[doc = "Normal operation."]
    DISISSCH1_0,
    #[doc = "Nothing can be dual-issued when this instruction type is in channel 1."]
    DISISSCH1_1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DISISSCH1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DISISSCH1R::DISISSCH1_0 => 0,
            DISISSCH1R::DISISSCH1_1 => 1,
            DISISSCH1R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DISISSCH1R {
        match value {
            0 => DISISSCH1R::DISISSCH1_0,
            1 => DISISSCH1R::DISISSCH1_1,
            i => DISISSCH1R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISISSCH1_0`"]
    #[inline]
    pub fn is_disissch1_0(&self) -> bool {
        *self == DISISSCH1R::DISISSCH1_0
    }
    #[doc = "Checks if the value of the field is `DISISSCH1_1`"]
    #[inline]
    pub fn is_disissch1_1(&self) -> bool {
        *self == DISISSCH1R::DISISSCH1_1
    }
}
#[doc = "Possible values of the field `DISDYNADD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DISDYNADDR {
    #[doc = "Normal operation. Some ADD and SUB instrctions are resolved in EX1."]
    DISDYNADD_0,
    #[doc = "All ADD and SUB instructions are resolved in EX2."]
    DISDYNADD_1,
}
impl DISDYNADDR {
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
            DISDYNADDR::DISDYNADD_0 => false,
            DISDYNADDR::DISDYNADD_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DISDYNADDR {
        match value {
            false => DISDYNADDR::DISDYNADD_0,
            true => DISDYNADDR::DISDYNADD_1,
        }
    }
    #[doc = "Checks if the value of the field is `DISDYNADD_0`"]
    #[inline]
    pub fn is_disdynadd_0(&self) -> bool {
        *self == DISDYNADDR::DISDYNADD_0
    }
    #[doc = "Checks if the value of the field is `DISDYNADD_1`"]
    #[inline]
    pub fn is_disdynadd_1(&self) -> bool {
        *self == DISDYNADDR::DISDYNADD_1
    }
}
#[doc = "Possible values of the field `DISCRITAXIRUW`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DISCRITAXIRUWR {
    #[doc = "Normal operation. This is backwards compatible with r0."]
    DISCRITAXIRUW_0,
    #[doc = "AXI reads to DEV/SO memory. Exclusive reads to Shareable memory are not initiated on the AXIM AR channel until all outstanding stores on AXI are complete."]
    DISCRITAXIRUW_1,
}
impl DISCRITAXIRUWR {
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
            DISCRITAXIRUWR::DISCRITAXIRUW_0 => false,
            DISCRITAXIRUWR::DISCRITAXIRUW_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DISCRITAXIRUWR {
        match value {
            false => DISCRITAXIRUWR::DISCRITAXIRUW_0,
            true => DISCRITAXIRUWR::DISCRITAXIRUW_1,
        }
    }
    #[doc = "Checks if the value of the field is `DISCRITAXIRUW_0`"]
    #[inline]
    pub fn is_discritaxiruw_0(&self) -> bool {
        *self == DISCRITAXIRUWR::DISCRITAXIRUW_0
    }
    #[doc = "Checks if the value of the field is `DISCRITAXIRUW_1`"]
    #[inline]
    pub fn is_discritaxiruw_1(&self) -> bool {
        *self == DISCRITAXIRUWR::DISCRITAXIRUW_1
    }
}
#[doc = "Possible values of the field `DISFPUISSOPT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DISFPUISSOPTR {
    #[doc = "Normal operation."]
    DISFPUISSOPT_0,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl DISFPUISSOPTR {
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
            DISFPUISSOPTR::DISFPUISSOPT_0 => false,
            DISFPUISSOPTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DISFPUISSOPTR {
        match value {
            false => DISFPUISSOPTR::DISFPUISSOPT_0,
            i => DISFPUISSOPTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISFPUISSOPT_0`"]
    #[inline]
    pub fn is_disfpuissopt_0(&self) -> bool {
        *self == DISFPUISSOPTR::DISFPUISSOPT_0
    }
}
#[doc = "Values that can be written to the field `DISFOLD`"]
pub enum DISFOLDW {
    #[doc = "Normal operation."]
    DISFOLD_0,
}
impl DISFOLDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DISFOLDW::DISFOLD_0 => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DISFOLDW<'a> {
    w: &'a mut W,
}
impl<'a> _DISFOLDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DISFOLDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation."]
    #[inline]
    pub fn disfold_0(self) -> &'a mut W {
        self.variant(DISFOLDW::DISFOLD_0)
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FPEXCODIS`"]
pub enum FPEXCODISW {
    #[doc = "Normal operation."]
    FPEXCODIS_0,
    #[doc = "FPU exception outputs are disabled."]
    FPEXCODIS_1,
}
impl FPEXCODISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FPEXCODISW::FPEXCODIS_0 => false,
            FPEXCODISW::FPEXCODIS_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FPEXCODISW<'a> {
    w: &'a mut W,
}
impl<'a> _FPEXCODISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FPEXCODISW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation."]
    #[inline]
    pub fn fpexcodis_0(self) -> &'a mut W {
        self.variant(FPEXCODISW::FPEXCODIS_0)
    }
    #[doc = "FPU exception outputs are disabled."]
    #[inline]
    pub fn fpexcodis_1(self) -> &'a mut W {
        self.variant(FPEXCODISW::FPEXCODIS_1)
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
#[doc = "Values that can be written to the field `DISRAMODE`"]
pub enum DISRAMODEW {
    #[doc = "Normal operation."]
    DISRAMODE_0,
    #[doc = "Dynamic disabled."]
    DISRAMODE_1,
}
impl DISRAMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DISRAMODEW::DISRAMODE_0 => false,
            DISRAMODEW::DISRAMODE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DISRAMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _DISRAMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DISRAMODEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation."]
    #[inline]
    pub fn disramode_0(self) -> &'a mut W {
        self.variant(DISRAMODEW::DISRAMODE_0)
    }
    #[doc = "Dynamic disabled."]
    #[inline]
    pub fn disramode_1(self) -> &'a mut W {
        self.variant(DISRAMODEW::DISRAMODE_1)
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
#[doc = "Values that can be written to the field `DISITMATBFLUSH`"]
pub enum DISITMATBFLUSHW {
    #[doc = "ITM and DWT ATB flush disabled, this bit is always 1."]
    DISITMATBFLUSH_1,
}
impl DISITMATBFLUSHW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DISITMATBFLUSHW::DISITMATBFLUSH_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DISITMATBFLUSHW<'a> {
    w: &'a mut W,
}
impl<'a> _DISITMATBFLUSHW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DISITMATBFLUSHW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "ITM and DWT ATB flush disabled, this bit is always 1."]
    #[inline]
    pub fn disitmatbflush_1(self) -> &'a mut W {
        self.variant(DISITMATBFLUSHW::DISITMATBFLUSH_1)
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
#[doc = "Values that can be written to the field `DISBTACREAD`"]
pub enum DISBTACREADW {
    #[doc = "Normal operation."]
    DISBTACREAD_0,
    #[doc = "BTAC is not used and only static branch prediction can occur."]
    DISBTACREAD_1,
}
impl DISBTACREADW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DISBTACREADW::DISBTACREAD_0 => false,
            DISBTACREADW::DISBTACREAD_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DISBTACREADW<'a> {
    w: &'a mut W,
}
impl<'a> _DISBTACREADW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DISBTACREADW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation."]
    #[inline]
    pub fn disbtacread_0(self) -> &'a mut W {
        self.variant(DISBTACREADW::DISBTACREAD_0)
    }
    #[doc = "BTAC is not used and only static branch prediction can occur."]
    #[inline]
    pub fn disbtacread_1(self) -> &'a mut W {
        self.variant(DISBTACREADW::DISBTACREAD_1)
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
#[doc = "Values that can be written to the field `DISBTACALLOC`"]
pub enum DISBTACALLOCW {
    #[doc = "Normal operation."]
    DISBTACALLOC_0,
    #[doc = "No new entries are allocated in Branch Target Address Cache (BTAC), but existing entries can be updated."]
    DISBTACALLOC_1,
}
impl DISBTACALLOCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DISBTACALLOCW::DISBTACALLOC_0 => false,
            DISBTACALLOCW::DISBTACALLOC_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DISBTACALLOCW<'a> {
    w: &'a mut W,
}
impl<'a> _DISBTACALLOCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DISBTACALLOCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation."]
    #[inline]
    pub fn disbtacalloc_0(self) -> &'a mut W {
        self.variant(DISBTACALLOCW::DISBTACALLOC_0)
    }
    #[doc = "No new entries are allocated in Branch Target Address Cache (BTAC), but existing entries can be updated."]
    #[inline]
    pub fn disbtacalloc_1(self) -> &'a mut W {
        self.variant(DISBTACALLOCW::DISBTACALLOC_1)
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
#[doc = "Values that can be written to the field `DISCRITAXIRUR`"]
pub enum DISCRITAXIRURW {
    #[doc = "Normal operation."]
    DISCRITAXIRUR_0,
    #[doc = "An AXI read to Strongly-Ordered or Device memory, or an LDREX to Shareable memory, is not put on AXI if there are any outstanding reads on AXI. Transactions on AXI cannot be interrupted. This bit might reduce the time that these transactions are in progress and might improve worst case interrupt latency. Performance is decreased when this bit is set."]
    DISCRITAXIRUR_1,
}
impl DISCRITAXIRURW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DISCRITAXIRURW::DISCRITAXIRUR_0 => false,
            DISCRITAXIRURW::DISCRITAXIRUR_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DISCRITAXIRURW<'a> {
    w: &'a mut W,
}
impl<'a> _DISCRITAXIRURW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DISCRITAXIRURW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation."]
    #[inline]
    pub fn discritaxirur_0(self) -> &'a mut W {
        self.variant(DISCRITAXIRURW::DISCRITAXIRUR_0)
    }
    #[doc = "An AXI read to Strongly-Ordered or Device memory, or an LDREX to Shareable memory, is not put on AXI if there are any outstanding reads on AXI. Transactions on AXI cannot be interrupted. This bit might reduce the time that these transactions are in progress and might improve worst case interrupt latency. Performance is decreased when this bit is set."]
    #[inline]
    pub fn discritaxirur_1(self) -> &'a mut W {
        self.variant(DISCRITAXIRURW::DISCRITAXIRUR_1)
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
#[doc = "Values that can be written to the field `DISDI`"]
pub enum DISDIW {
    #[doc = "Normal operation."]
    DISDI_0,
    #[doc = "Nothing can be dual-issued when this instruction type is in channel 0."]
    DISDI_1,
}
impl DISDIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DISDIW::DISDI_0 => 0,
            DISDIW::DISDI_1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DISDIW<'a> {
    w: &'a mut W,
}
impl<'a> _DISDIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DISDIW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Normal operation."]
    #[inline]
    pub fn disdi_0(self) -> &'a mut W {
        self.variant(DISDIW::DISDI_0)
    }
    #[doc = "Nothing can be dual-issued when this instruction type is in channel 0."]
    #[inline]
    pub fn disdi_1(self) -> &'a mut W {
        self.variant(DISDIW::DISDI_1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DISISSCH1`"]
pub enum DISISSCH1W {
    #[doc = "Normal operation."]
    DISISSCH1_0,
    #[doc = "Nothing can be dual-issued when this instruction type is in channel 1."]
    DISISSCH1_1,
}
impl DISISSCH1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DISISSCH1W::DISISSCH1_0 => 0,
            DISISSCH1W::DISISSCH1_1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DISISSCH1W<'a> {
    w: &'a mut W,
}
impl<'a> _DISISSCH1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DISISSCH1W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Normal operation."]
    #[inline]
    pub fn disissch1_0(self) -> &'a mut W {
        self.variant(DISISSCH1W::DISISSCH1_0)
    }
    #[doc = "Nothing can be dual-issued when this instruction type is in channel 1."]
    #[inline]
    pub fn disissch1_1(self) -> &'a mut W {
        self.variant(DISISSCH1W::DISISSCH1_1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DISDYNADD`"]
pub enum DISDYNADDW {
    #[doc = "Normal operation. Some ADD and SUB instrctions are resolved in EX1."]
    DISDYNADD_0,
    #[doc = "All ADD and SUB instructions are resolved in EX2."]
    DISDYNADD_1,
}
impl DISDYNADDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DISDYNADDW::DISDYNADD_0 => false,
            DISDYNADDW::DISDYNADD_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DISDYNADDW<'a> {
    w: &'a mut W,
}
impl<'a> _DISDYNADDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DISDYNADDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation. Some ADD and SUB instrctions are resolved in EX1."]
    #[inline]
    pub fn disdynadd_0(self) -> &'a mut W {
        self.variant(DISDYNADDW::DISDYNADD_0)
    }
    #[doc = "All ADD and SUB instructions are resolved in EX2."]
    #[inline]
    pub fn disdynadd_1(self) -> &'a mut W {
        self.variant(DISDYNADDW::DISDYNADD_1)
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
#[doc = "Values that can be written to the field `DISCRITAXIRUW`"]
pub enum DISCRITAXIRUWW {
    #[doc = "Normal operation. This is backwards compatible with r0."]
    DISCRITAXIRUW_0,
    #[doc = "AXI reads to DEV/SO memory. Exclusive reads to Shareable memory are not initiated on the AXIM AR channel until all outstanding stores on AXI are complete."]
    DISCRITAXIRUW_1,
}
impl DISCRITAXIRUWW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DISCRITAXIRUWW::DISCRITAXIRUW_0 => false,
            DISCRITAXIRUWW::DISCRITAXIRUW_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DISCRITAXIRUWW<'a> {
    w: &'a mut W,
}
impl<'a> _DISCRITAXIRUWW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DISCRITAXIRUWW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation. This is backwards compatible with r0."]
    #[inline]
    pub fn discritaxiruw_0(self) -> &'a mut W {
        self.variant(DISCRITAXIRUWW::DISCRITAXIRUW_0)
    }
    #[doc = "AXI reads to DEV/SO memory. Exclusive reads to Shareable memory are not initiated on the AXIM AR channel until all outstanding stores on AXI are complete."]
    #[inline]
    pub fn discritaxiruw_1(self) -> &'a mut W {
        self.variant(DISCRITAXIRUWW::DISCRITAXIRUW_1)
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
#[doc = "Values that can be written to the field `DISFPUISSOPT`"]
pub enum DISFPUISSOPTW {
    #[doc = "Normal operation."]
    DISFPUISSOPT_0,
}
impl DISFPUISSOPTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DISFPUISSOPTW::DISFPUISSOPT_0 => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DISFPUISSOPTW<'a> {
    w: &'a mut W,
}
impl<'a> _DISFPUISSOPTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DISFPUISSOPTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation."]
    #[inline]
    pub fn disfpuissopt_0(self) -> &'a mut W {
        self.variant(DISFPUISSOPTW::DISFPUISSOPT_0)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 2 - Disables folding of IT instructions."]
    #[inline]
    pub fn disfold(&self) -> DISFOLDR {
        DISFOLDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Disables FPU exception outputs."]
    #[inline]
    pub fn fpexcodis(&self) -> FPEXCODISR {
        FPEXCODISR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Disables dynamic read allocate mode for Write-Back Write-Allocate memory regions."]
    #[inline]
    pub fn disramode(&self) -> DISRAMODER {
        DISRAMODER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Disables ITM and DWT ATB flush."]
    #[inline]
    pub fn disitmatbflush(&self) -> DISITMATBFLUSHR {
        DISITMATBFLUSHR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Disables BTAC read."]
    #[inline]
    pub fn disbtacread(&self) -> DISBTACREADR {
        DISBTACREADR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Disables BTAC allocate."]
    #[inline]
    pub fn disbtacalloc(&self) -> DISBTACALLOCR {
        DISBTACALLOCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Disables critical AXI Read-Under-Read."]
    #[inline]
    pub fn discritaxirur(&self) -> DISCRITAXIRURR {
        DISCRITAXIRURR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:20 - Disables dual-issued."]
    #[inline]
    pub fn disdi(&self) -> DISDIR {
        DISDIR::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 21:25 - Disables dual-issued."]
    #[inline]
    pub fn disissch1(&self) -> DISISSCH1R {
        DISISSCH1R::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 26 - Disables dynamic allocation of ADD and SUB instructions"]
    #[inline]
    pub fn disdynadd(&self) -> DISDYNADDR {
        DISDYNADDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - Disables critical AXI read-under-write"]
    #[inline]
    pub fn discritaxiruw(&self) -> DISCRITAXIRUWR {
        DISCRITAXIRUWR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - Disables critical AXI read-under-write"]
    #[inline]
    pub fn disfpuissopt(&self) -> DISFPUISSOPTR {
        DISFPUISSOPTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
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
    #[doc = "Bit 2 - Disables folding of IT instructions."]
    #[inline]
    pub fn disfold(&mut self) -> _DISFOLDW {
        _DISFOLDW { w: self }
    }
    #[doc = "Bit 10 - Disables FPU exception outputs."]
    #[inline]
    pub fn fpexcodis(&mut self) -> _FPEXCODISW {
        _FPEXCODISW { w: self }
    }
    #[doc = "Bit 11 - Disables dynamic read allocate mode for Write-Back Write-Allocate memory regions."]
    #[inline]
    pub fn disramode(&mut self) -> _DISRAMODEW {
        _DISRAMODEW { w: self }
    }
    #[doc = "Bit 12 - Disables ITM and DWT ATB flush."]
    #[inline]
    pub fn disitmatbflush(&mut self) -> _DISITMATBFLUSHW {
        _DISITMATBFLUSHW { w: self }
    }
    #[doc = "Bit 13 - Disables BTAC read."]
    #[inline]
    pub fn disbtacread(&mut self) -> _DISBTACREADW {
        _DISBTACREADW { w: self }
    }
    #[doc = "Bit 14 - Disables BTAC allocate."]
    #[inline]
    pub fn disbtacalloc(&mut self) -> _DISBTACALLOCW {
        _DISBTACALLOCW { w: self }
    }
    #[doc = "Bit 15 - Disables critical AXI Read-Under-Read."]
    #[inline]
    pub fn discritaxirur(&mut self) -> _DISCRITAXIRURW {
        _DISCRITAXIRURW { w: self }
    }
    #[doc = "Bits 16:20 - Disables dual-issued."]
    #[inline]
    pub fn disdi(&mut self) -> _DISDIW {
        _DISDIW { w: self }
    }
    #[doc = "Bits 21:25 - Disables dual-issued."]
    #[inline]
    pub fn disissch1(&mut self) -> _DISISSCH1W {
        _DISISSCH1W { w: self }
    }
    #[doc = "Bit 26 - Disables dynamic allocation of ADD and SUB instructions"]
    #[inline]
    pub fn disdynadd(&mut self) -> _DISDYNADDW {
        _DISDYNADDW { w: self }
    }
    #[doc = "Bit 27 - Disables critical AXI read-under-write"]
    #[inline]
    pub fn discritaxiruw(&mut self) -> _DISCRITAXIRUWW {
        _DISCRITAXIRUWW { w: self }
    }
    #[doc = "Bit 28 - Disables critical AXI read-under-write"]
    #[inline]
    pub fn disfpuissopt(&mut self) -> _DISFPUISSOPTW {
        _DISFPUISSOPTW { w: self }
    }
}
