#[doc = "Register `EVENTS_FPUIXC` reader"]
pub type R = crate::R<EventsFpuixcSpec>;
#[doc = "Register `EVENTS_FPUIXC` writer"]
pub type W = crate::W<EventsFpuixcSpec>;
#[doc = "A floating-point inexact exception has occurred in the FPU.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsFpuixc {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsFpuixc> for bool {
    #[inline(always)]
    fn from(variant: EventsFpuixc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_FPUIXC` reader - A floating-point inexact exception has occurred in the FPU."]
pub type EventsFpuixcR = crate::BitReader<EventsFpuixc>;
impl EventsFpuixcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsFpuixc {
        match self.bits {
            false => EventsFpuixc::NotGenerated,
            true => EventsFpuixc::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsFpuixc::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsFpuixc::Generated
    }
}
#[doc = "Field `EVENTS_FPUIXC` writer - A floating-point inexact exception has occurred in the FPU."]
pub type EventsFpuixcW<'a, REG> = crate::BitWriter<'a, REG, EventsFpuixc>;
impl<'a, REG> EventsFpuixcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsFpuixc::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsFpuixc::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - A floating-point inexact exception has occurred in the FPU."]
    #[inline(always)]
    pub fn events_fpuixc(&self) -> EventsFpuixcR {
        EventsFpuixcR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - A floating-point inexact exception has occurred in the FPU."]
    #[inline(always)]
    pub fn events_fpuixc(&mut self) -> EventsFpuixcW<'_, EventsFpuixcSpec> {
        EventsFpuixcW::new(self, 0)
    }
}
#[doc = "A floating-point inexact exception has occurred in the FPU.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_fpuixc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_fpuixc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsFpuixcSpec;
impl crate::RegisterSpec for EventsFpuixcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_fpuixc::R`](R) reader structure"]
impl crate::Readable for EventsFpuixcSpec {}
#[doc = "`write(|w| ..)` method takes [`events_fpuixc::W`](W) writer structure"]
impl crate::Writable for EventsFpuixcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_FPUIXC to value 0"]
impl crate::Resettable for EventsFpuixcSpec {}
