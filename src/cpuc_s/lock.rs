#[doc = "Register `LOCK` reader"]
pub type R = crate::R<LockSpec>;
#[doc = "Register `LOCK` writer"]
pub type W = crate::W<LockSpec>;
#[doc = "Locks both the Vector table Offset Register (VTOR) and Application Interrupt and Reset Control Register (AIRCR) for secure mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lockvtoraircrs {
    #[doc = "0: Both VTOR and AIRCR can be changed."]
    NotLocked = 0,
    #[doc = "1: Prevents changes to both VTOR and AIRCR."]
    Locked = 1,
}
impl From<Lockvtoraircrs> for bool {
    #[inline(always)]
    fn from(variant: Lockvtoraircrs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCKVTORAIRCRS` reader - Locks both the Vector table Offset Register (VTOR) and Application Interrupt and Reset Control Register (AIRCR) for secure mode."]
pub type LockvtoraircrsR = crate::BitReader<Lockvtoraircrs>;
impl LockvtoraircrsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lockvtoraircrs {
        match self.bits {
            false => Lockvtoraircrs::NotLocked,
            true => Lockvtoraircrs::Locked,
        }
    }
    #[doc = "Both VTOR and AIRCR can be changed."]
    #[inline(always)]
    pub fn is_not_locked(&self) -> bool {
        *self == Lockvtoraircrs::NotLocked
    }
    #[doc = "Prevents changes to both VTOR and AIRCR."]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == Lockvtoraircrs::Locked
    }
}
#[doc = "Field `LOCKVTORAIRCRS` writer - Locks both the Vector table Offset Register (VTOR) and Application Interrupt and Reset Control Register (AIRCR) for secure mode."]
pub type LockvtoraircrsW<'a, REG> = crate::BitWriter<'a, REG, Lockvtoraircrs>;
impl<'a, REG> LockvtoraircrsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Both VTOR and AIRCR can be changed."]
    #[inline(always)]
    pub fn not_locked(self) -> &'a mut crate::W<REG> {
        self.variant(Lockvtoraircrs::NotLocked)
    }
    #[doc = "Prevents changes to both VTOR and AIRCR."]
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(Lockvtoraircrs::Locked)
    }
}
#[doc = "Locks the Vector table Offset Register (VTOR) for non-secure mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lockvtorns {
    #[doc = "0: VTOR can be changed."]
    NotLocked = 0,
    #[doc = "1: Prevents changes to VTOR."]
    Locked = 1,
}
impl From<Lockvtorns> for bool {
    #[inline(always)]
    fn from(variant: Lockvtorns) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCKVTORNS` reader - Locks the Vector table Offset Register (VTOR) for non-secure mode."]
pub type LockvtornsR = crate::BitReader<Lockvtorns>;
impl LockvtornsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lockvtorns {
        match self.bits {
            false => Lockvtorns::NotLocked,
            true => Lockvtorns::Locked,
        }
    }
    #[doc = "VTOR can be changed."]
    #[inline(always)]
    pub fn is_not_locked(&self) -> bool {
        *self == Lockvtorns::NotLocked
    }
    #[doc = "Prevents changes to VTOR."]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == Lockvtorns::Locked
    }
}
#[doc = "Field `LOCKVTORNS` writer - Locks the Vector table Offset Register (VTOR) for non-secure mode."]
pub type LockvtornsW<'a, REG> = crate::BitWriter<'a, REG, Lockvtorns>;
impl<'a, REG> LockvtornsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "VTOR can be changed."]
    #[inline(always)]
    pub fn not_locked(self) -> &'a mut crate::W<REG> {
        self.variant(Lockvtorns::NotLocked)
    }
    #[doc = "Prevents changes to VTOR."]
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(Lockvtorns::Locked)
    }
}
#[doc = "Locks the Memory Protection Unit (MPU) for secure mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lockmpus {
    #[doc = "0: MPU registers can be changed."]
    NotLocked = 0,
    #[doc = "1: Prevents changes to MPU registers."]
    Locked = 1,
}
impl From<Lockmpus> for bool {
    #[inline(always)]
    fn from(variant: Lockmpus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCKMPUS` reader - Locks the Memory Protection Unit (MPU) for secure mode."]
pub type LockmpusR = crate::BitReader<Lockmpus>;
impl LockmpusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lockmpus {
        match self.bits {
            false => Lockmpus::NotLocked,
            true => Lockmpus::Locked,
        }
    }
    #[doc = "MPU registers can be changed."]
    #[inline(always)]
    pub fn is_not_locked(&self) -> bool {
        *self == Lockmpus::NotLocked
    }
    #[doc = "Prevents changes to MPU registers."]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == Lockmpus::Locked
    }
}
#[doc = "Field `LOCKMPUS` writer - Locks the Memory Protection Unit (MPU) for secure mode."]
pub type LockmpusW<'a, REG> = crate::BitWriter<'a, REG, Lockmpus>;
impl<'a, REG> LockmpusW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MPU registers can be changed."]
    #[inline(always)]
    pub fn not_locked(self) -> &'a mut crate::W<REG> {
        self.variant(Lockmpus::NotLocked)
    }
    #[doc = "Prevents changes to MPU registers."]
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(Lockmpus::Locked)
    }
}
#[doc = "Locks the Memory Protection Unit (MPU) for non secure mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lockmpuns {
    #[doc = "0: MPU registers can be changed."]
    NotLocked = 0,
    #[doc = "1: Prevents changes to MPU registers."]
    Locked = 1,
}
impl From<Lockmpuns> for bool {
    #[inline(always)]
    fn from(variant: Lockmpuns) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCKMPUNS` reader - Locks the Memory Protection Unit (MPU) for non secure mode."]
pub type LockmpunsR = crate::BitReader<Lockmpuns>;
impl LockmpunsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lockmpuns {
        match self.bits {
            false => Lockmpuns::NotLocked,
            true => Lockmpuns::Locked,
        }
    }
    #[doc = "MPU registers can be changed."]
    #[inline(always)]
    pub fn is_not_locked(&self) -> bool {
        *self == Lockmpuns::NotLocked
    }
    #[doc = "Prevents changes to MPU registers."]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == Lockmpuns::Locked
    }
}
#[doc = "Field `LOCKMPUNS` writer - Locks the Memory Protection Unit (MPU) for non secure mode."]
pub type LockmpunsW<'a, REG> = crate::BitWriter<'a, REG, Lockmpuns>;
impl<'a, REG> LockmpunsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MPU registers can be changed."]
    #[inline(always)]
    pub fn not_locked(self) -> &'a mut crate::W<REG> {
        self.variant(Lockmpuns::NotLocked)
    }
    #[doc = "Prevents changes to MPU registers."]
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(Lockmpuns::Locked)
    }
}
#[doc = "Locks the Security Attribution Unit (SAU)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Locksau {
    #[doc = "0: SAU registers can be changed."]
    NotLocked = 0,
    #[doc = "1: Prevents changes to SAU registers."]
    Locked = 1,
}
impl From<Locksau> for bool {
    #[inline(always)]
    fn from(variant: Locksau) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCKSAU` reader - Locks the Security Attribution Unit (SAU)"]
pub type LocksauR = crate::BitReader<Locksau>;
impl LocksauR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Locksau {
        match self.bits {
            false => Locksau::NotLocked,
            true => Locksau::Locked,
        }
    }
    #[doc = "SAU registers can be changed."]
    #[inline(always)]
    pub fn is_not_locked(&self) -> bool {
        *self == Locksau::NotLocked
    }
    #[doc = "Prevents changes to SAU registers."]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == Locksau::Locked
    }
}
#[doc = "Field `LOCKSAU` writer - Locks the Security Attribution Unit (SAU)"]
pub type LocksauW<'a, REG> = crate::BitWriter<'a, REG, Locksau>;
impl<'a, REG> LocksauW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SAU registers can be changed."]
    #[inline(always)]
    pub fn not_locked(self) -> &'a mut crate::W<REG> {
        self.variant(Locksau::NotLocked)
    }
    #[doc = "Prevents changes to SAU registers."]
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(Locksau::Locked)
    }
}
impl R {
    #[doc = "Bit 0 - Locks both the Vector table Offset Register (VTOR) and Application Interrupt and Reset Control Register (AIRCR) for secure mode."]
    #[inline(always)]
    pub fn lockvtoraircrs(&self) -> LockvtoraircrsR {
        LockvtoraircrsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Locks the Vector table Offset Register (VTOR) for non-secure mode."]
    #[inline(always)]
    pub fn lockvtorns(&self) -> LockvtornsR {
        LockvtornsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Locks the Memory Protection Unit (MPU) for secure mode."]
    #[inline(always)]
    pub fn lockmpus(&self) -> LockmpusR {
        LockmpusR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Locks the Memory Protection Unit (MPU) for non secure mode."]
    #[inline(always)]
    pub fn lockmpuns(&self) -> LockmpunsR {
        LockmpunsR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Locks the Security Attribution Unit (SAU)"]
    #[inline(always)]
    pub fn locksau(&self) -> LocksauR {
        LocksauR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Locks both the Vector table Offset Register (VTOR) and Application Interrupt and Reset Control Register (AIRCR) for secure mode."]
    #[inline(always)]
    pub fn lockvtoraircrs(&mut self) -> LockvtoraircrsW<'_, LockSpec> {
        LockvtoraircrsW::new(self, 0)
    }
    #[doc = "Bit 1 - Locks the Vector table Offset Register (VTOR) for non-secure mode."]
    #[inline(always)]
    pub fn lockvtorns(&mut self) -> LockvtornsW<'_, LockSpec> {
        LockvtornsW::new(self, 1)
    }
    #[doc = "Bit 2 - Locks the Memory Protection Unit (MPU) for secure mode."]
    #[inline(always)]
    pub fn lockmpus(&mut self) -> LockmpusW<'_, LockSpec> {
        LockmpusW::new(self, 2)
    }
    #[doc = "Bit 3 - Locks the Memory Protection Unit (MPU) for non secure mode."]
    #[inline(always)]
    pub fn lockmpuns(&mut self) -> LockmpunsW<'_, LockSpec> {
        LockmpunsW::new(self, 3)
    }
    #[doc = "Bit 4 - Locks the Security Attribution Unit (SAU)"]
    #[inline(always)]
    pub fn locksau(&mut self) -> LocksauW<'_, LockSpec> {
        LocksauW::new(self, 4)
    }
}
#[doc = "Register to lock the certain parts of the CPU from being modified.\n\nYou can [`read`](crate::Reg::read) this register and get [`lock::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lock::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LockSpec;
impl crate::RegisterSpec for LockSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lock::R`](R) reader structure"]
impl crate::Readable for LockSpec {}
#[doc = "`write(|w| ..)` method takes [`lock::W`](W) writer structure"]
impl crate::Writable for LockSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LOCK to value 0"]
impl crate::Resettable for LockSpec {}
