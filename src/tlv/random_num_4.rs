#[doc = "Register `RANDOM_NUM_4` reader"]
pub struct R(crate::R<RANDOM_NUM_4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RANDOM_NUM_4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<RANDOM_NUM_4_SPEC>> for R {
    fn from(reader: crate::R<RANDOM_NUM_4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "32-bit Random Number 4\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [random_num_4](index.html) module"]
pub struct RANDOM_NUM_4_SPEC;
impl crate::RegisterSpec for RANDOM_NUM_4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [random_num_4::R](R) reader structure"]
impl crate::Readable for RANDOM_NUM_4_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RANDOM_NUM_4 to value 0"]
impl crate::Resettable for RANDOM_NUM_4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
