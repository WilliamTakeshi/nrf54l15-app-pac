#[doc = "Register `TASKS_ERASE` writer"]
pub type W = crate::W<TasksEraseSpec>;
#[doc = "Erase the cache.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TasksErase {
    #[doc = "1: Trigger task"]
    Trigger = 1,
}
impl From<TasksErase> for bool {
    #[inline(always)]
    fn from(variant: TasksErase) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_ERASE` writer - Erase the cache."]
pub type TasksEraseW<'a, REG> = crate::BitWriter<'a, REG, TasksErase>;
impl<'a, REG> TasksEraseW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TasksErase::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Erase the cache."]
    #[inline(always)]
    pub fn tasks_erase(&mut self) -> TasksEraseW<'_, TasksEraseSpec> {
        TasksEraseW::new(self, 0)
    }
}
#[doc = "Erase the cache.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_erase::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksEraseSpec;
impl crate::RegisterSpec for TasksEraseSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_erase::W`](W) writer structure"]
impl crate::Writable for TasksEraseSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_ERASE to value 0"]
impl crate::Resettable for TasksEraseSpec {}
