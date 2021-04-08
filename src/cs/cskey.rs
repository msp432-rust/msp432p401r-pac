#[doc = "Register `CSKEY` reader"]
pub struct R(crate::R<CSKEY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSKEY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CSKEY_SPEC>> for R {
    fn from(reader: crate::R<CSKEY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSKEY` writer"]
pub struct W(crate::W<CSKEY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSKEY_SPEC>;
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
impl core::convert::From<crate::W<CSKEY_SPEC>> for W {
    fn from(writer: crate::W<CSKEY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CSKEY` reader - Write xxxx_695Ah to unlock"]
pub struct CSKEY_R(crate::FieldReader<u16, u16>);
impl CSKEY_R {
    pub(crate) fn new(bits: u16) -> Self {
        CSKEY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSKEY_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSKEY` writer - Write xxxx_695Ah to unlock"]
pub struct CSKEY_W<'a> {
    w: &'a mut W,
}
impl<'a> CSKEY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Write xxxx_695Ah to unlock"]
    #[inline(always)]
    pub fn cskey(&self) -> CSKEY_R {
        CSKEY_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Write xxxx_695Ah to unlock"]
    #[inline(always)]
    pub fn cskey(&mut self) -> CSKEY_W {
        CSKEY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Key Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cskey](index.html) module"]
pub struct CSKEY_SPEC;
impl crate::RegisterSpec for CSKEY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cskey::R](R) reader structure"]
impl crate::Readable for CSKEY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cskey::W](W) writer structure"]
impl crate::Writable for CSKEY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CSKEY to value 0xa596"]
impl crate::Resettable for CSKEY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xa596
    }
}
