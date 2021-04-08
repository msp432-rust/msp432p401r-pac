#[doc = "Register `RESERVED8` reader"]
pub struct R(crate::R<RESERVED8_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESERVED8_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<RESERVED8_SPEC>> for R {
    fn from(reader: crate::R<RESERVED8_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Reserved\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reserved8](index.html) module"]
pub struct RESERVED8_SPEC;
impl crate::RegisterSpec for RESERVED8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reserved8::R](R) reader structure"]
impl crate::Readable for RESERVED8_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RESERVED8 to value 0"]
impl crate::Resettable for RESERVED8_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
