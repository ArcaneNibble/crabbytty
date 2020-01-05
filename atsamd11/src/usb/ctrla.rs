#[doc = "Reader of register CTRLA"]
pub type R = crate::R<u8, super::CTRLA>;
#[doc = "Writer for register CTRLA"]
pub type W = crate::W<u8, super::CTRLA>;
#[doc = "Register CTRLA `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRLA {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Software Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWRST_A {
    #[doc = "0: Not resetting"]
    NOP = 0,
    #[doc = "1: Reset"]
    RESET = 1,
}
impl From<SWRST_A> for bool {
    #[inline(always)]
    fn from(variant: SWRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SWRST`"]
pub type SWRST_R = crate::R<bool, SWRST_A>;
impl SWRST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWRST_A {
        match self.bits {
            false => SWRST_A::NOP,
            true => SWRST_A::RESET,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == SWRST_A::NOP
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == SWRST_A::RESET
    }
}
#[doc = "Write proxy for field `SWRST`"]
pub struct SWRST_W<'a> {
    w: &'a mut W,
}
impl<'a> SWRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWRST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Not resetting"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(SWRST_A::NOP)
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(SWRST_A::RESET)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u8) & 0x01);
        self.w
    }
}
#[doc = "Enable\n\nValue on reset: 0"]
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u8) & 0x01) << 1);
        self.w
    }
}
#[doc = "Run in Standby Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RUNSTDBY_A {
    #[doc = "0: Stopped in standby"]
    STOPPED = 0,
    #[doc = "1: Running in standby"]
    RUNNING = 1,
}
impl From<RUNSTDBY_A> for bool {
    #[inline(always)]
    fn from(variant: RUNSTDBY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RUNSTDBY`"]
pub type RUNSTDBY_R = crate::R<bool, RUNSTDBY_A>;
impl RUNSTDBY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RUNSTDBY_A {
        match self.bits {
            false => RUNSTDBY_A::STOPPED,
            true => RUNSTDBY_A::RUNNING,
        }
    }
    #[doc = "Checks if the value of the field is `STOPPED`"]
    #[inline(always)]
    pub fn is_stopped(&self) -> bool {
        *self == RUNSTDBY_A::STOPPED
    }
    #[doc = "Checks if the value of the field is `RUNNING`"]
    #[inline(always)]
    pub fn is_running(&self) -> bool {
        *self == RUNSTDBY_A::RUNNING
    }
}
#[doc = "Write proxy for field `RUNSTDBY`"]
pub struct RUNSTDBY_W<'a> {
    w: &'a mut W,
}
impl<'a> RUNSTDBY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RUNSTDBY_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Stopped in standby"]
    #[inline(always)]
    pub fn stopped(self) -> &'a mut W {
        self.variant(RUNSTDBY_A::STOPPED)
    }
    #[doc = "Running in standby"]
    #[inline(always)]
    pub fn running(self) -> &'a mut W {
        self.variant(RUNSTDBY_A::RUNNING)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u8) & 0x01) << 2);
        self.w
    }
}
#[doc = "Operating Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE_A {
    #[doc = "0: Device Mode"]
    DEVICE = 0,
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
    pub fn variant(&self) -> crate::Variant<bool, MODE_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(MODE_A::DEVICE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DEVICE`"]
    #[inline(always)]
    pub fn is_device(&self) -> bool {
        *self == MODE_A::DEVICE
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
    #[doc = "Device Mode"]
    #[inline(always)]
    pub fn device(self) -> &'a mut W {
        self.variant(MODE_A::DEVICE)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u8) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    pub fn swrst(&self) -> SWRST_R {
        SWRST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Run in Standby Mode"]
    #[inline(always)]
    pub fn runstdby(&self) -> RUNSTDBY_R {
        RUNSTDBY_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Operating Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    pub fn swrst(&mut self) -> SWRST_W {
        SWRST_W { w: self }
    }
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Bit 2 - Run in Standby Mode"]
    #[inline(always)]
    pub fn runstdby(&mut self) -> RUNSTDBY_W {
        RUNSTDBY_W { w: self }
    }
    #[doc = "Bit 7 - Operating Mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
}
