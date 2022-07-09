#[doc = "Register `PBSEL0` reader"]
pub struct R(crate::R<PBSEL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PBSEL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PBSEL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PBSEL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PBSEL0` writer"]
pub struct W(crate::W<PBSEL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PBSEL0_SPEC>;
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
impl From<crate::W<PBSEL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PBSEL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P4SEL0` reader - Port 4 Select 0"]
pub type P4SEL0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `P4SEL0` writer - Port 4 Select 0"]
pub type P4SEL0_W<'a, const O: u8> = crate::FieldWriter<'a, u16, PBSEL0_SPEC, u8, u8, 8, O>;
#[doc = "Field `P3SEL0` reader - Port 3 Select 0"]
pub type P3SEL0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `P3SEL0` writer - Port 3 Select 0"]
pub type P3SEL0_W<'a, const O: u8> = crate::FieldWriter<'a, u16, PBSEL0_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 8:15 - Port 4 Select 0"]
    #[inline(always)]
    pub fn p4sel0(&self) -> P4SEL0_R {
        P4SEL0_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - Port 3 Select 0"]
    #[inline(always)]
    pub fn p3sel0(&self) -> P3SEL0_R {
        P3SEL0_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15 - Port 4 Select 0"]
    #[inline(always)]
    pub fn p4sel0(&mut self) -> P4SEL0_W<8> {
        P4SEL0_W::new(self)
    }
    #[doc = "Bits 0:7 - Port 3 Select 0"]
    #[inline(always)]
    pub fn p3sel0(&mut self) -> P3SEL0_W<0> {
        P3SEL0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port B Select 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pbsel0](index.html) module"]
pub struct PBSEL0_SPEC;
impl crate::RegisterSpec for PBSEL0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pbsel0::R](R) reader structure"]
impl crate::Readable for PBSEL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pbsel0::W](W) writer structure"]
impl crate::Writable for PBSEL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PBSEL0 to value 0"]
impl crate::Resettable for PBSEL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
