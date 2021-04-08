#[doc = "Register `DCOER_CONSTK_RSEL04` reader"]
pub struct R(crate::R<DCOER_CONSTK_RSEL04_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCOER_CONSTK_RSEL04_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<DCOER_CONSTK_RSEL04_SPEC>> for R {
    fn from(reader: crate::R<DCOER_CONSTK_RSEL04_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "DCO ER mode: DCO Constant (K) for DCORSEL 0 to 4\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcoer_constk_rsel04](index.html) module"]
pub struct DCOER_CONSTK_RSEL04_SPEC;
impl crate::RegisterSpec for DCOER_CONSTK_RSEL04_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcoer_constk_rsel04::R](R) reader structure"]
impl crate::Readable for DCOER_CONSTK_RSEL04_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DCOER_CONSTK_RSEL04 to value 0"]
impl crate::Resettable for DCOER_CONSTK_RSEL04_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
