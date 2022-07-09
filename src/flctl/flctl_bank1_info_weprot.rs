#[doc = "Register `FLCTL_BANK1_INFO_WEPROT` reader"]
pub struct R(crate::R<FLCTL_BANK1_INFO_WEPROT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLCTL_BANK1_INFO_WEPROT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLCTL_BANK1_INFO_WEPROT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLCTL_BANK1_INFO_WEPROT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLCTL_BANK1_INFO_WEPROT` writer"]
pub struct W(crate::W<FLCTL_BANK1_INFO_WEPROT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLCTL_BANK1_INFO_WEPROT_SPEC>;
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
impl From<crate::W<FLCTL_BANK1_INFO_WEPROT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLCTL_BANK1_INFO_WEPROT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PROT0` reader - Protects Sector 0 from program or erase operations"]
pub type PROT0_R = crate::BitReader<bool>;
#[doc = "Field `PROT0` writer - Protects Sector 0 from program or erase operations"]
pub type PROT0_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK1_INFO_WEPROT_SPEC, bool, O>;
#[doc = "Field `PROT1` reader - Protects Sector 1 from program or erase operations"]
pub type PROT1_R = crate::BitReader<bool>;
#[doc = "Field `PROT1` writer - Protects Sector 1 from program or erase operations"]
pub type PROT1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK1_INFO_WEPROT_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Protects Sector 0 from program or erase operations"]
    #[inline(always)]
    pub fn prot0(&self) -> PROT0_R {
        PROT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Protects Sector 1 from program or erase operations"]
    #[inline(always)]
    pub fn prot1(&self) -> PROT1_R {
        PROT1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Protects Sector 0 from program or erase operations"]
    #[inline(always)]
    pub fn prot0(&mut self) -> PROT0_W<0> {
        PROT0_W::new(self)
    }
    #[doc = "Bit 1 - Protects Sector 1 from program or erase operations"]
    #[inline(always)]
    pub fn prot1(&mut self) -> PROT1_W<1> {
        PROT1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Information Memory Bank1 Write/Erase Protection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flctl_bank1_info_weprot](index.html) module"]
pub struct FLCTL_BANK1_INFO_WEPROT_SPEC;
impl crate::RegisterSpec for FLCTL_BANK1_INFO_WEPROT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flctl_bank1_info_weprot::R](R) reader structure"]
impl crate::Readable for FLCTL_BANK1_INFO_WEPROT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flctl_bank1_info_weprot::W](W) writer structure"]
impl crate::Writable for FLCTL_BANK1_INFO_WEPROT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLCTL_BANK1_INFO_WEPROT to value 0x03"]
impl crate::Resettable for FLCTL_BANK1_INFO_WEPROT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x03
    }
}
