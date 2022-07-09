#[doc = "Register `RSTCTL_PSSRESET_STAT` reader"]
pub struct R(crate::R<RSTCTL_PSSRESET_STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSTCTL_PSSRESET_STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSTCTL_PSSRESET_STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSTCTL_PSSRESET_STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SVSMH` reader - Indicates if POR was caused by an SVSMH trip condition int the PSS"]
pub type SVSMH_R = crate::BitReader<bool>;
#[doc = "Field `BGREF` reader - Indicates if POR was caused by a BGREF not okay condition in the PSS"]
pub type BGREF_R = crate::BitReader<bool>;
#[doc = "Field `VCCDET` reader - Indicates if POR was caused by a VCCDET trip condition in the PSS"]
pub type VCCDET_R = crate::BitReader<bool>;
#[doc = "Field `SVSL` reader - Indicates if POR was caused by an SVSL trip condition in the PSS"]
pub type SVSL_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 1 - Indicates if POR was caused by an SVSMH trip condition int the PSS"]
    #[inline(always)]
    pub fn svsmh(&self) -> SVSMH_R {
        SVSMH_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Indicates if POR was caused by a BGREF not okay condition in the PSS"]
    #[inline(always)]
    pub fn bgref(&self) -> BGREF_R {
        BGREF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Indicates if POR was caused by a VCCDET trip condition in the PSS"]
    #[inline(always)]
    pub fn vccdet(&self) -> VCCDET_R {
        VCCDET_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 0 - Indicates if POR was caused by an SVSL trip condition in the PSS"]
    #[inline(always)]
    pub fn svsl(&self) -> SVSL_R {
        SVSL_R::new((self.bits & 1) != 0)
    }
}
#[doc = "PSS Reset Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rstctl_pssreset_stat](index.html) module"]
pub struct RSTCTL_PSSRESET_STAT_SPEC;
impl crate::RegisterSpec for RSTCTL_PSSRESET_STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rstctl_pssreset_stat::R](R) reader structure"]
impl crate::Readable for RSTCTL_PSSRESET_STAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RSTCTL_PSSRESET_STAT to value 0x0f"]
impl crate::Resettable for RSTCTL_PSSRESET_STAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0f
    }
}
