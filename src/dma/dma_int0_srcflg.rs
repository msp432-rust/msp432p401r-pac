#[doc = "Register `DMA_INT0_SRCFLG` reader"]
pub struct R(crate::R<DMA_INT0_SRCFLG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_INT0_SRCFLG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_INT0_SRCFLG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_INT0_SRCFLG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CH0` reader - Channel 0 was the source of DMA_INT0"]
pub type CH0_R = crate::BitReader<bool>;
#[doc = "Field `CH1` reader - Channel 1 was the source of DMA_INT0"]
pub type CH1_R = crate::BitReader<bool>;
#[doc = "Field `CH2` reader - Channel 2 was the source of DMA_INT0"]
pub type CH2_R = crate::BitReader<bool>;
#[doc = "Field `CH3` reader - Channel 3 was the source of DMA_INT0"]
pub type CH3_R = crate::BitReader<bool>;
#[doc = "Field `CH4` reader - Channel 4 was the source of DMA_INT0"]
pub type CH4_R = crate::BitReader<bool>;
#[doc = "Field `CH5` reader - Channel 5 was the source of DMA_INT0"]
pub type CH5_R = crate::BitReader<bool>;
#[doc = "Field `CH6` reader - Channel 6 was the source of DMA_INT0"]
pub type CH6_R = crate::BitReader<bool>;
#[doc = "Field `CH7` reader - Channel 7 was the source of DMA_INT0"]
pub type CH7_R = crate::BitReader<bool>;
#[doc = "Field `CH8` reader - Channel 8 was the source of DMA_INT0"]
pub type CH8_R = crate::BitReader<bool>;
#[doc = "Field `CH9` reader - Channel 9 was the source of DMA_INT0"]
pub type CH9_R = crate::BitReader<bool>;
#[doc = "Field `CH10` reader - Channel 10 was the source of DMA_INT0"]
pub type CH10_R = crate::BitReader<bool>;
#[doc = "Field `CH11` reader - Channel 11 was the source of DMA_INT0"]
pub type CH11_R = crate::BitReader<bool>;
#[doc = "Field `CH12` reader - Channel 12 was the source of DMA_INT0"]
pub type CH12_R = crate::BitReader<bool>;
#[doc = "Field `CH13` reader - Channel 13 was the source of DMA_INT0"]
pub type CH13_R = crate::BitReader<bool>;
#[doc = "Field `CH14` reader - Channel 14 was the source of DMA_INT0"]
pub type CH14_R = crate::BitReader<bool>;
#[doc = "Field `CH15` reader - Channel 15 was the source of DMA_INT0"]
pub type CH15_R = crate::BitReader<bool>;
#[doc = "Field `CH16` reader - Channel 16 was the source of DMA_INT0"]
pub type CH16_R = crate::BitReader<bool>;
#[doc = "Field `CH17` reader - Channel 17 was the source of DMA_INT0"]
pub type CH17_R = crate::BitReader<bool>;
#[doc = "Field `CH18` reader - Channel 18 was the source of DMA_INT0"]
pub type CH18_R = crate::BitReader<bool>;
#[doc = "Field `CH19` reader - Channel 19 was the source of DMA_INT0"]
pub type CH19_R = crate::BitReader<bool>;
#[doc = "Field `CH20` reader - Channel 20 was the source of DMA_INT0"]
pub type CH20_R = crate::BitReader<bool>;
#[doc = "Field `CH21` reader - Channel 21 was the source of DMA_INT0"]
pub type CH21_R = crate::BitReader<bool>;
#[doc = "Field `CH22` reader - Channel 22 was the source of DMA_INT0"]
pub type CH22_R = crate::BitReader<bool>;
#[doc = "Field `CH23` reader - Channel 23 was the source of DMA_INT0"]
pub type CH23_R = crate::BitReader<bool>;
#[doc = "Field `CH24` reader - Channel 24 was the source of DMA_INT0"]
pub type CH24_R = crate::BitReader<bool>;
#[doc = "Field `CH25` reader - Channel 25 was the source of DMA_INT0"]
pub type CH25_R = crate::BitReader<bool>;
#[doc = "Field `CH26` reader - Channel 26 was the source of DMA_INT0"]
pub type CH26_R = crate::BitReader<bool>;
#[doc = "Field `CH27` reader - Channel 27 was the source of DMA_INT0"]
pub type CH27_R = crate::BitReader<bool>;
#[doc = "Field `CH28` reader - Channel 28 was the source of DMA_INT0"]
pub type CH28_R = crate::BitReader<bool>;
#[doc = "Field `CH29` reader - Channel 29 was the source of DMA_INT0"]
pub type CH29_R = crate::BitReader<bool>;
#[doc = "Field `CH30` reader - Channel 30 was the source of DMA_INT0"]
pub type CH30_R = crate::BitReader<bool>;
#[doc = "Field `CH31` reader - Channel 31 was the source of DMA_INT0"]
pub type CH31_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Channel 0 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch0(&self) -> CH0_R {
        CH0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch1(&self) -> CH1_R {
        CH1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 2 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch2(&self) -> CH2_R {
        CH2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 3 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch3(&self) -> CH3_R {
        CH3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 4 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch4(&self) -> CH4_R {
        CH4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel 5 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch5(&self) -> CH5_R {
        CH5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel 6 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch6(&self) -> CH6_R {
        CH6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel 7 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch7(&self) -> CH7_R {
        CH7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Channel 8 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch8(&self) -> CH8_R {
        CH8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel 9 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch9(&self) -> CH9_R {
        CH9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel 10 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch10(&self) -> CH10_R {
        CH10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel 11 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch11(&self) -> CH11_R {
        CH11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Channel 12 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch12(&self) -> CH12_R {
        CH12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Channel 13 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch13(&self) -> CH13_R {
        CH13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Channel 14 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch14(&self) -> CH14_R {
        CH14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Channel 15 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch15(&self) -> CH15_R {
        CH15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Channel 16 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch16(&self) -> CH16_R {
        CH16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Channel 17 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch17(&self) -> CH17_R {
        CH17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Channel 18 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch18(&self) -> CH18_R {
        CH18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Channel 19 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch19(&self) -> CH19_R {
        CH19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Channel 20 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch20(&self) -> CH20_R {
        CH20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Channel 21 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch21(&self) -> CH21_R {
        CH21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Channel 22 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch22(&self) -> CH22_R {
        CH22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Channel 23 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch23(&self) -> CH23_R {
        CH23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Channel 24 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch24(&self) -> CH24_R {
        CH24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Channel 25 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch25(&self) -> CH25_R {
        CH25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Channel 26 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch26(&self) -> CH26_R {
        CH26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Channel 27 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch27(&self) -> CH27_R {
        CH27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Channel 28 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch28(&self) -> CH28_R {
        CH28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Channel 29 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch29(&self) -> CH29_R {
        CH29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Channel 30 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch30(&self) -> CH30_R {
        CH30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Channel 31 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch31(&self) -> CH31_R {
        CH31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Interrupt 0 Source Channel Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_int0_srcflg](index.html) module"]
pub struct DMA_INT0_SRCFLG_SPEC;
impl crate::RegisterSpec for DMA_INT0_SRCFLG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_int0_srcflg::R](R) reader structure"]
impl crate::Readable for DMA_INT0_SRCFLG_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DMA_INT0_SRCFLG to value 0"]
impl crate::Resettable for DMA_INT0_SRCFLG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
