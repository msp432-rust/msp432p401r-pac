#[doc = "Register `PAIFG` reader"]
pub struct R(crate::R<PAIFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PAIFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PAIFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PAIFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PAIFG` writer"]
pub struct W(crate::W<PAIFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PAIFG_SPEC>;
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
impl From<crate::W<PAIFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PAIFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P1IFG` reader - Port 1 Interrupt Flag"]
pub type P1IFG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `P1IFG` writer - Port 1 Interrupt Flag"]
pub type P1IFG_W<'a, const O: u8> = crate::FieldWriter<'a, u16, PAIFG_SPEC, u8, u8, 8, O>;
#[doc = "Field `P2IFG` reader - Port 2 Interrupt Flag"]
pub type P2IFG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `P2IFG` writer - Port 2 Interrupt Flag"]
pub type P2IFG_W<'a, const O: u8> = crate::FieldWriter<'a, u16, PAIFG_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Port 1 Interrupt Flag"]
    #[inline(always)]
    pub fn p1ifg(&self) -> P1IFG_R {
        P1IFG_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 2 Interrupt Flag"]
    #[inline(always)]
    pub fn p2ifg(&self) -> P2IFG_R {
        P2IFG_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 1 Interrupt Flag"]
    #[inline(always)]
    pub fn p1ifg(&mut self) -> P1IFG_W<0> {
        P1IFG_W::new(self)
    }
    #[doc = "Bits 8:15 - Port 2 Interrupt Flag"]
    #[inline(always)]
    pub fn p2ifg(&mut self) -> P2IFG_W<8> {
        P2IFG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port A Interrupt Flag\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [paifg](index.html) module"]
pub struct PAIFG_SPEC;
impl crate::RegisterSpec for PAIFG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [paifg::R](R) reader structure"]
impl crate::Readable for PAIFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [paifg::W](W) writer structure"]
impl crate::Writable for PAIFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PAIFG to value 0"]
impl crate::Resettable for PAIFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
