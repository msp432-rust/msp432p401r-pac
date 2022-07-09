#[doc = "Register `PDIE` reader"]
pub struct R(crate::R<PDIE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDIE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDIE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDIE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDIE` writer"]
pub struct W(crate::W<PDIE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDIE_SPEC>;
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
impl From<crate::W<PDIE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDIE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P7IE` reader - Port 7 Interrupt Enable"]
pub type P7IE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `P7IE` writer - Port 7 Interrupt Enable"]
pub type P7IE_W<'a, const O: u8> = crate::FieldWriter<'a, u16, PDIE_SPEC, u8, u8, 8, O>;
#[doc = "Field `P8IE` reader - Port 8 Interrupt Enable"]
pub type P8IE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `P8IE` writer - Port 8 Interrupt Enable"]
pub type P8IE_W<'a, const O: u8> = crate::FieldWriter<'a, u16, PDIE_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Port 7 Interrupt Enable"]
    #[inline(always)]
    pub fn p7ie(&self) -> P7IE_R {
        P7IE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 8 Interrupt Enable"]
    #[inline(always)]
    pub fn p8ie(&self) -> P8IE_R {
        P8IE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 7 Interrupt Enable"]
    #[inline(always)]
    pub fn p7ie(&mut self) -> P7IE_W<0> {
        P7IE_W::new(self)
    }
    #[doc = "Bits 8:15 - Port 8 Interrupt Enable"]
    #[inline(always)]
    pub fn p8ie(&mut self) -> P8IE_W<8> {
        P8IE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port D Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdie](index.html) module"]
pub struct PDIE_SPEC;
impl crate::RegisterSpec for PDIE_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pdie::R](R) reader structure"]
impl crate::Readable for PDIE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdie::W](W) writer structure"]
impl crate::Writable for PDIE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PDIE to value 0"]
impl crate::Resettable for PDIE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
