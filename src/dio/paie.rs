#[doc = "Register `PAIE` reader"]
pub struct R(crate::R<PAIE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PAIE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PAIE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PAIE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PAIE` writer"]
pub struct W(crate::W<PAIE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PAIE_SPEC>;
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
impl From<crate::W<PAIE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PAIE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P1IE` reader - Port 1 Interrupt Enable"]
pub type P1IE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `P1IE` writer - Port 1 Interrupt Enable"]
pub type P1IE_W<'a, const O: u8> = crate::FieldWriter<'a, u16, PAIE_SPEC, u8, u8, 8, O>;
#[doc = "Field `P2IE` reader - Port 2 Interrupt Enable"]
pub type P2IE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `P2IE` writer - Port 2 Interrupt Enable"]
pub type P2IE_W<'a, const O: u8> = crate::FieldWriter<'a, u16, PAIE_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Port 1 Interrupt Enable"]
    #[inline(always)]
    pub fn p1ie(&self) -> P1IE_R {
        P1IE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 2 Interrupt Enable"]
    #[inline(always)]
    pub fn p2ie(&self) -> P2IE_R {
        P2IE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 1 Interrupt Enable"]
    #[inline(always)]
    pub fn p1ie(&mut self) -> P1IE_W<0> {
        P1IE_W::new(self)
    }
    #[doc = "Bits 8:15 - Port 2 Interrupt Enable"]
    #[inline(always)]
    pub fn p2ie(&mut self) -> P2IE_W<8> {
        P2IE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port A Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [paie](index.html) module"]
pub struct PAIE_SPEC;
impl crate::RegisterSpec for PAIE_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [paie::R](R) reader structure"]
impl crate::Readable for PAIE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [paie::W](W) writer structure"]
impl crate::Writable for PAIE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PAIE to value 0"]
impl crate::Resettable for PAIE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
