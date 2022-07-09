#[doc = "Register `SYS_RESET_REQ` reader"]
pub struct R(crate::R<SYS_RESET_REQ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYS_RESET_REQ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYS_RESET_REQ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYS_RESET_REQ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYS_RESET_REQ` writer"]
pub struct W(crate::W<SYS_RESET_REQ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYS_RESET_REQ_SPEC>;
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
impl From<crate::W<SYS_RESET_REQ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYS_RESET_REQ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `POR` writer - Generate POR"]
pub type POR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYS_RESET_REQ_SPEC, bool, O>;
#[doc = "Field `REBOOT` writer - Generate Reboot_Reset"]
pub type REBOOT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYS_RESET_REQ_SPEC, bool, O>;
#[doc = "Field `WKEY` writer - Write key"]
pub type WKEY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SYS_RESET_REQ_SPEC, u8, u8, 8, O>;
impl W {
    #[doc = "Bit 0 - Generate POR"]
    #[inline(always)]
    pub fn por(&mut self) -> POR_W<0> {
        POR_W::new(self)
    }
    #[doc = "Bit 1 - Generate Reboot_Reset"]
    #[inline(always)]
    pub fn reboot(&mut self) -> REBOOT_W<1> {
        REBOOT_W::new(self)
    }
    #[doc = "Bits 8:15 - Write key"]
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
#[doc = "Reset Request Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_reset_req](index.html) module"]
pub struct SYS_RESET_REQ_SPEC;
impl crate::RegisterSpec for SYS_RESET_REQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sys_reset_req::R](R) reader structure"]
impl crate::Readable for SYS_RESET_REQ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sys_reset_req::W](W) writer structure"]
impl crate::Writable for SYS_RESET_REQ_SPEC {
    type Writer = W;
}
