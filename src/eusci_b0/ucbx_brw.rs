#[doc = "Register `UCBxBRW` reader"]
pub struct R(crate::R<UCBXBRW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCBXBRW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCBXBRW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCBXBRW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCBxBRW` writer"]
pub struct W(crate::W<UCBXBRW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCBXBRW_SPEC>;
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
impl From<crate::W<UCBXBRW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCBXBRW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UCBR` reader - Bit clock prescaler"]
pub type UCBR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `UCBR` writer - Bit clock prescaler"]
pub type UCBR_W<'a, const O: u8> = crate::FieldWriter<'a, u16, UCBXBRW_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Bit clock prescaler"]
    #[inline(always)]
    pub fn ucbr(&self) -> UCBR_R {
        UCBR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Bit clock prescaler"]
    #[inline(always)]
    pub fn ucbr(&mut self) -> UCBR_W<0> {
        UCBR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "eUSCI_Bx Baud Rate Control Word Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucbx_brw](index.html) module"]
pub struct UCBXBRW_SPEC;
impl crate::RegisterSpec for UCBXBRW_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ucbx_brw::R](R) reader structure"]
impl crate::Readable for UCBXBRW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ucbx_brw::W](W) writer structure"]
impl crate::Writable for UCBXBRW_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UCBxBRW to value 0"]
impl crate::Resettable for UCBXBRW_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
