#[doc = "Register `DMA_ALTCLR` writer"]
pub struct W(crate::W<DMA_ALTCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_ALTCLR_SPEC>;
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
impl From<crate::W<DMA_ALTCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_ALTCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Channel Primary-Alternate Clear Register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum CLR_AW {
    #[doc = "0: No effect. Use the DMA_ALTSET Register to select the alternate data structure."]
    CLR_0 = 0,
    #[doc = "1: Selects the primary data structure for channel C. Writing to a bit where a DMA channel is not implemented has no effect."]
    CLR_1 = 1,
}
impl From<CLR_AW> for u32 {
    #[inline(always)]
    fn from(variant: CLR_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `CLR` writer - Channel Primary-Alternate Clear Register"]
pub type CLR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMA_ALTCLR_SPEC, u32, CLR_AW, 32, O>;
impl<'a, const O: u8> CLR_W<'a, O> {
    #[doc = "No effect. Use the DMA_ALTSET Register to select the alternate data structure."]
    #[inline(always)]
    pub fn clr_0(self) -> &'a mut W {
        self.variant(CLR_AW::CLR_0)
    }
    #[doc = "Selects the primary data structure for channel C. Writing to a bit where a DMA channel is not implemented has no effect."]
    #[inline(always)]
    pub fn clr_1(self) -> &'a mut W {
        self.variant(CLR_AW::CLR_1)
    }
}
impl W {
    #[doc = "Bits 0:31 - Channel Primary-Alternate Clear Register"]
    #[inline(always)]
    pub fn clr(&mut self) -> CLR_W<0> {
        CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Primary-Alternate Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_altclr](index.html) module"]
pub struct DMA_ALTCLR_SPEC;
impl crate::RegisterSpec for DMA_ALTCLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [dma_altclr::W](W) writer structure"]
impl crate::Writable for DMA_ALTCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_ALTCLR to value 0"]
impl crate::Resettable for DMA_ALTCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
