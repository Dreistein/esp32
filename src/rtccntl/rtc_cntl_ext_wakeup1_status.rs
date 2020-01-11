#[doc = "Reader of register RTC_CNTL_EXT_WAKEUP1_STATUS"]
pub type R = crate::R<u32, super::RTC_CNTL_EXT_WAKEUP1_STATUS>;
#[doc = "Writer for register RTC_CNTL_EXT_WAKEUP1_STATUS"]
pub type W = crate::W<u32, super::RTC_CNTL_EXT_WAKEUP1_STATUS>;
#[doc = "Register RTC_CNTL_EXT_WAKEUP1_STATUS `reset()`'s with value 0"]
impl crate::ResetValue for super::RTC_CNTL_EXT_WAKEUP1_STATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RTC_CNTL_EXT_WAKEUP1_STATUS`"]
pub type RTC_CNTL_EXT_WAKEUP1_STATUS_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RTC_CNTL_EXT_WAKEUP1_STATUS`"]
pub struct RTC_CNTL_EXT_WAKEUP1_STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_EXT_WAKEUP1_STATUS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0003_ffff) | ((value as u32) & 0x0003_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:17 - ext wakeup1 status"]
    #[inline(always)]
    pub fn rtc_cntl_ext_wakeup1_status(&self) -> RTC_CNTL_EXT_WAKEUP1_STATUS_R {
        RTC_CNTL_EXT_WAKEUP1_STATUS_R::new((self.bits & 0x0003_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:17 - ext wakeup1 status"]
    #[inline(always)]
    pub fn rtc_cntl_ext_wakeup1_status(&mut self) -> RTC_CNTL_EXT_WAKEUP1_STATUS_W {
        RTC_CNTL_EXT_WAKEUP1_STATUS_W { w: self }
    }
}