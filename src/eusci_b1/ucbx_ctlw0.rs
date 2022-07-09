#[doc = "Register `UCBxCTLW0` reader"]
pub struct R(crate::R<UCBXCTLW0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCBXCTLW0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCBXCTLW0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCBXCTLW0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCBxCTLW0` writer"]
pub struct W(crate::W<UCBXCTLW0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCBXCTLW0_SPEC>;
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
impl From<crate::W<UCBXCTLW0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCBXCTLW0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Software reset enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCSWRST_A {
    #[doc = "0: Disabled. eUSCI_B reset released for operation"]
    UCSWRST_0 = 0,
    #[doc = "1: Enabled. eUSCI_B logic held in reset state"]
    UCSWRST_1 = 1,
}
impl From<UCSWRST_A> for bool {
    #[inline(always)]
    fn from(variant: UCSWRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCSWRST` reader - Software reset enable"]
pub type UCSWRST_R = crate::BitReader<UCSWRST_A>;
impl UCSWRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCSWRST_A {
        match self.bits {
            false => UCSWRST_A::UCSWRST_0,
            true => UCSWRST_A::UCSWRST_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCSWRST_0`"]
    #[inline(always)]
    pub fn is_ucswrst_0(&self) -> bool {
        *self == UCSWRST_A::UCSWRST_0
    }
    #[doc = "Checks if the value of the field is `UCSWRST_1`"]
    #[inline(always)]
    pub fn is_ucswrst_1(&self) -> bool {
        *self == UCSWRST_A::UCSWRST_1
    }
}
#[doc = "Field `UCSWRST` writer - Software reset enable"]
pub type UCSWRST_W<'a, const O: u8> = crate::BitWriter<'a, u16, UCBXCTLW0_SPEC, UCSWRST_A, O>;
impl<'a, const O: u8> UCSWRST_W<'a, O> {
    #[doc = "Disabled. eUSCI_B reset released for operation"]
    #[inline(always)]
    pub fn ucswrst_0(self) -> &'a mut W {
        self.variant(UCSWRST_A::UCSWRST_0)
    }
    #[doc = "Enabled. eUSCI_B logic held in reset state"]
    #[inline(always)]
    pub fn ucswrst_1(self) -> &'a mut W {
        self.variant(UCSWRST_A::UCSWRST_1)
    }
}
#[doc = "Transmit START condition in master mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCTXSTT_A {
    #[doc = "0: Do not generate START condition"]
    UCTXSTT_0 = 0,
    #[doc = "1: Generate START condition"]
    UCTXSTT_1 = 1,
}
impl From<UCTXSTT_A> for bool {
    #[inline(always)]
    fn from(variant: UCTXSTT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCTXSTT` reader - Transmit START condition in master mode"]
pub type UCTXSTT_R = crate::BitReader<UCTXSTT_A>;
impl UCTXSTT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCTXSTT_A {
        match self.bits {
            false => UCTXSTT_A::UCTXSTT_0,
            true => UCTXSTT_A::UCTXSTT_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCTXSTT_0`"]
    #[inline(always)]
    pub fn is_uctxstt_0(&self) -> bool {
        *self == UCTXSTT_A::UCTXSTT_0
    }
    #[doc = "Checks if the value of the field is `UCTXSTT_1`"]
    #[inline(always)]
    pub fn is_uctxstt_1(&self) -> bool {
        *self == UCTXSTT_A::UCTXSTT_1
    }
}
#[doc = "Field `UCTXSTT` writer - Transmit START condition in master mode"]
pub type UCTXSTT_W<'a, const O: u8> = crate::BitWriter<'a, u16, UCBXCTLW0_SPEC, UCTXSTT_A, O>;
impl<'a, const O: u8> UCTXSTT_W<'a, O> {
    #[doc = "Do not generate START condition"]
    #[inline(always)]
    pub fn uctxstt_0(self) -> &'a mut W {
        self.variant(UCTXSTT_A::UCTXSTT_0)
    }
    #[doc = "Generate START condition"]
    #[inline(always)]
    pub fn uctxstt_1(self) -> &'a mut W {
        self.variant(UCTXSTT_A::UCTXSTT_1)
    }
}
#[doc = "Transmit STOP condition in master mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCTXSTP_A {
    #[doc = "0: No STOP generated"]
    UCTXSTP_0 = 0,
    #[doc = "1: Generate STOP"]
    UCTXSTP_1 = 1,
}
impl From<UCTXSTP_A> for bool {
    #[inline(always)]
    fn from(variant: UCTXSTP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCTXSTP` reader - Transmit STOP condition in master mode"]
pub type UCTXSTP_R = crate::BitReader<UCTXSTP_A>;
impl UCTXSTP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCTXSTP_A {
        match self.bits {
            false => UCTXSTP_A::UCTXSTP_0,
            true => UCTXSTP_A::UCTXSTP_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCTXSTP_0`"]
    #[inline(always)]
    pub fn is_uctxstp_0(&self) -> bool {
        *self == UCTXSTP_A::UCTXSTP_0
    }
    #[doc = "Checks if the value of the field is `UCTXSTP_1`"]
    #[inline(always)]
    pub fn is_uctxstp_1(&self) -> bool {
        *self == UCTXSTP_A::UCTXSTP_1
    }
}
#[doc = "Field `UCTXSTP` writer - Transmit STOP condition in master mode"]
pub type UCTXSTP_W<'a, const O: u8> = crate::BitWriter<'a, u16, UCBXCTLW0_SPEC, UCTXSTP_A, O>;
impl<'a, const O: u8> UCTXSTP_W<'a, O> {
    #[doc = "No STOP generated"]
    #[inline(always)]
    pub fn uctxstp_0(self) -> &'a mut W {
        self.variant(UCTXSTP_A::UCTXSTP_0)
    }
    #[doc = "Generate STOP"]
    #[inline(always)]
    pub fn uctxstp_1(self) -> &'a mut W {
        self.variant(UCTXSTP_A::UCTXSTP_1)
    }
}
#[doc = "Transmit a NACK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCTXNACK_A {
    #[doc = "0: Acknowledge normally"]
    UCTXNACK_0 = 0,
    #[doc = "1: Generate NACK"]
    UCTXNACK_1 = 1,
}
impl From<UCTXNACK_A> for bool {
    #[inline(always)]
    fn from(variant: UCTXNACK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCTXNACK` reader - Transmit a NACK"]
pub type UCTXNACK_R = crate::BitReader<UCTXNACK_A>;
impl UCTXNACK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCTXNACK_A {
        match self.bits {
            false => UCTXNACK_A::UCTXNACK_0,
            true => UCTXNACK_A::UCTXNACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCTXNACK_0`"]
    #[inline(always)]
    pub fn is_uctxnack_0(&self) -> bool {
        *self == UCTXNACK_A::UCTXNACK_0
    }
    #[doc = "Checks if the value of the field is `UCTXNACK_1`"]
    #[inline(always)]
    pub fn is_uctxnack_1(&self) -> bool {
        *self == UCTXNACK_A::UCTXNACK_1
    }
}
#[doc = "Field `UCTXNACK` writer - Transmit a NACK"]
pub type UCTXNACK_W<'a, const O: u8> = crate::BitWriter<'a, u16, UCBXCTLW0_SPEC, UCTXNACK_A, O>;
impl<'a, const O: u8> UCTXNACK_W<'a, O> {
    #[doc = "Acknowledge normally"]
    #[inline(always)]
    pub fn uctxnack_0(self) -> &'a mut W {
        self.variant(UCTXNACK_A::UCTXNACK_0)
    }
    #[doc = "Generate NACK"]
    #[inline(always)]
    pub fn uctxnack_1(self) -> &'a mut W {
        self.variant(UCTXNACK_A::UCTXNACK_1)
    }
}
#[doc = "Transmitter/receiver\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCTR_A {
    #[doc = "0: Receiver"]
    UCTR_0 = 0,
    #[doc = "1: Transmitter"]
    UCTR_1 = 1,
}
impl From<UCTR_A> for bool {
    #[inline(always)]
    fn from(variant: UCTR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCTR` reader - Transmitter/receiver"]
pub type UCTR_R = crate::BitReader<UCTR_A>;
impl UCTR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCTR_A {
        match self.bits {
            false => UCTR_A::UCTR_0,
            true => UCTR_A::UCTR_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCTR_0`"]
    #[inline(always)]
    pub fn is_uctr_0(&self) -> bool {
        *self == UCTR_A::UCTR_0
    }
    #[doc = "Checks if the value of the field is `UCTR_1`"]
    #[inline(always)]
    pub fn is_uctr_1(&self) -> bool {
        *self == UCTR_A::UCTR_1
    }
}
#[doc = "Field `UCTR` writer - Transmitter/receiver"]
pub type UCTR_W<'a, const O: u8> = crate::BitWriter<'a, u16, UCBXCTLW0_SPEC, UCTR_A, O>;
impl<'a, const O: u8> UCTR_W<'a, O> {
    #[doc = "Receiver"]
    #[inline(always)]
    pub fn uctr_0(self) -> &'a mut W {
        self.variant(UCTR_A::UCTR_0)
    }
    #[doc = "Transmitter"]
    #[inline(always)]
    pub fn uctr_1(self) -> &'a mut W {
        self.variant(UCTR_A::UCTR_1)
    }
}
#[doc = "Transmit ACK condition in slave mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCTXACK_A {
    #[doc = "0: Do not acknowledge the slave address"]
    UCTXACK_0 = 0,
    #[doc = "1: Acknowledge the slave address"]
    UCTXACK_1 = 1,
}
impl From<UCTXACK_A> for bool {
    #[inline(always)]
    fn from(variant: UCTXACK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCTXACK` reader - Transmit ACK condition in slave mode"]
pub type UCTXACK_R = crate::BitReader<UCTXACK_A>;
impl UCTXACK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCTXACK_A {
        match self.bits {
            false => UCTXACK_A::UCTXACK_0,
            true => UCTXACK_A::UCTXACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCTXACK_0`"]
    #[inline(always)]
    pub fn is_uctxack_0(&self) -> bool {
        *self == UCTXACK_A::UCTXACK_0
    }
    #[doc = "Checks if the value of the field is `UCTXACK_1`"]
    #[inline(always)]
    pub fn is_uctxack_1(&self) -> bool {
        *self == UCTXACK_A::UCTXACK_1
    }
}
#[doc = "Field `UCTXACK` writer - Transmit ACK condition in slave mode"]
pub type UCTXACK_W<'a, const O: u8> = crate::BitWriter<'a, u16, UCBXCTLW0_SPEC, UCTXACK_A, O>;
impl<'a, const O: u8> UCTXACK_W<'a, O> {
    #[doc = "Do not acknowledge the slave address"]
    #[inline(always)]
    pub fn uctxack_0(self) -> &'a mut W {
        self.variant(UCTXACK_A::UCTXACK_0)
    }
    #[doc = "Acknowledge the slave address"]
    #[inline(always)]
    pub fn uctxack_1(self) -> &'a mut W {
        self.variant(UCTXACK_A::UCTXACK_1)
    }
}
#[doc = "eUSCI_B clock source select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum UCSSEL_A {
    #[doc = "0: UCLKI"]
    UCSSEL_0 = 0,
    #[doc = "1: ACLK"]
    UCSSEL_1 = 1,
    #[doc = "2: SMCLK"]
    UCSSEL_2 = 2,
    #[doc = "3: SMCLK"]
    UCSSEL_3 = 3,
}
impl From<UCSSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: UCSSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `UCSSEL` reader - eUSCI_B clock source select"]
pub type UCSSEL_R = crate::FieldReader<u8, UCSSEL_A>;
impl UCSSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCSSEL_A {
        match self.bits {
            0 => UCSSEL_A::UCSSEL_0,
            1 => UCSSEL_A::UCSSEL_1,
            2 => UCSSEL_A::UCSSEL_2,
            3 => UCSSEL_A::UCSSEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `UCSSEL_0`"]
    #[inline(always)]
    pub fn is_ucssel_0(&self) -> bool {
        *self == UCSSEL_A::UCSSEL_0
    }
    #[doc = "Checks if the value of the field is `UCSSEL_1`"]
    #[inline(always)]
    pub fn is_ucssel_1(&self) -> bool {
        *self == UCSSEL_A::UCSSEL_1
    }
    #[doc = "Checks if the value of the field is `UCSSEL_2`"]
    #[inline(always)]
    pub fn is_ucssel_2(&self) -> bool {
        *self == UCSSEL_A::UCSSEL_2
    }
    #[doc = "Checks if the value of the field is `UCSSEL_3`"]
    #[inline(always)]
    pub fn is_ucssel_3(&self) -> bool {
        *self == UCSSEL_A::UCSSEL_3
    }
}
#[doc = "Field `UCSSEL` writer - eUSCI_B clock source select"]
pub type UCSSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, UCBXCTLW0_SPEC, u8, UCSSEL_A, 2, O>;
impl<'a, const O: u8> UCSSEL_W<'a, O> {
    #[doc = "UCLKI"]
    #[inline(always)]
    pub fn ucssel_0(self) -> &'a mut W {
        self.variant(UCSSEL_A::UCSSEL_0)
    }
    #[doc = "ACLK"]
    #[inline(always)]
    pub fn ucssel_1(self) -> &'a mut W {
        self.variant(UCSSEL_A::UCSSEL_1)
    }
    #[doc = "SMCLK"]
    #[inline(always)]
    pub fn ucssel_2(self) -> &'a mut W {
        self.variant(UCSSEL_A::UCSSEL_2)
    }
    #[doc = "SMCLK"]
    #[inline(always)]
    pub fn ucssel_3(self) -> &'a mut W {
        self.variant(UCSSEL_A::UCSSEL_3)
    }
}
#[doc = "Synchronous mode enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCSYNC_A {
    #[doc = "0: Asynchronous mode"]
    UCSYNC_0 = 0,
    #[doc = "1: Synchronous mode"]
    UCSYNC_1 = 1,
}
impl From<UCSYNC_A> for bool {
    #[inline(always)]
    fn from(variant: UCSYNC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCSYNC` reader - Synchronous mode enable"]
pub type UCSYNC_R = crate::BitReader<UCSYNC_A>;
impl UCSYNC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCSYNC_A {
        match self.bits {
            false => UCSYNC_A::UCSYNC_0,
            true => UCSYNC_A::UCSYNC_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCSYNC_0`"]
    #[inline(always)]
    pub fn is_ucsync_0(&self) -> bool {
        *self == UCSYNC_A::UCSYNC_0
    }
    #[doc = "Checks if the value of the field is `UCSYNC_1`"]
    #[inline(always)]
    pub fn is_ucsync_1(&self) -> bool {
        *self == UCSYNC_A::UCSYNC_1
    }
}
#[doc = "Field `UCSYNC` writer - Synchronous mode enable"]
pub type UCSYNC_W<'a, const O: u8> = crate::BitWriter<'a, u16, UCBXCTLW0_SPEC, UCSYNC_A, O>;
impl<'a, const O: u8> UCSYNC_W<'a, O> {
    #[doc = "Asynchronous mode"]
    #[inline(always)]
    pub fn ucsync_0(self) -> &'a mut W {
        self.variant(UCSYNC_A::UCSYNC_0)
    }
    #[doc = "Synchronous mode"]
    #[inline(always)]
    pub fn ucsync_1(self) -> &'a mut W {
        self.variant(UCSYNC_A::UCSYNC_1)
    }
}
#[doc = "eUSCI_B mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum UCMODE_A {
    #[doc = "0: 3-pin SPI"]
    UCMODE_0 = 0,
    #[doc = "1: 4-pin SPI (master or slave enabled if STE = 1)"]
    UCMODE_1 = 1,
    #[doc = "2: 4-pin SPI (master or slave enabled if STE = 0)"]
    UCMODE_2 = 2,
    #[doc = "3: I2C mode"]
    UCMODE_3 = 3,
}
impl From<UCMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: UCMODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `UCMODE` reader - eUSCI_B mode"]
pub type UCMODE_R = crate::FieldReader<u8, UCMODE_A>;
impl UCMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCMODE_A {
        match self.bits {
            0 => UCMODE_A::UCMODE_0,
            1 => UCMODE_A::UCMODE_1,
            2 => UCMODE_A::UCMODE_2,
            3 => UCMODE_A::UCMODE_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `UCMODE_0`"]
    #[inline(always)]
    pub fn is_ucmode_0(&self) -> bool {
        *self == UCMODE_A::UCMODE_0
    }
    #[doc = "Checks if the value of the field is `UCMODE_1`"]
    #[inline(always)]
    pub fn is_ucmode_1(&self) -> bool {
        *self == UCMODE_A::UCMODE_1
    }
    #[doc = "Checks if the value of the field is `UCMODE_2`"]
    #[inline(always)]
    pub fn is_ucmode_2(&self) -> bool {
        *self == UCMODE_A::UCMODE_2
    }
    #[doc = "Checks if the value of the field is `UCMODE_3`"]
    #[inline(always)]
    pub fn is_ucmode_3(&self) -> bool {
        *self == UCMODE_A::UCMODE_3
    }
}
#[doc = "Field `UCMODE` writer - eUSCI_B mode"]
pub type UCMODE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, UCBXCTLW0_SPEC, u8, UCMODE_A, 2, O>;
impl<'a, const O: u8> UCMODE_W<'a, O> {
    #[doc = "3-pin SPI"]
    #[inline(always)]
    pub fn ucmode_0(self) -> &'a mut W {
        self.variant(UCMODE_A::UCMODE_0)
    }
    #[doc = "4-pin SPI (master or slave enabled if STE = 1)"]
    #[inline(always)]
    pub fn ucmode_1(self) -> &'a mut W {
        self.variant(UCMODE_A::UCMODE_1)
    }
    #[doc = "4-pin SPI (master or slave enabled if STE = 0)"]
    #[inline(always)]
    pub fn ucmode_2(self) -> &'a mut W {
        self.variant(UCMODE_A::UCMODE_2)
    }
    #[doc = "I2C mode"]
    #[inline(always)]
    pub fn ucmode_3(self) -> &'a mut W {
        self.variant(UCMODE_A::UCMODE_3)
    }
}
#[doc = "Master mode select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCMST_A {
    #[doc = "0: Slave mode"]
    UCMST_0 = 0,
    #[doc = "1: Master mode"]
    UCMST_1 = 1,
}
impl From<UCMST_A> for bool {
    #[inline(always)]
    fn from(variant: UCMST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCMST` reader - Master mode select"]
pub type UCMST_R = crate::BitReader<UCMST_A>;
impl UCMST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCMST_A {
        match self.bits {
            false => UCMST_A::UCMST_0,
            true => UCMST_A::UCMST_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCMST_0`"]
    #[inline(always)]
    pub fn is_ucmst_0(&self) -> bool {
        *self == UCMST_A::UCMST_0
    }
    #[doc = "Checks if the value of the field is `UCMST_1`"]
    #[inline(always)]
    pub fn is_ucmst_1(&self) -> bool {
        *self == UCMST_A::UCMST_1
    }
}
#[doc = "Field `UCMST` writer - Master mode select"]
pub type UCMST_W<'a, const O: u8> = crate::BitWriter<'a, u16, UCBXCTLW0_SPEC, UCMST_A, O>;
impl<'a, const O: u8> UCMST_W<'a, O> {
    #[doc = "Slave mode"]
    #[inline(always)]
    pub fn ucmst_0(self) -> &'a mut W {
        self.variant(UCMST_A::UCMST_0)
    }
    #[doc = "Master mode"]
    #[inline(always)]
    pub fn ucmst_1(self) -> &'a mut W {
        self.variant(UCMST_A::UCMST_1)
    }
}
#[doc = "Multi-master environment select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCMM_A {
    #[doc = "0: Single master environment. There is no other master in the system. The address compare unit is disabled."]
    UCMM_0 = 0,
    #[doc = "1: Multi-master environment"]
    UCMM_1 = 1,
}
impl From<UCMM_A> for bool {
    #[inline(always)]
    fn from(variant: UCMM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCMM` reader - Multi-master environment select"]
pub type UCMM_R = crate::BitReader<UCMM_A>;
impl UCMM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCMM_A {
        match self.bits {
            false => UCMM_A::UCMM_0,
            true => UCMM_A::UCMM_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCMM_0`"]
    #[inline(always)]
    pub fn is_ucmm_0(&self) -> bool {
        *self == UCMM_A::UCMM_0
    }
    #[doc = "Checks if the value of the field is `UCMM_1`"]
    #[inline(always)]
    pub fn is_ucmm_1(&self) -> bool {
        *self == UCMM_A::UCMM_1
    }
}
#[doc = "Field `UCMM` writer - Multi-master environment select"]
pub type UCMM_W<'a, const O: u8> = crate::BitWriter<'a, u16, UCBXCTLW0_SPEC, UCMM_A, O>;
impl<'a, const O: u8> UCMM_W<'a, O> {
    #[doc = "Single master environment. There is no other master in the system. The address compare unit is disabled."]
    #[inline(always)]
    pub fn ucmm_0(self) -> &'a mut W {
        self.variant(UCMM_A::UCMM_0)
    }
    #[doc = "Multi-master environment"]
    #[inline(always)]
    pub fn ucmm_1(self) -> &'a mut W {
        self.variant(UCMM_A::UCMM_1)
    }
}
#[doc = "Slave addressing mode select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCSLA10_A {
    #[doc = "0: Address slave with 7-bit address"]
    UCSLA10_0 = 0,
    #[doc = "1: Address slave with 10-bit address"]
    UCSLA10_1 = 1,
}
impl From<UCSLA10_A> for bool {
    #[inline(always)]
    fn from(variant: UCSLA10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCSLA10` reader - Slave addressing mode select"]
pub type UCSLA10_R = crate::BitReader<UCSLA10_A>;
impl UCSLA10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCSLA10_A {
        match self.bits {
            false => UCSLA10_A::UCSLA10_0,
            true => UCSLA10_A::UCSLA10_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCSLA10_0`"]
    #[inline(always)]
    pub fn is_ucsla10_0(&self) -> bool {
        *self == UCSLA10_A::UCSLA10_0
    }
    #[doc = "Checks if the value of the field is `UCSLA10_1`"]
    #[inline(always)]
    pub fn is_ucsla10_1(&self) -> bool {
        *self == UCSLA10_A::UCSLA10_1
    }
}
#[doc = "Field `UCSLA10` writer - Slave addressing mode select"]
pub type UCSLA10_W<'a, const O: u8> = crate::BitWriter<'a, u16, UCBXCTLW0_SPEC, UCSLA10_A, O>;
impl<'a, const O: u8> UCSLA10_W<'a, O> {
    #[doc = "Address slave with 7-bit address"]
    #[inline(always)]
    pub fn ucsla10_0(self) -> &'a mut W {
        self.variant(UCSLA10_A::UCSLA10_0)
    }
    #[doc = "Address slave with 10-bit address"]
    #[inline(always)]
    pub fn ucsla10_1(self) -> &'a mut W {
        self.variant(UCSLA10_A::UCSLA10_1)
    }
}
#[doc = "Own addressing mode select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCA10_A {
    #[doc = "0: Own address is a 7-bit address"]
    UCA10_0 = 0,
    #[doc = "1: Own address is a 10-bit address"]
    UCA10_1 = 1,
}
impl From<UCA10_A> for bool {
    #[inline(always)]
    fn from(variant: UCA10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCA10` reader - Own addressing mode select"]
pub type UCA10_R = crate::BitReader<UCA10_A>;
impl UCA10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCA10_A {
        match self.bits {
            false => UCA10_A::UCA10_0,
            true => UCA10_A::UCA10_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCA10_0`"]
    #[inline(always)]
    pub fn is_uca10_0(&self) -> bool {
        *self == UCA10_A::UCA10_0
    }
    #[doc = "Checks if the value of the field is `UCA10_1`"]
    #[inline(always)]
    pub fn is_uca10_1(&self) -> bool {
        *self == UCA10_A::UCA10_1
    }
}
#[doc = "Field `UCA10` writer - Own addressing mode select"]
pub type UCA10_W<'a, const O: u8> = crate::BitWriter<'a, u16, UCBXCTLW0_SPEC, UCA10_A, O>;
impl<'a, const O: u8> UCA10_W<'a, O> {
    #[doc = "Own address is a 7-bit address"]
    #[inline(always)]
    pub fn uca10_0(self) -> &'a mut W {
        self.variant(UCA10_A::UCA10_0)
    }
    #[doc = "Own address is a 10-bit address"]
    #[inline(always)]
    pub fn uca10_1(self) -> &'a mut W {
        self.variant(UCA10_A::UCA10_1)
    }
}
impl R {
    #[doc = "Bit 0 - Software reset enable"]
    #[inline(always)]
    pub fn ucswrst(&self) -> UCSWRST_R {
        UCSWRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit START condition in master mode"]
    #[inline(always)]
    pub fn uctxstt(&self) -> UCTXSTT_R {
        UCTXSTT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit STOP condition in master mode"]
    #[inline(always)]
    pub fn uctxstp(&self) -> UCTXSTP_R {
        UCTXSTP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit a NACK"]
    #[inline(always)]
    pub fn uctxnack(&self) -> UCTXNACK_R {
        UCTXNACK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Transmitter/receiver"]
    #[inline(always)]
    pub fn uctr(&self) -> UCTR_R {
        UCTR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit ACK condition in slave mode"]
    #[inline(always)]
    pub fn uctxack(&self) -> UCTXACK_R {
        UCTXACK_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - eUSCI_B clock source select"]
    #[inline(always)]
    pub fn ucssel(&self) -> UCSSEL_R {
        UCSSEL_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - Synchronous mode enable"]
    #[inline(always)]
    pub fn ucsync(&self) -> UCSYNC_R {
        UCSYNC_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - eUSCI_B mode"]
    #[inline(always)]
    pub fn ucmode(&self) -> UCMODE_R {
        UCMODE_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 11 - Master mode select"]
    #[inline(always)]
    pub fn ucmst(&self) -> UCMST_R {
        UCMST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - Multi-master environment select"]
    #[inline(always)]
    pub fn ucmm(&self) -> UCMM_R {
        UCMM_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Slave addressing mode select"]
    #[inline(always)]
    pub fn ucsla10(&self) -> UCSLA10_R {
        UCSLA10_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Own addressing mode select"]
    #[inline(always)]
    pub fn uca10(&self) -> UCA10_R {
        UCA10_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software reset enable"]
    #[inline(always)]
    pub fn ucswrst(&mut self) -> UCSWRST_W<0> {
        UCSWRST_W::new(self)
    }
    #[doc = "Bit 1 - Transmit START condition in master mode"]
    #[inline(always)]
    pub fn uctxstt(&mut self) -> UCTXSTT_W<1> {
        UCTXSTT_W::new(self)
    }
    #[doc = "Bit 2 - Transmit STOP condition in master mode"]
    #[inline(always)]
    pub fn uctxstp(&mut self) -> UCTXSTP_W<2> {
        UCTXSTP_W::new(self)
    }
    #[doc = "Bit 3 - Transmit a NACK"]
    #[inline(always)]
    pub fn uctxnack(&mut self) -> UCTXNACK_W<3> {
        UCTXNACK_W::new(self)
    }
    #[doc = "Bit 4 - Transmitter/receiver"]
    #[inline(always)]
    pub fn uctr(&mut self) -> UCTR_W<4> {
        UCTR_W::new(self)
    }
    #[doc = "Bit 5 - Transmit ACK condition in slave mode"]
    #[inline(always)]
    pub fn uctxack(&mut self) -> UCTXACK_W<5> {
        UCTXACK_W::new(self)
    }
    #[doc = "Bits 6:7 - eUSCI_B clock source select"]
    #[inline(always)]
    pub fn ucssel(&mut self) -> UCSSEL_W<6> {
        UCSSEL_W::new(self)
    }
    #[doc = "Bit 8 - Synchronous mode enable"]
    #[inline(always)]
    pub fn ucsync(&mut self) -> UCSYNC_W<8> {
        UCSYNC_W::new(self)
    }
    #[doc = "Bits 9:10 - eUSCI_B mode"]
    #[inline(always)]
    pub fn ucmode(&mut self) -> UCMODE_W<9> {
        UCMODE_W::new(self)
    }
    #[doc = "Bit 11 - Master mode select"]
    #[inline(always)]
    pub fn ucmst(&mut self) -> UCMST_W<11> {
        UCMST_W::new(self)
    }
    #[doc = "Bit 13 - Multi-master environment select"]
    #[inline(always)]
    pub fn ucmm(&mut self) -> UCMM_W<13> {
        UCMM_W::new(self)
    }
    #[doc = "Bit 14 - Slave addressing mode select"]
    #[inline(always)]
    pub fn ucsla10(&mut self) -> UCSLA10_W<14> {
        UCSLA10_W::new(self)
    }
    #[doc = "Bit 15 - Own addressing mode select"]
    #[inline(always)]
    pub fn uca10(&mut self) -> UCA10_W<15> {
        UCA10_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "eUSCI_Bx Control Word Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucbx_ctlw0](index.html) module"]
pub struct UCBXCTLW0_SPEC;
impl crate::RegisterSpec for UCBXCTLW0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ucbx_ctlw0::R](R) reader structure"]
impl crate::Readable for UCBXCTLW0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ucbx_ctlw0::W](W) writer structure"]
impl crate::Writable for UCBXCTLW0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UCBxCTLW0 to value 0x01c1"]
impl crate::Resettable for UCBXCTLW0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01c1
    }
}
