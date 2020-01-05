#[doc = "Reader of register DIRSET%s"]
pub type R = crate::R<u32, super::DIRSET>;
#[doc = "Writer for register DIRSET%s"]
pub type W = crate::W<u32, super::DIRSET>;
#[doc = "Register DIRSET%s `reset()`'s with value 0"]
impl crate::ResetValue for super::DIRSET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Port Data Direction Set 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIRSET0_A {
    #[doc = "0: Input"]
    INPUT = 0,
    #[doc = "1: Output"]
    OUTPUT = 1,
}
impl From<DIRSET0_A> for bool {
    #[inline(always)]
    fn from(variant: DIRSET0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIRSET0`"]
pub type DIRSET0_R = crate::R<bool, DIRSET0_A>;
impl DIRSET0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRSET0_A {
        match self.bits {
            false => DIRSET0_A::INPUT,
            true => DIRSET0_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == DIRSET0_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == DIRSET0_A::OUTPUT
    }
}
#[doc = "Port Data Direction Set 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIRSET0_AW {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Make pin output"]
    SETOUTPUT = 1,
}
impl From<DIRSET0_AW> for bool {
    #[inline(always)]
    fn from(variant: DIRSET0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `DIRSET0`"]
pub struct DIRSET0_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRSET0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRSET0_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRSET0_AW::NOP)
    }
    #[doc = "Make pin output"]
    #[inline(always)]
    pub fn setoutput(self) -> &'a mut W {
        self.variant(DIRSET0_AW::SETOUTPUT)
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
#[doc = "Port Data Direction Set 1"]
pub type DIRSET1_A = DIRSET0_A;
#[doc = "Reader of field `DIRSET1`"]
pub type DIRSET1_R = crate::R<bool, DIRSET0_A>;
#[doc = "Port Data Direction Set 1"]
pub type DIRSET1_AW = DIRSET0_AW;
#[doc = "Write proxy for field `DIRSET1`"]
pub struct DIRSET1_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRSET1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRSET1_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRSET0_AW::NOP)
    }
    #[doc = "Make pin output"]
    #[inline(always)]
    pub fn setoutput(self) -> &'a mut W {
        self.variant(DIRSET0_AW::SETOUTPUT)
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
#[doc = "Port Data Direction Set 2"]
pub type DIRSET2_A = DIRSET0_A;
#[doc = "Reader of field `DIRSET2`"]
pub type DIRSET2_R = crate::R<bool, DIRSET0_A>;
#[doc = "Port Data Direction Set 2"]
pub type DIRSET2_AW = DIRSET0_AW;
#[doc = "Write proxy for field `DIRSET2`"]
pub struct DIRSET2_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRSET2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRSET2_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRSET0_AW::NOP)
    }
    #[doc = "Make pin output"]
    #[inline(always)]
    pub fn setoutput(self) -> &'a mut W {
        self.variant(DIRSET0_AW::SETOUTPUT)
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
#[doc = "Port Data Direction Set 3"]
pub type DIRSET3_A = DIRSET0_A;
#[doc = "Reader of field `DIRSET3`"]
pub type DIRSET3_R = crate::R<bool, DIRSET0_A>;
#[doc = "Port Data Direction Set 3"]
pub type DIRSET3_AW = DIRSET0_AW;
#[doc = "Write proxy for field `DIRSET3`"]
pub struct DIRSET3_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRSET3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRSET3_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRSET0_AW::NOP)
    }
    #[doc = "Make pin output"]
    #[inline(always)]
    pub fn setoutput(self) -> &'a mut W {
        self.variant(DIRSET0_AW::SETOUTPUT)
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
#[doc = "Port Data Direction Set 4"]
pub type DIRSET4_A = DIRSET0_A;
#[doc = "Reader of field `DIRSET4`"]
pub type DIRSET4_R = crate::R<bool, DIRSET0_A>;
#[doc = "Port Data Direction Set 4"]
pub type DIRSET4_AW = DIRSET0_AW;
#[doc = "Write proxy for field `DIRSET4`"]
pub struct DIRSET4_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRSET4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRSET4_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRSET0_AW::NOP)
    }
    #[doc = "Make pin output"]
    #[inline(always)]
    pub fn setoutput(self) -> &'a mut W {
        self.variant(DIRSET0_AW::SETOUTPUT)
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
#[doc = "Port Data Direction Set 5"]
pub type DIRSET5_A = DIRSET0_A;
#[doc = "Reader of field `DIRSET5`"]
pub type DIRSET5_R = crate::R<bool, DIRSET0_A>;
#[doc = "Port Data Direction Set 5"]
pub type DIRSET5_AW = DIRSET0_AW;
#[doc = "Write proxy for field `DIRSET5`"]
pub struct DIRSET5_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRSET5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRSET5_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRSET0_AW::NOP)
    }
    #[doc = "Make pin output"]
    #[inline(always)]
    pub fn setoutput(self) -> &'a mut W {
        self.variant(DIRSET0_AW::SETOUTPUT)
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
#[doc = "Port Data Direction Set 6"]
pub type DIRSET6_A = DIRSET0_A;
#[doc = "Reader of field `DIRSET6`"]
pub type DIRSET6_R = crate::R<bool, DIRSET0_A>;
#[doc = "Port Data Direction Set 6"]
pub type DIRSET6_AW = DIRSET0_AW;
#[doc = "Write proxy for field `DIRSET6`"]
pub struct DIRSET6_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRSET6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRSET6_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRSET0_AW::NOP)
    }
    #[doc = "Make pin output"]
    #[inline(always)]
    pub fn setoutput(self) -> &'a mut W {
        self.variant(DIRSET0_AW::SETOUTPUT)
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
#[doc = "Port Data Direction Set 7"]
pub type DIRSET7_A = DIRSET0_A;
#[doc = "Reader of field `DIRSET7`"]
pub type DIRSET7_R = crate::R<bool, DIRSET0_A>;
#[doc = "Port Data Direction Set 7"]
pub type DIRSET7_AW = DIRSET0_AW;
#[doc = "Write proxy for field `DIRSET7`"]
pub struct DIRSET7_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRSET7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRSET7_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRSET0_AW::NOP)
    }
    #[doc = "Make pin output"]
    #[inline(always)]
    pub fn setoutput(self) -> &'a mut W {
        self.variant(DIRSET0_AW::SETOUTPUT)
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
#[doc = "Port Data Direction Set 8"]
pub type DIRSET8_A = DIRSET0_A;
#[doc = "Reader of field `DIRSET8`"]
pub type DIRSET8_R = crate::R<bool, DIRSET0_A>;
#[doc = "Port Data Direction Set 8"]
pub type DIRSET8_AW = DIRSET0_AW;
#[doc = "Write proxy for field `DIRSET8`"]
pub struct DIRSET8_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRSET8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRSET8_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRSET0_AW::NOP)
    }
    #[doc = "Make pin output"]
    #[inline(always)]
    pub fn setoutput(self) -> &'a mut W {
        self.variant(DIRSET0_AW::SETOUTPUT)
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
#[doc = "Port Data Direction Set 9"]
pub type DIRSET9_A = DIRSET0_A;
#[doc = "Reader of field `DIRSET9`"]
pub type DIRSET9_R = crate::R<bool, DIRSET0_A>;
#[doc = "Port Data Direction Set 9"]
pub type DIRSET9_AW = DIRSET0_AW;
#[doc = "Write proxy for field `DIRSET9`"]
pub struct DIRSET9_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRSET9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRSET9_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRSET0_AW::NOP)
    }
    #[doc = "Make pin output"]
    #[inline(always)]
    pub fn setoutput(self) -> &'a mut W {
        self.variant(DIRSET0_AW::SETOUTPUT)
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
#[doc = "Port Data Direction Set 10"]
pub type DIRSET10_A = DIRSET0_A;
#[doc = "Reader of field `DIRSET10`"]
pub type DIRSET10_R = crate::R<bool, DIRSET0_A>;
#[doc = "Port Data Direction Set 10"]
pub type DIRSET10_AW = DIRSET0_AW;
#[doc = "Write proxy for field `DIRSET10`"]
pub struct DIRSET10_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRSET10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRSET10_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRSET0_AW::NOP)
    }
    #[doc = "Make pin output"]
    #[inline(always)]
    pub fn setoutput(self) -> &'a mut W {
        self.variant(DIRSET0_AW::SETOUTPUT)
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
#[doc = "Port Data Direction Set 11"]
pub type DIRSET11_A = DIRSET0_A;
#[doc = "Reader of field `DIRSET11`"]
pub type DIRSET11_R = crate::R<bool, DIRSET0_A>;
#[doc = "Port Data Direction Set 11"]
pub type DIRSET11_AW = DIRSET0_AW;
#[doc = "Write proxy for field `DIRSET11`"]
pub struct DIRSET11_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRSET11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRSET11_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRSET0_AW::NOP)
    }
    #[doc = "Make pin output"]
    #[inline(always)]
    pub fn setoutput(self) -> &'a mut W {
        self.variant(DIRSET0_AW::SETOUTPUT)
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
#[doc = "Port Data Direction Set 12"]
pub type DIRSET12_A = DIRSET0_A;
#[doc = "Reader of field `DIRSET12`"]
pub type DIRSET12_R = crate::R<bool, DIRSET0_A>;
#[doc = "Port Data Direction Set 12"]
pub type DIRSET12_AW = DIRSET0_AW;
#[doc = "Write proxy for field `DIRSET12`"]
pub struct DIRSET12_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRSET12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRSET12_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRSET0_AW::NOP)
    }
    #[doc = "Make pin output"]
    #[inline(always)]
    pub fn setoutput(self) -> &'a mut W {
        self.variant(DIRSET0_AW::SETOUTPUT)
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
#[doc = "Port Data Direction Set 13"]
pub type DIRSET13_A = DIRSET0_A;
#[doc = "Reader of field `DIRSET13`"]
pub type DIRSET13_R = crate::R<bool, DIRSET0_A>;
#[doc = "Port Data Direction Set 13"]
pub type DIRSET13_AW = DIRSET0_AW;
#[doc = "Write proxy for field `DIRSET13`"]
pub struct DIRSET13_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRSET13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRSET13_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRSET0_AW::NOP)
    }
    #[doc = "Make pin output"]
    #[inline(always)]
    pub fn setoutput(self) -> &'a mut W {
        self.variant(DIRSET0_AW::SETOUTPUT)
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
#[doc = "Port Data Direction Set 14"]
pub type DIRSET14_A = DIRSET0_A;
#[doc = "Reader of field `DIRSET14`"]
pub type DIRSET14_R = crate::R<bool, DIRSET0_A>;
#[doc = "Port Data Direction Set 14"]
pub type DIRSET14_AW = DIRSET0_AW;
#[doc = "Write proxy for field `DIRSET14`"]
pub struct DIRSET14_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRSET14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRSET14_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRSET0_AW::NOP)
    }
    #[doc = "Make pin output"]
    #[inline(always)]
    pub fn setoutput(self) -> &'a mut W {
        self.variant(DIRSET0_AW::SETOUTPUT)
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
#[doc = "Port Data Direction Set 15"]
pub type DIRSET15_A = DIRSET0_A;
#[doc = "Reader of field `DIRSET15`"]
pub type DIRSET15_R = crate::R<bool, DIRSET0_A>;
#[doc = "Port Data Direction Set 15"]
pub type DIRSET15_AW = DIRSET0_AW;
#[doc = "Write proxy for field `DIRSET15`"]
pub struct DIRSET15_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRSET15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRSET15_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRSET0_AW::NOP)
    }
    #[doc = "Make pin output"]
    #[inline(always)]
    pub fn setoutput(self) -> &'a mut W {
        self.variant(DIRSET0_AW::SETOUTPUT)
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
#[doc = "Port Data Direction Set 16"]
pub type DIRSET16_A = DIRSET0_A;
#[doc = "Reader of field `DIRSET16`"]
pub type DIRSET16_R = crate::R<bool, DIRSET0_A>;
#[doc = "Port Data Direction Set 16"]
pub type DIRSET16_AW = DIRSET0_AW;
#[doc = "Write proxy for field `DIRSET16`"]
pub struct DIRSET16_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRSET16_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRSET16_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRSET0_AW::NOP)
    }
    #[doc = "Make pin output"]
    #[inline(always)]
    pub fn setoutput(self) -> &'a mut W {
        self.variant(DIRSET0_AW::SETOUTPUT)
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
#[doc = "Port Data Direction Set 17"]
pub type DIRSET17_A = DIRSET0_A;
#[doc = "Reader of field `DIRSET17`"]
pub type DIRSET17_R = crate::R<bool, DIRSET0_A>;
#[doc = "Port Data Direction Set 17"]
pub type DIRSET17_AW = DIRSET0_AW;
#[doc = "Write proxy for field `DIRSET17`"]
pub struct DIRSET17_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRSET17_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRSET17_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRSET0_AW::NOP)
    }
    #[doc = "Make pin output"]
    #[inline(always)]
    pub fn setoutput(self) -> &'a mut W {
        self.variant(DIRSET0_AW::SETOUTPUT)
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
#[doc = "Port Data Direction Set 18"]
pub type DIRSET18_A = DIRSET0_A;
#[doc = "Reader of field `DIRSET18`"]
pub type DIRSET18_R = crate::R<bool, DIRSET0_A>;
#[doc = "Port Data Direction Set 18"]
pub type DIRSET18_AW = DIRSET0_AW;
#[doc = "Write proxy for field `DIRSET18`"]
pub struct DIRSET18_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRSET18_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRSET18_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRSET0_AW::NOP)
    }
    #[doc = "Make pin output"]
    #[inline(always)]
    pub fn setoutput(self) -> &'a mut W {
        self.variant(DIRSET0_AW::SETOUTPUT)
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
#[doc = "Port Data Direction Set 19"]
pub type DIRSET19_A = DIRSET0_A;
#[doc = "Reader of field `DIRSET19`"]
pub type DIRSET19_R = crate::R<bool, DIRSET0_A>;
#[doc = "Port Data Direction Set 19"]
pub type DIRSET19_AW = DIRSET0_AW;
#[doc = "Write proxy for field `DIRSET19`"]
pub struct DIRSET19_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRSET19_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRSET19_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRSET0_AW::NOP)
    }
    #[doc = "Make pin output"]
    #[inline(always)]
    pub fn setoutput(self) -> &'a mut W {
        self.variant(DIRSET0_AW::SETOUTPUT)
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
#[doc = "Port Data Direction Set 20"]
pub type DIRSET20_A = DIRSET0_A;
#[doc = "Reader of field `DIRSET20`"]
pub type DIRSET20_R = crate::R<bool, DIRSET0_A>;
#[doc = "Port Data Direction Set 20"]
pub type DIRSET20_AW = DIRSET0_AW;
#[doc = "Write proxy for field `DIRSET20`"]
pub struct DIRSET20_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRSET20_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRSET20_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRSET0_AW::NOP)
    }
    #[doc = "Make pin output"]
    #[inline(always)]
    pub fn setoutput(self) -> &'a mut W {
        self.variant(DIRSET0_AW::SETOUTPUT)
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
#[doc = "Port Data Direction Set 21"]
pub type DIRSET21_A = DIRSET0_A;
#[doc = "Reader of field `DIRSET21`"]
pub type DIRSET21_R = crate::R<bool, DIRSET0_A>;
#[doc = "Port Data Direction Set 21"]
pub type DIRSET21_AW = DIRSET0_AW;
#[doc = "Write proxy for field `DIRSET21`"]
pub struct DIRSET21_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRSET21_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRSET21_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRSET0_AW::NOP)
    }
    #[doc = "Make pin output"]
    #[inline(always)]
    pub fn setoutput(self) -> &'a mut W {
        self.variant(DIRSET0_AW::SETOUTPUT)
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
#[doc = "Port Data Direction Set 22"]
pub type DIRSET22_A = DIRSET0_A;
#[doc = "Reader of field `DIRSET22`"]
pub type DIRSET22_R = crate::R<bool, DIRSET0_A>;
#[doc = "Port Data Direction Set 22"]
pub type DIRSET22_AW = DIRSET0_AW;
#[doc = "Write proxy for field `DIRSET22`"]
pub struct DIRSET22_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRSET22_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRSET22_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRSET0_AW::NOP)
    }
    #[doc = "Make pin output"]
    #[inline(always)]
    pub fn setoutput(self) -> &'a mut W {
        self.variant(DIRSET0_AW::SETOUTPUT)
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
#[doc = "Port Data Direction Set 23"]
pub type DIRSET23_A = DIRSET0_A;
#[doc = "Reader of field `DIRSET23`"]
pub type DIRSET23_R = crate::R<bool, DIRSET0_A>;
#[doc = "Port Data Direction Set 23"]
pub type DIRSET23_AW = DIRSET0_AW;
#[doc = "Write proxy for field `DIRSET23`"]
pub struct DIRSET23_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRSET23_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRSET23_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRSET0_AW::NOP)
    }
    #[doc = "Make pin output"]
    #[inline(always)]
    pub fn setoutput(self) -> &'a mut W {
        self.variant(DIRSET0_AW::SETOUTPUT)
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
#[doc = "Port Data Direction Set 24"]
pub type DIRSET24_A = DIRSET0_A;
#[doc = "Reader of field `DIRSET24`"]
pub type DIRSET24_R = crate::R<bool, DIRSET0_A>;
#[doc = "Port Data Direction Set 24"]
pub type DIRSET24_AW = DIRSET0_AW;
#[doc = "Write proxy for field `DIRSET24`"]
pub struct DIRSET24_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRSET24_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRSET24_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRSET0_AW::NOP)
    }
    #[doc = "Make pin output"]
    #[inline(always)]
    pub fn setoutput(self) -> &'a mut W {
        self.variant(DIRSET0_AW::SETOUTPUT)
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
#[doc = "Port Data Direction Set 25"]
pub type DIRSET25_A = DIRSET0_A;
#[doc = "Reader of field `DIRSET25`"]
pub type DIRSET25_R = crate::R<bool, DIRSET0_A>;
#[doc = "Port Data Direction Set 25"]
pub type DIRSET25_AW = DIRSET0_AW;
#[doc = "Write proxy for field `DIRSET25`"]
pub struct DIRSET25_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRSET25_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRSET25_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRSET0_AW::NOP)
    }
    #[doc = "Make pin output"]
    #[inline(always)]
    pub fn setoutput(self) -> &'a mut W {
        self.variant(DIRSET0_AW::SETOUTPUT)
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
#[doc = "Port Data Direction Set 26"]
pub type DIRSET26_A = DIRSET0_A;
#[doc = "Reader of field `DIRSET26`"]
pub type DIRSET26_R = crate::R<bool, DIRSET0_A>;
#[doc = "Port Data Direction Set 26"]
pub type DIRSET26_AW = DIRSET0_AW;
#[doc = "Write proxy for field `DIRSET26`"]
pub struct DIRSET26_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRSET26_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRSET26_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRSET0_AW::NOP)
    }
    #[doc = "Make pin output"]
    #[inline(always)]
    pub fn setoutput(self) -> &'a mut W {
        self.variant(DIRSET0_AW::SETOUTPUT)
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
#[doc = "Port Data Direction Set 27"]
pub type DIRSET27_A = DIRSET0_A;
#[doc = "Reader of field `DIRSET27`"]
pub type DIRSET27_R = crate::R<bool, DIRSET0_A>;
#[doc = "Port Data Direction Set 27"]
pub type DIRSET27_AW = DIRSET0_AW;
#[doc = "Write proxy for field `DIRSET27`"]
pub struct DIRSET27_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRSET27_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRSET27_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRSET0_AW::NOP)
    }
    #[doc = "Make pin output"]
    #[inline(always)]
    pub fn setoutput(self) -> &'a mut W {
        self.variant(DIRSET0_AW::SETOUTPUT)
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
#[doc = "Port Data Direction Set 28"]
pub type DIRSET28_A = DIRSET0_A;
#[doc = "Reader of field `DIRSET28`"]
pub type DIRSET28_R = crate::R<bool, DIRSET0_A>;
#[doc = "Port Data Direction Set 28"]
pub type DIRSET28_AW = DIRSET0_AW;
#[doc = "Write proxy for field `DIRSET28`"]
pub struct DIRSET28_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRSET28_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRSET28_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRSET0_AW::NOP)
    }
    #[doc = "Make pin output"]
    #[inline(always)]
    pub fn setoutput(self) -> &'a mut W {
        self.variant(DIRSET0_AW::SETOUTPUT)
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
#[doc = "Port Data Direction Set 29"]
pub type DIRSET29_A = DIRSET0_A;
#[doc = "Reader of field `DIRSET29`"]
pub type DIRSET29_R = crate::R<bool, DIRSET0_A>;
#[doc = "Port Data Direction Set 29"]
pub type DIRSET29_AW = DIRSET0_AW;
#[doc = "Write proxy for field `DIRSET29`"]
pub struct DIRSET29_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRSET29_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRSET29_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRSET0_AW::NOP)
    }
    #[doc = "Make pin output"]
    #[inline(always)]
    pub fn setoutput(self) -> &'a mut W {
        self.variant(DIRSET0_AW::SETOUTPUT)
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
#[doc = "Port Data Direction Set 30"]
pub type DIRSET30_A = DIRSET0_A;
#[doc = "Reader of field `DIRSET30`"]
pub type DIRSET30_R = crate::R<bool, DIRSET0_A>;
#[doc = "Port Data Direction Set 30"]
pub type DIRSET30_AW = DIRSET0_AW;
#[doc = "Write proxy for field `DIRSET30`"]
pub struct DIRSET30_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRSET30_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRSET30_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRSET0_AW::NOP)
    }
    #[doc = "Make pin output"]
    #[inline(always)]
    pub fn setoutput(self) -> &'a mut W {
        self.variant(DIRSET0_AW::SETOUTPUT)
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
#[doc = "Port Data Direction Set 31"]
pub type DIRSET31_A = DIRSET0_A;
#[doc = "Reader of field `DIRSET31`"]
pub type DIRSET31_R = crate::R<bool, DIRSET0_A>;
#[doc = "Port Data Direction Set 31"]
pub type DIRSET31_AW = DIRSET0_AW;
#[doc = "Write proxy for field `DIRSET31`"]
pub struct DIRSET31_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRSET31_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRSET31_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRSET0_AW::NOP)
    }
    #[doc = "Make pin output"]
    #[inline(always)]
    pub fn setoutput(self) -> &'a mut W {
        self.variant(DIRSET0_AW::SETOUTPUT)
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
    #[doc = "Bit 0 - Port Data Direction Set 0"]
    #[inline(always)]
    pub fn dirset0(&self) -> DIRSET0_R {
        DIRSET0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Port Data Direction Set 1"]
    #[inline(always)]
    pub fn dirset1(&self) -> DIRSET1_R {
        DIRSET1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Port Data Direction Set 2"]
    #[inline(always)]
    pub fn dirset2(&self) -> DIRSET2_R {
        DIRSET2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Port Data Direction Set 3"]
    #[inline(always)]
    pub fn dirset3(&self) -> DIRSET3_R {
        DIRSET3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Port Data Direction Set 4"]
    #[inline(always)]
    pub fn dirset4(&self) -> DIRSET4_R {
        DIRSET4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Port Data Direction Set 5"]
    #[inline(always)]
    pub fn dirset5(&self) -> DIRSET5_R {
        DIRSET5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Port Data Direction Set 6"]
    #[inline(always)]
    pub fn dirset6(&self) -> DIRSET6_R {
        DIRSET6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Port Data Direction Set 7"]
    #[inline(always)]
    pub fn dirset7(&self) -> DIRSET7_R {
        DIRSET7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Port Data Direction Set 8"]
    #[inline(always)]
    pub fn dirset8(&self) -> DIRSET8_R {
        DIRSET8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Port Data Direction Set 9"]
    #[inline(always)]
    pub fn dirset9(&self) -> DIRSET9_R {
        DIRSET9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Port Data Direction Set 10"]
    #[inline(always)]
    pub fn dirset10(&self) -> DIRSET10_R {
        DIRSET10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Port Data Direction Set 11"]
    #[inline(always)]
    pub fn dirset11(&self) -> DIRSET11_R {
        DIRSET11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Port Data Direction Set 12"]
    #[inline(always)]
    pub fn dirset12(&self) -> DIRSET12_R {
        DIRSET12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Port Data Direction Set 13"]
    #[inline(always)]
    pub fn dirset13(&self) -> DIRSET13_R {
        DIRSET13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Port Data Direction Set 14"]
    #[inline(always)]
    pub fn dirset14(&self) -> DIRSET14_R {
        DIRSET14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Port Data Direction Set 15"]
    #[inline(always)]
    pub fn dirset15(&self) -> DIRSET15_R {
        DIRSET15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Port Data Direction Set 16"]
    #[inline(always)]
    pub fn dirset16(&self) -> DIRSET16_R {
        DIRSET16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Port Data Direction Set 17"]
    #[inline(always)]
    pub fn dirset17(&self) -> DIRSET17_R {
        DIRSET17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Port Data Direction Set 18"]
    #[inline(always)]
    pub fn dirset18(&self) -> DIRSET18_R {
        DIRSET18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Port Data Direction Set 19"]
    #[inline(always)]
    pub fn dirset19(&self) -> DIRSET19_R {
        DIRSET19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Port Data Direction Set 20"]
    #[inline(always)]
    pub fn dirset20(&self) -> DIRSET20_R {
        DIRSET20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Port Data Direction Set 21"]
    #[inline(always)]
    pub fn dirset21(&self) -> DIRSET21_R {
        DIRSET21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Port Data Direction Set 22"]
    #[inline(always)]
    pub fn dirset22(&self) -> DIRSET22_R {
        DIRSET22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Port Data Direction Set 23"]
    #[inline(always)]
    pub fn dirset23(&self) -> DIRSET23_R {
        DIRSET23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Port Data Direction Set 24"]
    #[inline(always)]
    pub fn dirset24(&self) -> DIRSET24_R {
        DIRSET24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Port Data Direction Set 25"]
    #[inline(always)]
    pub fn dirset25(&self) -> DIRSET25_R {
        DIRSET25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Port Data Direction Set 26"]
    #[inline(always)]
    pub fn dirset26(&self) -> DIRSET26_R {
        DIRSET26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Port Data Direction Set 27"]
    #[inline(always)]
    pub fn dirset27(&self) -> DIRSET27_R {
        DIRSET27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Port Data Direction Set 28"]
    #[inline(always)]
    pub fn dirset28(&self) -> DIRSET28_R {
        DIRSET28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Port Data Direction Set 29"]
    #[inline(always)]
    pub fn dirset29(&self) -> DIRSET29_R {
        DIRSET29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Port Data Direction Set 30"]
    #[inline(always)]
    pub fn dirset30(&self) -> DIRSET30_R {
        DIRSET30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Port Data Direction Set 31"]
    #[inline(always)]
    pub fn dirset31(&self) -> DIRSET31_R {
        DIRSET31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port Data Direction Set 0"]
    #[inline(always)]
    pub fn dirset0(&mut self) -> DIRSET0_W {
        DIRSET0_W { w: self }
    }
    #[doc = "Bit 1 - Port Data Direction Set 1"]
    #[inline(always)]
    pub fn dirset1(&mut self) -> DIRSET1_W {
        DIRSET1_W { w: self }
    }
    #[doc = "Bit 2 - Port Data Direction Set 2"]
    #[inline(always)]
    pub fn dirset2(&mut self) -> DIRSET2_W {
        DIRSET2_W { w: self }
    }
    #[doc = "Bit 3 - Port Data Direction Set 3"]
    #[inline(always)]
    pub fn dirset3(&mut self) -> DIRSET3_W {
        DIRSET3_W { w: self }
    }
    #[doc = "Bit 4 - Port Data Direction Set 4"]
    #[inline(always)]
    pub fn dirset4(&mut self) -> DIRSET4_W {
        DIRSET4_W { w: self }
    }
    #[doc = "Bit 5 - Port Data Direction Set 5"]
    #[inline(always)]
    pub fn dirset5(&mut self) -> DIRSET5_W {
        DIRSET5_W { w: self }
    }
    #[doc = "Bit 6 - Port Data Direction Set 6"]
    #[inline(always)]
    pub fn dirset6(&mut self) -> DIRSET6_W {
        DIRSET6_W { w: self }
    }
    #[doc = "Bit 7 - Port Data Direction Set 7"]
    #[inline(always)]
    pub fn dirset7(&mut self) -> DIRSET7_W {
        DIRSET7_W { w: self }
    }
    #[doc = "Bit 8 - Port Data Direction Set 8"]
    #[inline(always)]
    pub fn dirset8(&mut self) -> DIRSET8_W {
        DIRSET8_W { w: self }
    }
    #[doc = "Bit 9 - Port Data Direction Set 9"]
    #[inline(always)]
    pub fn dirset9(&mut self) -> DIRSET9_W {
        DIRSET9_W { w: self }
    }
    #[doc = "Bit 10 - Port Data Direction Set 10"]
    #[inline(always)]
    pub fn dirset10(&mut self) -> DIRSET10_W {
        DIRSET10_W { w: self }
    }
    #[doc = "Bit 11 - Port Data Direction Set 11"]
    #[inline(always)]
    pub fn dirset11(&mut self) -> DIRSET11_W {
        DIRSET11_W { w: self }
    }
    #[doc = "Bit 12 - Port Data Direction Set 12"]
    #[inline(always)]
    pub fn dirset12(&mut self) -> DIRSET12_W {
        DIRSET12_W { w: self }
    }
    #[doc = "Bit 13 - Port Data Direction Set 13"]
    #[inline(always)]
    pub fn dirset13(&mut self) -> DIRSET13_W {
        DIRSET13_W { w: self }
    }
    #[doc = "Bit 14 - Port Data Direction Set 14"]
    #[inline(always)]
    pub fn dirset14(&mut self) -> DIRSET14_W {
        DIRSET14_W { w: self }
    }
    #[doc = "Bit 15 - Port Data Direction Set 15"]
    #[inline(always)]
    pub fn dirset15(&mut self) -> DIRSET15_W {
        DIRSET15_W { w: self }
    }
    #[doc = "Bit 16 - Port Data Direction Set 16"]
    #[inline(always)]
    pub fn dirset16(&mut self) -> DIRSET16_W {
        DIRSET16_W { w: self }
    }
    #[doc = "Bit 17 - Port Data Direction Set 17"]
    #[inline(always)]
    pub fn dirset17(&mut self) -> DIRSET17_W {
        DIRSET17_W { w: self }
    }
    #[doc = "Bit 18 - Port Data Direction Set 18"]
    #[inline(always)]
    pub fn dirset18(&mut self) -> DIRSET18_W {
        DIRSET18_W { w: self }
    }
    #[doc = "Bit 19 - Port Data Direction Set 19"]
    #[inline(always)]
    pub fn dirset19(&mut self) -> DIRSET19_W {
        DIRSET19_W { w: self }
    }
    #[doc = "Bit 20 - Port Data Direction Set 20"]
    #[inline(always)]
    pub fn dirset20(&mut self) -> DIRSET20_W {
        DIRSET20_W { w: self }
    }
    #[doc = "Bit 21 - Port Data Direction Set 21"]
    #[inline(always)]
    pub fn dirset21(&mut self) -> DIRSET21_W {
        DIRSET21_W { w: self }
    }
    #[doc = "Bit 22 - Port Data Direction Set 22"]
    #[inline(always)]
    pub fn dirset22(&mut self) -> DIRSET22_W {
        DIRSET22_W { w: self }
    }
    #[doc = "Bit 23 - Port Data Direction Set 23"]
    #[inline(always)]
    pub fn dirset23(&mut self) -> DIRSET23_W {
        DIRSET23_W { w: self }
    }
    #[doc = "Bit 24 - Port Data Direction Set 24"]
    #[inline(always)]
    pub fn dirset24(&mut self) -> DIRSET24_W {
        DIRSET24_W { w: self }
    }
    #[doc = "Bit 25 - Port Data Direction Set 25"]
    #[inline(always)]
    pub fn dirset25(&mut self) -> DIRSET25_W {
        DIRSET25_W { w: self }
    }
    #[doc = "Bit 26 - Port Data Direction Set 26"]
    #[inline(always)]
    pub fn dirset26(&mut self) -> DIRSET26_W {
        DIRSET26_W { w: self }
    }
    #[doc = "Bit 27 - Port Data Direction Set 27"]
    #[inline(always)]
    pub fn dirset27(&mut self) -> DIRSET27_W {
        DIRSET27_W { w: self }
    }
    #[doc = "Bit 28 - Port Data Direction Set 28"]
    #[inline(always)]
    pub fn dirset28(&mut self) -> DIRSET28_W {
        DIRSET28_W { w: self }
    }
    #[doc = "Bit 29 - Port Data Direction Set 29"]
    #[inline(always)]
    pub fn dirset29(&mut self) -> DIRSET29_W {
        DIRSET29_W { w: self }
    }
    #[doc = "Bit 30 - Port Data Direction Set 30"]
    #[inline(always)]
    pub fn dirset30(&mut self) -> DIRSET30_W {
        DIRSET30_W { w: self }
    }
    #[doc = "Bit 31 - Port Data Direction Set 31"]
    #[inline(always)]
    pub fn dirset31(&mut self) -> DIRSET31_W {
        DIRSET31_W { w: self }
    }
}
