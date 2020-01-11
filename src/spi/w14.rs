#[doc = "Reader of register W14"]
pub type R = crate::R<u32, super::W14>;
#[doc = "Writer for register W14"]
pub type W = crate::W<u32, super::W14>;
#[doc = "Register W14 `reset()`'s with value 0"]
impl crate::ResetValue for super::W14 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SPI_BUF14`"]
pub type SPI_BUF14_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SPI_BUF14`"]
pub struct SPI_BUF14_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_BUF14_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - data buffer"]
    #[inline(always)]
    pub fn spi_buf14(&self) -> SPI_BUF14_R {
        SPI_BUF14_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - data buffer"]
    #[inline(always)]
    pub fn spi_buf14(&mut self) -> SPI_BUF14_W {
        SPI_BUF14_W { w: self }
    }
}