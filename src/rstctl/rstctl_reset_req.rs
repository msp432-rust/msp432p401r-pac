#[doc = "Register `RSTCTL_RESET_REQ` reader"]
pub struct R(crate::R<RSTCTL_RESET_REQ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSTCTL_RESET_REQ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSTCTL_RESET_REQ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSTCTL_RESET_REQ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RSTCTL_RESET_REQ` writer"]
pub struct W(crate::W<RSTCTL_RESET_REQ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RSTCTL_RESET_REQ_SPEC>;
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
impl From<crate::W<RSTCTL_RESET_REQ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RSTCTL_RESET_REQ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SOFT_REQ` writer - Soft Reset request"]
pub type SOFT_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTCTL_RESET_REQ_SPEC, bool, O>;
#[doc = "Field `HARD_REQ` writer - Hard Reset request"]
pub type HARD_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTCTL_RESET_REQ_SPEC, bool, O>;
#[doc = "Field `RSTKEY` writer - Write key to unlock reset request bits"]
pub type RSTKEY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RSTCTL_RESET_REQ_SPEC, u8, u8, 8, O>;
impl W {
    #[doc = "Bit 0 - Soft Reset request"]
    #[inline(always)]
    pub fn soft_req(&mut self) -> SOFT_REQ_W<0> {
        SOFT_REQ_W::new(self)
    }
    #[doc = "Bit 1 - Hard Reset request"]
    #[inline(always)]
    pub fn hard_req(&mut self) -> HARD_REQ_W<1> {
        HARD_REQ_W::new(self)
    }
    #[doc = "Bits 8:15 - Write key to unlock reset request bits"]
    #[inline(always)]
    pub fn rstkey(&mut self) -> RSTKEY_W<8> {
        RSTKEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reset Request Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rstctl_reset_req](index.html) module"]
pub struct RSTCTL_RESET_REQ_SPEC;
impl crate::RegisterSpec for RSTCTL_RESET_REQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rstctl_reset_req::R](R) reader structure"]
impl crate::Readable for RSTCTL_RESET_REQ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rstctl_reset_req::W](W) writer structure"]
impl crate::Writable for RSTCTL_RESET_REQ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RSTCTL_RESET_REQ to value 0"]
impl crate::Resettable for RSTCTL_RESET_REQ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
