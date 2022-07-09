#[doc = "Register `SYS_REBOOT_CTL` reader"]
pub struct R(crate::R<SYS_REBOOT_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYS_REBOOT_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYS_REBOOT_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYS_REBOOT_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYS_REBOOT_CTL` writer"]
pub struct W(crate::W<SYS_REBOOT_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYS_REBOOT_CTL_SPEC>;
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
impl From<crate::W<SYS_REBOOT_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYS_REBOOT_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REBOOT` reader - Write 1 initiates a Reboot of the device"]
pub type REBOOT_R = crate::BitReader<bool>;
#[doc = "Field `REBOOT` writer - Write 1 initiates a Reboot of the device"]
pub type REBOOT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYS_REBOOT_CTL_SPEC, bool, O>;
#[doc = "Field `WKEY` writer - Key to enable writes to bit 0"]
pub type WKEY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SYS_REBOOT_CTL_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 0 - Write 1 initiates a Reboot of the device"]
    #[inline(always)]
    pub fn reboot(&self) -> REBOOT_R {
        REBOOT_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 initiates a Reboot of the device"]
    #[inline(always)]
    pub fn reboot(&mut self) -> REBOOT_W<0> {
        REBOOT_W::new(self)
    }
    #[doc = "Bits 8:15 - Key to enable writes to bit 0"]
    #[inline(always)]
    pub fn wkey(&mut self) -> WKEY_W<8> {
        WKEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reboot Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_reboot_ctl](index.html) module"]
pub struct SYS_REBOOT_CTL_SPEC;
impl crate::RegisterSpec for SYS_REBOOT_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sys_reboot_ctl::R](R) reader structure"]
impl crate::Readable for SYS_REBOOT_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sys_reboot_ctl::W](W) writer structure"]
impl crate::Writable for SYS_REBOOT_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYS_REBOOT_CTL to value 0xfe"]
impl crate::Resettable for SYS_REBOOT_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xfe
    }
}
