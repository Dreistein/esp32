#[doc = "Reader of register SHROM_MPU_TABLE11"]
pub type R = crate::R<u32, super::SHROM_MPU_TABLE11>;
#[doc = "Writer for register SHROM_MPU_TABLE11"]
pub type W = crate::W<u32, super::SHROM_MPU_TABLE11>;
#[doc = "Register SHROM_MPU_TABLE11 `reset()`'s with value 0"]
impl crate::ResetValue for super::SHROM_MPU_TABLE11 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DPORT_SHROM_MPU_TABLE11`"]
pub type DPORT_SHROM_MPU_TABLE11_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DPORT_SHROM_MPU_TABLE11`"]
pub struct DPORT_SHROM_MPU_TABLE11_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_SHROM_MPU_TABLE11_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn dport_shrom_mpu_table11(&self) -> DPORT_SHROM_MPU_TABLE11_R {
        DPORT_SHROM_MPU_TABLE11_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn dport_shrom_mpu_table11(&mut self) -> DPORT_SHROM_MPU_TABLE11_W {
        DPORT_SHROM_MPU_TABLE11_W { w: self }
    }
}