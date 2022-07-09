#[doc = "Register `PEIES` reader"]
pub struct R(crate::R<PEIES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PEIES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PEIES_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PEIES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PEIES` writer"]
pub struct W(crate::W<PEIES_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PEIES_SPEC>;
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
impl From<crate::W<PEIES_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PEIES_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P9IES` reader - Port 9 Interrupt Edge Select"]
pub type P9IES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `P9IES` writer - Port 9 Interrupt Edge Select"]
pub type P9IES_W<'a, const O: u8> = crate::FieldWriter<'a, u16, PEIES_SPEC, u8, u8, 8, O>;
#[doc = "Field `P10IES` reader - Port 10 Interrupt Edge Select"]
pub type P10IES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `P10IES` writer - Port 10 Interrupt Edge Select"]
pub type P10IES_W<'a, const O: u8> = crate::FieldWriter<'a, u16, PEIES_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Port 9 Interrupt Edge Select"]
    #[inline(always)]
    pub fn p9ies(&self) -> P9IES_R {
        P9IES_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 10 Interrupt Edge Select"]
    #[inline(always)]
    pub fn p10ies(&self) -> P10IES_R {
        P10IES_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 9 Interrupt Edge Select"]
    #[inline(always)]
    pub fn p9ies(&mut self) -> P9IES_W<0> {
        P9IES_W::new(self)
    }
    #[doc = "Bits 8:15 - Port 10 Interrupt Edge Select"]
    #[inline(always)]
    pub fn p10ies(&mut self) -> P10IES_W<8> {
        P10IES_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port E Interrupt Edge Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [peies](index.html) module"]
pub struct PEIES_SPEC;
impl crate::RegisterSpec for PEIES_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [peies::R](R) reader structure"]
impl crate::Readable for PEIES_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [peies::W](W) writer structure"]
impl crate::Writable for PEIES_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PEIES to value 0"]
impl crate::Resettable for PEIES_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
