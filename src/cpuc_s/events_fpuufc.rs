#[doc = "Register `EVENTS_FPUUFC` reader"]
pub type R = crate::R<EventsFpuufcSpec>;
#[doc = "Register `EVENTS_FPUUFC` writer"]
pub type W = crate::W<EventsFpuufcSpec>;
#[doc = "A floating-point underflow exception has occurred in the FPU.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsFpuufc {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsFpuufc> for bool {
    #[inline(always)]
    fn from(variant: EventsFpuufc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_FPUUFC` reader - A floating-point underflow exception has occurred in the FPU."]
pub type EventsFpuufcR = crate::BitReader<EventsFpuufc>;
impl EventsFpuufcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsFpuufc {
        match self.bits {
            false => EventsFpuufc::NotGenerated,
            true => EventsFpuufc::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsFpuufc::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsFpuufc::Generated
    }
}
#[doc = "Field `EVENTS_FPUUFC` writer - A floating-point underflow exception has occurred in the FPU."]
pub type EventsFpuufcW<'a, REG> = crate::BitWriter<'a, REG, EventsFpuufc>;
impl<'a, REG> EventsFpuufcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsFpuufc::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsFpuufc::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - A floating-point underflow exception has occurred in the FPU."]
    #[inline(always)]
    pub fn events_fpuufc(&self) -> EventsFpuufcR {
        EventsFpuufcR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - A floating-point underflow exception has occurred in the FPU."]
    #[inline(always)]
    pub fn events_fpuufc(&mut self) -> EventsFpuufcW<'_, EventsFpuufcSpec> {
        EventsFpuufcW::new(self, 0)
    }
}
#[doc = "A floating-point underflow exception has occurred in the FPU.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_fpuufc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_fpuufc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsFpuufcSpec;
impl crate::RegisterSpec for EventsFpuufcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_fpuufc::R`](R) reader structure"]
impl crate::Readable for EventsFpuufcSpec {}
#[doc = "`write(|w| ..)` method takes [`events_fpuufc::W`](W) writer structure"]
impl crate::Writable for EventsFpuufcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_FPUUFC to value 0"]
impl crate::Resettable for EventsFpuufcSpec {}
