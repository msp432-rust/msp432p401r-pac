#[doc = "Register `FLCTL_PRGVER_TIMCTL` reader"]
pub struct R(crate::R<FLCTL_PRGVER_TIMCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLCTL_PRGVER_TIMCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<FLCTL_PRGVER_TIMCTL_SPEC>> for R {
    fn from(reader: crate::R<FLCTL_PRGVER_TIMCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SETUP` reader - Length of the Setup phase for this operation"]
pub struct SETUP_R(crate::FieldReader<u8, u8>);
impl SETUP_R {
    pub(crate) fn new(bits: u8) -> Self {
        SETUP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SETUP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACTIVE` reader - Length of the Active phase for this operation"]
pub struct ACTIVE_R(crate::FieldReader<u8, u8>);
impl ACTIVE_R {
    pub(crate) fn new(bits: u8) -> Self {
        ACTIVE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACTIVE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HOLD` reader - Length of the Hold phase for this operation"]
pub struct HOLD_R(crate::FieldReader<u8, u8>);
impl HOLD_R {
    pub(crate) fn new(bits: u8) -> Self {
        HOLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HOLD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Length of the Setup phase for this operation"]
    #[inline(always)]
    pub fn setup(&self) -> SETUP_R {
        SETUP_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - Length of the Active phase for this operation"]
    #[inline(always)]
    pub fn active(&self) -> ACTIVE_R {
        ACTIVE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Length of the Hold phase for this operation"]
    #[inline(always)]
    pub fn hold(&self) -> HOLD_R {
        HOLD_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
#[doc = "Program Verify Timing Control Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flctl_prgver_timctl](index.html) module"]
pub struct FLCTL_PRGVER_TIMCTL_SPEC;
impl crate::RegisterSpec for FLCTL_PRGVER_TIMCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flctl_prgver_timctl::R](R) reader structure"]
impl crate::Readable for FLCTL_PRGVER_TIMCTL_SPEC {
    type Reader = R;
}
