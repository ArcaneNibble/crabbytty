#[doc = "Writer for register EPSTATUSSET%s"]
pub type W = crate::W<u8, super::EPSTATUSSET>;
#[doc = "Register EPSTATUSSET%s `reset()`'s with value 0"]
impl crate::ResetValue for super::EPSTATUSSET {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Data Toggle OUT Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTGLOUT_AW {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Set flag"]
    SET = 1,
}
impl From<DTGLOUT_AW> for bool {
    #[inline(always)]
    fn from(variant: DTGLOUT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `DTGLOUT`"]
pub struct DTGLOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> DTGLOUT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DTGLOUT_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DTGLOUT_AW::NOP)
    }
    #[doc = "Set flag"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(DTGLOUT_AW::SET)
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
#[doc = "Data Toggle IN Set"]
pub type DTGLIN_AW = DTGLOUT_AW;
#[doc = "Write proxy for field `DTGLIN`"]
pub struct DTGLIN_W<'a> {
    w: &'a mut W,
}
impl<'a> DTGLIN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DTGLIN_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DTGLOUT_AW::NOP)
    }
    #[doc = "Set flag"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(DTGLOUT_AW::SET)
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
#[doc = "Current Bank Set"]
pub type CURBK_AW = DTGLOUT_AW;
#[doc = "Write proxy for field `CURBK`"]
pub struct CURBK_W<'a> {
    w: &'a mut W,
}
impl<'a> CURBK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CURBK_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DTGLOUT_AW::NOP)
    }
    #[doc = "Set flag"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(DTGLOUT_AW::SET)
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
#[doc = "Stall 0 Request Set"]
pub type STALLRQ0_AW = DTGLOUT_AW;
#[doc = "Write proxy for field `STALLRQ0`"]
pub struct STALLRQ0_W<'a> {
    w: &'a mut W,
}
impl<'a> STALLRQ0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STALLRQ0_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DTGLOUT_AW::NOP)
    }
    #[doc = "Set flag"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(DTGLOUT_AW::SET)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u8) & 0x01) << 4);
        self.w
    }
}
#[doc = "Stall 1 Request Set"]
pub type STALLRQ1_AW = DTGLOUT_AW;
#[doc = "Write proxy for field `STALLRQ1`"]
pub struct STALLRQ1_W<'a> {
    w: &'a mut W,
}
impl<'a> STALLRQ1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STALLRQ1_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DTGLOUT_AW::NOP)
    }
    #[doc = "Set flag"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(DTGLOUT_AW::SET)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u8) & 0x01) << 5);
        self.w
    }
}
#[doc = "Bank 0 Ready Set"]
pub type BK0RDY_AW = DTGLOUT_AW;
#[doc = "Write proxy for field `BK0RDY`"]
pub struct BK0RDY_W<'a> {
    w: &'a mut W,
}
impl<'a> BK0RDY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BK0RDY_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DTGLOUT_AW::NOP)
    }
    #[doc = "Set flag"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(DTGLOUT_AW::SET)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u8) & 0x01) << 6);
        self.w
    }
}
#[doc = "Bank 1 Ready Set"]
pub type BK1RDY_AW = DTGLOUT_AW;
#[doc = "Write proxy for field `BK1RDY`"]
pub struct BK1RDY_W<'a> {
    w: &'a mut W,
}
impl<'a> BK1RDY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BK1RDY_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DTGLOUT_AW::NOP)
    }
    #[doc = "Set flag"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(DTGLOUT_AW::SET)
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
impl W {
    #[doc = "Bit 0 - Data Toggle OUT Set"]
    #[inline(always)]
    pub fn dtglout(&mut self) -> DTGLOUT_W {
        DTGLOUT_W { w: self }
    }
    #[doc = "Bit 1 - Data Toggle IN Set"]
    #[inline(always)]
    pub fn dtglin(&mut self) -> DTGLIN_W {
        DTGLIN_W { w: self }
    }
    #[doc = "Bit 2 - Current Bank Set"]
    #[inline(always)]
    pub fn curbk(&mut self) -> CURBK_W {
        CURBK_W { w: self }
    }
    #[doc = "Bit 4 - Stall 0 Request Set"]
    #[inline(always)]
    pub fn stallrq0(&mut self) -> STALLRQ0_W {
        STALLRQ0_W { w: self }
    }
    #[doc = "Bit 5 - Stall 1 Request Set"]
    #[inline(always)]
    pub fn stallrq1(&mut self) -> STALLRQ1_W {
        STALLRQ1_W { w: self }
    }
    #[doc = "Bit 6 - Bank 0 Ready Set"]
    #[inline(always)]
    pub fn bk0rdy(&mut self) -> BK0RDY_W {
        BK0RDY_W { w: self }
    }
    #[doc = "Bit 7 - Bank 1 Ready Set"]
    #[inline(always)]
    pub fn bk1rdy(&mut self) -> BK1RDY_W {
        BK1RDY_W { w: self }
    }
}
