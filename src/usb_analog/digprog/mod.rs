#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::DIGPROG {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `SILICON_REVISION`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SILICON_REVISIONR {
    #[doc = "Silicon revision 1.1"]
    SILICON_REVISION_6946817,
    #[doc = r" Reserved"]
    _Reserved(u32),
}
impl SILICON_REVISIONR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        match *self {
            SILICON_REVISIONR::SILICON_REVISION_6946817 => 6946817,
            SILICON_REVISIONR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u32) -> SILICON_REVISIONR {
        match value {
            6946817 => SILICON_REVISIONR::SILICON_REVISION_6946817,
            i => SILICON_REVISIONR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SILICON_REVISION_6946817`"]
    #[inline]
    pub fn is_silicon_revision_6946817(&self) -> bool {
        *self == SILICON_REVISIONR::SILICON_REVISION_6946817
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:31 - Chip silicon revision"]
    #[inline]
    pub fn silicon_revision(&self) -> SILICON_REVISIONR {
        SILICON_REVISIONR::_from({
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        })
    }
}
