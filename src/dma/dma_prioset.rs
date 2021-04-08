#[doc = "Register `DMA_PRIOSET` reader"]
pub struct R(crate::R<DMA_PRIOSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_PRIOSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<DMA_PRIOSET_SPEC>> for R {
    fn from(reader: crate::R<DMA_PRIOSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_PRIOSET` writer"]
pub struct W(crate::W<DMA_PRIOSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_PRIOSET_SPEC>;
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
impl core::convert::From<crate::W<DMA_PRIOSET_SPEC>> for W {
    fn from(writer: crate::W<DMA_PRIOSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Returns the channel priority mask status, or sets the channel priority to high.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum SET_A {
    #[doc = "0: DMA channel C is using the default priority level."]
    SET_0_READ = 0,
    #[doc = "1: DMA channel C is using a high priority level."]
    SET_1_READ = 1,
}
impl From<SET_A> for u32 {
    #[inline(always)]
    fn from(variant: SET_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SET` reader - Returns the channel priority mask status, or sets the channel priority to high."]
pub struct SET_R(crate::FieldReader<u32, SET_A>);
impl SET_R {
    pub(crate) fn new(bits: u32) -> Self {
        SET_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SET_A> {
        match self.bits {
            0 => Some(SET_A::SET_0_READ),
            1 => Some(SET_A::SET_1_READ),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SET_0_READ`"]
    #[inline(always)]
    pub fn is_set_0_read(&self) -> bool {
        **self == SET_A::SET_0_READ
    }
    #[doc = "Checks if the value of the field is `SET_1_READ`"]
    #[inline(always)]
    pub fn is_set_1_read(&self) -> bool {
        **self == SET_A::SET_1_READ
    }
}
impl core::ops::Deref for SET_R {
    type Target = crate::FieldReader<u32, SET_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Returns the channel priority mask status, or sets the channel priority to high.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum SET_AW {
    #[doc = "0: No effect.\r\r\nUse the DMA_PRIOCLR Register to set channel C to the default priority level."]
    SET_0_WRITE = 0,
    #[doc = "1: Channel C uses the high priority level.\r\r\nWriting to a bit where a DMA channel is not implemented has no effect."]
    SET_1_WRITE = 1,
}
impl From<SET_AW> for u32 {
    #[inline(always)]
    fn from(variant: SET_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `SET` writer - Returns the channel priority mask status, or sets the channel priority to high."]
pub struct SET_W<'a> {
    w: &'a mut W,
}
impl<'a> SET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SET_AW) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No effect. Use the DMA_PRIOCLR Register to set channel C to the default priority level."]
    #[inline(always)]
    pub fn set_0_write(self) -> &'a mut W {
        self.variant(SET_AW::SET_0_WRITE)
    }
    #[doc = "Channel C uses the high priority level. Writing to a bit where a DMA channel is not implemented has no effect."]
    #[inline(always)]
    pub fn set_1_write(self) -> &'a mut W {
        self.variant(SET_AW::SET_1_WRITE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Returns the channel priority mask status, or sets the channel priority to high."]
    #[inline(always)]
    pub fn set(&self) -> SET_R {
        SET_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Returns the channel priority mask status, or sets the channel priority to high."]
    #[inline(always)]
    pub fn set(&mut self) -> SET_W {
        SET_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Priority Set Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_prioset](index.html) module"]
pub struct DMA_PRIOSET_SPEC;
impl crate::RegisterSpec for DMA_PRIOSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_prioset::R](R) reader structure"]
impl crate::Readable for DMA_PRIOSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_prioset::W](W) writer structure"]
impl crate::Writable for DMA_PRIOSET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_PRIOSET to value 0"]
impl crate::Resettable for DMA_PRIOSET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
