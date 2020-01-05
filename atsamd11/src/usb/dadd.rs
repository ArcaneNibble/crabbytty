#[doc = "Reader of register DADD"]
pub type R = crate::R<u8, super::DADD>;
#[doc = "Writer for register DADD"]
pub type W = crate::W<u8, super::DADD>;
#[doc = "Register DADD `reset()`'s with value 0"]
impl crate::ResetValue for super::DADD {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DADD`"]
pub type DADD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DADD`"]
pub struct DADD_W<'a> {
    w: &'a mut W,
}
impl<'a> DADD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u8) & 0x7f);
        self.w
    }
}
#[doc = "Device Address Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADDEN_A {
    #[doc = "0: Address 0"]
    DEACTIVATE = 0,
    #[doc = "1: Address active"]
    ACTIVATE = 1,
}
impl From<ADDEN_A> for bool {
    #[inline(always)]
    fn from(variant: ADDEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADDEN`"]
pub type ADDEN_R = crate::R<bool, ADDEN_A>;
impl ADDEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADDEN_A {
        match self.bits {
            false => ADDEN_A::DEACTIVATE,
            true => ADDEN_A::ACTIVATE,
        }
    }
    #[doc = "Checks if the value of the field is `DEACTIVATE`"]
    #[inline(always)]
    pub fn is_deactivate(&self) -> bool {
        *self == ADDEN_A::DEACTIVATE
    }
    #[doc = "Checks if the value of the field is `ACTIVATE`"]
    #[inline(always)]
    pub fn is_activate(&self) -> bool {
        *self == ADDEN_A::ACTIVATE
    }
}
#[doc = "Write proxy for field `ADDEN`"]
pub struct ADDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADDEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Address 0"]
    #[inline(always)]
    pub fn deactivate(self) -> &'a mut W {
        self.variant(ADDEN_A::DEACTIVATE)
    }
    #[doc = "Address active"]
    #[inline(always)]
    pub fn activate(self) -> &'a mut W {
        self.variant(ADDEN_A::ACTIVATE)
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
    #[doc = "Bits 0:6 - Device Address"]
    #[inline(always)]
    pub fn dadd(&self) -> DADD_R {
        DADD_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Device Address Enable"]
    #[inline(always)]
    pub fn adden(&self) -> ADDEN_R {
        ADDEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Device Address"]
    #[inline(always)]
    pub fn dadd(&mut self) -> DADD_W {
        DADD_W { w: self }
    }
    #[doc = "Bit 7 - Device Address Enable"]
    #[inline(always)]
    pub fn adden(&mut self) -> ADDEN_W {
        ADDEN_W { w: self }
    }
}
