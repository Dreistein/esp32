#[doc = "Reader of register NEGPULSE"]
pub type R = crate::R<u32, super::NEGPULSE>;
#[doc = "Writer for register NEGPULSE"]
pub type W = crate::W<u32, super::NEGPULSE>;
#[doc = "Register NEGPULSE `reset()`'s with value 0"]
impl crate::ResetValue for super::NEGPULSE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UART_NEGEDGE_MIN_CNT`"]
pub type UART_NEGEDGE_MIN_CNT_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `UART_NEGEDGE_MIN_CNT`"]
pub struct UART_NEGEDGE_MIN_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_NEGEDGE_MIN_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x000f_ffff) | ((value as u32) & 0x000f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:19 - This register stores the count of rxd negedge edge. it is used in boudrate-detect process."]
    #[inline(always)]
    pub fn uart_negedge_min_cnt(&self) -> UART_NEGEDGE_MIN_CNT_R {
        UART_NEGEDGE_MIN_CNT_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:19 - This register stores the count of rxd negedge edge. it is used in boudrate-detect process."]
    #[inline(always)]
    pub fn uart_negedge_min_cnt(&mut self) -> UART_NEGEDGE_MIN_CNT_W {
        UART_NEGEDGE_MIN_CNT_W { w: self }
    }
}