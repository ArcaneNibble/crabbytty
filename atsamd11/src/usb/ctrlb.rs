#[doc = "Reader of register CTRLB"]
pub type R = crate::R<u16, super::CTRLB>;
#[doc = "Writer for register CTRLB"]
pub type W = crate::W<u16, super::CTRLB>;
#[doc = "Register CTRLB `reset()`'s with value 0x01"]
impl crate::ResetValue for super::CTRLB {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Detach\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DETACH_A {
    #[doc = "0: Attach USB"]
    ATTACH = 0,
    #[doc = "1: Detach USB"]
    DETACH = 1,
}
impl From<DETACH_A> for bool {
    #[inline(always)]
    fn from(variant: DETACH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DETACH`"]
pub type DETACH_R = crate::R<bool, DETACH_A>;
impl DETACH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DETACH_A {
        match self.bits {
            false => DETACH_A::ATTACH,
            true => DETACH_A::DETACH,
        }
    }
    #[doc = "Checks if the value of the field is `ATTACH`"]
    #[inline(always)]
    pub fn is_attach(&self) -> bool {
        *self == DETACH_A::ATTACH
    }
    #[doc = "Checks if the value of the field is `DETACH`"]
    #[inline(always)]
    pub fn is_detach(&self) -> bool {
        *self == DETACH_A::DETACH
    }
}
#[doc = "Write proxy for field `DETACH`"]
pub struct DETACH_W<'a> {
    w: &'a mut W,
}
impl<'a> DETACH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DETACH_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Attach USB"]
    #[inline(always)]
    pub fn attach(self) -> &'a mut W {
        self.variant(DETACH_A::ATTACH)
    }
    #[doc = "Detach USB"]
    #[inline(always)]
    pub fn detach(self) -> &'a mut W {
        self.variant(DETACH_A::DETACH)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u16) & 0x01);
        self.w
    }
}
#[doc = "Upstream Resume\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UPRSM_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Send resume"]
    RESUME = 1,
}
impl From<UPRSM_A> for bool {
    #[inline(always)]
    fn from(variant: UPRSM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `UPRSM`"]
pub type UPRSM_R = crate::R<bool, UPRSM_A>;
impl UPRSM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UPRSM_A {
        match self.bits {
            false => UPRSM_A::NOP,
            true => UPRSM_A::RESUME,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == UPRSM_A::NOP
    }
    #[doc = "Checks if the value of the field is `RESUME`"]
    #[inline(always)]
    pub fn is_resume(&self) -> bool {
        *self == UPRSM_A::RESUME
    }
}
#[doc = "Write proxy for field `UPRSM`"]
pub struct UPRSM_W<'a> {
    w: &'a mut W,
}
impl<'a> UPRSM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UPRSM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(UPRSM_A::NOP)
    }
    #[doc = "Send resume"]
    #[inline(always)]
    pub fn resume(self) -> &'a mut W {
        self.variant(UPRSM_A::RESUME)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u16) & 0x01) << 1);
        self.w
    }
}
#[doc = "Speed Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SPDCONF_A {
    #[doc = "0: FS : Full Speed"]
    FS = 0,
    #[doc = "1: LS : Low Speed"]
    LS = 1,
    #[doc = "2: HS : High Speed capable"]
    HS = 2,
    #[doc = "3: HSTM: High Speed Test Mode (force high-speed mode for test mode)"]
    HSTM = 3,
}
impl From<SPDCONF_A> for u8 {
    #[inline(always)]
    fn from(variant: SPDCONF_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SPDCONF`"]
pub type SPDCONF_R = crate::R<u8, SPDCONF_A>;
impl SPDCONF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPDCONF_A {
        match self.bits {
            0 => SPDCONF_A::FS,
            1 => SPDCONF_A::LS,
            2 => SPDCONF_A::HS,
            3 => SPDCONF_A::HSTM,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FS`"]
    #[inline(always)]
    pub fn is_fs(&self) -> bool {
        *self == SPDCONF_A::FS
    }
    #[doc = "Checks if the value of the field is `LS`"]
    #[inline(always)]
    pub fn is_ls(&self) -> bool {
        *self == SPDCONF_A::LS
    }
    #[doc = "Checks if the value of the field is `HS`"]
    #[inline(always)]
    pub fn is_hs(&self) -> bool {
        *self == SPDCONF_A::HS
    }
    #[doc = "Checks if the value of the field is `HSTM`"]
    #[inline(always)]
    pub fn is_hstm(&self) -> bool {
        *self == SPDCONF_A::HSTM
    }
}
#[doc = "Write proxy for field `SPDCONF`"]
pub struct SPDCONF_W<'a> {
    w: &'a mut W,
}
impl<'a> SPDCONF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPDCONF_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "FS : Full Speed"]
    #[inline(always)]
    pub fn fs(self) -> &'a mut W {
        self.variant(SPDCONF_A::FS)
    }
    #[doc = "LS : Low Speed"]
    #[inline(always)]
    pub fn ls(self) -> &'a mut W {
        self.variant(SPDCONF_A::LS)
    }
    #[doc = "HS : High Speed capable"]
    #[inline(always)]
    pub fn hs(self) -> &'a mut W {
        self.variant(SPDCONF_A::HS)
    }
    #[doc = "HSTM: High Speed Test Mode (force high-speed mode for test mode)"]
    #[inline(always)]
    pub fn hstm(self) -> &'a mut W {
        self.variant(SPDCONF_A::HSTM)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u16) & 0x03) << 2);
        self.w
    }
}
#[doc = "No Reply\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NREPLY_A {
    #[doc = "0: Follow USB standard"]
    STANDARD = 0,
    #[doc = "1: Only respond to SETUP"]
    ONLYSETUP = 1,
}
impl From<NREPLY_A> for bool {
    #[inline(always)]
    fn from(variant: NREPLY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `NREPLY`"]
pub type NREPLY_R = crate::R<bool, NREPLY_A>;
impl NREPLY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NREPLY_A {
        match self.bits {
            false => NREPLY_A::STANDARD,
            true => NREPLY_A::ONLYSETUP,
        }
    }
    #[doc = "Checks if the value of the field is `STANDARD`"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == NREPLY_A::STANDARD
    }
    #[doc = "Checks if the value of the field is `ONLYSETUP`"]
    #[inline(always)]
    pub fn is_onlysetup(&self) -> bool {
        *self == NREPLY_A::ONLYSETUP
    }
}
#[doc = "Write proxy for field `NREPLY`"]
pub struct NREPLY_W<'a> {
    w: &'a mut W,
}
impl<'a> NREPLY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NREPLY_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Follow USB standard"]
    #[inline(always)]
    pub fn standard(self) -> &'a mut W {
        self.variant(NREPLY_A::STANDARD)
    }
    #[doc = "Only respond to SETUP"]
    #[inline(always)]
    pub fn onlysetup(self) -> &'a mut W {
        self.variant(NREPLY_A::ONLYSETUP)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u16) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `TSTJ`"]
pub type TSTJ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSTJ`"]
pub struct TSTJ_W<'a> {
    w: &'a mut W,
}
impl<'a> TSTJ_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u16) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `TSTK`"]
pub type TSTK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSTK`"]
pub struct TSTK_W<'a> {
    w: &'a mut W,
}
impl<'a> TSTK_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u16) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `TSTPCKT`"]
pub type TSTPCKT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSTPCKT`"]
pub struct TSTPCKT_W<'a> {
    w: &'a mut W,
}
impl<'a> TSTPCKT_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u16) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `OPMODE2`"]
pub type OPMODE2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OPMODE2`"]
pub struct OPMODE2_W<'a> {
    w: &'a mut W,
}
impl<'a> OPMODE2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u16) & 0x01) << 8);
        self.w
    }
}
#[doc = "Global NAK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GNAK_A {
    #[doc = "0: Respond according to status"]
    STANDARD = 0,
    #[doc = "1: Always respond NAK"]
    ALWAYSNAK = 1,
}
impl From<GNAK_A> for bool {
    #[inline(always)]
    fn from(variant: GNAK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GNAK`"]
pub type GNAK_R = crate::R<bool, GNAK_A>;
impl GNAK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GNAK_A {
        match self.bits {
            false => GNAK_A::STANDARD,
            true => GNAK_A::ALWAYSNAK,
        }
    }
    #[doc = "Checks if the value of the field is `STANDARD`"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == GNAK_A::STANDARD
    }
    #[doc = "Checks if the value of the field is `ALWAYSNAK`"]
    #[inline(always)]
    pub fn is_alwaysnak(&self) -> bool {
        *self == GNAK_A::ALWAYSNAK
    }
}
#[doc = "Write proxy for field `GNAK`"]
pub struct GNAK_W<'a> {
    w: &'a mut W,
}
impl<'a> GNAK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GNAK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Respond according to status"]
    #[inline(always)]
    pub fn standard(self) -> &'a mut W {
        self.variant(GNAK_A::STANDARD)
    }
    #[doc = "Always respond NAK"]
    #[inline(always)]
    pub fn alwaysnak(self) -> &'a mut W {
        self.variant(GNAK_A::ALWAYSNAK)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u16) & 0x01) << 9);
        self.w
    }
}
#[doc = "Link Power Management Handshake\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LPMHDSK_A {
    #[doc = "0: No handshake. LPM is not supported"]
    NO = 0,
    #[doc = "1: ACK"]
    ACK = 1,
    #[doc = "2: NYET"]
    NYET = 2,
    #[doc = "3: STALL"]
    STALL = 3,
}
impl From<LPMHDSK_A> for u8 {
    #[inline(always)]
    fn from(variant: LPMHDSK_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LPMHDSK`"]
pub type LPMHDSK_R = crate::R<u8, LPMHDSK_A>;
impl LPMHDSK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPMHDSK_A {
        match self.bits {
            0 => LPMHDSK_A::NO,
            1 => LPMHDSK_A::ACK,
            2 => LPMHDSK_A::NYET,
            3 => LPMHDSK_A::STALL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == LPMHDSK_A::NO
    }
    #[doc = "Checks if the value of the field is `ACK`"]
    #[inline(always)]
    pub fn is_ack(&self) -> bool {
        *self == LPMHDSK_A::ACK
    }
    #[doc = "Checks if the value of the field is `NYET`"]
    #[inline(always)]
    pub fn is_nyet(&self) -> bool {
        *self == LPMHDSK_A::NYET
    }
    #[doc = "Checks if the value of the field is `STALL`"]
    #[inline(always)]
    pub fn is_stall(&self) -> bool {
        *self == LPMHDSK_A::STALL
    }
}
#[doc = "Write proxy for field `LPMHDSK`"]
pub struct LPMHDSK_W<'a> {
    w: &'a mut W,
}
impl<'a> LPMHDSK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPMHDSK_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No handshake. LPM is not supported"]
    #[inline(always)]
    pub fn no(self) -> &'a mut W {
        self.variant(LPMHDSK_A::NO)
    }
    #[doc = "ACK"]
    #[inline(always)]
    pub fn ack(self) -> &'a mut W {
        self.variant(LPMHDSK_A::ACK)
    }
    #[doc = "NYET"]
    #[inline(always)]
    pub fn nyet(self) -> &'a mut W {
        self.variant(LPMHDSK_A::NYET)
    }
    #[doc = "STALL"]
    #[inline(always)]
    pub fn stall(self) -> &'a mut W {
        self.variant(LPMHDSK_A::STALL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u16) & 0x03) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Detach"]
    #[inline(always)]
    pub fn detach(&self) -> DETACH_R {
        DETACH_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Upstream Resume"]
    #[inline(always)]
    pub fn uprsm(&self) -> UPRSM_R {
        UPRSM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - Speed Configuration"]
    #[inline(always)]
    pub fn spdconf(&self) -> SPDCONF_R {
        SPDCONF_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 4 - No Reply"]
    #[inline(always)]
    pub fn nreply(&self) -> NREPLY_R {
        NREPLY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Test mode J"]
    #[inline(always)]
    pub fn tstj(&self) -> TSTJ_R {
        TSTJ_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Test mode K"]
    #[inline(always)]
    pub fn tstk(&self) -> TSTK_R {
        TSTK_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Test packet mode"]
    #[inline(always)]
    pub fn tstpckt(&self) -> TSTPCKT_R {
        TSTPCKT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Specific Operational Mode"]
    #[inline(always)]
    pub fn opmode2(&self) -> OPMODE2_R {
        OPMODE2_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Global NAK"]
    #[inline(always)]
    pub fn gnak(&self) -> GNAK_R {
        GNAK_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 10:11 - Link Power Management Handshake"]
    #[inline(always)]
    pub fn lpmhdsk(&self) -> LPMHDSK_R {
        LPMHDSK_R::new(((self.bits >> 10) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Detach"]
    #[inline(always)]
    pub fn detach(&mut self) -> DETACH_W {
        DETACH_W { w: self }
    }
    #[doc = "Bit 1 - Upstream Resume"]
    #[inline(always)]
    pub fn uprsm(&mut self) -> UPRSM_W {
        UPRSM_W { w: self }
    }
    #[doc = "Bits 2:3 - Speed Configuration"]
    #[inline(always)]
    pub fn spdconf(&mut self) -> SPDCONF_W {
        SPDCONF_W { w: self }
    }
    #[doc = "Bit 4 - No Reply"]
    #[inline(always)]
    pub fn nreply(&mut self) -> NREPLY_W {
        NREPLY_W { w: self }
    }
    #[doc = "Bit 5 - Test mode J"]
    #[inline(always)]
    pub fn tstj(&mut self) -> TSTJ_W {
        TSTJ_W { w: self }
    }
    #[doc = "Bit 6 - Test mode K"]
    #[inline(always)]
    pub fn tstk(&mut self) -> TSTK_W {
        TSTK_W { w: self }
    }
    #[doc = "Bit 7 - Test packet mode"]
    #[inline(always)]
    pub fn tstpckt(&mut self) -> TSTPCKT_W {
        TSTPCKT_W { w: self }
    }
    #[doc = "Bit 8 - Specific Operational Mode"]
    #[inline(always)]
    pub fn opmode2(&mut self) -> OPMODE2_W {
        OPMODE2_W { w: self }
    }
    #[doc = "Bit 9 - Global NAK"]
    #[inline(always)]
    pub fn gnak(&mut self) -> GNAK_W {
        GNAK_W { w: self }
    }
    #[doc = "Bits 10:11 - Link Power Management Handshake"]
    #[inline(always)]
    pub fn lpmhdsk(&mut self) -> LPMHDSK_W {
        LPMHDSK_W { w: self }
    }
}
