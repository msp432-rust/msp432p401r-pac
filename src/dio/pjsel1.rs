#[doc = "Register `PJSEL1` reader"]
pub struct R(crate::R<PJSEL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PJSEL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PJSEL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PJSEL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PJSEL1` writer"]
pub struct W(crate::W<PJSEL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PJSEL1_SPEC>;
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
impl From<crate::W<PJSEL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PJSEL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PJSEL1` reader - Port J Select 1"]
pub type PJSEL1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PJSEL1` writer - Port J Select 1"]
pub type PJSEL1_W<'a, const O: u8> = crate::FieldWriter<'a, u16, PJSEL1_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Port J Select 1"]
    #[inline(always)]
    pub fn pjsel1(&self) -> PJSEL1_R {
        PJSEL1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Port J Select 1"]
    #[inline(always)]
    pub fn pjsel1(&mut self) -> PJSEL1_W<0> {
        PJSEL1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port J Select 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pjsel1](index.html) module"]
pub struct PJSEL1_SPEC;
impl crate::RegisterSpec for PJSEL1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pjsel1::R](R) reader structure"]
impl crate::Readable for PJSEL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pjsel1::W](W) writer structure"]
impl crate::Writable for PJSEL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PJSEL1 to value 0"]
impl crate::Resettable for PJSEL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
