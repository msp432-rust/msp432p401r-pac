#[doc = "Register `PCIFG` reader"]
pub struct R(crate::R<PCIFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCIFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCIFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCIFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCIFG` writer"]
pub struct W(crate::W<PCIFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCIFG_SPEC>;
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
impl From<crate::W<PCIFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCIFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P5IFG` reader - Port 5 Interrupt Flag"]
pub type P5IFG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `P5IFG` writer - Port 5 Interrupt Flag"]
pub type P5IFG_W<'a, const O: u8> = crate::FieldWriter<'a, u16, PCIFG_SPEC, u8, u8, 8, O>;
#[doc = "Field `P6IFG` reader - Port 6 Interrupt Flag"]
pub type P6IFG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `P6IFG` writer - Port 6 Interrupt Flag"]
pub type P6IFG_W<'a, const O: u8> = crate::FieldWriter<'a, u16, PCIFG_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Port 5 Interrupt Flag"]
    #[inline(always)]
    pub fn p5ifg(&self) -> P5IFG_R {
        P5IFG_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 6 Interrupt Flag"]
    #[inline(always)]
    pub fn p6ifg(&self) -> P6IFG_R {
        P6IFG_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 5 Interrupt Flag"]
    #[inline(always)]
    pub fn p5ifg(&mut self) -> P5IFG_W<0> {
        P5IFG_W::new(self)
    }
    #[doc = "Bits 8:15 - Port 6 Interrupt Flag"]
    #[inline(always)]
    pub fn p6ifg(&mut self) -> P6IFG_W<8> {
        P6IFG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port C Interrupt Flag\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcifg](index.html) module"]
pub struct PCIFG_SPEC;
impl crate::RegisterSpec for PCIFG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pcifg::R](R) reader structure"]
impl crate::Readable for PCIFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcifg::W](W) writer structure"]
impl crate::Writable for PCIFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PCIFG to value 0"]
impl crate::Resettable for PCIFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
