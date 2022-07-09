#[doc = "Register `PEDIR` reader"]
pub struct R(crate::R<PEDIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PEDIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PEDIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PEDIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PEDIR` writer"]
pub struct W(crate::W<PEDIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PEDIR_SPEC>;
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
impl From<crate::W<PEDIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PEDIR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P9DIR` reader - Port 9 Direction"]
pub type P9DIR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `P9DIR` writer - Port 9 Direction"]
pub type P9DIR_W<'a, const O: u8> = crate::FieldWriter<'a, u16, PEDIR_SPEC, u8, u8, 8, O>;
#[doc = "Field `P10DIR` reader - Port 10 Direction"]
pub type P10DIR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `P10DIR` writer - Port 10 Direction"]
pub type P10DIR_W<'a, const O: u8> = crate::FieldWriter<'a, u16, PEDIR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Port 9 Direction"]
    #[inline(always)]
    pub fn p9dir(&self) -> P9DIR_R {
        P9DIR_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 10 Direction"]
    #[inline(always)]
    pub fn p10dir(&self) -> P10DIR_R {
        P10DIR_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 9 Direction"]
    #[inline(always)]
    pub fn p9dir(&mut self) -> P9DIR_W<0> {
        P9DIR_W::new(self)
    }
    #[doc = "Bits 8:15 - Port 10 Direction"]
    #[inline(always)]
    pub fn p10dir(&mut self) -> P10DIR_W<8> {
        P10DIR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port E Direction\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pedir](index.html) module"]
pub struct PEDIR_SPEC;
impl crate::RegisterSpec for PEDIR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pedir::R](R) reader structure"]
impl crate::Readable for PEDIR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pedir::W](W) writer structure"]
impl crate::Writable for PEDIR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PEDIR to value 0"]
impl crate::Resettable for PEDIR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
