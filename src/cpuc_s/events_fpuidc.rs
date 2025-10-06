#[doc = "Register `EVENTS_FPUIDC` reader"]
pub type R = crate::R<EventsFpuidcSpec>;
#[doc = "Register `EVENTS_FPUIDC` writer"]
pub type W = crate::W<EventsFpuidcSpec>;
#[doc = "A floating-point input denormal exception has occurred in the FPU.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsFpuidc {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsFpuidc> for bool {
    #[inline(always)]
    fn from(variant: EventsFpuidc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_FPUIDC` reader - A floating-point input denormal exception has occurred in the FPU."]
pub type EventsFpuidcR = crate::BitReader<EventsFpuidc>;
impl EventsFpuidcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsFpuidc {
        match self.bits {
            false => EventsFpuidc::NotGenerated,
            true => EventsFpuidc::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsFpuidc::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsFpuidc::Generated
    }
}
#[doc = "Field `EVENTS_FPUIDC` writer - A floating-point input denormal exception has occurred in the FPU."]
pub type EventsFpuidcW<'a, REG> = crate::BitWriter<'a, REG, EventsFpuidc>;
impl<'a, REG> EventsFpuidcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsFpuidc::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsFpuidc::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - A floating-point input denormal exception has occurred in the FPU."]
    #[inline(always)]
    pub fn events_fpuidc(&self) -> EventsFpuidcR {
        EventsFpuidcR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - A floating-point input denormal exception has occurred in the FPU."]
    #[inline(always)]
    pub fn events_fpuidc(&mut self) -> EventsFpuidcW<'_, EventsFpuidcSpec> {
        EventsFpuidcW::new(self, 0)
    }
}
#[doc = "A floating-point input denormal exception has occurred in the FPU.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_fpuidc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_fpuidc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsFpuidcSpec;
impl crate::RegisterSpec for EventsFpuidcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_fpuidc::R`](R) reader structure"]
impl crate::Readable for EventsFpuidcSpec {}
#[doc = "`write(|w| ..)` method takes [`events_fpuidc::W`](W) writer structure"]
impl crate::Writable for EventsFpuidcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_FPUIDC to value 0"]
impl crate::Resettable for EventsFpuidcSpec {}
