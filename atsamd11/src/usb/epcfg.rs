#[doc = "Reader of register EPCFG%s"]
pub type R = crate::R<u8, super::EPCFG>;
#[doc = "Writer for register EPCFG%s"]
pub type W = crate::W<u8, super::EPCFG>;
#[doc = "Register EPCFG%s `reset()`'s with value 0"]
impl crate::ResetValue for super::EPCFG {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "End Point Type0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EPTYPE0_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Control OUT"]
    CONTROLOUT = 1,
    #[doc = "2: Isochronous OUT"]
    ISOCHOUT = 2,
    #[doc = "3: Bulk OUT"]
    BULKOUT = 3,
    #[doc = "4: Interrupt OUT"]
    INTERRUPTOUT = 4,
    #[doc = "5: Dual-Bank IN"]
    DUALBANKIN = 5,
}
impl From<EPTYPE0_A> for u8 {
    #[inline(always)]
    fn from(variant: EPTYPE0_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `EPTYPE0`"]
pub type EPTYPE0_R = crate::R<u8, EPTYPE0_A>;
impl EPTYPE0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, EPTYPE0_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(EPTYPE0_A::DISABLED),
            1 => Val(EPTYPE0_A::CONTROLOUT),
            2 => Val(EPTYPE0_A::ISOCHOUT),
            3 => Val(EPTYPE0_A::BULKOUT),
            4 => Val(EPTYPE0_A::INTERRUPTOUT),
            5 => Val(EPTYPE0_A::DUALBANKIN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EPTYPE0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `CONTROLOUT`"]
    #[inline(always)]
    pub fn is_controlout(&self) -> bool {
        *self == EPTYPE0_A::CONTROLOUT
    }
    #[doc = "Checks if the value of the field is `ISOCHOUT`"]
    #[inline(always)]
    pub fn is_isochout(&self) -> bool {
        *self == EPTYPE0_A::ISOCHOUT
    }
    #[doc = "Checks if the value of the field is `BULKOUT`"]
    #[inline(always)]
    pub fn is_bulkout(&self) -> bool {
        *self == EPTYPE0_A::BULKOUT
    }
    #[doc = "Checks if the value of the field is `INTERRUPTOUT`"]
    #[inline(always)]
    pub fn is_interruptout(&self) -> bool {
        *self == EPTYPE0_A::INTERRUPTOUT
    }
    #[doc = "Checks if the value of the field is `DUALBANKIN`"]
    #[inline(always)]
    pub fn is_dualbankin(&self) -> bool {
        *self == EPTYPE0_A::DUALBANKIN
    }
}
#[doc = "Write proxy for field `EPTYPE0`"]
pub struct EPTYPE0_W<'a> {
    w: &'a mut W,
}
impl<'a> EPTYPE0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EPTYPE0_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EPTYPE0_A::DISABLED)
    }
    #[doc = "Control OUT"]
    #[inline(always)]
    pub fn controlout(self) -> &'a mut W {
        self.variant(EPTYPE0_A::CONTROLOUT)
    }
    #[doc = "Isochronous OUT"]
    #[inline(always)]
    pub fn isochout(self) -> &'a mut W {
        self.variant(EPTYPE0_A::ISOCHOUT)
    }
    #[doc = "Bulk OUT"]
    #[inline(always)]
    pub fn bulkout(self) -> &'a mut W {
        self.variant(EPTYPE0_A::BULKOUT)
    }
    #[doc = "Interrupt OUT"]
    #[inline(always)]
    pub fn interruptout(self) -> &'a mut W {
        self.variant(EPTYPE0_A::INTERRUPTOUT)
    }
    #[doc = "Dual-Bank IN"]
    #[inline(always)]
    pub fn dualbankin(self) -> &'a mut W {
        self.variant(EPTYPE0_A::DUALBANKIN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u8) & 0x07);
        self.w
    }
}
#[doc = "End Point Type1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EPTYPE1_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Control IN"]
    CONTROLIN = 1,
    #[doc = "2: Isochronous IN"]
    ISOCHIN = 2,
    #[doc = "3: Bulk IN"]
    BULKIN = 3,
    #[doc = "4: Interrupt IN"]
    INTERRUPTIN = 4,
    #[doc = "5: Dual-Bank OUT"]
    DUALBANKOUT = 5,
}
impl From<EPTYPE1_A> for u8 {
    #[inline(always)]
    fn from(variant: EPTYPE1_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `EPTYPE1`"]
pub type EPTYPE1_R = crate::R<u8, EPTYPE1_A>;
impl EPTYPE1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, EPTYPE1_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(EPTYPE1_A::DISABLED),
            1 => Val(EPTYPE1_A::CONTROLIN),
            2 => Val(EPTYPE1_A::ISOCHIN),
            3 => Val(EPTYPE1_A::BULKIN),
            4 => Val(EPTYPE1_A::INTERRUPTIN),
            5 => Val(EPTYPE1_A::DUALBANKOUT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EPTYPE1_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `CONTROLIN`"]
    #[inline(always)]
    pub fn is_controlin(&self) -> bool {
        *self == EPTYPE1_A::CONTROLIN
    }
    #[doc = "Checks if the value of the field is `ISOCHIN`"]
    #[inline(always)]
    pub fn is_isochin(&self) -> bool {
        *self == EPTYPE1_A::ISOCHIN
    }
    #[doc = "Checks if the value of the field is `BULKIN`"]
    #[inline(always)]
    pub fn is_bulkin(&self) -> bool {
        *self == EPTYPE1_A::BULKIN
    }
    #[doc = "Checks if the value of the field is `INTERRUPTIN`"]
    #[inline(always)]
    pub fn is_interruptin(&self) -> bool {
        *self == EPTYPE1_A::INTERRUPTIN
    }
    #[doc = "Checks if the value of the field is `DUALBANKOUT`"]
    #[inline(always)]
    pub fn is_dualbankout(&self) -> bool {
        *self == EPTYPE1_A::DUALBANKOUT
    }
}
#[doc = "Write proxy for field `EPTYPE1`"]
pub struct EPTYPE1_W<'a> {
    w: &'a mut W,
}
impl<'a> EPTYPE1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EPTYPE1_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EPTYPE1_A::DISABLED)
    }
    #[doc = "Control IN"]
    #[inline(always)]
    pub fn controlin(self) -> &'a mut W {
        self.variant(EPTYPE1_A::CONTROLIN)
    }
    #[doc = "Isochronous IN"]
    #[inline(always)]
    pub fn isochin(self) -> &'a mut W {
        self.variant(EPTYPE1_A::ISOCHIN)
    }
    #[doc = "Bulk IN"]
    #[inline(always)]
    pub fn bulkin(self) -> &'a mut W {
        self.variant(EPTYPE1_A::BULKIN)
    }
    #[doc = "Interrupt IN"]
    #[inline(always)]
    pub fn interruptin(self) -> &'a mut W {
        self.variant(EPTYPE1_A::INTERRUPTIN)
    }
    #[doc = "Dual-Bank OUT"]
    #[inline(always)]
    pub fn dualbankout(self) -> &'a mut W {
        self.variant(EPTYPE1_A::DUALBANKOUT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u8) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `NYETDIS`"]
pub type NYETDIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NYETDIS`"]
pub struct NYETDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> NYETDIS_W<'a> {
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
    #[doc = "Bits 0:2 - End Point Type0"]
    #[inline(always)]
    pub fn eptype0(&self) -> EPTYPE0_R {
        EPTYPE0_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 4:6 - End Point Type1"]
    #[inline(always)]
    pub fn eptype1(&self) -> EPTYPE1_R {
        EPTYPE1_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 7 - NYET Token Disable"]
    #[inline(always)]
    pub fn nyetdis(&self) -> NYETDIS_R {
        NYETDIS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - End Point Type0"]
    #[inline(always)]
    pub fn eptype0(&mut self) -> EPTYPE0_W {
        EPTYPE0_W { w: self }
    }
    #[doc = "Bits 4:6 - End Point Type1"]
    #[inline(always)]
    pub fn eptype1(&mut self) -> EPTYPE1_W {
        EPTYPE1_W { w: self }
    }
    #[doc = "Bit 7 - NYET Token Disable"]
    #[inline(always)]
    pub fn nyetdis(&mut self) -> NYETDIS_W {
        NYETDIS_W { w: self }
    }
}
