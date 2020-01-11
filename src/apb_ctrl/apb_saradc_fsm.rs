#[doc = "Reader of register APB_SARADC_FSM"]
pub type R = crate::R<u32, super::APB_SARADC_FSM>;
#[doc = "Writer for register APB_SARADC_FSM"]
pub type W = crate::W<u32, super::APB_SARADC_FSM>;
#[doc = "Register APB_SARADC_FSM `reset()`'s with value 0"]
impl crate::ResetValue for super::APB_SARADC_FSM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `APB_CTRL_SARADC_SAMPLE_CYCLE`"]
pub type APB_CTRL_SARADC_SAMPLE_CYCLE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `APB_CTRL_SARADC_SAMPLE_CYCLE`"]
pub struct APB_CTRL_SARADC_SAMPLE_CYCLE_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_CTRL_SARADC_SAMPLE_CYCLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `APB_CTRL_SARADC_START_WAIT`"]
pub type APB_CTRL_SARADC_START_WAIT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `APB_CTRL_SARADC_START_WAIT`"]
pub struct APB_CTRL_SARADC_START_WAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_CTRL_SARADC_START_WAIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `APB_CTRL_SARADC_STANDBY_WAIT`"]
pub type APB_CTRL_SARADC_STANDBY_WAIT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `APB_CTRL_SARADC_STANDBY_WAIT`"]
pub struct APB_CTRL_SARADC_STANDBY_WAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_CTRL_SARADC_STANDBY_WAIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `APB_CTRL_SARADC_RSTB_WAIT`"]
pub type APB_CTRL_SARADC_RSTB_WAIT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `APB_CTRL_SARADC_RSTB_WAIT`"]
pub struct APB_CTRL_SARADC_RSTB_WAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_CTRL_SARADC_RSTB_WAIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31 - sample cycles"]
    #[inline(always)]
    pub fn apb_ctrl_saradc_sample_cycle(&self) -> APB_CTRL_SARADC_SAMPLE_CYCLE_R {
        APB_CTRL_SARADC_SAMPLE_CYCLE_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn apb_ctrl_saradc_start_wait(&self) -> APB_CTRL_SARADC_START_WAIT_R {
        APB_CTRL_SARADC_START_WAIT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn apb_ctrl_saradc_standby_wait(&self) -> APB_CTRL_SARADC_STANDBY_WAIT_R {
        APB_CTRL_SARADC_STANDBY_WAIT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn apb_ctrl_saradc_rstb_wait(&self) -> APB_CTRL_SARADC_RSTB_WAIT_R {
        APB_CTRL_SARADC_RSTB_WAIT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - sample cycles"]
    #[inline(always)]
    pub fn apb_ctrl_saradc_sample_cycle(&mut self) -> APB_CTRL_SARADC_SAMPLE_CYCLE_W {
        APB_CTRL_SARADC_SAMPLE_CYCLE_W { w: self }
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn apb_ctrl_saradc_start_wait(&mut self) -> APB_CTRL_SARADC_START_WAIT_W {
        APB_CTRL_SARADC_START_WAIT_W { w: self }
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn apb_ctrl_saradc_standby_wait(&mut self) -> APB_CTRL_SARADC_STANDBY_WAIT_W {
        APB_CTRL_SARADC_STANDBY_WAIT_W { w: self }
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn apb_ctrl_saradc_rstb_wait(&mut self) -> APB_CTRL_SARADC_RSTB_WAIT_W {
        APB_CTRL_SARADC_RSTB_WAIT_W { w: self }
    }
}