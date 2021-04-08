#[doc = "Register `CRC32INIRES_HI` reader"]
pub struct R(crate::R<CRC32INIRES_HI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRC32INIRES_HI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CRC32INIRES_HI_SPEC>> for R {
    fn from(reader: crate::R<CRC32INIRES_HI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRC32INIRES_HI` writer"]
pub struct W(crate::W<CRC32INIRES_HI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRC32INIRES_HI_SPEC>;
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
impl core::convert::From<crate::W<CRC32INIRES_HI_SPEC>> for W {
    fn from(writer: crate::W<CRC32INIRES_HI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CRC32INIRES_HI` reader - CRC32 initialization and result, upper 16 bits"]
pub struct CRC32INIRES_HI_R(crate::FieldReader<u16, u16>);
impl CRC32INIRES_HI_R {
    pub(crate) fn new(bits: u16) -> Self {
        CRC32INIRES_HI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRC32INIRES_HI_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRC32INIRES_HI` writer - CRC32 initialization and result, upper 16 bits"]
pub struct CRC32INIRES_HI_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC32INIRES_HI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u16 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - CRC32 initialization and result, upper 16 bits"]
    #[inline(always)]
    pub fn crc32inires_hi(&self) -> CRC32INIRES_HI_R {
        CRC32INIRES_HI_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CRC32 initialization and result, upper 16 bits"]
    #[inline(always)]
    pub fn crc32inires_hi(&mut self) -> CRC32INIRES_HI_W {
        CRC32INIRES_HI_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CRC32 Initialization and Result, upper 16 bits\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crc32inires_hi](index.html) module"]
pub struct CRC32INIRES_HI_SPEC;
impl crate::RegisterSpec for CRC32INIRES_HI_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [crc32inires_hi::R](R) reader structure"]
impl crate::Readable for CRC32INIRES_HI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crc32inires_hi::W](W) writer structure"]
impl crate::Writable for CRC32INIRES_HI_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CRC32INIRES_HI to value 0"]
impl crate::Resettable for CRC32INIRES_HI_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
