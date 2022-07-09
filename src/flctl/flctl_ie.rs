#[doc = "Register `FLCTL_IE` reader"]
pub struct R(crate::R<FLCTL_IE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLCTL_IE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLCTL_IE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLCTL_IE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLCTL_IE` writer"]
pub struct W(crate::W<FLCTL_IE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLCTL_IE_SPEC>;
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
impl From<crate::W<FLCTL_IE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLCTL_IE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RDBRST` reader - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
pub type RDBRST_R = crate::BitReader<bool>;
#[doc = "Field `RDBRST` writer - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
pub type RDBRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLCTL_IE_SPEC, bool, O>;
#[doc = "Field `AVPRE` reader - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
pub type AVPRE_R = crate::BitReader<bool>;
#[doc = "Field `AVPRE` writer - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
pub type AVPRE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLCTL_IE_SPEC, bool, O>;
#[doc = "Field `AVPST` reader - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
pub type AVPST_R = crate::BitReader<bool>;
#[doc = "Field `AVPST` writer - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
pub type AVPST_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLCTL_IE_SPEC, bool, O>;
#[doc = "Field `PRG` reader - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
pub type PRG_R = crate::BitReader<bool>;
#[doc = "Field `PRG` writer - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
pub type PRG_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLCTL_IE_SPEC, bool, O>;
#[doc = "Field `PRGB` reader - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
pub type PRGB_R = crate::BitReader<bool>;
#[doc = "Field `PRGB` writer - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
pub type PRGB_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLCTL_IE_SPEC, bool, O>;
#[doc = "Field `ERASE` reader - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
pub type ERASE_R = crate::BitReader<bool>;
#[doc = "Field `ERASE` writer - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
pub type ERASE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLCTL_IE_SPEC, bool, O>;
#[doc = "Field `BMRK` reader - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
pub type BMRK_R = crate::BitReader<bool>;
#[doc = "Field `BMRK` writer - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
pub type BMRK_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLCTL_IE_SPEC, bool, O>;
#[doc = "Field `PRG_ERR` reader - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
pub type PRG_ERR_R = crate::BitReader<bool>;
#[doc = "Field `PRG_ERR` writer - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
pub type PRG_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLCTL_IE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
    #[inline(always)]
    pub fn rdbrst(&self) -> RDBRST_R {
        RDBRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
    #[inline(always)]
    pub fn avpre(&self) -> AVPRE_R {
        AVPRE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
    #[inline(always)]
    pub fn avpst(&self) -> AVPST_R {
        AVPST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
    #[inline(always)]
    pub fn prg(&self) -> PRG_R {
        PRG_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
    #[inline(always)]
    pub fn prgb(&self) -> PRGB_R {
        PRGB_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
    #[inline(always)]
    pub fn erase(&self) -> ERASE_R {
        ERASE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
    #[inline(always)]
    pub fn bmrk(&self) -> BMRK_R {
        BMRK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
    #[inline(always)]
    pub fn prg_err(&self) -> PRG_ERR_R {
        PRG_ERR_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
    #[inline(always)]
    pub fn rdbrst(&mut self) -> RDBRST_W<0> {
        RDBRST_W::new(self)
    }
    #[doc = "Bit 1 - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
    #[inline(always)]
    pub fn avpre(&mut self) -> AVPRE_W<1> {
        AVPRE_W::new(self)
    }
    #[doc = "Bit 2 - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
    #[inline(always)]
    pub fn avpst(&mut self) -> AVPST_W<2> {
        AVPST_W::new(self)
    }
    #[doc = "Bit 3 - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
    #[inline(always)]
    pub fn prg(&mut self) -> PRG_W<3> {
        PRG_W::new(self)
    }
    #[doc = "Bit 4 - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
    #[inline(always)]
    pub fn prgb(&mut self) -> PRGB_W<4> {
        PRGB_W::new(self)
    }
    #[doc = "Bit 5 - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
    #[inline(always)]
    pub fn erase(&mut self) -> ERASE_W<5> {
        ERASE_W::new(self)
    }
    #[doc = "Bit 8 - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
    #[inline(always)]
    pub fn bmrk(&mut self) -> BMRK_W<8> {
        BMRK_W::new(self)
    }
    #[doc = "Bit 9 - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
    #[inline(always)]
    pub fn prg_err(&mut self) -> PRG_ERR_W<9> {
        PRG_ERR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flctl_ie](index.html) module"]
pub struct FLCTL_IE_SPEC;
impl crate::RegisterSpec for FLCTL_IE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flctl_ie::R](R) reader structure"]
impl crate::Readable for FLCTL_IE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flctl_ie::W](W) writer structure"]
impl crate::Writable for FLCTL_IE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLCTL_IE to value 0"]
impl crate::Resettable for FLCTL_IE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
