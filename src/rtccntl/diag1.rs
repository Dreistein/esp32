#[doc = "Reader of register DIAG1"]
pub type R = crate::R<u32, super::DIAG1>;
#[doc = "Writer for register DIAG1"]
pub type W = crate::W<u32, super::DIAG1>;
#[doc = "Register DIAG1 `reset()`'s with value 0"]
impl crate::ResetValue for super::DIAG1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RTC_CNTL_LOW_POWER_DIAG1`"]
pub type RTC_CNTL_LOW_POWER_DIAG1_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RTC_CNTL_LOW_POWER_DIAG1`"]
pub struct RTC_CNTL_LOW_POWER_DIAG1_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_LOW_POWER_DIAG1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn rtc_cntl_low_power_diag1(&self) -> RTC_CNTL_LOW_POWER_DIAG1_R {
        RTC_CNTL_LOW_POWER_DIAG1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn rtc_cntl_low_power_diag1(&mut self) -> RTC_CNTL_LOW_POWER_DIAG1_W {
        RTC_CNTL_LOW_POWER_DIAG1_W { w: self }
    }
}