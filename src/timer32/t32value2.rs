#[doc = "Register `T32VALUE2` reader"]
pub struct R(crate::R<T32VALUE2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<T32VALUE2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<T32VALUE2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<T32VALUE2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `VALUE` reader - Current value of the decrementing counter"]
pub type VALUE_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Current value of the decrementing counter"]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new(self.bits)
    }
}
#[doc = "Timer 2 Current Value Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t32value2](index.html) module"]
pub struct T32VALUE2_SPEC;
impl crate::RegisterSpec for T32VALUE2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [t32value2::R](R) reader structure"]
impl crate::Readable for T32VALUE2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets T32VALUE2 to value 0xffff_ffff"]
impl crate::Resettable for T32VALUE2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
