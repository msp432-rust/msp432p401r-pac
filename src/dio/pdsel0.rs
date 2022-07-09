#[doc = "Register `PDSEL0` reader"]
pub struct R(crate::R<PDSEL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDSEL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDSEL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDSEL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDSEL0` writer"]
pub struct W(crate::W<PDSEL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDSEL0_SPEC>;
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
impl From<crate::W<PDSEL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDSEL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P7SEL0` reader - Port 7 Select 0"]
pub type P7SEL0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `P7SEL0` writer - Port 7 Select 0"]
pub type P7SEL0_W<'a, const O: u8> = crate::FieldWriter<'a, u16, PDSEL0_SPEC, u8, u8, 8, O>;
#[doc = "Field `P8SEL0` reader - Port 8 Select 0"]
pub type P8SEL0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `P8SEL0` writer - Port 8 Select 0"]
pub type P8SEL0_W<'a, const O: u8> = crate::FieldWriter<'a, u16, PDSEL0_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Port 7 Select 0"]
    #[inline(always)]
    pub fn p7sel0(&self) -> P7SEL0_R {
        P7SEL0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 8 Select 0"]
    #[inline(always)]
    pub fn p8sel0(&self) -> P8SEL0_R {
        P8SEL0_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 7 Select 0"]
    #[inline(always)]
    pub fn p7sel0(&mut self) -> P7SEL0_W<0> {
        P7SEL0_W::new(self)
    }
    #[doc = "Bits 8:15 - Port 8 Select 0"]
    #[inline(always)]
    pub fn p8sel0(&mut self) -> P8SEL0_W<8> {
        P8SEL0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port D Select 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdsel0](index.html) module"]
pub struct PDSEL0_SPEC;
impl crate::RegisterSpec for PDSEL0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pdsel0::R](R) reader structure"]
impl crate::Readable for PDSEL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdsel0::W](W) writer structure"]
impl crate::Writable for PDSEL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PDSEL0 to value 0"]
impl crate::Resettable for PDSEL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
