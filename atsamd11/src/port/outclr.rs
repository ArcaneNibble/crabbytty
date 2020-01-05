#[doc = "Reader of register OUTCLR%s"]
pub type R = crate::R<u32, super::OUTCLR>;
#[doc = "Writer for register OUTCLR%s"]
pub type W = crate::W<u32, super::OUTCLR>;
#[doc = "Register OUTCLR%s `reset()`'s with value 0"]
impl crate::ResetValue for super::OUTCLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Port Data Output Value Clear 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTCLR0_A {
    #[doc = "0: Output 0"]
    _0 = 0,
    #[doc = "1: Output 1"]
    _1 = 1,
}
impl From<OUTCLR0_A> for bool {
    #[inline(always)]
    fn from(variant: OUTCLR0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUTCLR0`"]
pub type OUTCLR0_R = crate::R<bool, OUTCLR0_A>;
impl OUTCLR0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTCLR0_A {
        match self.bits {
            false => OUTCLR0_A::_0,
            true => OUTCLR0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OUTCLR0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OUTCLR0_A::_1
    }
}
#[doc = "Port Data Output Value Clear 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTCLR0_AW {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Output 0"]
    OUT0 = 1,
}
impl From<OUTCLR0_AW> for bool {
    #[inline(always)]
    fn from(variant: OUTCLR0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `OUTCLR0`"]
pub struct OUTCLR0_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTCLR0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTCLR0_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTCLR0_AW::NOP)
    }
    #[doc = "Output 0"]
    #[inline(always)]
    pub fn out0(self) -> &'a mut W {
        self.variant(OUTCLR0_AW::OUT0)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Port Data Output Value Clear 1"]
pub type OUTCLR1_A = OUTCLR0_A;
#[doc = "Reader of field `OUTCLR1`"]
pub type OUTCLR1_R = crate::R<bool, OUTCLR0_A>;
#[doc = "Port Data Output Value Clear 1"]
pub type OUTCLR1_AW = OUTCLR0_AW;
#[doc = "Write proxy for field `OUTCLR1`"]
pub struct OUTCLR1_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTCLR1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTCLR1_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTCLR0_AW::NOP)
    }
    #[doc = "Output 0"]
    #[inline(always)]
    pub fn out0(self) -> &'a mut W {
        self.variant(OUTCLR0_AW::OUT0)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Port Data Output Value Clear 2"]
pub type OUTCLR2_A = OUTCLR0_A;
#[doc = "Reader of field `OUTCLR2`"]
pub type OUTCLR2_R = crate::R<bool, OUTCLR0_A>;
#[doc = "Port Data Output Value Clear 2"]
pub type OUTCLR2_AW = OUTCLR0_AW;
#[doc = "Write proxy for field `OUTCLR2`"]
pub struct OUTCLR2_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTCLR2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTCLR2_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTCLR0_AW::NOP)
    }
    #[doc = "Output 0"]
    #[inline(always)]
    pub fn out0(self) -> &'a mut W {
        self.variant(OUTCLR0_AW::OUT0)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Port Data Output Value Clear 3"]
pub type OUTCLR3_A = OUTCLR0_A;
#[doc = "Reader of field `OUTCLR3`"]
pub type OUTCLR3_R = crate::R<bool, OUTCLR0_A>;
#[doc = "Port Data Output Value Clear 3"]
pub type OUTCLR3_AW = OUTCLR0_AW;
#[doc = "Write proxy for field `OUTCLR3`"]
pub struct OUTCLR3_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTCLR3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTCLR3_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTCLR0_AW::NOP)
    }
    #[doc = "Output 0"]
    #[inline(always)]
    pub fn out0(self) -> &'a mut W {
        self.variant(OUTCLR0_AW::OUT0)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Port Data Output Value Clear 4"]
pub type OUTCLR4_A = OUTCLR0_A;
#[doc = "Reader of field `OUTCLR4`"]
pub type OUTCLR4_R = crate::R<bool, OUTCLR0_A>;
#[doc = "Port Data Output Value Clear 4"]
pub type OUTCLR4_AW = OUTCLR0_AW;
#[doc = "Write proxy for field `OUTCLR4`"]
pub struct OUTCLR4_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTCLR4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTCLR4_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTCLR0_AW::NOP)
    }
    #[doc = "Output 0"]
    #[inline(always)]
    pub fn out0(self) -> &'a mut W {
        self.variant(OUTCLR0_AW::OUT0)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Port Data Output Value Clear 5"]
pub type OUTCLR5_A = OUTCLR0_A;
#[doc = "Reader of field `OUTCLR5`"]
pub type OUTCLR5_R = crate::R<bool, OUTCLR0_A>;
#[doc = "Port Data Output Value Clear 5"]
pub type OUTCLR5_AW = OUTCLR0_AW;
#[doc = "Write proxy for field `OUTCLR5`"]
pub struct OUTCLR5_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTCLR5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTCLR5_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTCLR0_AW::NOP)
    }
    #[doc = "Output 0"]
    #[inline(always)]
    pub fn out0(self) -> &'a mut W {
        self.variant(OUTCLR0_AW::OUT0)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Port Data Output Value Clear 6"]
pub type OUTCLR6_A = OUTCLR0_A;
#[doc = "Reader of field `OUTCLR6`"]
pub type OUTCLR6_R = crate::R<bool, OUTCLR0_A>;
#[doc = "Port Data Output Value Clear 6"]
pub type OUTCLR6_AW = OUTCLR0_AW;
#[doc = "Write proxy for field `OUTCLR6`"]
pub struct OUTCLR6_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTCLR6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTCLR6_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTCLR0_AW::NOP)
    }
    #[doc = "Output 0"]
    #[inline(always)]
    pub fn out0(self) -> &'a mut W {
        self.variant(OUTCLR0_AW::OUT0)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Port Data Output Value Clear 7"]
pub type OUTCLR7_A = OUTCLR0_A;
#[doc = "Reader of field `OUTCLR7`"]
pub type OUTCLR7_R = crate::R<bool, OUTCLR0_A>;
#[doc = "Port Data Output Value Clear 7"]
pub type OUTCLR7_AW = OUTCLR0_AW;
#[doc = "Write proxy for field `OUTCLR7`"]
pub struct OUTCLR7_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTCLR7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTCLR7_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTCLR0_AW::NOP)
    }
    #[doc = "Output 0"]
    #[inline(always)]
    pub fn out0(self) -> &'a mut W {
        self.variant(OUTCLR0_AW::OUT0)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Port Data Output Value Clear 8"]
pub type OUTCLR8_A = OUTCLR0_A;
#[doc = "Reader of field `OUTCLR8`"]
pub type OUTCLR8_R = crate::R<bool, OUTCLR0_A>;
#[doc = "Port Data Output Value Clear 8"]
pub type OUTCLR8_AW = OUTCLR0_AW;
#[doc = "Write proxy for field `OUTCLR8`"]
pub struct OUTCLR8_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTCLR8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTCLR8_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTCLR0_AW::NOP)
    }
    #[doc = "Output 0"]
    #[inline(always)]
    pub fn out0(self) -> &'a mut W {
        self.variant(OUTCLR0_AW::OUT0)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Port Data Output Value Clear 9"]
pub type OUTCLR9_A = OUTCLR0_A;
#[doc = "Reader of field `OUTCLR9`"]
pub type OUTCLR9_R = crate::R<bool, OUTCLR0_A>;
#[doc = "Port Data Output Value Clear 9"]
pub type OUTCLR9_AW = OUTCLR0_AW;
#[doc = "Write proxy for field `OUTCLR9`"]
pub struct OUTCLR9_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTCLR9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTCLR9_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTCLR0_AW::NOP)
    }
    #[doc = "Output 0"]
    #[inline(always)]
    pub fn out0(self) -> &'a mut W {
        self.variant(OUTCLR0_AW::OUT0)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Port Data Output Value Clear 10"]
pub type OUTCLR10_A = OUTCLR0_A;
#[doc = "Reader of field `OUTCLR10`"]
pub type OUTCLR10_R = crate::R<bool, OUTCLR0_A>;
#[doc = "Port Data Output Value Clear 10"]
pub type OUTCLR10_AW = OUTCLR0_AW;
#[doc = "Write proxy for field `OUTCLR10`"]
pub struct OUTCLR10_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTCLR10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTCLR10_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTCLR0_AW::NOP)
    }
    #[doc = "Output 0"]
    #[inline(always)]
    pub fn out0(self) -> &'a mut W {
        self.variant(OUTCLR0_AW::OUT0)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Port Data Output Value Clear 11"]
pub type OUTCLR11_A = OUTCLR0_A;
#[doc = "Reader of field `OUTCLR11`"]
pub type OUTCLR11_R = crate::R<bool, OUTCLR0_A>;
#[doc = "Port Data Output Value Clear 11"]
pub type OUTCLR11_AW = OUTCLR0_AW;
#[doc = "Write proxy for field `OUTCLR11`"]
pub struct OUTCLR11_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTCLR11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTCLR11_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTCLR0_AW::NOP)
    }
    #[doc = "Output 0"]
    #[inline(always)]
    pub fn out0(self) -> &'a mut W {
        self.variant(OUTCLR0_AW::OUT0)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Port Data Output Value Clear 12"]
pub type OUTCLR12_A = OUTCLR0_A;
#[doc = "Reader of field `OUTCLR12`"]
pub type OUTCLR12_R = crate::R<bool, OUTCLR0_A>;
#[doc = "Port Data Output Value Clear 12"]
pub type OUTCLR12_AW = OUTCLR0_AW;
#[doc = "Write proxy for field `OUTCLR12`"]
pub struct OUTCLR12_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTCLR12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTCLR12_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTCLR0_AW::NOP)
    }
    #[doc = "Output 0"]
    #[inline(always)]
    pub fn out0(self) -> &'a mut W {
        self.variant(OUTCLR0_AW::OUT0)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Port Data Output Value Clear 13"]
pub type OUTCLR13_A = OUTCLR0_A;
#[doc = "Reader of field `OUTCLR13`"]
pub type OUTCLR13_R = crate::R<bool, OUTCLR0_A>;
#[doc = "Port Data Output Value Clear 13"]
pub type OUTCLR13_AW = OUTCLR0_AW;
#[doc = "Write proxy for field `OUTCLR13`"]
pub struct OUTCLR13_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTCLR13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTCLR13_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTCLR0_AW::NOP)
    }
    #[doc = "Output 0"]
    #[inline(always)]
    pub fn out0(self) -> &'a mut W {
        self.variant(OUTCLR0_AW::OUT0)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Port Data Output Value Clear 14"]
pub type OUTCLR14_A = OUTCLR0_A;
#[doc = "Reader of field `OUTCLR14`"]
pub type OUTCLR14_R = crate::R<bool, OUTCLR0_A>;
#[doc = "Port Data Output Value Clear 14"]
pub type OUTCLR14_AW = OUTCLR0_AW;
#[doc = "Write proxy for field `OUTCLR14`"]
pub struct OUTCLR14_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTCLR14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTCLR14_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTCLR0_AW::NOP)
    }
    #[doc = "Output 0"]
    #[inline(always)]
    pub fn out0(self) -> &'a mut W {
        self.variant(OUTCLR0_AW::OUT0)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Port Data Output Value Clear 15"]
pub type OUTCLR15_A = OUTCLR0_A;
#[doc = "Reader of field `OUTCLR15`"]
pub type OUTCLR15_R = crate::R<bool, OUTCLR0_A>;
#[doc = "Port Data Output Value Clear 15"]
pub type OUTCLR15_AW = OUTCLR0_AW;
#[doc = "Write proxy for field `OUTCLR15`"]
pub struct OUTCLR15_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTCLR15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTCLR15_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTCLR0_AW::NOP)
    }
    #[doc = "Output 0"]
    #[inline(always)]
    pub fn out0(self) -> &'a mut W {
        self.variant(OUTCLR0_AW::OUT0)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Port Data Output Value Clear 16"]
pub type OUTCLR16_A = OUTCLR0_A;
#[doc = "Reader of field `OUTCLR16`"]
pub type OUTCLR16_R = crate::R<bool, OUTCLR0_A>;
#[doc = "Port Data Output Value Clear 16"]
pub type OUTCLR16_AW = OUTCLR0_AW;
#[doc = "Write proxy for field `OUTCLR16`"]
pub struct OUTCLR16_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTCLR16_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTCLR16_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTCLR0_AW::NOP)
    }
    #[doc = "Output 0"]
    #[inline(always)]
    pub fn out0(self) -> &'a mut W {
        self.variant(OUTCLR0_AW::OUT0)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Port Data Output Value Clear 17"]
pub type OUTCLR17_A = OUTCLR0_A;
#[doc = "Reader of field `OUTCLR17`"]
pub type OUTCLR17_R = crate::R<bool, OUTCLR0_A>;
#[doc = "Port Data Output Value Clear 17"]
pub type OUTCLR17_AW = OUTCLR0_AW;
#[doc = "Write proxy for field `OUTCLR17`"]
pub struct OUTCLR17_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTCLR17_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTCLR17_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTCLR0_AW::NOP)
    }
    #[doc = "Output 0"]
    #[inline(always)]
    pub fn out0(self) -> &'a mut W {
        self.variant(OUTCLR0_AW::OUT0)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Port Data Output Value Clear 18"]
pub type OUTCLR18_A = OUTCLR0_A;
#[doc = "Reader of field `OUTCLR18`"]
pub type OUTCLR18_R = crate::R<bool, OUTCLR0_A>;
#[doc = "Port Data Output Value Clear 18"]
pub type OUTCLR18_AW = OUTCLR0_AW;
#[doc = "Write proxy for field `OUTCLR18`"]
pub struct OUTCLR18_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTCLR18_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTCLR18_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTCLR0_AW::NOP)
    }
    #[doc = "Output 0"]
    #[inline(always)]
    pub fn out0(self) -> &'a mut W {
        self.variant(OUTCLR0_AW::OUT0)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Port Data Output Value Clear 19"]
pub type OUTCLR19_A = OUTCLR0_A;
#[doc = "Reader of field `OUTCLR19`"]
pub type OUTCLR19_R = crate::R<bool, OUTCLR0_A>;
#[doc = "Port Data Output Value Clear 19"]
pub type OUTCLR19_AW = OUTCLR0_AW;
#[doc = "Write proxy for field `OUTCLR19`"]
pub struct OUTCLR19_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTCLR19_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTCLR19_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTCLR0_AW::NOP)
    }
    #[doc = "Output 0"]
    #[inline(always)]
    pub fn out0(self) -> &'a mut W {
        self.variant(OUTCLR0_AW::OUT0)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Port Data Output Value Clear 20"]
pub type OUTCLR20_A = OUTCLR0_A;
#[doc = "Reader of field `OUTCLR20`"]
pub type OUTCLR20_R = crate::R<bool, OUTCLR0_A>;
#[doc = "Port Data Output Value Clear 20"]
pub type OUTCLR20_AW = OUTCLR0_AW;
#[doc = "Write proxy for field `OUTCLR20`"]
pub struct OUTCLR20_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTCLR20_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTCLR20_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTCLR0_AW::NOP)
    }
    #[doc = "Output 0"]
    #[inline(always)]
    pub fn out0(self) -> &'a mut W {
        self.variant(OUTCLR0_AW::OUT0)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Port Data Output Value Clear 21"]
pub type OUTCLR21_A = OUTCLR0_A;
#[doc = "Reader of field `OUTCLR21`"]
pub type OUTCLR21_R = crate::R<bool, OUTCLR0_A>;
#[doc = "Port Data Output Value Clear 21"]
pub type OUTCLR21_AW = OUTCLR0_AW;
#[doc = "Write proxy for field `OUTCLR21`"]
pub struct OUTCLR21_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTCLR21_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTCLR21_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTCLR0_AW::NOP)
    }
    #[doc = "Output 0"]
    #[inline(always)]
    pub fn out0(self) -> &'a mut W {
        self.variant(OUTCLR0_AW::OUT0)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Port Data Output Value Clear 22"]
pub type OUTCLR22_A = OUTCLR0_A;
#[doc = "Reader of field `OUTCLR22`"]
pub type OUTCLR22_R = crate::R<bool, OUTCLR0_A>;
#[doc = "Port Data Output Value Clear 22"]
pub type OUTCLR22_AW = OUTCLR0_AW;
#[doc = "Write proxy for field `OUTCLR22`"]
pub struct OUTCLR22_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTCLR22_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTCLR22_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTCLR0_AW::NOP)
    }
    #[doc = "Output 0"]
    #[inline(always)]
    pub fn out0(self) -> &'a mut W {
        self.variant(OUTCLR0_AW::OUT0)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Port Data Output Value Clear 23"]
pub type OUTCLR23_A = OUTCLR0_A;
#[doc = "Reader of field `OUTCLR23`"]
pub type OUTCLR23_R = crate::R<bool, OUTCLR0_A>;
#[doc = "Port Data Output Value Clear 23"]
pub type OUTCLR23_AW = OUTCLR0_AW;
#[doc = "Write proxy for field `OUTCLR23`"]
pub struct OUTCLR23_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTCLR23_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTCLR23_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTCLR0_AW::NOP)
    }
    #[doc = "Output 0"]
    #[inline(always)]
    pub fn out0(self) -> &'a mut W {
        self.variant(OUTCLR0_AW::OUT0)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Port Data Output Value Clear 24"]
pub type OUTCLR24_A = OUTCLR0_A;
#[doc = "Reader of field `OUTCLR24`"]
pub type OUTCLR24_R = crate::R<bool, OUTCLR0_A>;
#[doc = "Port Data Output Value Clear 24"]
pub type OUTCLR24_AW = OUTCLR0_AW;
#[doc = "Write proxy for field `OUTCLR24`"]
pub struct OUTCLR24_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTCLR24_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTCLR24_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTCLR0_AW::NOP)
    }
    #[doc = "Output 0"]
    #[inline(always)]
    pub fn out0(self) -> &'a mut W {
        self.variant(OUTCLR0_AW::OUT0)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Port Data Output Value Clear 25"]
pub type OUTCLR25_A = OUTCLR0_A;
#[doc = "Reader of field `OUTCLR25`"]
pub type OUTCLR25_R = crate::R<bool, OUTCLR0_A>;
#[doc = "Port Data Output Value Clear 25"]
pub type OUTCLR25_AW = OUTCLR0_AW;
#[doc = "Write proxy for field `OUTCLR25`"]
pub struct OUTCLR25_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTCLR25_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTCLR25_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTCLR0_AW::NOP)
    }
    #[doc = "Output 0"]
    #[inline(always)]
    pub fn out0(self) -> &'a mut W {
        self.variant(OUTCLR0_AW::OUT0)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Port Data Output Value Clear 26"]
pub type OUTCLR26_A = OUTCLR0_A;
#[doc = "Reader of field `OUTCLR26`"]
pub type OUTCLR26_R = crate::R<bool, OUTCLR0_A>;
#[doc = "Port Data Output Value Clear 26"]
pub type OUTCLR26_AW = OUTCLR0_AW;
#[doc = "Write proxy for field `OUTCLR26`"]
pub struct OUTCLR26_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTCLR26_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTCLR26_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTCLR0_AW::NOP)
    }
    #[doc = "Output 0"]
    #[inline(always)]
    pub fn out0(self) -> &'a mut W {
        self.variant(OUTCLR0_AW::OUT0)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Port Data Output Value Clear 27"]
pub type OUTCLR27_A = OUTCLR0_A;
#[doc = "Reader of field `OUTCLR27`"]
pub type OUTCLR27_R = crate::R<bool, OUTCLR0_A>;
#[doc = "Port Data Output Value Clear 27"]
pub type OUTCLR27_AW = OUTCLR0_AW;
#[doc = "Write proxy for field `OUTCLR27`"]
pub struct OUTCLR27_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTCLR27_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTCLR27_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTCLR0_AW::NOP)
    }
    #[doc = "Output 0"]
    #[inline(always)]
    pub fn out0(self) -> &'a mut W {
        self.variant(OUTCLR0_AW::OUT0)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Port Data Output Value Clear 28"]
pub type OUTCLR28_A = OUTCLR0_A;
#[doc = "Reader of field `OUTCLR28`"]
pub type OUTCLR28_R = crate::R<bool, OUTCLR0_A>;
#[doc = "Port Data Output Value Clear 28"]
pub type OUTCLR28_AW = OUTCLR0_AW;
#[doc = "Write proxy for field `OUTCLR28`"]
pub struct OUTCLR28_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTCLR28_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTCLR28_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTCLR0_AW::NOP)
    }
    #[doc = "Output 0"]
    #[inline(always)]
    pub fn out0(self) -> &'a mut W {
        self.variant(OUTCLR0_AW::OUT0)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Port Data Output Value Clear 29"]
pub type OUTCLR29_A = OUTCLR0_A;
#[doc = "Reader of field `OUTCLR29`"]
pub type OUTCLR29_R = crate::R<bool, OUTCLR0_A>;
#[doc = "Port Data Output Value Clear 29"]
pub type OUTCLR29_AW = OUTCLR0_AW;
#[doc = "Write proxy for field `OUTCLR29`"]
pub struct OUTCLR29_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTCLR29_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTCLR29_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTCLR0_AW::NOP)
    }
    #[doc = "Output 0"]
    #[inline(always)]
    pub fn out0(self) -> &'a mut W {
        self.variant(OUTCLR0_AW::OUT0)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Port Data Output Value Clear 30"]
pub type OUTCLR30_A = OUTCLR0_A;
#[doc = "Reader of field `OUTCLR30`"]
pub type OUTCLR30_R = crate::R<bool, OUTCLR0_A>;
#[doc = "Port Data Output Value Clear 30"]
pub type OUTCLR30_AW = OUTCLR0_AW;
#[doc = "Write proxy for field `OUTCLR30`"]
pub struct OUTCLR30_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTCLR30_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTCLR30_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTCLR0_AW::NOP)
    }
    #[doc = "Output 0"]
    #[inline(always)]
    pub fn out0(self) -> &'a mut W {
        self.variant(OUTCLR0_AW::OUT0)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Port Data Output Value Clear 31"]
pub type OUTCLR31_A = OUTCLR0_A;
#[doc = "Reader of field `OUTCLR31`"]
pub type OUTCLR31_R = crate::R<bool, OUTCLR0_A>;
#[doc = "Port Data Output Value Clear 31"]
pub type OUTCLR31_AW = OUTCLR0_AW;
#[doc = "Write proxy for field `OUTCLR31`"]
pub struct OUTCLR31_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTCLR31_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTCLR31_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTCLR0_AW::NOP)
    }
    #[doc = "Output 0"]
    #[inline(always)]
    pub fn out0(self) -> &'a mut W {
        self.variant(OUTCLR0_AW::OUT0)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Port Data Output Value Clear 0"]
    #[inline(always)]
    pub fn outclr0(&self) -> OUTCLR0_R {
        OUTCLR0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Port Data Output Value Clear 1"]
    #[inline(always)]
    pub fn outclr1(&self) -> OUTCLR1_R {
        OUTCLR1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Port Data Output Value Clear 2"]
    #[inline(always)]
    pub fn outclr2(&self) -> OUTCLR2_R {
        OUTCLR2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Port Data Output Value Clear 3"]
    #[inline(always)]
    pub fn outclr3(&self) -> OUTCLR3_R {
        OUTCLR3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Port Data Output Value Clear 4"]
    #[inline(always)]
    pub fn outclr4(&self) -> OUTCLR4_R {
        OUTCLR4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Port Data Output Value Clear 5"]
    #[inline(always)]
    pub fn outclr5(&self) -> OUTCLR5_R {
        OUTCLR5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Port Data Output Value Clear 6"]
    #[inline(always)]
    pub fn outclr6(&self) -> OUTCLR6_R {
        OUTCLR6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Port Data Output Value Clear 7"]
    #[inline(always)]
    pub fn outclr7(&self) -> OUTCLR7_R {
        OUTCLR7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Port Data Output Value Clear 8"]
    #[inline(always)]
    pub fn outclr8(&self) -> OUTCLR8_R {
        OUTCLR8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Port Data Output Value Clear 9"]
    #[inline(always)]
    pub fn outclr9(&self) -> OUTCLR9_R {
        OUTCLR9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Port Data Output Value Clear 10"]
    #[inline(always)]
    pub fn outclr10(&self) -> OUTCLR10_R {
        OUTCLR10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Port Data Output Value Clear 11"]
    #[inline(always)]
    pub fn outclr11(&self) -> OUTCLR11_R {
        OUTCLR11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Port Data Output Value Clear 12"]
    #[inline(always)]
    pub fn outclr12(&self) -> OUTCLR12_R {
        OUTCLR12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Port Data Output Value Clear 13"]
    #[inline(always)]
    pub fn outclr13(&self) -> OUTCLR13_R {
        OUTCLR13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Port Data Output Value Clear 14"]
    #[inline(always)]
    pub fn outclr14(&self) -> OUTCLR14_R {
        OUTCLR14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Port Data Output Value Clear 15"]
    #[inline(always)]
    pub fn outclr15(&self) -> OUTCLR15_R {
        OUTCLR15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Port Data Output Value Clear 16"]
    #[inline(always)]
    pub fn outclr16(&self) -> OUTCLR16_R {
        OUTCLR16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Port Data Output Value Clear 17"]
    #[inline(always)]
    pub fn outclr17(&self) -> OUTCLR17_R {
        OUTCLR17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Port Data Output Value Clear 18"]
    #[inline(always)]
    pub fn outclr18(&self) -> OUTCLR18_R {
        OUTCLR18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Port Data Output Value Clear 19"]
    #[inline(always)]
    pub fn outclr19(&self) -> OUTCLR19_R {
        OUTCLR19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Port Data Output Value Clear 20"]
    #[inline(always)]
    pub fn outclr20(&self) -> OUTCLR20_R {
        OUTCLR20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Port Data Output Value Clear 21"]
    #[inline(always)]
    pub fn outclr21(&self) -> OUTCLR21_R {
        OUTCLR21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Port Data Output Value Clear 22"]
    #[inline(always)]
    pub fn outclr22(&self) -> OUTCLR22_R {
        OUTCLR22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Port Data Output Value Clear 23"]
    #[inline(always)]
    pub fn outclr23(&self) -> OUTCLR23_R {
        OUTCLR23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Port Data Output Value Clear 24"]
    #[inline(always)]
    pub fn outclr24(&self) -> OUTCLR24_R {
        OUTCLR24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Port Data Output Value Clear 25"]
    #[inline(always)]
    pub fn outclr25(&self) -> OUTCLR25_R {
        OUTCLR25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Port Data Output Value Clear 26"]
    #[inline(always)]
    pub fn outclr26(&self) -> OUTCLR26_R {
        OUTCLR26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Port Data Output Value Clear 27"]
    #[inline(always)]
    pub fn outclr27(&self) -> OUTCLR27_R {
        OUTCLR27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Port Data Output Value Clear 28"]
    #[inline(always)]
    pub fn outclr28(&self) -> OUTCLR28_R {
        OUTCLR28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Port Data Output Value Clear 29"]
    #[inline(always)]
    pub fn outclr29(&self) -> OUTCLR29_R {
        OUTCLR29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Port Data Output Value Clear 30"]
    #[inline(always)]
    pub fn outclr30(&self) -> OUTCLR30_R {
        OUTCLR30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Port Data Output Value Clear 31"]
    #[inline(always)]
    pub fn outclr31(&self) -> OUTCLR31_R {
        OUTCLR31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port Data Output Value Clear 0"]
    #[inline(always)]
    pub fn outclr0(&mut self) -> OUTCLR0_W {
        OUTCLR0_W { w: self }
    }
    #[doc = "Bit 1 - Port Data Output Value Clear 1"]
    #[inline(always)]
    pub fn outclr1(&mut self) -> OUTCLR1_W {
        OUTCLR1_W { w: self }
    }
    #[doc = "Bit 2 - Port Data Output Value Clear 2"]
    #[inline(always)]
    pub fn outclr2(&mut self) -> OUTCLR2_W {
        OUTCLR2_W { w: self }
    }
    #[doc = "Bit 3 - Port Data Output Value Clear 3"]
    #[inline(always)]
    pub fn outclr3(&mut self) -> OUTCLR3_W {
        OUTCLR3_W { w: self }
    }
    #[doc = "Bit 4 - Port Data Output Value Clear 4"]
    #[inline(always)]
    pub fn outclr4(&mut self) -> OUTCLR4_W {
        OUTCLR4_W { w: self }
    }
    #[doc = "Bit 5 - Port Data Output Value Clear 5"]
    #[inline(always)]
    pub fn outclr5(&mut self) -> OUTCLR5_W {
        OUTCLR5_W { w: self }
    }
    #[doc = "Bit 6 - Port Data Output Value Clear 6"]
    #[inline(always)]
    pub fn outclr6(&mut self) -> OUTCLR6_W {
        OUTCLR6_W { w: self }
    }
    #[doc = "Bit 7 - Port Data Output Value Clear 7"]
    #[inline(always)]
    pub fn outclr7(&mut self) -> OUTCLR7_W {
        OUTCLR7_W { w: self }
    }
    #[doc = "Bit 8 - Port Data Output Value Clear 8"]
    #[inline(always)]
    pub fn outclr8(&mut self) -> OUTCLR8_W {
        OUTCLR8_W { w: self }
    }
    #[doc = "Bit 9 - Port Data Output Value Clear 9"]
    #[inline(always)]
    pub fn outclr9(&mut self) -> OUTCLR9_W {
        OUTCLR9_W { w: self }
    }
    #[doc = "Bit 10 - Port Data Output Value Clear 10"]
    #[inline(always)]
    pub fn outclr10(&mut self) -> OUTCLR10_W {
        OUTCLR10_W { w: self }
    }
    #[doc = "Bit 11 - Port Data Output Value Clear 11"]
    #[inline(always)]
    pub fn outclr11(&mut self) -> OUTCLR11_W {
        OUTCLR11_W { w: self }
    }
    #[doc = "Bit 12 - Port Data Output Value Clear 12"]
    #[inline(always)]
    pub fn outclr12(&mut self) -> OUTCLR12_W {
        OUTCLR12_W { w: self }
    }
    #[doc = "Bit 13 - Port Data Output Value Clear 13"]
    #[inline(always)]
    pub fn outclr13(&mut self) -> OUTCLR13_W {
        OUTCLR13_W { w: self }
    }
    #[doc = "Bit 14 - Port Data Output Value Clear 14"]
    #[inline(always)]
    pub fn outclr14(&mut self) -> OUTCLR14_W {
        OUTCLR14_W { w: self }
    }
    #[doc = "Bit 15 - Port Data Output Value Clear 15"]
    #[inline(always)]
    pub fn outclr15(&mut self) -> OUTCLR15_W {
        OUTCLR15_W { w: self }
    }
    #[doc = "Bit 16 - Port Data Output Value Clear 16"]
    #[inline(always)]
    pub fn outclr16(&mut self) -> OUTCLR16_W {
        OUTCLR16_W { w: self }
    }
    #[doc = "Bit 17 - Port Data Output Value Clear 17"]
    #[inline(always)]
    pub fn outclr17(&mut self) -> OUTCLR17_W {
        OUTCLR17_W { w: self }
    }
    #[doc = "Bit 18 - Port Data Output Value Clear 18"]
    #[inline(always)]
    pub fn outclr18(&mut self) -> OUTCLR18_W {
        OUTCLR18_W { w: self }
    }
    #[doc = "Bit 19 - Port Data Output Value Clear 19"]
    #[inline(always)]
    pub fn outclr19(&mut self) -> OUTCLR19_W {
        OUTCLR19_W { w: self }
    }
    #[doc = "Bit 20 - Port Data Output Value Clear 20"]
    #[inline(always)]
    pub fn outclr20(&mut self) -> OUTCLR20_W {
        OUTCLR20_W { w: self }
    }
    #[doc = "Bit 21 - Port Data Output Value Clear 21"]
    #[inline(always)]
    pub fn outclr21(&mut self) -> OUTCLR21_W {
        OUTCLR21_W { w: self }
    }
    #[doc = "Bit 22 - Port Data Output Value Clear 22"]
    #[inline(always)]
    pub fn outclr22(&mut self) -> OUTCLR22_W {
        OUTCLR22_W { w: self }
    }
    #[doc = "Bit 23 - Port Data Output Value Clear 23"]
    #[inline(always)]
    pub fn outclr23(&mut self) -> OUTCLR23_W {
        OUTCLR23_W { w: self }
    }
    #[doc = "Bit 24 - Port Data Output Value Clear 24"]
    #[inline(always)]
    pub fn outclr24(&mut self) -> OUTCLR24_W {
        OUTCLR24_W { w: self }
    }
    #[doc = "Bit 25 - Port Data Output Value Clear 25"]
    #[inline(always)]
    pub fn outclr25(&mut self) -> OUTCLR25_W {
        OUTCLR25_W { w: self }
    }
    #[doc = "Bit 26 - Port Data Output Value Clear 26"]
    #[inline(always)]
    pub fn outclr26(&mut self) -> OUTCLR26_W {
        OUTCLR26_W { w: self }
    }
    #[doc = "Bit 27 - Port Data Output Value Clear 27"]
    #[inline(always)]
    pub fn outclr27(&mut self) -> OUTCLR27_W {
        OUTCLR27_W { w: self }
    }
    #[doc = "Bit 28 - Port Data Output Value Clear 28"]
    #[inline(always)]
    pub fn outclr28(&mut self) -> OUTCLR28_W {
        OUTCLR28_W { w: self }
    }
    #[doc = "Bit 29 - Port Data Output Value Clear 29"]
    #[inline(always)]
    pub fn outclr29(&mut self) -> OUTCLR29_W {
        OUTCLR29_W { w: self }
    }
    #[doc = "Bit 30 - Port Data Output Value Clear 30"]
    #[inline(always)]
    pub fn outclr30(&mut self) -> OUTCLR30_W {
        OUTCLR30_W { w: self }
    }
    #[doc = "Bit 31 - Port Data Output Value Clear 31"]
    #[inline(always)]
    pub fn outclr31(&mut self) -> OUTCLR31_W {
        OUTCLR31_W { w: self }
    }
}
