#[doc = "Reader of register GENCTRL"]
pub type R = crate::R<u32, super::GENCTRL>;
#[doc = "Writer for register GENCTRL"]
pub type W = crate::W<u32, super::GENCTRL>;
#[doc = "Register GENCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::GENCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Generic Clock Generator Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ID_A {
    #[doc = "0: Generic clock generator 0"]
    GCLK0,
    #[doc = "1: Generic clock generator 1"]
    GCLK1,
    #[doc = "2: Generic clock generator 2"]
    GCLK2,
    #[doc = "3: Generic clock generator 3"]
    GCLK3,
    #[doc = "4: Generic clock generator 4"]
    GCLK4,
    #[doc = "5: Generic clock generator 5"]
    GCLK5,
}
impl From<ID_A> for u8 {
    #[inline(always)]
    fn from(variant: ID_A) -> Self {
        match variant {
            ID_A::GCLK0 => 0,
            ID_A::GCLK1 => 1,
            ID_A::GCLK2 => 2,
            ID_A::GCLK3 => 3,
            ID_A::GCLK4 => 4,
            ID_A::GCLK5 => 5,
        }
    }
}
#[doc = "Reader of field `ID`"]
pub type ID_R = crate::R<u8, ID_A>;
impl ID_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ID_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ID_A::GCLK0),
            1 => Val(ID_A::GCLK1),
            2 => Val(ID_A::GCLK2),
            3 => Val(ID_A::GCLK3),
            4 => Val(ID_A::GCLK4),
            5 => Val(ID_A::GCLK5),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `GCLK0`"]
    #[inline(always)]
    pub fn is_gclk0(&self) -> bool {
        *self == ID_A::GCLK0
    }
    #[doc = "Checks if the value of the field is `GCLK1`"]
    #[inline(always)]
    pub fn is_gclk1(&self) -> bool {
        *self == ID_A::GCLK1
    }
    #[doc = "Checks if the value of the field is `GCLK2`"]
    #[inline(always)]
    pub fn is_gclk2(&self) -> bool {
        *self == ID_A::GCLK2
    }
    #[doc = "Checks if the value of the field is `GCLK3`"]
    #[inline(always)]
    pub fn is_gclk3(&self) -> bool {
        *self == ID_A::GCLK3
    }
    #[doc = "Checks if the value of the field is `GCLK4`"]
    #[inline(always)]
    pub fn is_gclk4(&self) -> bool {
        *self == ID_A::GCLK4
    }
    #[doc = "Checks if the value of the field is `GCLK5`"]
    #[inline(always)]
    pub fn is_gclk5(&self) -> bool {
        *self == ID_A::GCLK5
    }
}
#[doc = "Write proxy for field `ID`"]
pub struct ID_W<'a> {
    w: &'a mut W,
}
impl<'a> ID_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ID_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Generic clock generator 0"]
    #[inline(always)]
    pub fn gclk0(self) -> &'a mut W {
        self.variant(ID_A::GCLK0)
    }
    #[doc = "Generic clock generator 1"]
    #[inline(always)]
    pub fn gclk1(self) -> &'a mut W {
        self.variant(ID_A::GCLK1)
    }
    #[doc = "Generic clock generator 2"]
    #[inline(always)]
    pub fn gclk2(self) -> &'a mut W {
        self.variant(ID_A::GCLK2)
    }
    #[doc = "Generic clock generator 3"]
    #[inline(always)]
    pub fn gclk3(self) -> &'a mut W {
        self.variant(ID_A::GCLK3)
    }
    #[doc = "Generic clock generator 4"]
    #[inline(always)]
    pub fn gclk4(self) -> &'a mut W {
        self.variant(ID_A::GCLK4)
    }
    #[doc = "Generic clock generator 5"]
    #[inline(always)]
    pub fn gclk5(self) -> &'a mut W {
        self.variant(ID_A::GCLK5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRC_A {
    #[doc = "0: XOSC oscillator output"]
    XOSC,
    #[doc = "1: Generator input pad"]
    GCLKIN,
    #[doc = "2: Generic clock generator 1 output"]
    GCLKGEN1,
    #[doc = "3: OSCULP32K oscillator output"]
    OSCULP32K,
    #[doc = "4: OSC32K oscillator output"]
    OSC32K,
    #[doc = "5: XOSC32K oscillator output"]
    XOSC32K,
    #[doc = "6: OSC8M oscillator output"]
    OSC8M,
    #[doc = "7: DFLL48M output"]
    DFLL48M,
    #[doc = "8: DPLL96M output"]
    DPLL96M,
}
impl From<SRC_A> for u8 {
    #[inline(always)]
    fn from(variant: SRC_A) -> Self {
        match variant {
            SRC_A::XOSC => 0,
            SRC_A::GCLKIN => 1,
            SRC_A::GCLKGEN1 => 2,
            SRC_A::OSCULP32K => 3,
            SRC_A::OSC32K => 4,
            SRC_A::XOSC32K => 5,
            SRC_A::OSC8M => 6,
            SRC_A::DFLL48M => 7,
            SRC_A::DPLL96M => 8,
        }
    }
}
#[doc = "Reader of field `SRC`"]
pub type SRC_R = crate::R<u8, SRC_A>;
impl SRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SRC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SRC_A::XOSC),
            1 => Val(SRC_A::GCLKIN),
            2 => Val(SRC_A::GCLKGEN1),
            3 => Val(SRC_A::OSCULP32K),
            4 => Val(SRC_A::OSC32K),
            5 => Val(SRC_A::XOSC32K),
            6 => Val(SRC_A::OSC8M),
            7 => Val(SRC_A::DFLL48M),
            8 => Val(SRC_A::DPLL96M),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `XOSC`"]
    #[inline(always)]
    pub fn is_xosc(&self) -> bool {
        *self == SRC_A::XOSC
    }
    #[doc = "Checks if the value of the field is `GCLKIN`"]
    #[inline(always)]
    pub fn is_gclkin(&self) -> bool {
        *self == SRC_A::GCLKIN
    }
    #[doc = "Checks if the value of the field is `GCLKGEN1`"]
    #[inline(always)]
    pub fn is_gclkgen1(&self) -> bool {
        *self == SRC_A::GCLKGEN1
    }
    #[doc = "Checks if the value of the field is `OSCULP32K`"]
    #[inline(always)]
    pub fn is_osculp32k(&self) -> bool {
        *self == SRC_A::OSCULP32K
    }
    #[doc = "Checks if the value of the field is `OSC32K`"]
    #[inline(always)]
    pub fn is_osc32k(&self) -> bool {
        *self == SRC_A::OSC32K
    }
    #[doc = "Checks if the value of the field is `XOSC32K`"]
    #[inline(always)]
    pub fn is_xosc32k(&self) -> bool {
        *self == SRC_A::XOSC32K
    }
    #[doc = "Checks if the value of the field is `OSC8M`"]
    #[inline(always)]
    pub fn is_osc8m(&self) -> bool {
        *self == SRC_A::OSC8M
    }
    #[doc = "Checks if the value of the field is `DFLL48M`"]
    #[inline(always)]
    pub fn is_dfll48m(&self) -> bool {
        *self == SRC_A::DFLL48M
    }
    #[doc = "Checks if the value of the field is `DPLL96M`"]
    #[inline(always)]
    pub fn is_dpll96m(&self) -> bool {
        *self == SRC_A::DPLL96M
    }
}
#[doc = "Write proxy for field `SRC`"]
pub struct SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "XOSC oscillator output"]
    #[inline(always)]
    pub fn xosc(self) -> &'a mut W {
        self.variant(SRC_A::XOSC)
    }
    #[doc = "Generator input pad"]
    #[inline(always)]
    pub fn gclkin(self) -> &'a mut W {
        self.variant(SRC_A::GCLKIN)
    }
    #[doc = "Generic clock generator 1 output"]
    #[inline(always)]
    pub fn gclkgen1(self) -> &'a mut W {
        self.variant(SRC_A::GCLKGEN1)
    }
    #[doc = "OSCULP32K oscillator output"]
    #[inline(always)]
    pub fn osculp32k(self) -> &'a mut W {
        self.variant(SRC_A::OSCULP32K)
    }
    #[doc = "OSC32K oscillator output"]
    #[inline(always)]
    pub fn osc32k(self) -> &'a mut W {
        self.variant(SRC_A::OSC32K)
    }
    #[doc = "XOSC32K oscillator output"]
    #[inline(always)]
    pub fn xosc32k(self) -> &'a mut W {
        self.variant(SRC_A::XOSC32K)
    }
    #[doc = "OSC8M oscillator output"]
    #[inline(always)]
    pub fn osc8m(self) -> &'a mut W {
        self.variant(SRC_A::OSC8M)
    }
    #[doc = "DFLL48M output"]
    #[inline(always)]
    pub fn dfll48m(self) -> &'a mut W {
        self.variant(SRC_A::DFLL48M)
    }
    #[doc = "DPLL96M output"]
    #[inline(always)]
    pub fn dpll96m(self) -> &'a mut W {
        self.variant(SRC_A::DPLL96M)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = "Generic Clock Generator Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GENEN_A {
    #[doc = "0: Disabled"]
    DISABLED,
    #[doc = "1: Enabled"]
    ENABLED,
}
impl From<GENEN_A> for bool {
    #[inline(always)]
    fn from(variant: GENEN_A) -> Self {
        match variant {
            GENEN_A::DISABLED => false,
            GENEN_A::ENABLED => true,
        }
    }
}
#[doc = "Reader of field `GENEN`"]
pub type GENEN_R = crate::R<bool, GENEN_A>;
impl GENEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GENEN_A {
        match self.bits {
            false => GENEN_A::DISABLED,
            true => GENEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GENEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GENEN_A::ENABLED
    }
}
#[doc = "Write proxy for field `GENEN`"]
pub struct GENEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GENEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GENEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GENEN_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GENEN_A::ENABLED)
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
#[doc = "Improve Duty Cycle\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDC_A {
    #[doc = "0: Not 50/50"]
    UNEVEN,
    #[doc = "1: 50/50"]
    EVEN,
}
impl From<IDC_A> for bool {
    #[inline(always)]
    fn from(variant: IDC_A) -> Self {
        match variant {
            IDC_A::UNEVEN => false,
            IDC_A::EVEN => true,
        }
    }
}
#[doc = "Reader of field `IDC`"]
pub type IDC_R = crate::R<bool, IDC_A>;
impl IDC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IDC_A {
        match self.bits {
            false => IDC_A::UNEVEN,
            true => IDC_A::EVEN,
        }
    }
    #[doc = "Checks if the value of the field is `UNEVEN`"]
    #[inline(always)]
    pub fn is_uneven(&self) -> bool {
        *self == IDC_A::UNEVEN
    }
    #[doc = "Checks if the value of the field is `EVEN`"]
    #[inline(always)]
    pub fn is_even(&self) -> bool {
        *self == IDC_A::EVEN
    }
}
#[doc = "Write proxy for field `IDC`"]
pub struct IDC_W<'a> {
    w: &'a mut W,
}
impl<'a> IDC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IDC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Not 50/50"]
    #[inline(always)]
    pub fn uneven(self) -> &'a mut W {
        self.variant(IDC_A::UNEVEN)
    }
    #[doc = "50/50"]
    #[inline(always)]
    pub fn even(self) -> &'a mut W {
        self.variant(IDC_A::EVEN)
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
#[doc = "Output Off Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OOV_A {
    #[doc = "0: 0"]
    ZERO,
    #[doc = "1: 1"]
    ONE,
}
impl From<OOV_A> for bool {
    #[inline(always)]
    fn from(variant: OOV_A) -> Self {
        match variant {
            OOV_A::ZERO => false,
            OOV_A::ONE => true,
        }
    }
}
#[doc = "Reader of field `OOV`"]
pub type OOV_R = crate::R<bool, OOV_A>;
impl OOV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OOV_A {
        match self.bits {
            false => OOV_A::ZERO,
            true => OOV_A::ONE,
        }
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == OOV_A::ZERO
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == OOV_A::ONE
    }
}
#[doc = "Write proxy for field `OOV`"]
pub struct OOV_W<'a> {
    w: &'a mut W,
}
impl<'a> OOV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OOV_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(OOV_A::ZERO)
    }
    #[doc = "1"]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(OOV_A::ONE)
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
#[doc = "Output Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OE_A {
    #[doc = "0: Not output to pin"]
    NOTOUTPUT,
    #[doc = "1: Output to pin"]
    OUTPUT,
}
impl From<OE_A> for bool {
    #[inline(always)]
    fn from(variant: OE_A) -> Self {
        match variant {
            OE_A::NOTOUTPUT => false,
            OE_A::OUTPUT => true,
        }
    }
}
#[doc = "Reader of field `OE`"]
pub type OE_R = crate::R<bool, OE_A>;
impl OE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OE_A {
        match self.bits {
            false => OE_A::NOTOUTPUT,
            true => OE_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `NOTOUTPUT`"]
    #[inline(always)]
    pub fn is_notoutput(&self) -> bool {
        *self == OE_A::NOTOUTPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == OE_A::OUTPUT
    }
}
#[doc = "Write proxy for field `OE`"]
pub struct OE_W<'a> {
    w: &'a mut W,
}
impl<'a> OE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Not output to pin"]
    #[inline(always)]
    pub fn notoutput(self) -> &'a mut W {
        self.variant(OE_A::NOTOUTPUT)
    }
    #[doc = "Output to pin"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(OE_A::OUTPUT)
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
#[doc = "Divide Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIVSEL_A {
    #[doc = "0: Divide by DIV"]
    DIV,
    #[doc = "1: Divide by 2^(DIV+1)"]
    POWER,
}
impl From<DIVSEL_A> for bool {
    #[inline(always)]
    fn from(variant: DIVSEL_A) -> Self {
        match variant {
            DIVSEL_A::DIV => false,
            DIVSEL_A::POWER => true,
        }
    }
}
#[doc = "Reader of field `DIVSEL`"]
pub type DIVSEL_R = crate::R<bool, DIVSEL_A>;
impl DIVSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIVSEL_A {
        match self.bits {
            false => DIVSEL_A::DIV,
            true => DIVSEL_A::POWER,
        }
    }
    #[doc = "Checks if the value of the field is `DIV`"]
    #[inline(always)]
    pub fn is_div(&self) -> bool {
        *self == DIVSEL_A::DIV
    }
    #[doc = "Checks if the value of the field is `POWER`"]
    #[inline(always)]
    pub fn is_power(&self) -> bool {
        *self == DIVSEL_A::POWER
    }
}
#[doc = "Write proxy for field `DIVSEL`"]
pub struct DIVSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIVSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Divide by DIV"]
    #[inline(always)]
    pub fn div(self) -> &'a mut W {
        self.variant(DIVSEL_A::DIV)
    }
    #[doc = "Divide by 2^(DIV+1)"]
    #[inline(always)]
    pub fn power(self) -> &'a mut W {
        self.variant(DIVSEL_A::POWER)
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
#[doc = "Run in Standby\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RUNSTDBY_A {
    #[doc = "0: Stopped in standby"]
    STOPPED,
    #[doc = "1: Running in standby"]
    RUNNING,
}
impl From<RUNSTDBY_A> for bool {
    #[inline(always)]
    fn from(variant: RUNSTDBY_A) -> Self {
        match variant {
            RUNSTDBY_A::STOPPED => false,
            RUNSTDBY_A::RUNNING => true,
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Generic Clock Generator Selection"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:12 - Source Select"]
    #[inline(always)]
    pub fn src(&self) -> SRC_R {
        SRC_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 16 - Generic Clock Generator Enable"]
    #[inline(always)]
    pub fn genen(&self) -> GENEN_R {
        GENEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Improve Duty Cycle"]
    #[inline(always)]
    pub fn idc(&self) -> IDC_R {
        IDC_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Output Off Value"]
    #[inline(always)]
    pub fn oov(&self) -> OOV_R {
        OOV_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Output Enable"]
    #[inline(always)]
    pub fn oe(&self) -> OE_R {
        OE_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Divide Selection"]
    #[inline(always)]
    pub fn divsel(&self) -> DIVSEL_R {
        DIVSEL_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RUNSTDBY_R {
        RUNSTDBY_R::new(((self.bits >> 21) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Generic Clock Generator Selection"]
    #[inline(always)]
    pub fn id(&mut self) -> ID_W {
        ID_W { w: self }
    }
    #[doc = "Bits 8:12 - Source Select"]
    #[inline(always)]
    pub fn src(&mut self) -> SRC_W {
        SRC_W { w: self }
    }
    #[doc = "Bit 16 - Generic Clock Generator Enable"]
    #[inline(always)]
    pub fn genen(&mut self) -> GENEN_W {
        GENEN_W { w: self }
    }
    #[doc = "Bit 17 - Improve Duty Cycle"]
    #[inline(always)]
    pub fn idc(&mut self) -> IDC_W {
        IDC_W { w: self }
    }
    #[doc = "Bit 18 - Output Off Value"]
    #[inline(always)]
    pub fn oov(&mut self) -> OOV_W {
        OOV_W { w: self }
    }
    #[doc = "Bit 19 - Output Enable"]
    #[inline(always)]
    pub fn oe(&mut self) -> OE_W {
        OE_W { w: self }
    }
    #[doc = "Bit 20 - Divide Selection"]
    #[inline(always)]
    pub fn divsel(&mut self) -> DIVSEL_W {
        DIVSEL_W { w: self }
    }
    #[doc = "Bit 21 - Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&mut self) -> RUNSTDBY_W {
        RUNSTDBY_W { w: self }
    }
}
