#[doc = "Register `PJOUT` reader"]
pub struct R(crate::R<PJOUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PJOUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PJOUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PJOUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PJOUT` writer"]
pub struct W(crate::W<PJOUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PJOUT_SPEC>;
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
impl From<crate::W<PJOUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PJOUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PJOUT` reader - Port J Output"]
pub type PJOUT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PJOUT` writer - Port J Output"]
pub type PJOUT_W<'a, const O: u8> = crate::FieldWriter<'a, u16, PJOUT_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Port J Output"]
    #[inline(always)]
    pub fn pjout(&self) -> PJOUT_R {
        PJOUT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Port J Output"]
    #[inline(always)]
    pub fn pjout(&mut self) -> PJOUT_W<0> {
        PJOUT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port J Output\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pjout](index.html) module"]
pub struct PJOUT_SPEC;
impl crate::RegisterSpec for PJOUT_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pjout::R](R) reader structure"]
impl crate::Readable for PJOUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pjout::W](W) writer structure"]
impl crate::Writable for PJOUT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PJOUT to value 0"]
impl crate::Resettable for PJOUT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
