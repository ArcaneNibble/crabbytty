#[doc = "Reader of register DFLLCTRL"]
pub type R = crate::R<u16, super::DFLLCTRL>;
#[doc = "Writer for register DFLLCTRL"]
pub type W = crate::W<u16, super::DFLLCTRL>;
#[doc = "Register DFLLCTRL `reset()`'s with value 0x80"]
impl crate::ResetValue for super::DFLLCTRL {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x80
    }
}
#[doc = "DFLL Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLE_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ENABLE`"]
pub type ENABLE_R = crate::R<bool, ENABLE_A>;
impl ENABLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLE_A {
        match self.bits {
            false => ENABLE_A::DISABLED,
            true => ENABLE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENABLE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENABLE_A::ENABLED
    }
}
#[doc = "Write proxy for field `ENABLE`"]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENABLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENABLE_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENABLE_A::ENABLED)
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
#[doc = "Operating Mode Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE_A {
    #[doc = "0: Open-loop"]
    OPENLOOP = 0,
    #[doc = "1: Closed-loop"]
    CLOSEDLOOP = 1,
}
impl From<MODE_A> for bool {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MODE`"]
pub type MODE_R = crate::R<bool, MODE_A>;
impl MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            false => MODE_A::OPENLOOP,
            true => MODE_A::CLOSEDLOOP,
        }
    }
    #[doc = "Checks if the value of the field is `OPENLOOP`"]
    #[inline(always)]
    pub fn is_openloop(&self) -> bool {
        *self == MODE_A::OPENLOOP
    }
    #[doc = "Checks if the value of the field is `CLOSEDLOOP`"]
    #[inline(always)]
    pub fn is_closedloop(&self) -> bool {
        *self == MODE_A::CLOSEDLOOP
    }
}
#[doc = "Write proxy for field `MODE`"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Open-loop"]
    #[inline(always)]
    pub fn openloop(self) -> &'a mut W {
        self.variant(MODE_A::OPENLOOP)
    }
    #[doc = "Closed-loop"]
    #[inline(always)]
    pub fn closedloop(self) -> &'a mut W {
        self.variant(MODE_A::CLOSEDLOOP)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u16) & 0x01) << 2);
        self.w
    }
}
#[doc = "Stable DFLL Frequency\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STABLE_A {
    #[doc = "0: Track after lock"]
    TRACK = 0,
    #[doc = "1: Fixed after lock"]
    FIXED = 1,
}
impl From<STABLE_A> for bool {
    #[inline(always)]
    fn from(variant: STABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `STABLE`"]
pub type STABLE_R = crate::R<bool, STABLE_A>;
impl STABLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STABLE_A {
        match self.bits {
            false => STABLE_A::TRACK,
            true => STABLE_A::FIXED,
        }
    }
    #[doc = "Checks if the value of the field is `TRACK`"]
    #[inline(always)]
    pub fn is_track(&self) -> bool {
        *self == STABLE_A::TRACK
    }
    #[doc = "Checks if the value of the field is `FIXED`"]
    #[inline(always)]
    pub fn is_fixed(&self) -> bool {
        *self == STABLE_A::FIXED
    }
}
#[doc = "Write proxy for field `STABLE`"]
pub struct STABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> STABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STABLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Track after lock"]
    #[inline(always)]
    pub fn track(self) -> &'a mut W {
        self.variant(STABLE_A::TRACK)
    }
    #[doc = "Fixed after lock"]
    #[inline(always)]
    pub fn fixed(self) -> &'a mut W {
        self.variant(STABLE_A::FIXED)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u16) & 0x01) << 3);
        self.w
    }
}
#[doc = "Lose Lock After Wake\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LLAW_A {
    #[doc = "0: Keep lock"]
    KEEP = 0,
    #[doc = "1: Lose lock"]
    LOSE = 1,
}
impl From<LLAW_A> for bool {
    #[inline(always)]
    fn from(variant: LLAW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LLAW`"]
pub type LLAW_R = crate::R<bool, LLAW_A>;
impl LLAW_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LLAW_A {
        match self.bits {
            false => LLAW_A::KEEP,
            true => LLAW_A::LOSE,
        }
    }
    #[doc = "Checks if the value of the field is `KEEP`"]
    #[inline(always)]
    pub fn is_keep(&self) -> bool {
        *self == LLAW_A::KEEP
    }
    #[doc = "Checks if the value of the field is `LOSE`"]
    #[inline(always)]
    pub fn is_lose(&self) -> bool {
        *self == LLAW_A::LOSE
    }
}
#[doc = "Write proxy for field `LLAW`"]
pub struct LLAW_W<'a> {
    w: &'a mut W,
}
impl<'a> LLAW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LLAW_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Keep lock"]
    #[inline(always)]
    pub fn keep(self) -> &'a mut W {
        self.variant(LLAW_A::KEEP)
    }
    #[doc = "Lose lock"]
    #[inline(always)]
    pub fn lose(self) -> &'a mut W {
        self.variant(LLAW_A::LOSE)
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
#[doc = "USB Clock Recovery Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBCRM_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<USBCRM_A> for bool {
    #[inline(always)]
    fn from(variant: USBCRM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `USBCRM`"]
pub type USBCRM_R = crate::R<bool, USBCRM_A>;
impl USBCRM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBCRM_A {
        match self.bits {
            false => USBCRM_A::DISABLED,
            true => USBCRM_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == USBCRM_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == USBCRM_A::ENABLED
    }
}
#[doc = "Write proxy for field `USBCRM`"]
pub struct USBCRM_W<'a> {
    w: &'a mut W,
}
impl<'a> USBCRM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBCRM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(USBCRM_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(USBCRM_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u16) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `RUNSTDBY`"]
pub type RUNSTDBY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RUNSTDBY`"]
pub struct RUNSTDBY_W<'a> {
    w: &'a mut W,
}
impl<'a> RUNSTDBY_W<'a> {
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
#[doc = "On Demand Control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ONDEMAND_A {
    #[doc = "0: Always on"]
    ALWAYSON = 0,
    #[doc = "1: On demand"]
    ONDEMAND = 1,
}
impl From<ONDEMAND_A> for bool {
    #[inline(always)]
    fn from(variant: ONDEMAND_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ONDEMAND`"]
pub type ONDEMAND_R = crate::R<bool, ONDEMAND_A>;
impl ONDEMAND_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ONDEMAND_A {
        match self.bits {
            false => ONDEMAND_A::ALWAYSON,
            true => ONDEMAND_A::ONDEMAND,
        }
    }
    #[doc = "Checks if the value of the field is `ALWAYSON`"]
    #[inline(always)]
    pub fn is_alwayson(&self) -> bool {
        *self == ONDEMAND_A::ALWAYSON
    }
    #[doc = "Checks if the value of the field is `ONDEMAND`"]
    #[inline(always)]
    pub fn is_ondemand(&self) -> bool {
        *self == ONDEMAND_A::ONDEMAND
    }
}
#[doc = "Write proxy for field `ONDEMAND`"]
pub struct ONDEMAND_W<'a> {
    w: &'a mut W,
}
impl<'a> ONDEMAND_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ONDEMAND_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Always on"]
    #[inline(always)]
    pub fn alwayson(self) -> &'a mut W {
        self.variant(ONDEMAND_A::ALWAYSON)
    }
    #[doc = "On demand"]
    #[inline(always)]
    pub fn ondemand(self) -> &'a mut W {
        self.variant(ONDEMAND_A::ONDEMAND)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u16) & 0x01) << 7);
        self.w
    }
}
#[doc = "Chill Cycle Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCDIS_A {
    #[doc = "0: Chill Cycle enabled"]
    CC = 0,
    #[doc = "1: Chill Cycle disabled"]
    NOCC = 1,
}
impl From<CCDIS_A> for bool {
    #[inline(always)]
    fn from(variant: CCDIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CCDIS`"]
pub type CCDIS_R = crate::R<bool, CCDIS_A>;
impl CCDIS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCDIS_A {
        match self.bits {
            false => CCDIS_A::CC,
            true => CCDIS_A::NOCC,
        }
    }
    #[doc = "Checks if the value of the field is `CC`"]
    #[inline(always)]
    pub fn is_cc(&self) -> bool {
        *self == CCDIS_A::CC
    }
    #[doc = "Checks if the value of the field is `NOCC`"]
    #[inline(always)]
    pub fn is_nocc(&self) -> bool {
        *self == CCDIS_A::NOCC
    }
}
#[doc = "Write proxy for field `CCDIS`"]
pub struct CCDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> CCDIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCDIS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Chill Cycle enabled"]
    #[inline(always)]
    pub fn cc(self) -> &'a mut W {
        self.variant(CCDIS_A::CC)
    }
    #[doc = "Chill Cycle disabled"]
    #[inline(always)]
    pub fn nocc(self) -> &'a mut W {
        self.variant(CCDIS_A::NOCC)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u16) & 0x01) << 8);
        self.w
    }
}
#[doc = "Quick Lock Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QLDIS_A {
    #[doc = "0: Quick Lock enabled"]
    QL = 0,
    #[doc = "1: Quick Lock disabled"]
    NOQL = 1,
}
impl From<QLDIS_A> for bool {
    #[inline(always)]
    fn from(variant: QLDIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `QLDIS`"]
pub type QLDIS_R = crate::R<bool, QLDIS_A>;
impl QLDIS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> QLDIS_A {
        match self.bits {
            false => QLDIS_A::QL,
            true => QLDIS_A::NOQL,
        }
    }
    #[doc = "Checks if the value of the field is `QL`"]
    #[inline(always)]
    pub fn is_ql(&self) -> bool {
        *self == QLDIS_A::QL
    }
    #[doc = "Checks if the value of the field is `NOQL`"]
    #[inline(always)]
    pub fn is_noql(&self) -> bool {
        *self == QLDIS_A::NOQL
    }
}
#[doc = "Write proxy for field `QLDIS`"]
pub struct QLDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> QLDIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: QLDIS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Quick Lock enabled"]
    #[inline(always)]
    pub fn ql(self) -> &'a mut W {
        self.variant(QLDIS_A::QL)
    }
    #[doc = "Quick Lock disabled"]
    #[inline(always)]
    pub fn noql(self) -> &'a mut W {
        self.variant(QLDIS_A::NOQL)
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
#[doc = "Bypass Coarse Lock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BPLCKC_A {
    #[doc = "0: Do not bypass"]
    NOBYPASS = 0,
    #[doc = "1: Bypass"]
    BYPASS = 1,
}
impl From<BPLCKC_A> for bool {
    #[inline(always)]
    fn from(variant: BPLCKC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BPLCKC`"]
pub type BPLCKC_R = crate::R<bool, BPLCKC_A>;
impl BPLCKC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BPLCKC_A {
        match self.bits {
            false => BPLCKC_A::NOBYPASS,
            true => BPLCKC_A::BYPASS,
        }
    }
    #[doc = "Checks if the value of the field is `NOBYPASS`"]
    #[inline(always)]
    pub fn is_nobypass(&self) -> bool {
        *self == BPLCKC_A::NOBYPASS
    }
    #[doc = "Checks if the value of the field is `BYPASS`"]
    #[inline(always)]
    pub fn is_bypass(&self) -> bool {
        *self == BPLCKC_A::BYPASS
    }
}
#[doc = "Write proxy for field `BPLCKC`"]
pub struct BPLCKC_W<'a> {
    w: &'a mut W,
}
impl<'a> BPLCKC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BPLCKC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Do not bypass"]
    #[inline(always)]
    pub fn nobypass(self) -> &'a mut W {
        self.variant(BPLCKC_A::NOBYPASS)
    }
    #[doc = "Bypass"]
    #[inline(always)]
    pub fn bypass(self) -> &'a mut W {
        self.variant(BPLCKC_A::BYPASS)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u16) & 0x01) << 10);
        self.w
    }
}
#[doc = "Wait Lock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAITLOCK_A {
    #[doc = "0: Output before lock"]
    BEFORELOCK = 0,
    #[doc = "1: Output after lock"]
    AFTERLOCK = 1,
}
impl From<WAITLOCK_A> for bool {
    #[inline(always)]
    fn from(variant: WAITLOCK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WAITLOCK`"]
pub type WAITLOCK_R = crate::R<bool, WAITLOCK_A>;
impl WAITLOCK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAITLOCK_A {
        match self.bits {
            false => WAITLOCK_A::BEFORELOCK,
            true => WAITLOCK_A::AFTERLOCK,
        }
    }
    #[doc = "Checks if the value of the field is `BEFORELOCK`"]
    #[inline(always)]
    pub fn is_beforelock(&self) -> bool {
        *self == WAITLOCK_A::BEFORELOCK
    }
    #[doc = "Checks if the value of the field is `AFTERLOCK`"]
    #[inline(always)]
    pub fn is_afterlock(&self) -> bool {
        *self == WAITLOCK_A::AFTERLOCK
    }
}
#[doc = "Write proxy for field `WAITLOCK`"]
pub struct WAITLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> WAITLOCK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WAITLOCK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Output before lock"]
    #[inline(always)]
    pub fn beforelock(self) -> &'a mut W {
        self.variant(WAITLOCK_A::BEFORELOCK)
    }
    #[doc = "Output after lock"]
    #[inline(always)]
    pub fn afterlock(self) -> &'a mut W {
        self.variant(WAITLOCK_A::AFTERLOCK)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u16) & 0x01) << 11);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - DFLL Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Operating Mode Selection"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Stable DFLL Frequency"]
    #[inline(always)]
    pub fn stable(&self) -> STABLE_R {
        STABLE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Lose Lock After Wake"]
    #[inline(always)]
    pub fn llaw(&self) -> LLAW_R {
        LLAW_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - USB Clock Recovery Mode"]
    #[inline(always)]
    pub fn usbcrm(&self) -> USBCRM_R {
        USBCRM_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RUNSTDBY_R {
        RUNSTDBY_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - On Demand Control"]
    #[inline(always)]
    pub fn ondemand(&self) -> ONDEMAND_R {
        ONDEMAND_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Chill Cycle Disable"]
    #[inline(always)]
    pub fn ccdis(&self) -> CCDIS_R {
        CCDIS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Quick Lock Disable"]
    #[inline(always)]
    pub fn qldis(&self) -> QLDIS_R {
        QLDIS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Bypass Coarse Lock"]
    #[inline(always)]
    pub fn bplckc(&self) -> BPLCKC_R {
        BPLCKC_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Wait Lock"]
    #[inline(always)]
    pub fn waitlock(&self) -> WAITLOCK_R {
        WAITLOCK_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - DFLL Enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Bit 2 - Operating Mode Selection"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bit 3 - Stable DFLL Frequency"]
    #[inline(always)]
    pub fn stable(&mut self) -> STABLE_W {
        STABLE_W { w: self }
    }
    #[doc = "Bit 4 - Lose Lock After Wake"]
    #[inline(always)]
    pub fn llaw(&mut self) -> LLAW_W {
        LLAW_W { w: self }
    }
    #[doc = "Bit 5 - USB Clock Recovery Mode"]
    #[inline(always)]
    pub fn usbcrm(&mut self) -> USBCRM_W {
        USBCRM_W { w: self }
    }
    #[doc = "Bit 6 - Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&mut self) -> RUNSTDBY_W {
        RUNSTDBY_W { w: self }
    }
    #[doc = "Bit 7 - On Demand Control"]
    #[inline(always)]
    pub fn ondemand(&mut self) -> ONDEMAND_W {
        ONDEMAND_W { w: self }
    }
    #[doc = "Bit 8 - Chill Cycle Disable"]
    #[inline(always)]
    pub fn ccdis(&mut self) -> CCDIS_W {
        CCDIS_W { w: self }
    }
    #[doc = "Bit 9 - Quick Lock Disable"]
    #[inline(always)]
    pub fn qldis(&mut self) -> QLDIS_W {
        QLDIS_W { w: self }
    }
    #[doc = "Bit 10 - Bypass Coarse Lock"]
    #[inline(always)]
    pub fn bplckc(&mut self) -> BPLCKC_W {
        BPLCKC_W { w: self }
    }
    #[doc = "Bit 11 - Wait Lock"]
    #[inline(always)]
    pub fn waitlock(&mut self) -> WAITLOCK_W {
        WAITLOCK_W { w: self }
    }
}
