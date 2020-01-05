#[doc = "Reader of register SYNCBUSY"]
pub type R = crate::R<u8, super::SYNCBUSY>;
#[doc = "Software Reset Synchronization Busy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWRST_A {
    #[doc = "0: Sync complete"]
    COMPLETE = 0,
    #[doc = "1: Syncing"]
    SYNCING = 1,
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
            false => SWRST_A::COMPLETE,
            true => SWRST_A::SYNCING,
        }
    }
    #[doc = "Checks if the value of the field is `COMPLETE`"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == SWRST_A::COMPLETE
    }
    #[doc = "Checks if the value of the field is `SYNCING`"]
    #[inline(always)]
    pub fn is_syncing(&self) -> bool {
        *self == SWRST_A::SYNCING
    }
}
#[doc = "Enable Synchronization Busy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLE_A {
    #[doc = "0: Sync complete"]
    COMPLETE = 0,
    #[doc = "1: Syncing"]
    SYNCING = 1,
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
            false => ENABLE_A::COMPLETE,
            true => ENABLE_A::SYNCING,
        }
    }
    #[doc = "Checks if the value of the field is `COMPLETE`"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == ENABLE_A::COMPLETE
    }
    #[doc = "Checks if the value of the field is `SYNCING`"]
    #[inline(always)]
    pub fn is_syncing(&self) -> bool {
        *self == ENABLE_A::SYNCING
    }
}
impl R {
    #[doc = "Bit 0 - Software Reset Synchronization Busy"]
    #[inline(always)]
    pub fn swrst(&self) -> SWRST_R {
        SWRST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable Synchronization Busy"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
