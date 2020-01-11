#[doc = "Reader of register RTC_GPIO_ENABLE"]
pub type R = crate::R<u32, super::RTC_GPIO_ENABLE>;
#[doc = "Writer for register RTC_GPIO_ENABLE"]
pub type W = crate::W<u32, super::RTC_GPIO_ENABLE>;
#[doc = "Register RTC_GPIO_ENABLE `reset()`'s with value 0"]
impl crate::ResetValue for super::RTC_GPIO_ENABLE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RTC_GPIO_ENABLE`"]
pub type RTC_GPIO_ENABLE_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RTC_GPIO_ENABLE`"]
pub struct RTC_GPIO_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_GPIO_ENABLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0003_ffff << 14)) | (((value as u32) & 0x0003_ffff) << 14);
        self.w
    }
}
impl R {
    #[doc = "Bits 14:31 - GPIO0~17 output enable"]
    #[inline(always)]
    pub fn rtc_gpio_enable(&self) -> RTC_GPIO_ENABLE_R {
        RTC_GPIO_ENABLE_R::new(((self.bits >> 14) & 0x0003_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 14:31 - GPIO0~17 output enable"]
    #[inline(always)]
    pub fn rtc_gpio_enable(&mut self) -> RTC_GPIO_ENABLE_W {
        RTC_GPIO_ENABLE_W { w: self }
    }
}