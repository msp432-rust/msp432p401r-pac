#[doc = "Register `PADIR` reader"]
pub struct R(crate::R<PADIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PADIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PADIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PADIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PADIR` writer"]
pub struct W(crate::W<PADIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PADIR_SPEC>;
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
impl From<crate::W<PADIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PADIR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P1DIR` reader - Port 1 Direction"]
pub type P1DIR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `P1DIR` writer - Port 1 Direction"]
pub type P1DIR_W<'a, const O: u8> = crate::FieldWriter<'a, u16, PADIR_SPEC, u8, u8, 8, O>;
#[doc = "Field `P2DIR` reader - Port 2 Direction"]
pub type P2DIR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `P2DIR` writer - Port 2 Direction"]
pub type P2DIR_W<'a, const O: u8> = crate::FieldWriter<'a, u16, PADIR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Port 1 Direction"]
    #[inline(always)]
    pub fn p1dir(&self) -> P1DIR_R {
        P1DIR_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 2 Direction"]
    #[inline(always)]
    pub fn p2dir(&self) -> P2DIR_R {
        P2DIR_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 1 Direction"]
    #[inline(always)]
    pub fn p1dir(&mut self) -> P1DIR_W<0> {
        P1DIR_W::new(self)
    }
    #[doc = "Bits 8:15 - Port 2 Direction"]
    #[inline(always)]
    pub fn p2dir(&mut self) -> P2DIR_W<8> {
        P2DIR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port A Direction\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [padir](index.html) module"]
pub struct PADIR_SPEC;
impl crate::RegisterSpec for PADIR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [padir::R](R) reader structure"]
impl crate::Readable for PADIR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [padir::W](W) writer structure"]
impl crate::Writable for PADIR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PADIR to value 0"]
impl crate::Resettable for PADIR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
