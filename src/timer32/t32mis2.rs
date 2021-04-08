#[doc = "Register `T32MIS2` reader"]
pub struct R(crate::R<T32MIS2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<T32MIS2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<T32MIS2_SPEC>> for R {
    fn from(reader: crate::R<T32MIS2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `IFG` reader - Enabled interrupt status"]
pub struct IFG_R(crate::FieldReader<bool, bool>);
impl IFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        IFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IFG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Enabled interrupt status"]
    #[inline(always)]
    pub fn ifg(&self) -> IFG_R {
        IFG_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "Timer 2 Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t32mis2](index.html) module"]
pub struct T32MIS2_SPEC;
impl crate::RegisterSpec for T32MIS2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [t32mis2::R](R) reader structure"]
impl crate::Readable for T32MIS2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets T32MIS2 to value 0"]
impl crate::Resettable for T32MIS2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
