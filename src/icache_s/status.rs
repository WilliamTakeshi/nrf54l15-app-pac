#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Ready status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ready {
    #[doc = "0: Activity is done and ready for the next activity."]
    Ready = 0,
    #[doc = "1: Activity is in progress."]
    Busy = 1,
}
impl From<Ready> for bool {
    #[inline(always)]
    fn from(variant: Ready) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `READY` reader - Ready status."]
pub type ReadyR = crate::BitReader<Ready>;
impl ReadyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ready {
        match self.bits {
            false => Ready::Ready,
            true => Ready::Busy,
        }
    }
    #[doc = "Activity is done and ready for the next activity."]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == Ready::Ready
    }
    #[doc = "Activity is in progress."]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == Ready::Busy
    }
}
impl R {
    #[doc = "Bit 0 - Ready status."]
    #[inline(always)]
    pub fn ready(&self) -> ReadyR {
        ReadyR::new((self.bits & 1) != 0)
    }
}
#[doc = "Status of the cache activities.\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {}
