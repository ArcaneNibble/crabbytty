#[doc = "Reader of register PINCFG0_%s"]
pub type R = crate::R<u8, super::PINCFG0_>;
#[doc = "Writer for register PINCFG0_%s"]
pub type W = crate::W<u8, super::PINCFG0_>;
#[doc = "Register PINCFG0_%s `reset()`'s with value 0"]
impl crate::ResetValue for super::PINCFG0_ {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Peripheral Multiplexer Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PMUXEN_A {
    #[doc = "0: Controlled by PORT"]
    PORT = 0,
    #[doc = "1: Controlled by peripheral"]
    PERIPH = 1,
}
impl From<PMUXEN_A> for bool {
    #[inline(always)]
    fn from(variant: PMUXEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PMUXEN`"]
pub type PMUXEN_R = crate::R<bool, PMUXEN_A>;
impl PMUXEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PMUXEN_A {
        match self.bits {
            false => PMUXEN_A::PORT,
            true => PMUXEN_A::PERIPH,
        }
    }
    #[doc = "Checks if the value of the field is `PORT`"]
    #[inline(always)]
    pub fn is_port(&self) -> bool {
        *self == PMUXEN_A::PORT
    }
    #[doc = "Checks if the value of the field is `PERIPH`"]
    #[inline(always)]
    pub fn is_periph(&self) -> bool {
        *self == PMUXEN_A::PERIPH
    }
}
#[doc = "Write proxy for field `PMUXEN`"]
pub struct PMUXEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PMUXEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PMUXEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Controlled by PORT"]
    #[inline(always)]
    pub fn port(self) -> &'a mut W {
        self.variant(PMUXEN_A::PORT)
    }
    #[doc = "Controlled by peripheral"]
    #[inline(always)]
    pub fn periph(self) -> &'a mut W {
        self.variant(PMUXEN_A::PERIPH)
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
#[doc = "Input Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INEN_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<INEN_A> for bool {
    #[inline(always)]
    fn from(variant: INEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `INEN`"]
pub type INEN_R = crate::R<bool, INEN_A>;
impl INEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INEN_A {
        match self.bits {
            false => INEN_A::DISABLED,
            true => INEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == INEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == INEN_A::ENABLED
    }
}
#[doc = "Write proxy for field `INEN`"]
pub struct INEN_W<'a> {
    w: &'a mut W,
}
impl<'a> INEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(INEN_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(INEN_A::ENABLED)
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
#[doc = "Pull Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PULLEN_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<PULLEN_A> for bool {
    #[inline(always)]
    fn from(variant: PULLEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PULLEN`"]
pub type PULLEN_R = crate::R<bool, PULLEN_A>;
impl PULLEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PULLEN_A {
        match self.bits {
            false => PULLEN_A::DISABLED,
            true => PULLEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PULLEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PULLEN_A::ENABLED
    }
}
#[doc = "Write proxy for field `PULLEN`"]
pub struct PULLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PULLEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PULLEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PULLEN_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PULLEN_A::ENABLED)
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
#[doc = "Output Driver Strength Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DRVSTR_AW {
    #[doc = "0: Normal drive"]
    NORMAL = 0,
    #[doc = "1: Stronger drive"]
    STRONG = 1,
}
impl From<DRVSTR_AW> for bool {
    #[inline(always)]
    fn from(variant: DRVSTR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `DRVSTR`"]
pub struct DRVSTR_W<'a> {
    w: &'a mut W,
}
impl<'a> DRVSTR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DRVSTR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal drive"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(DRVSTR_AW::NORMAL)
    }
    #[doc = "Stronger drive"]
    #[inline(always)]
    pub fn strong(self) -> &'a mut W {
        self.variant(DRVSTR_AW::STRONG)
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
impl R {
    #[doc = "Bit 0 - Peripheral Multiplexer Enable"]
    #[inline(always)]
    pub fn pmuxen(&self) -> PMUXEN_R {
        PMUXEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Input Enable"]
    #[inline(always)]
    pub fn inen(&self) -> INEN_R {
        INEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Pull Enable"]
    #[inline(always)]
    pub fn pullen(&self) -> PULLEN_R {
        PULLEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Peripheral Multiplexer Enable"]
    #[inline(always)]
    pub fn pmuxen(&mut self) -> PMUXEN_W {
        PMUXEN_W { w: self }
    }
    #[doc = "Bit 1 - Input Enable"]
    #[inline(always)]
    pub fn inen(&mut self) -> INEN_W {
        INEN_W { w: self }
    }
    #[doc = "Bit 2 - Pull Enable"]
    #[inline(always)]
    pub fn pullen(&mut self) -> PULLEN_W {
        PULLEN_W { w: self }
    }
    #[doc = "Bit 6 - Output Driver Strength Selection"]
    #[inline(always)]
    pub fn drvstr(&mut self) -> DRVSTR_W {
        DRVSTR_W { w: self }
    }
}
