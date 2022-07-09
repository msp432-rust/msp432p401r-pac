#[doc = "Register `PBIES` reader"]
pub struct R(crate::R<PBIES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PBIES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PBIES_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PBIES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PBIES` writer"]
pub struct W(crate::W<PBIES_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PBIES_SPEC>;
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
impl From<crate::W<PBIES_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PBIES_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P3IES` reader - Port 3 Interrupt Edge Select"]
pub type P3IES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `P3IES` writer - Port 3 Interrupt Edge Select"]
pub type P3IES_W<'a, const O: u8> = crate::FieldWriter<'a, u16, PBIES_SPEC, u8, u8, 8, O>;
#[doc = "Field `P4IES` reader - Port 4 Interrupt Edge Select"]
pub type P4IES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `P4IES` writer - Port 4 Interrupt Edge Select"]
pub type P4IES_W<'a, const O: u8> = crate::FieldWriter<'a, u16, PBIES_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Port 3 Interrupt Edge Select"]
    #[inline(always)]
    pub fn p3ies(&self) -> P3IES_R {
        P3IES_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 4 Interrupt Edge Select"]
    #[inline(always)]
    pub fn p4ies(&self) -> P4IES_R {
        P4IES_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 3 Interrupt Edge Select"]
    #[inline(always)]
    pub fn p3ies(&mut self) -> P3IES_W<0> {
        P3IES_W::new(self)
    }
    #[doc = "Bits 8:15 - Port 4 Interrupt Edge Select"]
    #[inline(always)]
    pub fn p4ies(&mut self) -> P4IES_W<8> {
        P4IES_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port B Interrupt Edge Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pbies](index.html) module"]
pub struct PBIES_SPEC;
impl crate::RegisterSpec for PBIES_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pbies::R](R) reader structure"]
impl crate::Readable for PBIES_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pbies::W](W) writer structure"]
impl crate::Writable for PBIES_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PBIES to value 0"]
impl crate::Resettable for PBIES_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
