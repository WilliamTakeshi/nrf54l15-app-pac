#[doc = "Register `TASKS_INVALIDATELINE` writer"]
pub type W = crate::W<TasksInvalidatelineSpec>;
#[doc = "Invalidate the line.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TasksInvalidateline {
    #[doc = "1: Trigger task"]
    Trigger = 1,
}
impl From<TasksInvalidateline> for bool {
    #[inline(always)]
    fn from(variant: TasksInvalidateline) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_INVALIDATELINE` writer - Invalidate the line."]
pub type TasksInvalidatelineW<'a, REG> = crate::BitWriter<'a, REG, TasksInvalidateline>;
impl<'a, REG> TasksInvalidatelineW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TasksInvalidateline::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Invalidate the line."]
    #[inline(always)]
    pub fn tasks_invalidateline(&mut self) -> TasksInvalidatelineW<'_, TasksInvalidatelineSpec> {
        TasksInvalidatelineW::new(self, 0)
    }
}
#[doc = "Invalidate the line.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_invalidateline::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksInvalidatelineSpec;
impl crate::RegisterSpec for TasksInvalidatelineSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_invalidateline::W`](W) writer structure"]
impl crate::Writable for TasksInvalidatelineSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_INVALIDATELINE to value 0"]
impl crate::Resettable for TasksInvalidatelineSpec {}
