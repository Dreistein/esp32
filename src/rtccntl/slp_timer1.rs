#[doc = "Reader of register SLP_TIMER1"]
pub type R = crate::R<u32, super::SLP_TIMER1>;
#[doc = "Writer for register SLP_TIMER1"]
pub type W = crate::W<u32, super::SLP_TIMER1>;
#[doc = "Register SLP_TIMER1 `reset()`'s with value 0"]
impl crate::ResetValue for super::SLP_TIMER1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RTC_CNTL_MAIN_TIMER_ALARM_EN`"]
pub type RTC_CNTL_MAIN_TIMER_ALARM_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_MAIN_TIMER_ALARM_EN`"]
pub struct RTC_CNTL_MAIN_TIMER_ALARM_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_MAIN_TIMER_ALARM_EN_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_SLP_VAL_HI`"]
pub type RTC_CNTL_SLP_VAL_HI_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RTC_CNTL_SLP_VAL_HI`"]
pub struct RTC_CNTL_SLP_VAL_HI_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_SLP_VAL_HI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bit 16 - timer alarm enable bit"]
    #[inline(always)]
    pub fn rtc_cntl_main_timer_alarm_en(&self) -> RTC_CNTL_MAIN_TIMER_ALARM_EN_R {
        RTC_CNTL_MAIN_TIMER_ALARM_EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 0:15 - RTC sleep timer high 16 bits"]
    #[inline(always)]
    pub fn rtc_cntl_slp_val_hi(&self) -> RTC_CNTL_SLP_VAL_HI_R {
        RTC_CNTL_SLP_VAL_HI_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 16 - timer alarm enable bit"]
    #[inline(always)]
    pub fn rtc_cntl_main_timer_alarm_en(&mut self) -> RTC_CNTL_MAIN_TIMER_ALARM_EN_W {
        RTC_CNTL_MAIN_TIMER_ALARM_EN_W { w: self }
    }
    #[doc = "Bits 0:15 - RTC sleep timer high 16 bits"]
    #[inline(always)]
    pub fn rtc_cntl_slp_val_hi(&mut self) -> RTC_CNTL_SLP_VAL_HI_W {
        RTC_CNTL_SLP_VAL_HI_W { w: self }
    }
}