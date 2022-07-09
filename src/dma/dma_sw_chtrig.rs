#[doc = "Register `DMA_SW_CHTRIG` reader"]
pub struct R(crate::R<DMA_SW_CHTRIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_SW_CHTRIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_SW_CHTRIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_SW_CHTRIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_SW_CHTRIG` writer"]
pub struct W(crate::W<DMA_SW_CHTRIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_SW_CHTRIG_SPEC>;
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
impl From<crate::W<DMA_SW_CHTRIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_SW_CHTRIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH0` reader - Write 1, triggers DMA_CHANNEL0"]
pub type CH0_R = crate::BitReader<bool>;
#[doc = "Field `CH0` writer - Write 1, triggers DMA_CHANNEL0"]
pub type CH0_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_SW_CHTRIG_SPEC, bool, O>;
#[doc = "Field `CH1` reader - Write 1, triggers DMA_CHANNEL1"]
pub type CH1_R = crate::BitReader<bool>;
#[doc = "Field `CH1` writer - Write 1, triggers DMA_CHANNEL1"]
pub type CH1_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_SW_CHTRIG_SPEC, bool, O>;
#[doc = "Field `CH2` reader - Write 1, triggers DMA_CHANNEL2"]
pub type CH2_R = crate::BitReader<bool>;
#[doc = "Field `CH2` writer - Write 1, triggers DMA_CHANNEL2"]
pub type CH2_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_SW_CHTRIG_SPEC, bool, O>;
#[doc = "Field `CH3` reader - Write 1, triggers DMA_CHANNEL3"]
pub type CH3_R = crate::BitReader<bool>;
#[doc = "Field `CH3` writer - Write 1, triggers DMA_CHANNEL3"]
pub type CH3_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_SW_CHTRIG_SPEC, bool, O>;
#[doc = "Field `CH4` reader - Write 1, triggers DMA_CHANNEL4"]
pub type CH4_R = crate::BitReader<bool>;
#[doc = "Field `CH4` writer - Write 1, triggers DMA_CHANNEL4"]
pub type CH4_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_SW_CHTRIG_SPEC, bool, O>;
#[doc = "Field `CH5` reader - Write 1, triggers DMA_CHANNEL5"]
pub type CH5_R = crate::BitReader<bool>;
#[doc = "Field `CH5` writer - Write 1, triggers DMA_CHANNEL5"]
pub type CH5_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_SW_CHTRIG_SPEC, bool, O>;
#[doc = "Field `CH6` reader - Write 1, triggers DMA_CHANNEL6"]
pub type CH6_R = crate::BitReader<bool>;
#[doc = "Field `CH6` writer - Write 1, triggers DMA_CHANNEL6"]
pub type CH6_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_SW_CHTRIG_SPEC, bool, O>;
#[doc = "Field `CH7` reader - Write 1, triggers DMA_CHANNEL7"]
pub type CH7_R = crate::BitReader<bool>;
#[doc = "Field `CH7` writer - Write 1, triggers DMA_CHANNEL7"]
pub type CH7_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_SW_CHTRIG_SPEC, bool, O>;
#[doc = "Field `CH8` reader - Write 1, triggers DMA_CHANNEL8"]
pub type CH8_R = crate::BitReader<bool>;
#[doc = "Field `CH8` writer - Write 1, triggers DMA_CHANNEL8"]
pub type CH8_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_SW_CHTRIG_SPEC, bool, O>;
#[doc = "Field `CH9` reader - Write 1, triggers DMA_CHANNEL9"]
pub type CH9_R = crate::BitReader<bool>;
#[doc = "Field `CH9` writer - Write 1, triggers DMA_CHANNEL9"]
pub type CH9_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_SW_CHTRIG_SPEC, bool, O>;
#[doc = "Field `CH10` reader - Write 1, triggers DMA_CHANNEL10"]
pub type CH10_R = crate::BitReader<bool>;
#[doc = "Field `CH10` writer - Write 1, triggers DMA_CHANNEL10"]
pub type CH10_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_SW_CHTRIG_SPEC, bool, O>;
#[doc = "Field `CH11` reader - Write 1, triggers DMA_CHANNEL11"]
pub type CH11_R = crate::BitReader<bool>;
#[doc = "Field `CH11` writer - Write 1, triggers DMA_CHANNEL11"]
pub type CH11_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_SW_CHTRIG_SPEC, bool, O>;
#[doc = "Field `CH12` reader - Write 1, triggers DMA_CHANNEL12"]
pub type CH12_R = crate::BitReader<bool>;
#[doc = "Field `CH12` writer - Write 1, triggers DMA_CHANNEL12"]
pub type CH12_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_SW_CHTRIG_SPEC, bool, O>;
#[doc = "Field `CH13` reader - Write 1, triggers DMA_CHANNEL13"]
pub type CH13_R = crate::BitReader<bool>;
#[doc = "Field `CH13` writer - Write 1, triggers DMA_CHANNEL13"]
pub type CH13_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_SW_CHTRIG_SPEC, bool, O>;
#[doc = "Field `CH14` reader - Write 1, triggers DMA_CHANNEL14"]
pub type CH14_R = crate::BitReader<bool>;
#[doc = "Field `CH14` writer - Write 1, triggers DMA_CHANNEL14"]
pub type CH14_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_SW_CHTRIG_SPEC, bool, O>;
#[doc = "Field `CH15` reader - Write 1, triggers DMA_CHANNEL15"]
pub type CH15_R = crate::BitReader<bool>;
#[doc = "Field `CH15` writer - Write 1, triggers DMA_CHANNEL15"]
pub type CH15_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_SW_CHTRIG_SPEC, bool, O>;
#[doc = "Field `CH16` reader - Write 1, triggers DMA_CHANNEL16"]
pub type CH16_R = crate::BitReader<bool>;
#[doc = "Field `CH16` writer - Write 1, triggers DMA_CHANNEL16"]
pub type CH16_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_SW_CHTRIG_SPEC, bool, O>;
#[doc = "Field `CH17` reader - Write 1, triggers DMA_CHANNEL17"]
pub type CH17_R = crate::BitReader<bool>;
#[doc = "Field `CH17` writer - Write 1, triggers DMA_CHANNEL17"]
pub type CH17_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_SW_CHTRIG_SPEC, bool, O>;
#[doc = "Field `CH18` reader - Write 1, triggers DMA_CHANNEL18"]
pub type CH18_R = crate::BitReader<bool>;
#[doc = "Field `CH18` writer - Write 1, triggers DMA_CHANNEL18"]
pub type CH18_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_SW_CHTRIG_SPEC, bool, O>;
#[doc = "Field `CH19` reader - Write 1, triggers DMA_CHANNEL19"]
pub type CH19_R = crate::BitReader<bool>;
#[doc = "Field `CH19` writer - Write 1, triggers DMA_CHANNEL19"]
pub type CH19_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_SW_CHTRIG_SPEC, bool, O>;
#[doc = "Field `CH20` reader - Write 1, triggers DMA_CHANNEL20"]
pub type CH20_R = crate::BitReader<bool>;
#[doc = "Field `CH20` writer - Write 1, triggers DMA_CHANNEL20"]
pub type CH20_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_SW_CHTRIG_SPEC, bool, O>;
#[doc = "Field `CH21` reader - Write 1, triggers DMA_CHANNEL21"]
pub type CH21_R = crate::BitReader<bool>;
#[doc = "Field `CH21` writer - Write 1, triggers DMA_CHANNEL21"]
pub type CH21_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_SW_CHTRIG_SPEC, bool, O>;
#[doc = "Field `CH22` reader - Write 1, triggers DMA_CHANNEL22"]
pub type CH22_R = crate::BitReader<bool>;
#[doc = "Field `CH22` writer - Write 1, triggers DMA_CHANNEL22"]
pub type CH22_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_SW_CHTRIG_SPEC, bool, O>;
#[doc = "Field `CH23` reader - Write 1, triggers DMA_CHANNEL23"]
pub type CH23_R = crate::BitReader<bool>;
#[doc = "Field `CH23` writer - Write 1, triggers DMA_CHANNEL23"]
pub type CH23_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_SW_CHTRIG_SPEC, bool, O>;
#[doc = "Field `CH24` reader - Write 1, triggers DMA_CHANNEL24"]
pub type CH24_R = crate::BitReader<bool>;
#[doc = "Field `CH24` writer - Write 1, triggers DMA_CHANNEL24"]
pub type CH24_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_SW_CHTRIG_SPEC, bool, O>;
#[doc = "Field `CH25` reader - Write 1, triggers DMA_CHANNEL25"]
pub type CH25_R = crate::BitReader<bool>;
#[doc = "Field `CH25` writer - Write 1, triggers DMA_CHANNEL25"]
pub type CH25_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_SW_CHTRIG_SPEC, bool, O>;
#[doc = "Field `CH26` reader - Write 1, triggers DMA_CHANNEL26"]
pub type CH26_R = crate::BitReader<bool>;
#[doc = "Field `CH26` writer - Write 1, triggers DMA_CHANNEL26"]
pub type CH26_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_SW_CHTRIG_SPEC, bool, O>;
#[doc = "Field `CH27` reader - Write 1, triggers DMA_CHANNEL27"]
pub type CH27_R = crate::BitReader<bool>;
#[doc = "Field `CH27` writer - Write 1, triggers DMA_CHANNEL27"]
pub type CH27_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_SW_CHTRIG_SPEC, bool, O>;
#[doc = "Field `CH28` reader - Write 1, triggers DMA_CHANNEL28"]
pub type CH28_R = crate::BitReader<bool>;
#[doc = "Field `CH28` writer - Write 1, triggers DMA_CHANNEL28"]
pub type CH28_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_SW_CHTRIG_SPEC, bool, O>;
#[doc = "Field `CH29` reader - Write 1, triggers DMA_CHANNEL29"]
pub type CH29_R = crate::BitReader<bool>;
#[doc = "Field `CH29` writer - Write 1, triggers DMA_CHANNEL29"]
pub type CH29_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_SW_CHTRIG_SPEC, bool, O>;
#[doc = "Field `CH30` reader - Write 1, triggers DMA_CHANNEL30"]
pub type CH30_R = crate::BitReader<bool>;
#[doc = "Field `CH30` writer - Write 1, triggers DMA_CHANNEL30"]
pub type CH30_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_SW_CHTRIG_SPEC, bool, O>;
#[doc = "Field `CH31` reader - Write 1, triggers DMA_CHANNEL31"]
pub type CH31_R = crate::BitReader<bool>;
#[doc = "Field `CH31` writer - Write 1, triggers DMA_CHANNEL31"]
pub type CH31_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_SW_CHTRIG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Write 1, triggers DMA_CHANNEL0"]
    #[inline(always)]
    pub fn ch0(&self) -> CH0_R {
        CH0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write 1, triggers DMA_CHANNEL1"]
    #[inline(always)]
    pub fn ch1(&self) -> CH1_R {
        CH1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write 1, triggers DMA_CHANNEL2"]
    #[inline(always)]
    pub fn ch2(&self) -> CH2_R {
        CH2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Write 1, triggers DMA_CHANNEL3"]
    #[inline(always)]
    pub fn ch3(&self) -> CH3_R {
        CH3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write 1, triggers DMA_CHANNEL4"]
    #[inline(always)]
    pub fn ch4(&self) -> CH4_R {
        CH4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Write 1, triggers DMA_CHANNEL5"]
    #[inline(always)]
    pub fn ch5(&self) -> CH5_R {
        CH5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Write 1, triggers DMA_CHANNEL6"]
    #[inline(always)]
    pub fn ch6(&self) -> CH6_R {
        CH6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Write 1, triggers DMA_CHANNEL7"]
    #[inline(always)]
    pub fn ch7(&self) -> CH7_R {
        CH7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Write 1, triggers DMA_CHANNEL8"]
    #[inline(always)]
    pub fn ch8(&self) -> CH8_R {
        CH8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Write 1, triggers DMA_CHANNEL9"]
    #[inline(always)]
    pub fn ch9(&self) -> CH9_R {
        CH9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Write 1, triggers DMA_CHANNEL10"]
    #[inline(always)]
    pub fn ch10(&self) -> CH10_R {
        CH10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Write 1, triggers DMA_CHANNEL11"]
    #[inline(always)]
    pub fn ch11(&self) -> CH11_R {
        CH11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Write 1, triggers DMA_CHANNEL12"]
    #[inline(always)]
    pub fn ch12(&self) -> CH12_R {
        CH12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Write 1, triggers DMA_CHANNEL13"]
    #[inline(always)]
    pub fn ch13(&self) -> CH13_R {
        CH13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Write 1, triggers DMA_CHANNEL14"]
    #[inline(always)]
    pub fn ch14(&self) -> CH14_R {
        CH14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Write 1, triggers DMA_CHANNEL15"]
    #[inline(always)]
    pub fn ch15(&self) -> CH15_R {
        CH15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Write 1, triggers DMA_CHANNEL16"]
    #[inline(always)]
    pub fn ch16(&self) -> CH16_R {
        CH16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Write 1, triggers DMA_CHANNEL17"]
    #[inline(always)]
    pub fn ch17(&self) -> CH17_R {
        CH17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Write 1, triggers DMA_CHANNEL18"]
    #[inline(always)]
    pub fn ch18(&self) -> CH18_R {
        CH18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Write 1, triggers DMA_CHANNEL19"]
    #[inline(always)]
    pub fn ch19(&self) -> CH19_R {
        CH19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Write 1, triggers DMA_CHANNEL20"]
    #[inline(always)]
    pub fn ch20(&self) -> CH20_R {
        CH20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Write 1, triggers DMA_CHANNEL21"]
    #[inline(always)]
    pub fn ch21(&self) -> CH21_R {
        CH21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Write 1, triggers DMA_CHANNEL22"]
    #[inline(always)]
    pub fn ch22(&self) -> CH22_R {
        CH22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Write 1, triggers DMA_CHANNEL23"]
    #[inline(always)]
    pub fn ch23(&self) -> CH23_R {
        CH23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Write 1, triggers DMA_CHANNEL24"]
    #[inline(always)]
    pub fn ch24(&self) -> CH24_R {
        CH24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Write 1, triggers DMA_CHANNEL25"]
    #[inline(always)]
    pub fn ch25(&self) -> CH25_R {
        CH25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Write 1, triggers DMA_CHANNEL26"]
    #[inline(always)]
    pub fn ch26(&self) -> CH26_R {
        CH26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Write 1, triggers DMA_CHANNEL27"]
    #[inline(always)]
    pub fn ch27(&self) -> CH27_R {
        CH27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Write 1, triggers DMA_CHANNEL28"]
    #[inline(always)]
    pub fn ch28(&self) -> CH28_R {
        CH28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Write 1, triggers DMA_CHANNEL29"]
    #[inline(always)]
    pub fn ch29(&self) -> CH29_R {
        CH29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Write 1, triggers DMA_CHANNEL30"]
    #[inline(always)]
    pub fn ch30(&self) -> CH30_R {
        CH30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Write 1, triggers DMA_CHANNEL31"]
    #[inline(always)]
    pub fn ch31(&self) -> CH31_R {
        CH31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write 1, triggers DMA_CHANNEL0"]
    #[inline(always)]
    pub fn ch0(&mut self) -> CH0_W<0> {
        CH0_W::new(self)
    }
    #[doc = "Bit 1 - Write 1, triggers DMA_CHANNEL1"]
    #[inline(always)]
    pub fn ch1(&mut self) -> CH1_W<1> {
        CH1_W::new(self)
    }
    #[doc = "Bit 2 - Write 1, triggers DMA_CHANNEL2"]
    #[inline(always)]
    pub fn ch2(&mut self) -> CH2_W<2> {
        CH2_W::new(self)
    }
    #[doc = "Bit 3 - Write 1, triggers DMA_CHANNEL3"]
    #[inline(always)]
    pub fn ch3(&mut self) -> CH3_W<3> {
        CH3_W::new(self)
    }
    #[doc = "Bit 4 - Write 1, triggers DMA_CHANNEL4"]
    #[inline(always)]
    pub fn ch4(&mut self) -> CH4_W<4> {
        CH4_W::new(self)
    }
    #[doc = "Bit 5 - Write 1, triggers DMA_CHANNEL5"]
    #[inline(always)]
    pub fn ch5(&mut self) -> CH5_W<5> {
        CH5_W::new(self)
    }
    #[doc = "Bit 6 - Write 1, triggers DMA_CHANNEL6"]
    #[inline(always)]
    pub fn ch6(&mut self) -> CH6_W<6> {
        CH6_W::new(self)
    }
    #[doc = "Bit 7 - Write 1, triggers DMA_CHANNEL7"]
    #[inline(always)]
    pub fn ch7(&mut self) -> CH7_W<7> {
        CH7_W::new(self)
    }
    #[doc = "Bit 8 - Write 1, triggers DMA_CHANNEL8"]
    #[inline(always)]
    pub fn ch8(&mut self) -> CH8_W<8> {
        CH8_W::new(self)
    }
    #[doc = "Bit 9 - Write 1, triggers DMA_CHANNEL9"]
    #[inline(always)]
    pub fn ch9(&mut self) -> CH9_W<9> {
        CH9_W::new(self)
    }
    #[doc = "Bit 10 - Write 1, triggers DMA_CHANNEL10"]
    #[inline(always)]
    pub fn ch10(&mut self) -> CH10_W<10> {
        CH10_W::new(self)
    }
    #[doc = "Bit 11 - Write 1, triggers DMA_CHANNEL11"]
    #[inline(always)]
    pub fn ch11(&mut self) -> CH11_W<11> {
        CH11_W::new(self)
    }
    #[doc = "Bit 12 - Write 1, triggers DMA_CHANNEL12"]
    #[inline(always)]
    pub fn ch12(&mut self) -> CH12_W<12> {
        CH12_W::new(self)
    }
    #[doc = "Bit 13 - Write 1, triggers DMA_CHANNEL13"]
    #[inline(always)]
    pub fn ch13(&mut self) -> CH13_W<13> {
        CH13_W::new(self)
    }
    #[doc = "Bit 14 - Write 1, triggers DMA_CHANNEL14"]
    #[inline(always)]
    pub fn ch14(&mut self) -> CH14_W<14> {
        CH14_W::new(self)
    }
    #[doc = "Bit 15 - Write 1, triggers DMA_CHANNEL15"]
    #[inline(always)]
    pub fn ch15(&mut self) -> CH15_W<15> {
        CH15_W::new(self)
    }
    #[doc = "Bit 16 - Write 1, triggers DMA_CHANNEL16"]
    #[inline(always)]
    pub fn ch16(&mut self) -> CH16_W<16> {
        CH16_W::new(self)
    }
    #[doc = "Bit 17 - Write 1, triggers DMA_CHANNEL17"]
    #[inline(always)]
    pub fn ch17(&mut self) -> CH17_W<17> {
        CH17_W::new(self)
    }
    #[doc = "Bit 18 - Write 1, triggers DMA_CHANNEL18"]
    #[inline(always)]
    pub fn ch18(&mut self) -> CH18_W<18> {
        CH18_W::new(self)
    }
    #[doc = "Bit 19 - Write 1, triggers DMA_CHANNEL19"]
    #[inline(always)]
    pub fn ch19(&mut self) -> CH19_W<19> {
        CH19_W::new(self)
    }
    #[doc = "Bit 20 - Write 1, triggers DMA_CHANNEL20"]
    #[inline(always)]
    pub fn ch20(&mut self) -> CH20_W<20> {
        CH20_W::new(self)
    }
    #[doc = "Bit 21 - Write 1, triggers DMA_CHANNEL21"]
    #[inline(always)]
    pub fn ch21(&mut self) -> CH21_W<21> {
        CH21_W::new(self)
    }
    #[doc = "Bit 22 - Write 1, triggers DMA_CHANNEL22"]
    #[inline(always)]
    pub fn ch22(&mut self) -> CH22_W<22> {
        CH22_W::new(self)
    }
    #[doc = "Bit 23 - Write 1, triggers DMA_CHANNEL23"]
    #[inline(always)]
    pub fn ch23(&mut self) -> CH23_W<23> {
        CH23_W::new(self)
    }
    #[doc = "Bit 24 - Write 1, triggers DMA_CHANNEL24"]
    #[inline(always)]
    pub fn ch24(&mut self) -> CH24_W<24> {
        CH24_W::new(self)
    }
    #[doc = "Bit 25 - Write 1, triggers DMA_CHANNEL25"]
    #[inline(always)]
    pub fn ch25(&mut self) -> CH25_W<25> {
        CH25_W::new(self)
    }
    #[doc = "Bit 26 - Write 1, triggers DMA_CHANNEL26"]
    #[inline(always)]
    pub fn ch26(&mut self) -> CH26_W<26> {
        CH26_W::new(self)
    }
    #[doc = "Bit 27 - Write 1, triggers DMA_CHANNEL27"]
    #[inline(always)]
    pub fn ch27(&mut self) -> CH27_W<27> {
        CH27_W::new(self)
    }
    #[doc = "Bit 28 - Write 1, triggers DMA_CHANNEL28"]
    #[inline(always)]
    pub fn ch28(&mut self) -> CH28_W<28> {
        CH28_W::new(self)
    }
    #[doc = "Bit 29 - Write 1, triggers DMA_CHANNEL29"]
    #[inline(always)]
    pub fn ch29(&mut self) -> CH29_W<29> {
        CH29_W::new(self)
    }
    #[doc = "Bit 30 - Write 1, triggers DMA_CHANNEL30"]
    #[inline(always)]
    pub fn ch30(&mut self) -> CH30_W<30> {
        CH30_W::new(self)
    }
    #[doc = "Bit 31 - Write 1, triggers DMA_CHANNEL31"]
    #[inline(always)]
    pub fn ch31(&mut self) -> CH31_W<31> {
        CH31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Software Channel Trigger Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_sw_chtrig](index.html) module"]
pub struct DMA_SW_CHTRIG_SPEC;
impl crate::RegisterSpec for DMA_SW_CHTRIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_sw_chtrig::R](R) reader structure"]
impl crate::Readable for DMA_SW_CHTRIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_sw_chtrig::W](W) writer structure"]
impl crate::Writable for DMA_SW_CHTRIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_SW_CHTRIG to value 0"]
impl crate::Resettable for DMA_SW_CHTRIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
