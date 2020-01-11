#[doc = "Reader of register STATE1"]
pub type R = crate::R<u32, super::STATE1>;
#[doc = "Writer for register STATE1"]
pub type W = crate::W<u32, super::STATE1>;
#[doc = "Register STATE1 `reset()`'s with value 0"]
impl crate::ResetValue for super::STATE1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UHCI_STATE1`"]
pub type UHCI_STATE1_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `UHCI_STATE1`"]
pub struct UHCI_STATE1_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_STATE1_W<'a> {
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
    pub fn uhci_state1(&self) -> UHCI_STATE1_R {
        UHCI_STATE1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn uhci_state1(&mut self) -> UHCI_STATE1_W {
        UHCI_STATE1_W { w: self }
    }
}