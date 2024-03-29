#[doc = "Register `TAxCTL` reader"]
pub struct R(crate::R<TAXCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TAXCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TAXCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TAXCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TAxCTL` writer"]
pub struct W(crate::W<TAXCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TAXCTL_SPEC>;
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
impl From<crate::W<TAXCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TAXCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "TimerA interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAIFG_A {
    #[doc = "0: No interrupt pending"]
    TAIFG_0 = 0,
    #[doc = "1: Interrupt pending"]
    TAIFG_1 = 1,
}
impl From<TAIFG_A> for bool {
    #[inline(always)]
    fn from(variant: TAIFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAIFG` reader - TimerA interrupt flag"]
pub type TAIFG_R = crate::BitReader<TAIFG_A>;
impl TAIFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TAIFG_A {
        match self.bits {
            false => TAIFG_A::TAIFG_0,
            true => TAIFG_A::TAIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `TAIFG_0`"]
    #[inline(always)]
    pub fn is_taifg_0(&self) -> bool {
        *self == TAIFG_A::TAIFG_0
    }
    #[doc = "Checks if the value of the field is `TAIFG_1`"]
    #[inline(always)]
    pub fn is_taifg_1(&self) -> bool {
        *self == TAIFG_A::TAIFG_1
    }
}
#[doc = "Field `TAIFG` writer - TimerA interrupt flag"]
pub type TAIFG_W<'a, const O: u8> = crate::BitWriter<'a, u16, TAXCTL_SPEC, TAIFG_A, O>;
impl<'a, const O: u8> TAIFG_W<'a, O> {
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn taifg_0(self) -> &'a mut W {
        self.variant(TAIFG_A::TAIFG_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn taifg_1(self) -> &'a mut W {
        self.variant(TAIFG_A::TAIFG_1)
    }
}
#[doc = "TimerA interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAIE_A {
    #[doc = "0: Interrupt disabled"]
    TAIE_0 = 0,
    #[doc = "1: Interrupt enabled"]
    TAIE_1 = 1,
}
impl From<TAIE_A> for bool {
    #[inline(always)]
    fn from(variant: TAIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAIE` reader - TimerA interrupt enable"]
pub type TAIE_R = crate::BitReader<TAIE_A>;
impl TAIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TAIE_A {
        match self.bits {
            false => TAIE_A::TAIE_0,
            true => TAIE_A::TAIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `TAIE_0`"]
    #[inline(always)]
    pub fn is_taie_0(&self) -> bool {
        *self == TAIE_A::TAIE_0
    }
    #[doc = "Checks if the value of the field is `TAIE_1`"]
    #[inline(always)]
    pub fn is_taie_1(&self) -> bool {
        *self == TAIE_A::TAIE_1
    }
}
#[doc = "Field `TAIE` writer - TimerA interrupt enable"]
pub type TAIE_W<'a, const O: u8> = crate::BitWriter<'a, u16, TAXCTL_SPEC, TAIE_A, O>;
impl<'a, const O: u8> TAIE_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn taie_0(self) -> &'a mut W {
        self.variant(TAIE_A::TAIE_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn taie_1(self) -> &'a mut W {
        self.variant(TAIE_A::TAIE_1)
    }
}
#[doc = "Field `TACLR` reader - TimerA clear"]
pub type TACLR_R = crate::BitReader<bool>;
#[doc = "Field `TACLR` writer - TimerA clear"]
pub type TACLR_W<'a, const O: u8> = crate::BitWriter<'a, u16, TAXCTL_SPEC, bool, O>;
#[doc = "Mode control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MC_A {
    #[doc = "0: Stop mode: Timer is halted"]
    MC_0 = 0,
    #[doc = "1: Up mode: Timer counts up to TAxCCR0"]
    MC_1 = 1,
    #[doc = "2: Continuous mode: Timer counts up to 0FFFFh"]
    MC_2 = 2,
    #[doc = "3: Up/down mode: Timer counts up to TAxCCR0 then down to 0000h"]
    MC_3 = 3,
}
impl From<MC_A> for u8 {
    #[inline(always)]
    fn from(variant: MC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MC` reader - Mode control"]
pub type MC_R = crate::FieldReader<u8, MC_A>;
impl MC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MC_A {
        match self.bits {
            0 => MC_A::MC_0,
            1 => MC_A::MC_1,
            2 => MC_A::MC_2,
            3 => MC_A::MC_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MC_0`"]
    #[inline(always)]
    pub fn is_mc_0(&self) -> bool {
        *self == MC_A::MC_0
    }
    #[doc = "Checks if the value of the field is `MC_1`"]
    #[inline(always)]
    pub fn is_mc_1(&self) -> bool {
        *self == MC_A::MC_1
    }
    #[doc = "Checks if the value of the field is `MC_2`"]
    #[inline(always)]
    pub fn is_mc_2(&self) -> bool {
        *self == MC_A::MC_2
    }
    #[doc = "Checks if the value of the field is `MC_3`"]
    #[inline(always)]
    pub fn is_mc_3(&self) -> bool {
        *self == MC_A::MC_3
    }
}
#[doc = "Field `MC` writer - Mode control"]
pub type MC_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u16, TAXCTL_SPEC, u8, MC_A, 2, O>;
impl<'a, const O: u8> MC_W<'a, O> {
    #[doc = "Stop mode: Timer is halted"]
    #[inline(always)]
    pub fn mc_0(self) -> &'a mut W {
        self.variant(MC_A::MC_0)
    }
    #[doc = "Up mode: Timer counts up to TAxCCR0"]
    #[inline(always)]
    pub fn mc_1(self) -> &'a mut W {
        self.variant(MC_A::MC_1)
    }
    #[doc = "Continuous mode: Timer counts up to 0FFFFh"]
    #[inline(always)]
    pub fn mc_2(self) -> &'a mut W {
        self.variant(MC_A::MC_2)
    }
    #[doc = "Up/down mode: Timer counts up to TAxCCR0 then down to 0000h"]
    #[inline(always)]
    pub fn mc_3(self) -> &'a mut W {
        self.variant(MC_A::MC_3)
    }
}
#[doc = "Input divider\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ID_A {
    #[doc = "0: /1"]
    ID_0 = 0,
    #[doc = "1: /2"]
    ID_1 = 1,
    #[doc = "2: /4"]
    ID_2 = 2,
    #[doc = "3: /8"]
    ID_3 = 3,
}
impl From<ID_A> for u8 {
    #[inline(always)]
    fn from(variant: ID_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ID` reader - Input divider"]
pub type ID_R = crate::FieldReader<u8, ID_A>;
impl ID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ID_A {
        match self.bits {
            0 => ID_A::ID_0,
            1 => ID_A::ID_1,
            2 => ID_A::ID_2,
            3 => ID_A::ID_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ID_0`"]
    #[inline(always)]
    pub fn is_id_0(&self) -> bool {
        *self == ID_A::ID_0
    }
    #[doc = "Checks if the value of the field is `ID_1`"]
    #[inline(always)]
    pub fn is_id_1(&self) -> bool {
        *self == ID_A::ID_1
    }
    #[doc = "Checks if the value of the field is `ID_2`"]
    #[inline(always)]
    pub fn is_id_2(&self) -> bool {
        *self == ID_A::ID_2
    }
    #[doc = "Checks if the value of the field is `ID_3`"]
    #[inline(always)]
    pub fn is_id_3(&self) -> bool {
        *self == ID_A::ID_3
    }
}
#[doc = "Field `ID` writer - Input divider"]
pub type ID_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u16, TAXCTL_SPEC, u8, ID_A, 2, O>;
impl<'a, const O: u8> ID_W<'a, O> {
    #[doc = "/1"]
    #[inline(always)]
    pub fn id_0(self) -> &'a mut W {
        self.variant(ID_A::ID_0)
    }
    #[doc = "/2"]
    #[inline(always)]
    pub fn id_1(self) -> &'a mut W {
        self.variant(ID_A::ID_1)
    }
    #[doc = "/4"]
    #[inline(always)]
    pub fn id_2(self) -> &'a mut W {
        self.variant(ID_A::ID_2)
    }
    #[doc = "/8"]
    #[inline(always)]
    pub fn id_3(self) -> &'a mut W {
        self.variant(ID_A::ID_3)
    }
}
#[doc = "TimerA clock source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TASSEL_A {
    #[doc = "0: TAxCLK"]
    TASSEL_0 = 0,
    #[doc = "1: ACLK"]
    TASSEL_1 = 1,
    #[doc = "2: SMCLK"]
    TASSEL_2 = 2,
    #[doc = "3: INCLK"]
    TASSEL_3 = 3,
}
impl From<TASSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: TASSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TASSEL` reader - TimerA clock source select"]
pub type TASSEL_R = crate::FieldReader<u8, TASSEL_A>;
impl TASSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TASSEL_A {
        match self.bits {
            0 => TASSEL_A::TASSEL_0,
            1 => TASSEL_A::TASSEL_1,
            2 => TASSEL_A::TASSEL_2,
            3 => TASSEL_A::TASSEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TASSEL_0`"]
    #[inline(always)]
    pub fn is_tassel_0(&self) -> bool {
        *self == TASSEL_A::TASSEL_0
    }
    #[doc = "Checks if the value of the field is `TASSEL_1`"]
    #[inline(always)]
    pub fn is_tassel_1(&self) -> bool {
        *self == TASSEL_A::TASSEL_1
    }
    #[doc = "Checks if the value of the field is `TASSEL_2`"]
    #[inline(always)]
    pub fn is_tassel_2(&self) -> bool {
        *self == TASSEL_A::TASSEL_2
    }
    #[doc = "Checks if the value of the field is `TASSEL_3`"]
    #[inline(always)]
    pub fn is_tassel_3(&self) -> bool {
        *self == TASSEL_A::TASSEL_3
    }
}
#[doc = "Field `TASSEL` writer - TimerA clock source select"]
pub type TASSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, TAXCTL_SPEC, u8, TASSEL_A, 2, O>;
impl<'a, const O: u8> TASSEL_W<'a, O> {
    #[doc = "TAxCLK"]
    #[inline(always)]
    pub fn tassel_0(self) -> &'a mut W {
        self.variant(TASSEL_A::TASSEL_0)
    }
    #[doc = "ACLK"]
    #[inline(always)]
    pub fn tassel_1(self) -> &'a mut W {
        self.variant(TASSEL_A::TASSEL_1)
    }
    #[doc = "SMCLK"]
    #[inline(always)]
    pub fn tassel_2(self) -> &'a mut W {
        self.variant(TASSEL_A::TASSEL_2)
    }
    #[doc = "INCLK"]
    #[inline(always)]
    pub fn tassel_3(self) -> &'a mut W {
        self.variant(TASSEL_A::TASSEL_3)
    }
}
impl R {
    #[doc = "Bit 0 - TimerA interrupt flag"]
    #[inline(always)]
    pub fn taifg(&self) -> TAIFG_R {
        TAIFG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TimerA interrupt enable"]
    #[inline(always)]
    pub fn taie(&self) -> TAIE_R {
        TAIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TimerA clear"]
    #[inline(always)]
    pub fn taclr(&self) -> TACLR_R {
        TACLR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Mode control"]
    #[inline(always)]
    pub fn mc(&self) -> MC_R {
        MC_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Input divider"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - TimerA clock source select"]
    #[inline(always)]
    pub fn tassel(&self) -> TASSEL_R {
        TASSEL_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - TimerA interrupt flag"]
    #[inline(always)]
    pub fn taifg(&mut self) -> TAIFG_W<0> {
        TAIFG_W::new(self)
    }
    #[doc = "Bit 1 - TimerA interrupt enable"]
    #[inline(always)]
    pub fn taie(&mut self) -> TAIE_W<1> {
        TAIE_W::new(self)
    }
    #[doc = "Bit 2 - TimerA clear"]
    #[inline(always)]
    pub fn taclr(&mut self) -> TACLR_W<2> {
        TACLR_W::new(self)
    }
    #[doc = "Bits 4:5 - Mode control"]
    #[inline(always)]
    pub fn mc(&mut self) -> MC_W<4> {
        MC_W::new(self)
    }
    #[doc = "Bits 6:7 - Input divider"]
    #[inline(always)]
    pub fn id(&mut self) -> ID_W<6> {
        ID_W::new(self)
    }
    #[doc = "Bits 8:9 - TimerA clock source select"]
    #[inline(always)]
    pub fn tassel(&mut self) -> TASSEL_W<8> {
        TASSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TimerAx Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tax_ctl](index.html) module"]
pub struct TAXCTL_SPEC;
impl crate::RegisterSpec for TAXCTL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [tax_ctl::R](R) reader structure"]
impl crate::Readable for TAXCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tax_ctl::W](W) writer structure"]
impl crate::Writable for TAXCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TAxCTL to value 0"]
impl crate::Resettable for TAXCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
