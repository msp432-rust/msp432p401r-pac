#[doc = "Register `PCIES` reader"]
pub struct R(crate::R<PCIES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCIES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCIES_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCIES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCIES` writer"]
pub struct W(crate::W<PCIES_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCIES_SPEC>;
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
impl From<crate::W<PCIES_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCIES_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P5IES` reader - Port 5 Interrupt Edge Select"]
pub type P5IES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `P5IES` writer - Port 5 Interrupt Edge Select"]
pub type P5IES_W<'a, const O: u8> = crate::FieldWriter<'a, u16, PCIES_SPEC, u8, u8, 8, O>;
#[doc = "Field `P6IES` reader - Port 6 Interrupt Edge Select"]
pub type P6IES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `P6IES` writer - Port 6 Interrupt Edge Select"]
pub type P6IES_W<'a, const O: u8> = crate::FieldWriter<'a, u16, PCIES_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Port 5 Interrupt Edge Select"]
    #[inline(always)]
    pub fn p5ies(&self) -> P5IES_R {
        P5IES_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 6 Interrupt Edge Select"]
    #[inline(always)]
    pub fn p6ies(&self) -> P6IES_R {
        P6IES_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 5 Interrupt Edge Select"]
    #[inline(always)]
    pub fn p5ies(&mut self) -> P5IES_W<0> {
        P5IES_W::new(self)
    }
    #[doc = "Bits 8:15 - Port 6 Interrupt Edge Select"]
    #[inline(always)]
    pub fn p6ies(&mut self) -> P6IES_W<8> {
        P6IES_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port C Interrupt Edge Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcies](index.html) module"]
pub struct PCIES_SPEC;
impl crate::RegisterSpec for PCIES_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pcies::R](R) reader structure"]
impl crate::Readable for PCIES_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcies::W](W) writer structure"]
impl crate::Writable for PCIES_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PCIES to value 0"]
impl crate::Resettable for PCIES_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
