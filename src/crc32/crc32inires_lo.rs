#[doc = "Register `CRC32INIRES_LO` reader"]
pub struct R(crate::R<CRC32INIRES_LO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRC32INIRES_LO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRC32INIRES_LO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRC32INIRES_LO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRC32INIRES_LO` writer"]
pub struct W(crate::W<CRC32INIRES_LO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRC32INIRES_LO_SPEC>;
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
impl From<crate::W<CRC32INIRES_LO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRC32INIRES_LO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CRC32INIRES_LO` reader - CRC32 initialization and result, lower 16 bits"]
pub type CRC32INIRES_LO_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CRC32INIRES_LO` writer - CRC32 initialization and result, lower 16 bits"]
pub type CRC32INIRES_LO_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, CRC32INIRES_LO_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - CRC32 initialization and result, lower 16 bits"]
    #[inline(always)]
    pub fn crc32inires_lo(&self) -> CRC32INIRES_LO_R {
        CRC32INIRES_LO_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - CRC32 initialization and result, lower 16 bits"]
    #[inline(always)]
    pub fn crc32inires_lo(&mut self) -> CRC32INIRES_LO_W<0> {
        CRC32INIRES_LO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CRC32 Initialization and Result, lower 16 bits\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crc32inires_lo](index.html) module"]
pub struct CRC32INIRES_LO_SPEC;
impl crate::RegisterSpec for CRC32INIRES_LO_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [crc32inires_lo::R](R) reader structure"]
impl crate::Readable for CRC32INIRES_LO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crc32inires_lo::W](W) writer structure"]
impl crate::Writable for CRC32INIRES_LO_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CRC32INIRES_LO to value 0"]
impl crate::Resettable for CRC32INIRES_LO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
