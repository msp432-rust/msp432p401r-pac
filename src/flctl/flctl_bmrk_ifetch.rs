#[doc = "Register `FLCTL_BMRK_IFETCH` reader"]
pub struct R(crate::R<FLCTL_BMRK_IFETCH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLCTL_BMRK_IFETCH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLCTL_BMRK_IFETCH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLCTL_BMRK_IFETCH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLCTL_BMRK_IFETCH` writer"]
pub struct W(crate::W<FLCTL_BMRK_IFETCH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLCTL_BMRK_IFETCH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<FLCTL_BMRK_IFETCH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLCTL_BMRK_IFETCH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COUNT` reader - Reflects the number of Instruction Fetches to the Flash (increments by one on each fetch)"]
pub type COUNT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `COUNT` writer - Reflects the number of Instruction Fetches to the Flash (increments by one on each fetch)"]
pub type COUNT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FLCTL_BMRK_IFETCH_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Reflects the number of Instruction Fetches to the Flash (increments by one on each fetch)"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Reflects the number of Instruction Fetches to the Flash (increments by one on each fetch)"]
    #[inline(always)]
    pub fn count(&mut self) -> COUNT_W<0> {
        COUNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Benchmark Instruction Fetch Count Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flctl_bmrk_ifetch](index.html) module"]
pub struct FLCTL_BMRK_IFETCH_SPEC;
impl crate::RegisterSpec for FLCTL_BMRK_IFETCH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flctl_bmrk_ifetch::R](R) reader structure"]
impl crate::Readable for FLCTL_BMRK_IFETCH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flctl_bmrk_ifetch::W](W) writer structure"]
impl crate::Writable for FLCTL_BMRK_IFETCH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLCTL_BMRK_IFETCH to value 0"]
impl crate::Resettable for FLCTL_BMRK_IFETCH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
