#[doc = "Reader of register ENABLE"]
pub type R = crate::R<u32, super::ENABLE>;
#[doc = "Writer for register ENABLE"]
pub type W = crate::W<u32, super::ENABLE>;
#[doc = "Register ENABLE `reset()`'s with value 0"]
impl crate::ResetValue for super::ENABLE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GPIO_ENABLE_DATA`"]
pub type GPIO_ENABLE_DATA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `GPIO_ENABLE_DATA`"]
pub struct GPIO_ENABLE_DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_ENABLE_DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - GPIO0~31 output enable"]
    #[inline(always)]
    pub fn gpio_enable_data(&self) -> GPIO_ENABLE_DATA_R {
        GPIO_ENABLE_DATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - GPIO0~31 output enable"]
    #[inline(always)]
    pub fn gpio_enable_data(&mut self) -> GPIO_ENABLE_DATA_W {
        GPIO_ENABLE_DATA_W { w: self }
    }
}