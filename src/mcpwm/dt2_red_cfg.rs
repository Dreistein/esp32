#[doc = "Reader of register DT2_RED_CFG"]
pub type R = crate::R<u32, super::DT2_RED_CFG>;
#[doc = "Writer for register DT2_RED_CFG"]
pub type W = crate::W<u32, super::DT2_RED_CFG>;
#[doc = "Register DT2_RED_CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::DT2_RED_CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MCPWM_DT2_RED`"]
pub type MCPWM_DT2_RED_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MCPWM_DT2_RED`"]
pub struct MCPWM_DT2_RED_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_DT2_RED_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Shadow reg for RED"]
    #[inline(always)]
    pub fn mcpwm_dt2_red(&self) -> MCPWM_DT2_RED_R {
        MCPWM_DT2_RED_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Shadow reg for RED"]
    #[inline(always)]
    pub fn mcpwm_dt2_red(&mut self) -> MCPWM_DT2_RED_W {
        MCPWM_DT2_RED_W { w: self }
    }
}