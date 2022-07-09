#[doc = "Register `PDIES` reader"]
pub struct R(crate::R<PDIES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDIES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDIES_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDIES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDIES` writer"]
pub struct W(crate::W<PDIES_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDIES_SPEC>;
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
impl From<crate::W<PDIES_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDIES_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P7IES` reader - Port 7 Interrupt Edge Select"]
pub type P7IES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `P7IES` writer - Port 7 Interrupt Edge Select"]
pub type P7IES_W<'a, const O: u8> = crate::FieldWriter<'a, u16, PDIES_SPEC, u8, u8, 8, O>;
#[doc = "Field `P8IES` reader - Port 8 Interrupt Edge Select"]
pub type P8IES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `P8IES` writer - Port 8 Interrupt Edge Select"]
pub type P8IES_W<'a, const O: u8> = crate::FieldWriter<'a, u16, PDIES_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Port 7 Interrupt Edge Select"]
    #[inline(always)]
    pub fn p7ies(&self) -> P7IES_R {
        P7IES_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 8 Interrupt Edge Select"]
    #[inline(always)]
    pub fn p8ies(&self) -> P8IES_R {
        P8IES_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 7 Interrupt Edge Select"]
    #[inline(always)]
    pub fn p7ies(&mut self) -> P7IES_W<0> {
        P7IES_W::new(self)
    }
    #[doc = "Bits 8:15 - Port 8 Interrupt Edge Select"]
    #[inline(always)]
    pub fn p8ies(&mut self) -> P8IES_W<8> {
        P8IES_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port D Interrupt Edge Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdies](index.html) module"]
pub struct PDIES_SPEC;
impl crate::RegisterSpec for PDIES_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pdies::R](R) reader structure"]
impl crate::Readable for PDIES_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdies::W](W) writer structure"]
impl crate::Writable for PDIES_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PDIES to value 0"]
impl crate::Resettable for PDIES_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
