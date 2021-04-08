#[doc = "Register `FLCTL_ERASE_SECTADDR` reader"]
pub struct R(crate::R<FLCTL_ERASE_SECTADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLCTL_ERASE_SECTADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<FLCTL_ERASE_SECTADDR_SPEC>> for R {
    fn from(reader: crate::R<FLCTL_ERASE_SECTADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLCTL_ERASE_SECTADDR` writer"]
pub struct W(crate::W<FLCTL_ERASE_SECTADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLCTL_ERASE_SECTADDR_SPEC>;
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
impl core::convert::From<crate::W<FLCTL_ERASE_SECTADDR_SPEC>> for W {
    fn from(writer: crate::W<FLCTL_ERASE_SECTADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SECT_ADDRESS` reader - Address of Sector being Erased"]
pub struct SECT_ADDRESS_R(crate::FieldReader<u32, u32>);
impl SECT_ADDRESS_R {
    pub(crate) fn new(bits: u32) -> Self {
        SECT_ADDRESS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SECT_ADDRESS_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SECT_ADDRESS` writer - Address of Sector being Erased"]
pub struct SECT_ADDRESS_W<'a> {
    w: &'a mut W,
}
impl<'a> SECT_ADDRESS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x003f_ffff) | (value as u32 & 0x003f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:21 - Address of Sector being Erased"]
    #[inline(always)]
    pub fn sect_address(&self) -> SECT_ADDRESS_R {
        SECT_ADDRESS_R::new((self.bits & 0x003f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:21 - Address of Sector being Erased"]
    #[inline(always)]
    pub fn sect_address(&mut self) -> SECT_ADDRESS_W {
        SECT_ADDRESS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Erase Sector Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flctl_erase_sectaddr](index.html) module"]
pub struct FLCTL_ERASE_SECTADDR_SPEC;
impl crate::RegisterSpec for FLCTL_ERASE_SECTADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flctl_erase_sectaddr::R](R) reader structure"]
impl crate::Readable for FLCTL_ERASE_SECTADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flctl_erase_sectaddr::W](W) writer structure"]
impl crate::Writable for FLCTL_ERASE_SECTADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLCTL_ERASE_SECTADDR to value 0"]
impl crate::Resettable for FLCTL_ERASE_SECTADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
