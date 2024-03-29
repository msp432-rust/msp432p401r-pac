#[doc = "Register `CSCLRIFG` writer"]
pub struct W(crate::W<CSCLRIFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSCLRIFG_SPEC>;
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
impl From<crate::W<CSCLRIFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSCLRIFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Clear LFXT oscillator fault interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLR_LFXTIFG_AW {
    #[doc = "0: No effect"]
    CLR_LFXTIFG_0 = 0,
    #[doc = "1: Clear pending interrupt flag"]
    CLR_LFXTIFG_1 = 1,
}
impl From<CLR_LFXTIFG_AW> for bool {
    #[inline(always)]
    fn from(variant: CLR_LFXTIFG_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLR_LFXTIFG` writer - Clear LFXT oscillator fault interrupt flag"]
pub type CLR_LFXTIFG_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CSCLRIFG_SPEC, CLR_LFXTIFG_AW, O>;
impl<'a, const O: u8> CLR_LFXTIFG_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn clr_lfxtifg_0(self) -> &'a mut W {
        self.variant(CLR_LFXTIFG_AW::CLR_LFXTIFG_0)
    }
    #[doc = "Clear pending interrupt flag"]
    #[inline(always)]
    pub fn clr_lfxtifg_1(self) -> &'a mut W {
        self.variant(CLR_LFXTIFG_AW::CLR_LFXTIFG_1)
    }
}
#[doc = "Clear HFXT oscillator fault interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLR_HFXTIFG_AW {
    #[doc = "0: No effect"]
    CLR_HFXTIFG_0 = 0,
    #[doc = "1: Clear pending interrupt flag"]
    CLR_HFXTIFG_1 = 1,
}
impl From<CLR_HFXTIFG_AW> for bool {
    #[inline(always)]
    fn from(variant: CLR_HFXTIFG_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLR_HFXTIFG` writer - Clear HFXT oscillator fault interrupt flag"]
pub type CLR_HFXTIFG_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CSCLRIFG_SPEC, CLR_HFXTIFG_AW, O>;
impl<'a, const O: u8> CLR_HFXTIFG_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn clr_hfxtifg_0(self) -> &'a mut W {
        self.variant(CLR_HFXTIFG_AW::CLR_HFXTIFG_0)
    }
    #[doc = "Clear pending interrupt flag"]
    #[inline(always)]
    pub fn clr_hfxtifg_1(self) -> &'a mut W {
        self.variant(CLR_HFXTIFG_AW::CLR_HFXTIFG_1)
    }
}
#[doc = "Clear HFXT2 oscillator fault interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLR_HFXT2IFG_AW {
    #[doc = "0: No effect"]
    CLR_HFXT2IFG_0 = 0,
    #[doc = "1: Clear pending interrupt flag"]
    CLR_HFXT2IFG_1 = 1,
}
impl From<CLR_HFXT2IFG_AW> for bool {
    #[inline(always)]
    fn from(variant: CLR_HFXT2IFG_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLR_HFXT2IFG` writer - Clear HFXT2 oscillator fault interrupt flag"]
pub type CLR_HFXT2IFG_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CSCLRIFG_SPEC, CLR_HFXT2IFG_AW, O>;
impl<'a, const O: u8> CLR_HFXT2IFG_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn clr_hfxt2ifg_0(self) -> &'a mut W {
        self.variant(CLR_HFXT2IFG_AW::CLR_HFXT2IFG_0)
    }
    #[doc = "Clear pending interrupt flag"]
    #[inline(always)]
    pub fn clr_hfxt2ifg_1(self) -> &'a mut W {
        self.variant(CLR_HFXT2IFG_AW::CLR_HFXT2IFG_1)
    }
}
#[doc = "Clear DCO external resistor open circuit fault interrupt flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLR_DCOR_OPNIFG_AW {
    #[doc = "0: No effect"]
    CLR_DCOR_OPNIFG_0 = 0,
    #[doc = "1: Clear pending interrupt flag"]
    CLR_DCOR_OPNIFG_1 = 1,
}
impl From<CLR_DCOR_OPNIFG_AW> for bool {
    #[inline(always)]
    fn from(variant: CLR_DCOR_OPNIFG_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLR_DCOR_OPNIFG` writer - Clear DCO external resistor open circuit fault interrupt flag."]
pub type CLR_DCOR_OPNIFG_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CSCLRIFG_SPEC, CLR_DCOR_OPNIFG_AW, O>;
impl<'a, const O: u8> CLR_DCOR_OPNIFG_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn clr_dcor_opnifg_0(self) -> &'a mut W {
        self.variant(CLR_DCOR_OPNIFG_AW::CLR_DCOR_OPNIFG_0)
    }
    #[doc = "Clear pending interrupt flag"]
    #[inline(always)]
    pub fn clr_dcor_opnifg_1(self) -> &'a mut W {
        self.variant(CLR_DCOR_OPNIFG_AW::CLR_DCOR_OPNIFG_1)
    }
}
#[doc = "REFCNT period counter clear interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLR_CALIFG_AW {
    #[doc = "0: No effect"]
    CLR_CALIFG_0 = 0,
    #[doc = "1: Clear pending interrupt flag"]
    CLR_CALIFG_1 = 1,
}
impl From<CLR_CALIFG_AW> for bool {
    #[inline(always)]
    fn from(variant: CLR_CALIFG_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLR_CALIFG` writer - REFCNT period counter clear interrupt flag"]
pub type CLR_CALIFG_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSCLRIFG_SPEC, CLR_CALIFG_AW, O>;
impl<'a, const O: u8> CLR_CALIFG_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn clr_califg_0(self) -> &'a mut W {
        self.variant(CLR_CALIFG_AW::CLR_CALIFG_0)
    }
    #[doc = "Clear pending interrupt flag"]
    #[inline(always)]
    pub fn clr_califg_1(self) -> &'a mut W {
        self.variant(CLR_CALIFG_AW::CLR_CALIFG_1)
    }
}
#[doc = "Start fault counter clear interrupt flag LFXT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLR_FCNTLFIFG_AW {
    #[doc = "0: No effect"]
    CLR_FCNTLFIFG_0 = 0,
    #[doc = "1: Clear pending interrupt flag"]
    CLR_FCNTLFIFG_1 = 1,
}
impl From<CLR_FCNTLFIFG_AW> for bool {
    #[inline(always)]
    fn from(variant: CLR_FCNTLFIFG_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLR_FCNTLFIFG` writer - Start fault counter clear interrupt flag LFXT"]
pub type CLR_FCNTLFIFG_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CSCLRIFG_SPEC, CLR_FCNTLFIFG_AW, O>;
impl<'a, const O: u8> CLR_FCNTLFIFG_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn clr_fcntlfifg_0(self) -> &'a mut W {
        self.variant(CLR_FCNTLFIFG_AW::CLR_FCNTLFIFG_0)
    }
    #[doc = "Clear pending interrupt flag"]
    #[inline(always)]
    pub fn clr_fcntlfifg_1(self) -> &'a mut W {
        self.variant(CLR_FCNTLFIFG_AW::CLR_FCNTLFIFG_1)
    }
}
#[doc = "Start fault counter clear interrupt flag HFXT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLR_FCNTHFIFG_AW {
    #[doc = "0: No effect"]
    CLR_FCNTHFIFG_0 = 0,
    #[doc = "1: Clear pending interrupt flag"]
    CLR_FCNTHFIFG_1 = 1,
}
impl From<CLR_FCNTHFIFG_AW> for bool {
    #[inline(always)]
    fn from(variant: CLR_FCNTHFIFG_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLR_FCNTHFIFG` writer - Start fault counter clear interrupt flag HFXT"]
pub type CLR_FCNTHFIFG_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CSCLRIFG_SPEC, CLR_FCNTHFIFG_AW, O>;
impl<'a, const O: u8> CLR_FCNTHFIFG_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn clr_fcnthfifg_0(self) -> &'a mut W {
        self.variant(CLR_FCNTHFIFG_AW::CLR_FCNTHFIFG_0)
    }
    #[doc = "Clear pending interrupt flag"]
    #[inline(always)]
    pub fn clr_fcnthfifg_1(self) -> &'a mut W {
        self.variant(CLR_FCNTHFIFG_AW::CLR_FCNTHFIFG_1)
    }
}
#[doc = "Start fault counter clear interrupt flag HFXT2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLR_FCNTHF2IFG_AW {
    #[doc = "0: No effect"]
    CLR_FCNTHF2IFG_0 = 0,
    #[doc = "1: Clear pending interrupt flag"]
    CLR_FCNTHF2IFG_1 = 1,
}
impl From<CLR_FCNTHF2IFG_AW> for bool {
    #[inline(always)]
    fn from(variant: CLR_FCNTHF2IFG_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLR_FCNTHF2IFG` writer - Start fault counter clear interrupt flag HFXT2"]
pub type CLR_FCNTHF2IFG_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CSCLRIFG_SPEC, CLR_FCNTHF2IFG_AW, O>;
impl<'a, const O: u8> CLR_FCNTHF2IFG_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn clr_fcnthf2ifg_0(self) -> &'a mut W {
        self.variant(CLR_FCNTHF2IFG_AW::CLR_FCNTHF2IFG_0)
    }
    #[doc = "Clear pending interrupt flag"]
    #[inline(always)]
    pub fn clr_fcnthf2ifg_1(self) -> &'a mut W {
        self.variant(CLR_FCNTHF2IFG_AW::CLR_FCNTHF2IFG_1)
    }
}
#[doc = "PLL out-of-lock clear interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLR_PLLOOLIFG_AW {
    #[doc = "0: No effect"]
    CLR_PLLOOLIFG_0 = 0,
    #[doc = "1: Clear pending interrupt flag"]
    CLR_PLLOOLIFG_1 = 1,
}
impl From<CLR_PLLOOLIFG_AW> for bool {
    #[inline(always)]
    fn from(variant: CLR_PLLOOLIFG_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLR_PLLOOLIFG` writer - PLL out-of-lock clear interrupt flag"]
pub type CLR_PLLOOLIFG_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CSCLRIFG_SPEC, CLR_PLLOOLIFG_AW, O>;
impl<'a, const O: u8> CLR_PLLOOLIFG_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn clr_plloolifg_0(self) -> &'a mut W {
        self.variant(CLR_PLLOOLIFG_AW::CLR_PLLOOLIFG_0)
    }
    #[doc = "Clear pending interrupt flag"]
    #[inline(always)]
    pub fn clr_plloolifg_1(self) -> &'a mut W {
        self.variant(CLR_PLLOOLIFG_AW::CLR_PLLOOLIFG_1)
    }
}
#[doc = "PLL loss-of-signal clear interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLR_PLLLOSIFG_AW {
    #[doc = "0: No effect"]
    CLR_PLLLOSIFG_0 = 0,
    #[doc = "1: Clear pending interrupt flag"]
    CLR_PLLLOSIFG_1 = 1,
}
impl From<CLR_PLLLOSIFG_AW> for bool {
    #[inline(always)]
    fn from(variant: CLR_PLLLOSIFG_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLR_PLLLOSIFG` writer - PLL loss-of-signal clear interrupt flag"]
pub type CLR_PLLLOSIFG_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CSCLRIFG_SPEC, CLR_PLLLOSIFG_AW, O>;
impl<'a, const O: u8> CLR_PLLLOSIFG_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn clr_plllosifg_0(self) -> &'a mut W {
        self.variant(CLR_PLLLOSIFG_AW::CLR_PLLLOSIFG_0)
    }
    #[doc = "Clear pending interrupt flag"]
    #[inline(always)]
    pub fn clr_plllosifg_1(self) -> &'a mut W {
        self.variant(CLR_PLLLOSIFG_AW::CLR_PLLLOSIFG_1)
    }
}
#[doc = "PLL out-of-range clear interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLR_PLLOORIFG_AW {
    #[doc = "0: No effect"]
    CLR_PLLOORIFG_0 = 0,
    #[doc = "1: Clear pending interrupt flag"]
    CLR_PLLOORIFG_1 = 1,
}
impl From<CLR_PLLOORIFG_AW> for bool {
    #[inline(always)]
    fn from(variant: CLR_PLLOORIFG_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLR_PLLOORIFG` writer - PLL out-of-range clear interrupt flag"]
pub type CLR_PLLOORIFG_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CSCLRIFG_SPEC, CLR_PLLOORIFG_AW, O>;
impl<'a, const O: u8> CLR_PLLOORIFG_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn clr_plloorifg_0(self) -> &'a mut W {
        self.variant(CLR_PLLOORIFG_AW::CLR_PLLOORIFG_0)
    }
    #[doc = "Clear pending interrupt flag"]
    #[inline(always)]
    pub fn clr_plloorifg_1(self) -> &'a mut W {
        self.variant(CLR_PLLOORIFG_AW::CLR_PLLOORIFG_1)
    }
}
impl W {
    #[doc = "Bit 0 - Clear LFXT oscillator fault interrupt flag"]
    #[inline(always)]
    pub fn clr_lfxtifg(&mut self) -> CLR_LFXTIFG_W<0> {
        CLR_LFXTIFG_W::new(self)
    }
    #[doc = "Bit 1 - Clear HFXT oscillator fault interrupt flag"]
    #[inline(always)]
    pub fn clr_hfxtifg(&mut self) -> CLR_HFXTIFG_W<1> {
        CLR_HFXTIFG_W::new(self)
    }
    #[doc = "Bit 2 - Clear HFXT2 oscillator fault interrupt flag"]
    #[inline(always)]
    pub fn clr_hfxt2ifg(&mut self) -> CLR_HFXT2IFG_W<2> {
        CLR_HFXT2IFG_W::new(self)
    }
    #[doc = "Bit 6 - Clear DCO external resistor open circuit fault interrupt flag."]
    #[inline(always)]
    pub fn clr_dcor_opnifg(&mut self) -> CLR_DCOR_OPNIFG_W<6> {
        CLR_DCOR_OPNIFG_W::new(self)
    }
    #[doc = "Bit 15 - REFCNT period counter clear interrupt flag"]
    #[inline(always)]
    pub fn clr_califg(&mut self) -> CLR_CALIFG_W<15> {
        CLR_CALIFG_W::new(self)
    }
    #[doc = "Bit 8 - Start fault counter clear interrupt flag LFXT"]
    #[inline(always)]
    pub fn clr_fcntlfifg(&mut self) -> CLR_FCNTLFIFG_W<8> {
        CLR_FCNTLFIFG_W::new(self)
    }
    #[doc = "Bit 9 - Start fault counter clear interrupt flag HFXT"]
    #[inline(always)]
    pub fn clr_fcnthfifg(&mut self) -> CLR_FCNTHFIFG_W<9> {
        CLR_FCNTHFIFG_W::new(self)
    }
    #[doc = "Bit 10 - Start fault counter clear interrupt flag HFXT2"]
    #[inline(always)]
    pub fn clr_fcnthf2ifg(&mut self) -> CLR_FCNTHF2IFG_W<10> {
        CLR_FCNTHF2IFG_W::new(self)
    }
    #[doc = "Bit 12 - PLL out-of-lock clear interrupt flag"]
    #[inline(always)]
    pub fn clr_plloolifg(&mut self) -> CLR_PLLOOLIFG_W<12> {
        CLR_PLLOOLIFG_W::new(self)
    }
    #[doc = "Bit 13 - PLL loss-of-signal clear interrupt flag"]
    #[inline(always)]
    pub fn clr_plllosifg(&mut self) -> CLR_PLLLOSIFG_W<13> {
        CLR_PLLLOSIFG_W::new(self)
    }
    #[doc = "Bit 14 - PLL out-of-range clear interrupt flag"]
    #[inline(always)]
    pub fn clr_plloorifg(&mut self) -> CLR_PLLOORIFG_W<14> {
        CLR_PLLOORIFG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clear Interrupt Flag Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csclrifg](index.html) module"]
pub struct CSCLRIFG_SPEC;
impl crate::RegisterSpec for CSCLRIFG_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [csclrifg::W](W) writer structure"]
impl crate::Writable for CSCLRIFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CSCLRIFG to value 0"]
impl crate::Resettable for CSCLRIFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
