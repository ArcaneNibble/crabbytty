#[doc = "Writer for register WRCONFIG%s"]
pub type W = crate::W<u32, super::WRCONFIG>;
#[doc = "Register WRCONFIG%s `reset()`'s with value 0"]
impl crate::ResetValue for super::WRCONFIG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `PINMASK`"]
pub struct PINMASK_W<'a> {
    w: &'a mut W,
}
impl<'a> PINMASK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Peripheral Multiplexer Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PMUXEN_AW {
    #[doc = "0: Controlled by PORT"]
    PORT = 0,
    #[doc = "1: Controlled by peripheral"]
    PERIPH = 1,
}
impl From<PMUXEN_AW> for bool {
    #[inline(always)]
    fn from(variant: PMUXEN_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `PMUXEN`"]
pub struct PMUXEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PMUXEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PMUXEN_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Controlled by PORT"]
    #[inline(always)]
    pub fn port(self) -> &'a mut W {
        self.variant(PMUXEN_AW::PORT)
    }
    #[doc = "Controlled by peripheral"]
    #[inline(always)]
    pub fn periph(self) -> &'a mut W {
        self.variant(PMUXEN_AW::PERIPH)
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
#[doc = "Input Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INEN_AW {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<INEN_AW> for bool {
    #[inline(always)]
    fn from(variant: INEN_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `INEN`"]
pub struct INEN_W<'a> {
    w: &'a mut W,
}
impl<'a> INEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INEN_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(INEN_AW::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(INEN_AW::ENABLED)
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
#[doc = "Pull Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PULLEN_AW {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<PULLEN_AW> for bool {
    #[inline(always)]
    fn from(variant: PULLEN_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `PULLEN`"]
pub struct PULLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PULLEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PULLEN_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PULLEN_AW::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PULLEN_AW::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Peripheral Multiplexing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PMUX_AW {
    #[doc = "0: Peripheral function A selected"]
    A = 0,
    #[doc = "1: Peripheral function B selected"]
    B = 1,
    #[doc = "2: Peripheral function C selected"]
    C = 2,
    #[doc = "3: Peripheral function D selected"]
    D = 3,
    #[doc = "4: Peripheral function E selected"]
    E = 4,
    #[doc = "5: Peripheral function F selected"]
    F = 5,
    #[doc = "6: Peripheral function G selected"]
    G = 6,
    #[doc = "7: Peripheral function H selected"]
    H = 7,
}
impl From<PMUX_AW> for u8 {
    #[inline(always)]
    fn from(variant: PMUX_AW) -> Self {
        variant as _
    }
}
#[doc = "Write proxy for field `PMUX`"]
pub struct PMUX_W<'a> {
    w: &'a mut W,
}
impl<'a> PMUX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PMUX_AW) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Peripheral function A selected"]
    #[inline(always)]
    pub fn a(self) -> &'a mut W {
        self.variant(PMUX_AW::A)
    }
    #[doc = "Peripheral function B selected"]
    #[inline(always)]
    pub fn b(self) -> &'a mut W {
        self.variant(PMUX_AW::B)
    }
    #[doc = "Peripheral function C selected"]
    #[inline(always)]
    pub fn c(self) -> &'a mut W {
        self.variant(PMUX_AW::C)
    }
    #[doc = "Peripheral function D selected"]
    #[inline(always)]
    pub fn d(self) -> &'a mut W {
        self.variant(PMUX_AW::D)
    }
    #[doc = "Peripheral function E selected"]
    #[inline(always)]
    pub fn e(self) -> &'a mut W {
        self.variant(PMUX_AW::E)
    }
    #[doc = "Peripheral function F selected"]
    #[inline(always)]
    pub fn f(self) -> &'a mut W {
        self.variant(PMUX_AW::F)
    }
    #[doc = "Peripheral function G selected"]
    #[inline(always)]
    pub fn g(self) -> &'a mut W {
        self.variant(PMUX_AW::G)
    }
    #[doc = "Peripheral function H selected"]
    #[inline(always)]
    pub fn h(self) -> &'a mut W {
        self.variant(PMUX_AW::H)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Write PMUX\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WRPMUX_AW {
    #[doc = "0: Do not write"]
    NOWRITE = 0,
    #[doc = "1: Do write"]
    WRITE = 1,
}
impl From<WRPMUX_AW> for bool {
    #[inline(always)]
    fn from(variant: WRPMUX_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `WRPMUX`"]
pub struct WRPMUX_W<'a> {
    w: &'a mut W,
}
impl<'a> WRPMUX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WRPMUX_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Do not write"]
    #[inline(always)]
    pub fn nowrite(self) -> &'a mut W {
        self.variant(WRPMUX_AW::NOWRITE)
    }
    #[doc = "Do write"]
    #[inline(always)]
    pub fn write(self) -> &'a mut W {
        self.variant(WRPMUX_AW::WRITE)
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
#[doc = "Write PINCFG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WRPINCFG_AW {
    #[doc = "0: Do not write"]
    NOWRITE = 0,
    #[doc = "1: Do write"]
    WRITE = 1,
}
impl From<WRPINCFG_AW> for bool {
    #[inline(always)]
    fn from(variant: WRPINCFG_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `WRPINCFG`"]
pub struct WRPINCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> WRPINCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WRPINCFG_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Do not write"]
    #[inline(always)]
    pub fn nowrite(self) -> &'a mut W {
        self.variant(WRPINCFG_AW::NOWRITE)
    }
    #[doc = "Do write"]
    #[inline(always)]
    pub fn write(self) -> &'a mut W {
        self.variant(WRPINCFG_AW::WRITE)
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
#[doc = "Half-Word Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HWSEL_AW {
    #[doc = "0: Lower half"]
    LOWER = 0,
    #[doc = "1: Upper half"]
    UPPER = 1,
}
impl From<HWSEL_AW> for bool {
    #[inline(always)]
    fn from(variant: HWSEL_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `HWSEL`"]
pub struct HWSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> HWSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HWSEL_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Lower half"]
    #[inline(always)]
    pub fn lower(self) -> &'a mut W {
        self.variant(HWSEL_AW::LOWER)
    }
    #[doc = "Upper half"]
    #[inline(always)]
    pub fn upper(self) -> &'a mut W {
        self.variant(HWSEL_AW::UPPER)
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
impl W {
    #[doc = "Bits 0:15 - Pin Mask for Multiple Pin Configuration"]
    #[inline(always)]
    pub fn pinmask(&mut self) -> PINMASK_W {
        PINMASK_W { w: self }
    }
    #[doc = "Bit 16 - Peripheral Multiplexer Enable"]
    #[inline(always)]
    pub fn pmuxen(&mut self) -> PMUXEN_W {
        PMUXEN_W { w: self }
    }
    #[doc = "Bit 17 - Input Enable"]
    #[inline(always)]
    pub fn inen(&mut self) -> INEN_W {
        INEN_W { w: self }
    }
    #[doc = "Bit 18 - Pull Enable"]
    #[inline(always)]
    pub fn pullen(&mut self) -> PULLEN_W {
        PULLEN_W { w: self }
    }
    #[doc = "Bit 22 - Output Driver Strength Selection"]
    #[inline(always)]
    pub fn drvstr(&mut self) -> DRVSTR_W {
        DRVSTR_W { w: self }
    }
    #[doc = "Bits 24:27 - Peripheral Multiplexing"]
    #[inline(always)]
    pub fn pmux(&mut self) -> PMUX_W {
        PMUX_W { w: self }
    }
    #[doc = "Bit 28 - Write PMUX"]
    #[inline(always)]
    pub fn wrpmux(&mut self) -> WRPMUX_W {
        WRPMUX_W { w: self }
    }
    #[doc = "Bit 30 - Write PINCFG"]
    #[inline(always)]
    pub fn wrpincfg(&mut self) -> WRPINCFG_W {
        WRPINCFG_W { w: self }
    }
    #[doc = "Bit 31 - Half-Word Select"]
    #[inline(always)]
    pub fn hwsel(&mut self) -> HWSEL_W {
        HWSEL_W { w: self }
    }
}
