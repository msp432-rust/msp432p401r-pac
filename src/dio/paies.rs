#[doc = "Register `PAIES` reader"]
pub struct R(crate::R<PAIES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PAIES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PAIES_SPEC>> for R {
    fn from(reader: crate::R<PAIES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PAIES` writer"]
pub struct W(crate::W<PAIES_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PAIES_SPEC>;
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
impl core::convert::From<crate::W<PAIES_SPEC>> for W {
    fn from(writer: crate::W<PAIES_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P1IES` reader - Port 1 Interrupt Edge Select"]
pub struct P1IES_R(crate::FieldReader<u8, u8>);
impl P1IES_R {
    pub(crate) fn new(bits: u8) -> Self {
        P1IES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P1IES_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P1IES` writer - Port 1 Interrupt Edge Select"]
pub struct P1IES_W<'a> {
    w: &'a mut W,
}
impl<'a> P1IES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u16 & 0xff);
        self.w
    }
}
#[doc = "Field `P2IES` reader - Port 2 Interrupt Edge Select"]
pub struct P2IES_R(crate::FieldReader<u8, u8>);
impl P2IES_R {
    pub(crate) fn new(bits: u8) -> Self {
        P2IES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2IES_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2IES` writer - Port 2 Interrupt Edge Select"]
pub struct P2IES_W<'a> {
    w: &'a mut W,
}
impl<'a> P2IES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u16 & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Port 1 Interrupt Edge Select"]
    #[inline(always)]
    pub fn p1ies(&self) -> P1IES_R {
        P1IES_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 2 Interrupt Edge Select"]
    #[inline(always)]
    pub fn p2ies(&self) -> P2IES_R {
        P2IES_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 1 Interrupt Edge Select"]
    #[inline(always)]
    pub fn p1ies(&mut self) -> P1IES_W {
        P1IES_W { w: self }
    }
    #[doc = "Bits 8:15 - Port 2 Interrupt Edge Select"]
    #[inline(always)]
    pub fn p2ies(&mut self) -> P2IES_W {
        P2IES_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port A Interrupt Edge Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [paies](index.html) module"]
pub struct PAIES_SPEC;
impl crate::RegisterSpec for PAIES_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [paies::R](R) reader structure"]
impl crate::Readable for PAIES_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [paies::W](W) writer structure"]
impl crate::Writable for PAIES_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PAIES to value 0"]
impl crate::Resettable for PAIES_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
