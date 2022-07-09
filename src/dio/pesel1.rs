#[doc = "Register `PESEL1` reader"]
pub struct R(crate::R<PESEL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PESEL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PESEL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PESEL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PESEL1` writer"]
pub struct W(crate::W<PESEL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PESEL1_SPEC>;
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
impl From<crate::W<PESEL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PESEL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P9SEL1` reader - Port 9 Select 1"]
pub type P9SEL1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `P9SEL1` writer - Port 9 Select 1"]
pub type P9SEL1_W<'a, const O: u8> = crate::FieldWriter<'a, u16, PESEL1_SPEC, u8, u8, 8, O>;
#[doc = "Field `P10SEL1` reader - Port 10 Select 1"]
pub type P10SEL1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `P10SEL1` writer - Port 10 Select 1"]
pub type P10SEL1_W<'a, const O: u8> = crate::FieldWriter<'a, u16, PESEL1_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Port 9 Select 1"]
    #[inline(always)]
    pub fn p9sel1(&self) -> P9SEL1_R {
        P9SEL1_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 10 Select 1"]
    #[inline(always)]
    pub fn p10sel1(&self) -> P10SEL1_R {
        P10SEL1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 9 Select 1"]
    #[inline(always)]
    pub fn p9sel1(&mut self) -> P9SEL1_W<0> {
        P9SEL1_W::new(self)
    }
    #[doc = "Bits 8:15 - Port 10 Select 1"]
    #[inline(always)]
    pub fn p10sel1(&mut self) -> P10SEL1_W<8> {
        P10SEL1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port E Select 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pesel1](index.html) module"]
pub struct PESEL1_SPEC;
impl crate::RegisterSpec for PESEL1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pesel1::R](R) reader structure"]
impl crate::Readable for PESEL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pesel1::W](W) writer structure"]
impl crate::Writable for PESEL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PESEL1 to value 0"]
impl crate::Resettable for PESEL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
