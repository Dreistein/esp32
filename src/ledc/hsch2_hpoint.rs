#[doc = "Reader of register HSCH2_HPOINT"]
pub type R = crate::R<u32, super::HSCH2_HPOINT>;
#[doc = "Writer for register HSCH2_HPOINT"]
pub type W = crate::W<u32, super::HSCH2_HPOINT>;
#[doc = "Register HSCH2_HPOINT `reset()`'s with value 0"]
impl crate::ResetValue for super::HSCH2_HPOINT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LEDC_HPOINT_HSCH2`"]
pub type LEDC_HPOINT_HSCH2_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `LEDC_HPOINT_HSCH2`"]
pub struct LEDC_HPOINT_HSCH2_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDC_HPOINT_HSCH2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x000f_ffff) | ((value as u32) & 0x000f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:19 - The output value changes to high when htimerx(x=\\[0 3\\]) selected by high speed channel2 has reached reg_hpoint_hsch2\\[19:0\\]"]
    #[inline(always)]
    pub fn ledc_hpoint_hsch2(&self) -> LEDC_HPOINT_HSCH2_R {
        LEDC_HPOINT_HSCH2_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:19 - The output value changes to high when htimerx(x=\\[0 3\\]) selected by high speed channel2 has reached reg_hpoint_hsch2\\[19:0\\]"]
    #[inline(always)]
    pub fn ledc_hpoint_hsch2(&mut self) -> LEDC_HPOINT_HSCH2_W {
        LEDC_HPOINT_HSCH2_W { w: self }
    }
}