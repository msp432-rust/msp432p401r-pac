#[doc = "Register `SYS_NMI_CTLSTAT` reader"]
pub struct R(crate::R<SYS_NMI_CTLSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYS_NMI_CTLSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYS_NMI_CTLSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYS_NMI_CTLSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYS_NMI_CTLSTAT` writer"]
pub struct W(crate::W<SYS_NMI_CTLSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYS_NMI_CTLSTAT_SPEC>;
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
impl From<crate::W<SYS_NMI_CTLSTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYS_NMI_CTLSTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "CS interrupt as a source of NMI\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CS_SRC_A {
    #[doc = "0: Disables CS interrupt as a source of NMI"]
    CS_SRC_0 = 0,
    #[doc = "1: Enables CS interrupt as a source of NMI"]
    CS_SRC_1 = 1,
}
impl From<CS_SRC_A> for bool {
    #[inline(always)]
    fn from(variant: CS_SRC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CS_SRC` reader - CS interrupt as a source of NMI"]
pub type CS_SRC_R = crate::BitReader<CS_SRC_A>;
impl CS_SRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CS_SRC_A {
        match self.bits {
            false => CS_SRC_A::CS_SRC_0,
            true => CS_SRC_A::CS_SRC_1,
        }
    }
    #[doc = "Checks if the value of the field is `CS_SRC_0`"]
    #[inline(always)]
    pub fn is_cs_src_0(&self) -> bool {
        *self == CS_SRC_A::CS_SRC_0
    }
    #[doc = "Checks if the value of the field is `CS_SRC_1`"]
    #[inline(always)]
    pub fn is_cs_src_1(&self) -> bool {
        *self == CS_SRC_A::CS_SRC_1
    }
}
#[doc = "Field `CS_SRC` writer - CS interrupt as a source of NMI"]
pub type CS_SRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYS_NMI_CTLSTAT_SPEC, CS_SRC_A, O>;
impl<'a, const O: u8> CS_SRC_W<'a, O> {
    #[doc = "Disables CS interrupt as a source of NMI"]
    #[inline(always)]
    pub fn cs_src_0(self) -> &'a mut W {
        self.variant(CS_SRC_A::CS_SRC_0)
    }
    #[doc = "Enables CS interrupt as a source of NMI"]
    #[inline(always)]
    pub fn cs_src_1(self) -> &'a mut W {
        self.variant(CS_SRC_A::CS_SRC_1)
    }
}
#[doc = "PSS interrupt as a source of NMI\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PSS_SRC_A {
    #[doc = "0: Disables the PSS interrupt as a source of NMI"]
    PSS_SRC_0 = 0,
    #[doc = "1: Enables the PSS interrupt as a source of NMI"]
    PSS_SRC_1 = 1,
}
impl From<PSS_SRC_A> for bool {
    #[inline(always)]
    fn from(variant: PSS_SRC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PSS_SRC` reader - PSS interrupt as a source of NMI"]
pub type PSS_SRC_R = crate::BitReader<PSS_SRC_A>;
impl PSS_SRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSS_SRC_A {
        match self.bits {
            false => PSS_SRC_A::PSS_SRC_0,
            true => PSS_SRC_A::PSS_SRC_1,
        }
    }
    #[doc = "Checks if the value of the field is `PSS_SRC_0`"]
    #[inline(always)]
    pub fn is_pss_src_0(&self) -> bool {
        *self == PSS_SRC_A::PSS_SRC_0
    }
    #[doc = "Checks if the value of the field is `PSS_SRC_1`"]
    #[inline(always)]
    pub fn is_pss_src_1(&self) -> bool {
        *self == PSS_SRC_A::PSS_SRC_1
    }
}
#[doc = "Field `PSS_SRC` writer - PSS interrupt as a source of NMI"]
pub type PSS_SRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYS_NMI_CTLSTAT_SPEC, PSS_SRC_A, O>;
impl<'a, const O: u8> PSS_SRC_W<'a, O> {
    #[doc = "Disables the PSS interrupt as a source of NMI"]
    #[inline(always)]
    pub fn pss_src_0(self) -> &'a mut W {
        self.variant(PSS_SRC_A::PSS_SRC_0)
    }
    #[doc = "Enables the PSS interrupt as a source of NMI"]
    #[inline(always)]
    pub fn pss_src_1(self) -> &'a mut W {
        self.variant(PSS_SRC_A::PSS_SRC_1)
    }
}
#[doc = "PCM interrupt as a source of NMI\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCM_SRC_A {
    #[doc = "0: Disbles the PCM interrupt as a source of NMI"]
    PCM_SRC_0 = 0,
    #[doc = "1: Enables the PCM interrupt as a source of NMI"]
    PCM_SRC_1 = 1,
}
impl From<PCM_SRC_A> for bool {
    #[inline(always)]
    fn from(variant: PCM_SRC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PCM_SRC` reader - PCM interrupt as a source of NMI"]
pub type PCM_SRC_R = crate::BitReader<PCM_SRC_A>;
impl PCM_SRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCM_SRC_A {
        match self.bits {
            false => PCM_SRC_A::PCM_SRC_0,
            true => PCM_SRC_A::PCM_SRC_1,
        }
    }
    #[doc = "Checks if the value of the field is `PCM_SRC_0`"]
    #[inline(always)]
    pub fn is_pcm_src_0(&self) -> bool {
        *self == PCM_SRC_A::PCM_SRC_0
    }
    #[doc = "Checks if the value of the field is `PCM_SRC_1`"]
    #[inline(always)]
    pub fn is_pcm_src_1(&self) -> bool {
        *self == PCM_SRC_A::PCM_SRC_1
    }
}
#[doc = "Field `PCM_SRC` writer - PCM interrupt as a source of NMI"]
pub type PCM_SRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYS_NMI_CTLSTAT_SPEC, PCM_SRC_A, O>;
impl<'a, const O: u8> PCM_SRC_W<'a, O> {
    #[doc = "Disbles the PCM interrupt as a source of NMI"]
    #[inline(always)]
    pub fn pcm_src_0(self) -> &'a mut W {
        self.variant(PCM_SRC_A::PCM_SRC_0)
    }
    #[doc = "Enables the PCM interrupt as a source of NMI"]
    #[inline(always)]
    pub fn pcm_src_1(self) -> &'a mut W {
        self.variant(PCM_SRC_A::PCM_SRC_1)
    }
}
#[doc = "RSTn/NMI pin configuration Note: When the device enters LPM3/LPM4 modes of operation, the functionality selected by this bit is retained. If selected as an NMI, activity on this pin in LPM3/LPM4 wakes the device and processes the interrupt, without causing a POR. If selected as a Reset, activity on this pin in LPM3/LPM4 causes a device-level POR When the device enters LPM3.5/LPM4.5 modes of operation, this bit is always cleared to 0. In other words, the RSTn/NMI pin always assumes a reset functionality in LPM3.5/LPM4.5 modes.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN_SRC_A {
    #[doc = "0: Configures the RSTn_NMI pin as a source of POR Class Reset"]
    PIN_SRC_0 = 0,
    #[doc = "1: Configures the RSTn_NMI pin as a source of NMI"]
    PIN_SRC_1 = 1,
}
impl From<PIN_SRC_A> for bool {
    #[inline(always)]
    fn from(variant: PIN_SRC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN_SRC` reader - RSTn/NMI pin configuration Note: When the device enters LPM3/LPM4 modes of operation, the functionality selected by this bit is retained. If selected as an NMI, activity on this pin in LPM3/LPM4 wakes the device and processes the interrupt, without causing a POR. If selected as a Reset, activity on this pin in LPM3/LPM4 causes a device-level POR When the device enters LPM3.5/LPM4.5 modes of operation, this bit is always cleared to 0. In other words, the RSTn/NMI pin always assumes a reset functionality in LPM3.5/LPM4.5 modes."]
pub type PIN_SRC_R = crate::BitReader<PIN_SRC_A>;
impl PIN_SRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN_SRC_A {
        match self.bits {
            false => PIN_SRC_A::PIN_SRC_0,
            true => PIN_SRC_A::PIN_SRC_1,
        }
    }
    #[doc = "Checks if the value of the field is `PIN_SRC_0`"]
    #[inline(always)]
    pub fn is_pin_src_0(&self) -> bool {
        *self == PIN_SRC_A::PIN_SRC_0
    }
    #[doc = "Checks if the value of the field is `PIN_SRC_1`"]
    #[inline(always)]
    pub fn is_pin_src_1(&self) -> bool {
        *self == PIN_SRC_A::PIN_SRC_1
    }
}
#[doc = "Field `PIN_SRC` writer - RSTn/NMI pin configuration Note: When the device enters LPM3/LPM4 modes of operation, the functionality selected by this bit is retained. If selected as an NMI, activity on this pin in LPM3/LPM4 wakes the device and processes the interrupt, without causing a POR. If selected as a Reset, activity on this pin in LPM3/LPM4 causes a device-level POR When the device enters LPM3.5/LPM4.5 modes of operation, this bit is always cleared to 0. In other words, the RSTn/NMI pin always assumes a reset functionality in LPM3.5/LPM4.5 modes."]
pub type PIN_SRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYS_NMI_CTLSTAT_SPEC, PIN_SRC_A, O>;
impl<'a, const O: u8> PIN_SRC_W<'a, O> {
    #[doc = "Configures the RSTn_NMI pin as a source of POR Class Reset"]
    #[inline(always)]
    pub fn pin_src_0(self) -> &'a mut W {
        self.variant(PIN_SRC_A::PIN_SRC_0)
    }
    #[doc = "Configures the RSTn_NMI pin as a source of NMI"]
    #[inline(always)]
    pub fn pin_src_1(self) -> &'a mut W {
        self.variant(PIN_SRC_A::PIN_SRC_1)
    }
}
#[doc = "CS interrupt was the source of NMI\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CS_FLG_A {
    #[doc = "0: indicates CS interrupt was not the source of NMI"]
    CS_FLG_0 = 0,
    #[doc = "1: indicates CS interrupt was the source of NMI"]
    CS_FLG_1 = 1,
}
impl From<CS_FLG_A> for bool {
    #[inline(always)]
    fn from(variant: CS_FLG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CS_FLG` reader - CS interrupt was the source of NMI"]
pub type CS_FLG_R = crate::BitReader<CS_FLG_A>;
impl CS_FLG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CS_FLG_A {
        match self.bits {
            false => CS_FLG_A::CS_FLG_0,
            true => CS_FLG_A::CS_FLG_1,
        }
    }
    #[doc = "Checks if the value of the field is `CS_FLG_0`"]
    #[inline(always)]
    pub fn is_cs_flg_0(&self) -> bool {
        *self == CS_FLG_A::CS_FLG_0
    }
    #[doc = "Checks if the value of the field is `CS_FLG_1`"]
    #[inline(always)]
    pub fn is_cs_flg_1(&self) -> bool {
        *self == CS_FLG_A::CS_FLG_1
    }
}
#[doc = "PSS interrupt was the source of NMI\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PSS_FLG_A {
    #[doc = "0: indicates the PSS interrupt was not the source of NMI"]
    PSS_FLG_0 = 0,
    #[doc = "1: indicates the PSS interrupt was the source of NMI"]
    PSS_FLG_1 = 1,
}
impl From<PSS_FLG_A> for bool {
    #[inline(always)]
    fn from(variant: PSS_FLG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PSS_FLG` reader - PSS interrupt was the source of NMI"]
pub type PSS_FLG_R = crate::BitReader<PSS_FLG_A>;
impl PSS_FLG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSS_FLG_A {
        match self.bits {
            false => PSS_FLG_A::PSS_FLG_0,
            true => PSS_FLG_A::PSS_FLG_1,
        }
    }
    #[doc = "Checks if the value of the field is `PSS_FLG_0`"]
    #[inline(always)]
    pub fn is_pss_flg_0(&self) -> bool {
        *self == PSS_FLG_A::PSS_FLG_0
    }
    #[doc = "Checks if the value of the field is `PSS_FLG_1`"]
    #[inline(always)]
    pub fn is_pss_flg_1(&self) -> bool {
        *self == PSS_FLG_A::PSS_FLG_1
    }
}
#[doc = "PCM interrupt was the source of NMI\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCM_FLG_A {
    #[doc = "0: indicates the PCM interrupt was not the source of NMI"]
    PCM_FLG_0 = 0,
    #[doc = "1: indicates the PCM interrupt was the source of NMI"]
    PCM_FLG_1 = 1,
}
impl From<PCM_FLG_A> for bool {
    #[inline(always)]
    fn from(variant: PCM_FLG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PCM_FLG` reader - PCM interrupt was the source of NMI"]
pub type PCM_FLG_R = crate::BitReader<PCM_FLG_A>;
impl PCM_FLG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCM_FLG_A {
        match self.bits {
            false => PCM_FLG_A::PCM_FLG_0,
            true => PCM_FLG_A::PCM_FLG_1,
        }
    }
    #[doc = "Checks if the value of the field is `PCM_FLG_0`"]
    #[inline(always)]
    pub fn is_pcm_flg_0(&self) -> bool {
        *self == PCM_FLG_A::PCM_FLG_0
    }
    #[doc = "Checks if the value of the field is `PCM_FLG_1`"]
    #[inline(always)]
    pub fn is_pcm_flg_1(&self) -> bool {
        *self == PCM_FLG_A::PCM_FLG_1
    }
}
#[doc = "RSTn/NMI pin was the source of NMI\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN_FLG_A {
    #[doc = "0: Indicates the RSTn_NMI pin was not the source of NMI"]
    PIN_FLG_0 = 0,
    #[doc = "1: Indicates the RSTn_NMI pin was the source of NMI"]
    PIN_FLG_1 = 1,
}
impl From<PIN_FLG_A> for bool {
    #[inline(always)]
    fn from(variant: PIN_FLG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN_FLG` reader - RSTn/NMI pin was the source of NMI"]
pub type PIN_FLG_R = crate::BitReader<PIN_FLG_A>;
impl PIN_FLG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN_FLG_A {
        match self.bits {
            false => PIN_FLG_A::PIN_FLG_0,
            true => PIN_FLG_A::PIN_FLG_1,
        }
    }
    #[doc = "Checks if the value of the field is `PIN_FLG_0`"]
    #[inline(always)]
    pub fn is_pin_flg_0(&self) -> bool {
        *self == PIN_FLG_A::PIN_FLG_0
    }
    #[doc = "Checks if the value of the field is `PIN_FLG_1`"]
    #[inline(always)]
    pub fn is_pin_flg_1(&self) -> bool {
        *self == PIN_FLG_A::PIN_FLG_1
    }
}
#[doc = "Field `PIN_FLG` writer - RSTn/NMI pin was the source of NMI"]
pub type PIN_FLG_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYS_NMI_CTLSTAT_SPEC, PIN_FLG_A, O>;
impl<'a, const O: u8> PIN_FLG_W<'a, O> {
    #[doc = "Indicates the RSTn_NMI pin was not the source of NMI"]
    #[inline(always)]
    pub fn pin_flg_0(self) -> &'a mut W {
        self.variant(PIN_FLG_A::PIN_FLG_0)
    }
    #[doc = "Indicates the RSTn_NMI pin was the source of NMI"]
    #[inline(always)]
    pub fn pin_flg_1(self) -> &'a mut W {
        self.variant(PIN_FLG_A::PIN_FLG_1)
    }
}
impl R {
    #[doc = "Bit 0 - CS interrupt as a source of NMI"]
    #[inline(always)]
    pub fn cs_src(&self) -> CS_SRC_R {
        CS_SRC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PSS interrupt as a source of NMI"]
    #[inline(always)]
    pub fn pss_src(&self) -> PSS_SRC_R {
        PSS_SRC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PCM interrupt as a source of NMI"]
    #[inline(always)]
    pub fn pcm_src(&self) -> PCM_SRC_R {
        PCM_SRC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RSTn/NMI pin configuration Note: When the device enters LPM3/LPM4 modes of operation, the functionality selected by this bit is retained. If selected as an NMI, activity on this pin in LPM3/LPM4 wakes the device and processes the interrupt, without causing a POR. If selected as a Reset, activity on this pin in LPM3/LPM4 causes a device-level POR When the device enters LPM3.5/LPM4.5 modes of operation, this bit is always cleared to 0. In other words, the RSTn/NMI pin always assumes a reset functionality in LPM3.5/LPM4.5 modes."]
    #[inline(always)]
    pub fn pin_src(&self) -> PIN_SRC_R {
        PIN_SRC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 16 - CS interrupt was the source of NMI"]
    #[inline(always)]
    pub fn cs_flg(&self) -> CS_FLG_R {
        CS_FLG_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - PSS interrupt was the source of NMI"]
    #[inline(always)]
    pub fn pss_flg(&self) -> PSS_FLG_R {
        PSS_FLG_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - PCM interrupt was the source of NMI"]
    #[inline(always)]
    pub fn pcm_flg(&self) -> PCM_FLG_R {
        PCM_FLG_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - RSTn/NMI pin was the source of NMI"]
    #[inline(always)]
    pub fn pin_flg(&self) -> PIN_FLG_R {
        PIN_FLG_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CS interrupt as a source of NMI"]
    #[inline(always)]
    pub fn cs_src(&mut self) -> CS_SRC_W<0> {
        CS_SRC_W::new(self)
    }
    #[doc = "Bit 1 - PSS interrupt as a source of NMI"]
    #[inline(always)]
    pub fn pss_src(&mut self) -> PSS_SRC_W<1> {
        PSS_SRC_W::new(self)
    }
    #[doc = "Bit 2 - PCM interrupt as a source of NMI"]
    #[inline(always)]
    pub fn pcm_src(&mut self) -> PCM_SRC_W<2> {
        PCM_SRC_W::new(self)
    }
    #[doc = "Bit 3 - RSTn/NMI pin configuration Note: When the device enters LPM3/LPM4 modes of operation, the functionality selected by this bit is retained. If selected as an NMI, activity on this pin in LPM3/LPM4 wakes the device and processes the interrupt, without causing a POR. If selected as a Reset, activity on this pin in LPM3/LPM4 causes a device-level POR When the device enters LPM3.5/LPM4.5 modes of operation, this bit is always cleared to 0. In other words, the RSTn/NMI pin always assumes a reset functionality in LPM3.5/LPM4.5 modes."]
    #[inline(always)]
    pub fn pin_src(&mut self) -> PIN_SRC_W<3> {
        PIN_SRC_W::new(self)
    }
    #[doc = "Bit 19 - RSTn/NMI pin was the source of NMI"]
    #[inline(always)]
    pub fn pin_flg(&mut self) -> PIN_FLG_W<19> {
        PIN_FLG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "NMI Control and Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_nmi_ctlstat](index.html) module"]
pub struct SYS_NMI_CTLSTAT_SPEC;
impl crate::RegisterSpec for SYS_NMI_CTLSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sys_nmi_ctlstat::R](R) reader structure"]
impl crate::Readable for SYS_NMI_CTLSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sys_nmi_ctlstat::W](W) writer structure"]
impl crate::Writable for SYS_NMI_CTLSTAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYS_NMI_CTLSTAT to value 0x07"]
impl crate::Resettable for SYS_NMI_CTLSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x07
    }
}
