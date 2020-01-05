#[doc = "Reader of register INTENCLR"]
pub type R = crate::R<u16, super::INTENCLR>;
#[doc = "Writer for register INTENCLR"]
pub type W = crate::W<u16, super::INTENCLR>;
#[doc = "Register INTENCLR `reset()`'s with value 0"]
impl crate::ResetValue for super::INTENCLR {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Suspend Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SUSPEND_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<SUSPEND_A> for bool {
    #[inline(always)]
    fn from(variant: SUSPEND_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SUSPEND`"]
pub type SUSPEND_R = crate::R<bool, SUSPEND_A>;
impl SUSPEND_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SUSPEND_A {
        match self.bits {
            false => SUSPEND_A::DISABLED,
            true => SUSPEND_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SUSPEND_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SUSPEND_A::ENABLED
    }
}
#[doc = "Suspend Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SUSPEND_AW {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Set disabled"]
    DISABLE = 1,
}
impl From<SUSPEND_AW> for bool {
    #[inline(always)]
    fn from(variant: SUSPEND_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `SUSPEND`"]
pub struct SUSPEND_W<'a> {
    w: &'a mut W,
}
impl<'a> SUSPEND_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SUSPEND_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(SUSPEND_AW::NOP)
    }
    #[doc = "Set disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SUSPEND_AW::DISABLE)
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
#[doc = "Micro Start of Frame Interrupt Enable in High Speed Mode"]
pub type MSOF_A = SUSPEND_A;
#[doc = "Reader of field `MSOF`"]
pub type MSOF_R = crate::R<bool, SUSPEND_A>;
#[doc = "Micro Start of Frame Interrupt Enable in High Speed Mode"]
pub type MSOF_AW = SUSPEND_AW;
#[doc = "Write proxy for field `MSOF`"]
pub struct MSOF_W<'a> {
    w: &'a mut W,
}
impl<'a> MSOF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSOF_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(SUSPEND_AW::NOP)
    }
    #[doc = "Set disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SUSPEND_AW::DISABLE)
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
#[doc = "Start Of Frame Interrupt Enable"]
pub type SOF_A = SUSPEND_A;
#[doc = "Reader of field `SOF`"]
pub type SOF_R = crate::R<bool, SUSPEND_A>;
#[doc = "Start Of Frame Interrupt Enable"]
pub type SOF_AW = SUSPEND_AW;
#[doc = "Write proxy for field `SOF`"]
pub struct SOF_W<'a> {
    w: &'a mut W,
}
impl<'a> SOF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SOF_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(SUSPEND_AW::NOP)
    }
    #[doc = "Set disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SUSPEND_AW::DISABLE)
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
#[doc = "End of Reset Interrupt Enable"]
pub type EORST_A = SUSPEND_A;
#[doc = "Reader of field `EORST`"]
pub type EORST_R = crate::R<bool, SUSPEND_A>;
#[doc = "End of Reset Interrupt Enable"]
pub type EORST_AW = SUSPEND_AW;
#[doc = "Write proxy for field `EORST`"]
pub struct EORST_W<'a> {
    w: &'a mut W,
}
impl<'a> EORST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EORST_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(SUSPEND_AW::NOP)
    }
    #[doc = "Set disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SUSPEND_AW::DISABLE)
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
#[doc = "Wake Up Interrupt Enable"]
pub type WAKEUP_A = SUSPEND_A;
#[doc = "Reader of field `WAKEUP`"]
pub type WAKEUP_R = crate::R<bool, SUSPEND_A>;
#[doc = "Wake Up Interrupt Enable"]
pub type WAKEUP_AW = SUSPEND_AW;
#[doc = "Write proxy for field `WAKEUP`"]
pub struct WAKEUP_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEUP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WAKEUP_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(SUSPEND_AW::NOP)
    }
    #[doc = "Set disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SUSPEND_AW::DISABLE)
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
#[doc = "End Of Resume Interrupt Enable"]
pub type EORSM_A = SUSPEND_A;
#[doc = "Reader of field `EORSM`"]
pub type EORSM_R = crate::R<bool, SUSPEND_A>;
#[doc = "End Of Resume Interrupt Enable"]
pub type EORSM_AW = SUSPEND_AW;
#[doc = "Write proxy for field `EORSM`"]
pub struct EORSM_W<'a> {
    w: &'a mut W,
}
impl<'a> EORSM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EORSM_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(SUSPEND_AW::NOP)
    }
    #[doc = "Set disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SUSPEND_AW::DISABLE)
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
#[doc = "Upstream Resume Interrupt Enable"]
pub type UPRSM_A = SUSPEND_A;
#[doc = "Reader of field `UPRSM`"]
pub type UPRSM_R = crate::R<bool, SUSPEND_A>;
#[doc = "Upstream Resume Interrupt Enable"]
pub type UPRSM_AW = SUSPEND_AW;
#[doc = "Write proxy for field `UPRSM`"]
pub struct UPRSM_W<'a> {
    w: &'a mut W,
}
impl<'a> UPRSM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UPRSM_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(SUSPEND_AW::NOP)
    }
    #[doc = "Set disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SUSPEND_AW::DISABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u16) & 0x01) << 6);
        self.w
    }
}
#[doc = "Ram Access Interrupt Enable"]
pub type RAMACER_A = SUSPEND_A;
#[doc = "Reader of field `RAMACER`"]
pub type RAMACER_R = crate::R<bool, SUSPEND_A>;
#[doc = "Ram Access Interrupt Enable"]
pub type RAMACER_AW = SUSPEND_AW;
#[doc = "Write proxy for field `RAMACER`"]
pub struct RAMACER_W<'a> {
    w: &'a mut W,
}
impl<'a> RAMACER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RAMACER_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(SUSPEND_AW::NOP)
    }
    #[doc = "Set disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SUSPEND_AW::DISABLE)
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
#[doc = "Link Power Management Not Yet Interrupt Enable"]
pub type LPMNYET_A = SUSPEND_A;
#[doc = "Reader of field `LPMNYET`"]
pub type LPMNYET_R = crate::R<bool, SUSPEND_A>;
#[doc = "Link Power Management Not Yet Interrupt Enable"]
pub type LPMNYET_AW = SUSPEND_AW;
#[doc = "Write proxy for field `LPMNYET`"]
pub struct LPMNYET_W<'a> {
    w: &'a mut W,
}
impl<'a> LPMNYET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPMNYET_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(SUSPEND_AW::NOP)
    }
    #[doc = "Set disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SUSPEND_AW::DISABLE)
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
#[doc = "Link Power Management Suspend Interrupt Enable"]
pub type LPMSUSP_A = SUSPEND_A;
#[doc = "Reader of field `LPMSUSP`"]
pub type LPMSUSP_R = crate::R<bool, SUSPEND_A>;
#[doc = "Link Power Management Suspend Interrupt Enable"]
pub type LPMSUSP_AW = SUSPEND_AW;
#[doc = "Write proxy for field `LPMSUSP`"]
pub struct LPMSUSP_W<'a> {
    w: &'a mut W,
}
impl<'a> LPMSUSP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPMSUSP_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(SUSPEND_AW::NOP)
    }
    #[doc = "Set disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SUSPEND_AW::DISABLE)
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
impl R {
    #[doc = "Bit 0 - Suspend Interrupt Enable"]
    #[inline(always)]
    pub fn suspend(&self) -> SUSPEND_R {
        SUSPEND_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Micro Start of Frame Interrupt Enable in High Speed Mode"]
    #[inline(always)]
    pub fn msof(&self) -> MSOF_R {
        MSOF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Start Of Frame Interrupt Enable"]
    #[inline(always)]
    pub fn sof(&self) -> SOF_R {
        SOF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - End of Reset Interrupt Enable"]
    #[inline(always)]
    pub fn eorst(&self) -> EORST_R {
        EORST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Wake Up Interrupt Enable"]
    #[inline(always)]
    pub fn wakeup(&self) -> WAKEUP_R {
        WAKEUP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - End Of Resume Interrupt Enable"]
    #[inline(always)]
    pub fn eorsm(&self) -> EORSM_R {
        EORSM_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Upstream Resume Interrupt Enable"]
    #[inline(always)]
    pub fn uprsm(&self) -> UPRSM_R {
        UPRSM_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Ram Access Interrupt Enable"]
    #[inline(always)]
    pub fn ramacer(&self) -> RAMACER_R {
        RAMACER_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Link Power Management Not Yet Interrupt Enable"]
    #[inline(always)]
    pub fn lpmnyet(&self) -> LPMNYET_R {
        LPMNYET_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Link Power Management Suspend Interrupt Enable"]
    #[inline(always)]
    pub fn lpmsusp(&self) -> LPMSUSP_R {
        LPMSUSP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Suspend Interrupt Enable"]
    #[inline(always)]
    pub fn suspend(&mut self) -> SUSPEND_W {
        SUSPEND_W { w: self }
    }
    #[doc = "Bit 1 - Micro Start of Frame Interrupt Enable in High Speed Mode"]
    #[inline(always)]
    pub fn msof(&mut self) -> MSOF_W {
        MSOF_W { w: self }
    }
    #[doc = "Bit 2 - Start Of Frame Interrupt Enable"]
    #[inline(always)]
    pub fn sof(&mut self) -> SOF_W {
        SOF_W { w: self }
    }
    #[doc = "Bit 3 - End of Reset Interrupt Enable"]
    #[inline(always)]
    pub fn eorst(&mut self) -> EORST_W {
        EORST_W { w: self }
    }
    #[doc = "Bit 4 - Wake Up Interrupt Enable"]
    #[inline(always)]
    pub fn wakeup(&mut self) -> WAKEUP_W {
        WAKEUP_W { w: self }
    }
    #[doc = "Bit 5 - End Of Resume Interrupt Enable"]
    #[inline(always)]
    pub fn eorsm(&mut self) -> EORSM_W {
        EORSM_W { w: self }
    }
    #[doc = "Bit 6 - Upstream Resume Interrupt Enable"]
    #[inline(always)]
    pub fn uprsm(&mut self) -> UPRSM_W {
        UPRSM_W { w: self }
    }
    #[doc = "Bit 7 - Ram Access Interrupt Enable"]
    #[inline(always)]
    pub fn ramacer(&mut self) -> RAMACER_W {
        RAMACER_W { w: self }
    }
    #[doc = "Bit 8 - Link Power Management Not Yet Interrupt Enable"]
    #[inline(always)]
    pub fn lpmnyet(&mut self) -> LPMNYET_W {
        LPMNYET_W { w: self }
    }
    #[doc = "Bit 9 - Link Power Management Suspend Interrupt Enable"]
    #[inline(always)]
    pub fn lpmsusp(&mut self) -> LPMSUSP_W {
        LPMSUSP_W { w: self }
    }
}
