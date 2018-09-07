#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::ID_MMFR1 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct ID_MMFR1R {
    bits: u32,
}
impl ID_MMFR1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:31 - Gives information about the implemented memory model and memory management support."]
    #[inline]
    pub fn id_mmfr1(&self) -> ID_MMFR1R {
        let bits = {
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        ID_MMFR1R { bits }
    }
}
